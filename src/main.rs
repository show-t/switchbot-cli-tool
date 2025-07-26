use clap::Parser;
use anyhow::Result;

use switchbot_cli_tool::domain::models::value_objects::Command;
use switchbot_cli_tool::application::dto::ExecuteCommandDto;
use switchbot_cli_tool::presentation::cli::{Cli, Commands};
use switchbot_cli_tool::infrastructure::api::SwitchBotApi;
use switchbot_cli_tool::application::ControlDeviceUseCase;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let api = SwitchBotApi::new(config.host, config.token, config.secret);
    let use_case = ControlDeviceUseCase::new(&api);

    let cli = Cli::parse();
    match cli.command {
        Commands::DeviceList => {
            let devices = use_case.fetch_devices().await?;
            devices.into_iter().for_each(|v| println!("{v:?}"));
        }
        Commands::Exec { device, command, values } => {
            println!("{device:?} {command:?} {values:?}");

            let device_id = "01-202507021553-77385742".into();
            let command = match command.as_str() {
                "turn_on" => Command::TurnOn,
                "turn_off" => Command::TurnOff,
                other => Command::Custom {
                    name: other.to_string(),
                    params: values.unwrap_or_default().into_iter()
                        .map(|s| serde_json::from_str(&s).ok())
                        .collect()
                },
            }; 
            let _ = use_case.execute(ExecuteCommandDto::new(
                device_id,
                command
            )).await?;
        }
    }

    Ok(())

}