mod runtime;
mod scenarios;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_engine::execute::<runtime::VideoRuntime>().await
}
