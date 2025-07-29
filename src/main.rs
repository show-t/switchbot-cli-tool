use anyhow::Result;
use switchbot_cli_tool::application::ControlDeviceUseCase;
use switchbot_cli_tool::infrastructure::api::SwitchBotApi;
use switchbot_cli_tool::presentation::cli;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let api = SwitchBotApi::new(config.host, config.token, config.secret);
    let use_case = ControlDeviceUseCase::new(&api);

    cli::dispatch(&use_case).await
}
