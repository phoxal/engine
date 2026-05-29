mod core;
mod runtime;
mod scenarios;
mod selector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_core_engine::execute::<runtime::MapRuntime>().await
}
