use std::path::{Path, PathBuf};

use anyhow::Result;
use phoxal_engine::clock::Step;
use phoxal_engine::step::{Io, RequestResponder, Runtime, RuntimeInputs};
use phoxal_engine::{EmptyArgs, RobotRuntimeArgs};
use phoxal_runtime_asset_api::v1::{
    GetRequest as AssetRequest, GetResponse as AssetResponse, InvalidPathReason, UnavailableReason,
};

pub enum Input {
    Get {
        request: AssetRequest,
        responder: RequestResponder<AssetRequest, AssetResponse>,
    },
}

pub struct AssetRuntime {
    bundle_root: PathBuf,
}

impl AssetRuntime {
    #[cfg(test)]
    fn for_test(bundle_root: PathBuf) -> Self {
        Self { bundle_root }
    }

    fn resolve(&self, request: &AssetRequest) -> AssetResponse {
        let requested = request.path.trim().trim_start_matches('/').to_string();
        match Self::validate_requested_path(&requested) {
            Ok(()) => self.read_asset(&requested),
            Err(reason) => AssetResponse::InvalidPath(reason),
        }
    }

    fn validate_requested_path(requested: &str) -> std::result::Result<(), InvalidPathReason> {
        if requested.is_empty() {
            return Err(InvalidPathReason::Empty);
        }
        if requested.contains('\\') {
            return Err(InvalidPathReason::BackslashSeparator);
        }
        if requested.split('/').any(|segment| segment == "..") {
            return Err(InvalidPathReason::ParentTraversal);
        }
        if requested.split('/').any(|segment| segment.is_empty()) {
            return Err(InvalidPathReason::EmptyComponent);
        }
        Ok(())
    }

    fn read_asset(&self, requested: &str) -> AssetResponse {
        let bundle_path = self.bundle_root.join(requested);
        if bundle_path.is_file() {
            return Self::read_asset_bytes(&bundle_path);
        }
        AssetResponse::NotFound
    }

    fn read_asset_bytes(asset_path: &Path) -> AssetResponse {
        match std::fs::read(asset_path) {
            Ok(bytes) => AssetResponse::Ok { bytes },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => AssetResponse::NotFound,
            Err(_) => AssetResponse::Unavailable(UnavailableReason::Io),
        }
    }
}

#[async_trait::async_trait]
impl Runtime for AssetRuntime {
    const RUNTIME_ID: &'static str = "asset";

    type Args = EmptyArgs;
    type Config = PathBuf;
    type Input = Input;

    fn config(_args: &Self::Args, common: &RobotRuntimeArgs) -> Result<Self::Config> {
        Ok(common.robot_config.clone())
    }

    fn clock_period(_config: &Self::Config) -> std::time::Duration {
        std::time::Duration::from_millis(20)
    }

    async fn new(io: &mut Io<Self::Input>, bundle_root: Self::Config) -> Result<Self> {
        io.serve_request::<AssetRequest, AssetResponse, _>(
            phoxal_runtime_asset_api::v1::get::TOPIC,
            |request, responder| Input::Get { request, responder },
        )
        .await?;
        Ok(Self { bundle_root })
    }

    async fn step(&mut self, _step: Step, inputs: RuntimeInputs<Self::Input>) -> Result<()> {
        for input in inputs {
            match input {
                Input::Get { request, responder } => {
                    responder.reply(&self.resolve(&request)).await?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::AssetRuntime;
    use anyhow::Result;
    use phoxal_engine::MESHES_DIR;
    use phoxal_runtime_asset_api::v1::{
        GetRequest as Request, GetResponse as Response, InvalidPathReason,
    };
    use std::fs;

    #[test]
    fn resolve_existing_asset_returns_bytes() -> Result<()> {
        let bundle_root =
            std::env::temp_dir().join(format!("phoxal-runtime-asset-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(bundle_root.join(MESHES_DIR))?;
        fs::write(bundle_root.join(format!("{MESHES_DIR}/test.bin")), b"robot")?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new(format!("{MESHES_DIR}/test.bin")));

        assert!(matches!(response, Response::Ok { bytes } if bytes == b"robot"));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }

    #[test]
    fn resolve_missing_asset_returns_not_found() -> Result<()> {
        let bundle_root = std::env::temp_dir().join(format!(
            "phoxal-runtime-asset-test-missing-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(&bundle_root)?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new("meshes/missing.bin"));

        assert!(matches!(response, Response::NotFound));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }

    #[test]
    fn resolve_parent_traversal_path_returns_typed_reason() -> Result<()> {
        let bundle_root = std::env::temp_dir().join(format!(
            "phoxal-runtime-asset-test-traversal-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(&bundle_root)?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new("../secret.bin"));

        assert!(matches!(
            response,
            Response::InvalidPath(InvalidPathReason::ParentTraversal)
        ));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }

    #[test]
    fn resolve_empty_path_returns_typed_reason() -> Result<()> {
        let bundle_root = std::env::temp_dir().join(format!(
            "phoxal-runtime-asset-test-empty-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(&bundle_root)?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new(""));

        assert!(matches!(
            response,
            Response::InvalidPath(InvalidPathReason::Empty)
        ));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }

    #[test]
    fn resolve_backslash_path_returns_typed_reason() -> Result<()> {
        let bundle_root = std::env::temp_dir().join(format!(
            "phoxal-runtime-asset-test-backslash-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(&bundle_root)?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new("meshes\\foo.bin"));

        assert!(matches!(
            response,
            Response::InvalidPath(InvalidPathReason::BackslashSeparator)
        ));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }

    #[test]
    fn resolve_empty_component_path_returns_typed_reason() -> Result<()> {
        let bundle_root = std::env::temp_dir().join(format!(
            "phoxal-runtime-asset-test-empty-comp-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&bundle_root);
        fs::create_dir_all(&bundle_root)?;

        let runtime = AssetRuntime::for_test(bundle_root.clone());

        let response = runtime.resolve(&Request::new("meshes//foo.bin"));

        assert!(matches!(
            response,
            Response::InvalidPath(InvalidPathReason::EmptyComponent)
        ));

        fs::remove_dir_all(&bundle_root)?;
        Ok(())
    }
}
