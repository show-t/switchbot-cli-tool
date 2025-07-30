use anyhow::Result;
use dotenvy;
use tracing;
use tracing_subscriber::{self, EnvFilter};
use tracing_subscriber::fmt::time::LocalTime;

use switchbot_cli_tool::application::adapter::alias::AliasResolver;
use switchbot_cli_tool::application::ControlDeviceUseCase;
use switchbot_cli_tool::infrastructure::api::SwitchBotApi;
use switchbot_cli_tool::infrastructure::io::JsonAliasLoader;
use switchbot_cli_tool::presentation::cli;

mod config;
use config::Config;

fn tracing_init() -> Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    _ = tracing_init()?;

    let config = Config::from_env()?;

    let api = SwitchBotApi::new(config.host, config.token, config.secret);
    let use_case = ControlDeviceUseCase::new(&api);
    let resolver = AliasResolver::new(JsonAliasLoader::load("device_aliases.json")?.0);

    tracing::info!("Process Start");
    let dispatcher = cli::Dispatcher::new(
        &use_case,
        &resolver
    );
    _ = dispatcher.dispatch(&use_case).await;
    tracing::info!("Process End");

    Ok(())
}
