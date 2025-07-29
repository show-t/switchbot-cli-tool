use anyhow::{Result, anyhow};
use clap::Parser;

use crate::application::ControlDeviceUseCase;
use crate::application::dto::ExecuteCommandDto;
use crate::domain::models::value_objects::{
    BrightnessValue,
    ColorTemperatureValue,
    ColorValues,
    Command,
};
use crate::domain::repositories::IDeviceRepository;
use crate::presentation::cli::{Args, Commands};

pub async fn dispatch<'a, R: IDeviceRepository>(
    use_case: &'a ControlDeviceUseCase<'a, R>,
) -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::DeviceList => {
            let devices = use_case.fetch_devices().await?;
            devices.into_iter().for_each(|v| println!("{v:?}"));
        }
        Commands::Exec {
            device,
            command,
            values,
        } => {
            println!("{device:?} {command:?} {values:?}");

            let device_id = device;
            let command = match command.as_str() {
                "on" => Command::TurnOn,
                "off" => Command::TurnOff,
                "brightness" => Command::SetBrightness(BrightnessValue::try_from(
                    values
                        .as_ref()
                        .and_then(|v| v.get(0))
                        .ok_or_else(|| anyhow!("value does not exist"))?
                        .parse::<u8>()?,
                )?),
                "color" => {
                    let [r, g, b]: [u8; 3] = values
                        .ok_or_else(|| anyhow!("No values"))?
                        .iter()
                        .map(|s| s.parse::<u8>().map_err(|e| anyhow!(e)))
                        .collect::<Result<Vec<u8>>>()?
                        .as_slice()
                        .try_into()?;

                    Command::SetColor(ColorValues::try_from((r, g, b))?)
                }
                "color_temp" => Command::SetColorTemperature(ColorTemperatureValue::try_from(
                    values
                        .as_ref()
                        .and_then(|v| v.get(0))
                        .ok_or_else(|| anyhow!("value does not exist"))?
                        .parse::<u16>()?,
                )?),
                other => Command::Custom {
                    name: other.to_string(),
                    params: values
                        .unwrap_or_default()
                        .into_iter()
                        .map(|s| serde_json::from_str(&s).ok())
                        .collect(),
                },
            };

            let _ = use_case
                .execute(ExecuteCommandDto::new(device_id, command))
                .await?;
        }
    }

    Ok(())
}
