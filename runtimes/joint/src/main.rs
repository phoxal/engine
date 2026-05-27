mod runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_engine::execute::<runtime::JointRuntime>().await
}
