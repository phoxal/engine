pub const SCHEMA_NAME: &str = "phoxal-runtime-asset-api/v1";
pub const SCHEMA_VERSION: u32 = 1;

use phoxal_bus::zenoh_typed::TypedSchema;
use serde::{Deserialize, Serialize};

pub const GET_TOPIC: &str = "runtime/asset/get";
pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 40,
        cpu_sustained_pct: 2,
        gpu_memory_mb: None,
    };

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetRequest {
    pub path: String,
}

impl GetRequest {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl TypedSchema for GetRequest {
    const SCHEMA_NAME: &'static str = "runtime/asset/get/request";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetResponse {
    Ok { bytes: Vec<u8> },
    NotFound,
    InvalidPath(InvalidPathReason),
    Unavailable(UnavailableReason),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidPathReason {
    Empty,
    ParentTraversal,
    BackslashSeparator,
    EmptyComponent,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnavailableReason {
    Io,
}

impl TypedSchema for GetResponse {
    const SCHEMA_NAME: &'static str = "runtime/asset/get/response";
    const SCHEMA_VERSION: u32 = 1;
}

pub fn topic(bus: &phoxal_bus::Bus) -> String {
    bus.topic(GET_TOPIC)
}

pub fn get_builder<'a>(
    bus: &'a phoxal_bus::Bus,
    request: &'a GetRequest,
) -> phoxal_bus::zenoh_typed::TypedGetBuilder<'a, 'static, GetResponse> {
    phoxal_bus::query::get_builder(bus, GET_TOPIC, request)
}

pub fn queryable_builder(
    bus: &phoxal_bus::Bus,
) -> phoxal_bus::Result<
    phoxal_bus::zenoh_typed::TypedQueryableBuilder<'_, 'static, GetRequest, GetResponse>,
> {
    phoxal_bus::query::queryable_builder(bus, GET_TOPIC)
}

pub mod get {
    use super::{GET_TOPIC, GetRequest, GetResponse};

    pub const TOPIC: &str = GET_TOPIC;

    pub fn topic(bus: &phoxal_bus::Bus) -> String {
        bus.topic(TOPIC)
    }

    pub async fn query(
        bus: &phoxal_bus::Bus,
        request: &GetRequest,
        retry: &phoxal_bus::query::Retry,
    ) -> phoxal_bus::Result<Option<GetResponse>> {
        phoxal_bus::query::query(bus, TOPIC, request, retry).await
    }
}

#[cfg(test)]
mod tests {
    use super::{GetRequest, GetResponse};
    use phoxal_bus::zenoh_typed::TypedSchema;

    #[test]
    fn get_request_schema_contract_is_stable() {
        assert_eq!(GetRequest::SCHEMA_NAME, "runtime/asset/get/request");
        assert_eq!(GetRequest::SCHEMA_VERSION, 1);
    }

    #[test]
    fn get_response_schema_contract_is_stable() {
        assert_eq!(GetResponse::SCHEMA_NAME, "runtime/asset/get/response");
        assert_eq!(GetResponse::SCHEMA_VERSION, 1);
    }
}

#[cfg(test)]
mod v1_version_tests {
    use super::{SCHEMA_NAME, SCHEMA_VERSION};

    #[test]
    fn api_contract_version_is_stable() {
        assert_eq!(SCHEMA_NAME, "phoxal-runtime-asset-api/v1");
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
