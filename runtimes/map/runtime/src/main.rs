mod core;
mod runtime;
mod scenarios;
mod selector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_engine::execute::<runtime::MapRuntime>().await
}
