use clap::Parser;
use anyhow::{Result, anyhow};

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
        Commands::Exec { device, command, value} => {
            println!("{device:?} {command:?} {value:?}")
        }
    }

    Ok(())

    /* 
    let command = match args.command.as_str() {
        "turn_on" => Command::TurnOn,
        "turn_off" => Command::TurnOff,
        "set_brightness" => {
            let level: u8 = args.value
                .as_ref()
                .ok_or_else(|| anyhow!("--value is required for set_brightness"))?
                .parse()?;
            Command::SetBrightness(level)
        }
        other => Command::Custom {
            name: other.to_string(),
            params: args.value
                .as_ref()
                .map(|v| serde_json::from_str(v))
                .transpose()?,
        },
    };
    
    use_case.execute(DeviceId(args.device), command).await */

    //Ok(())
}