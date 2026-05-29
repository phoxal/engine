mod runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_core_engine::execute::<runtime::JointRuntime>().await
}
