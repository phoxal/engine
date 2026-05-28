use phoxal_runtime_localize::runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_engine::execute::<runtime::LocalizeRuntime>().await
}
