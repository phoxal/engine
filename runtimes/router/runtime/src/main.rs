use anyhow::Result;
use clap::Parser;
use phoxal_bus::builder::{endpoints_json, insert_router_config};
use phoxal_helpers::init_tracing;
use tracing::info;

const DEFAULT_LISTEN_ENDPOINT: &str = "tcp/0.0.0.0:7447";
const ENV_UPSTREAM_ROUTERS: &str = "UPSTREAM_ROUTERS";

#[derive(Debug, Parser)]
#[command(
    name = "phoxal-runtime-router",
    about = "Per-robot Zenoh router runtime."
)]
struct Args {
    #[arg(long = "listen-endpoint", default_value = DEFAULT_LISTEN_ENDPOINT)]
    listen_endpoints: Vec<String>,

    #[arg(
        long = "upstream-router",
        env = ENV_UPSTREAM_ROUTERS,
        value_delimiter = ','
    )]
    upstream_routers: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;
    let args = Args::parse();

    let mut config = zenoh::Config::default();
    insert_router_config(&mut config, "mode", "\"router\"")?;
    insert_router_config(
        &mut config,
        "listen/endpoints",
        endpoints_json(
            args.listen_endpoints
                .iter()
                .map(|endpoint| endpoint.as_str()),
        )?,
    )?;
    if !args.upstream_routers.is_empty() {
        insert_router_config(
            &mut config,
            "connect/endpoints",
            endpoints_json(
                args.upstream_routers
                    .iter()
                    .map(|endpoint| endpoint.as_str()),
            )?,
        )?;
    }
    insert_router_config(&mut config, "scouting/multicast/enabled", "false")?;

    let _session = zenoh::open(config)
        .await
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    info!(
        listen_endpoints = ?args.listen_endpoints,
        upstream_routers = ?args.upstream_routers,
        "Robot router ready"
    );

    tokio::signal::ctrl_c().await?;
    Ok(())
}
