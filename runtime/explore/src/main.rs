mod frontiers;
mod runtime;
mod scenarios;
mod scoring;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_core_engine::execute::<runtime::ExploreRuntime>().await
}
