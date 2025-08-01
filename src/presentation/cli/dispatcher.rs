use anyhow::{Result, anyhow};
use clap::Parser;
use tracing::{self, instrument};

use crate::application::adapter::alias::AliasResolver;
use crate::application::services::{ControlDeviceUseCase, IControlDeviceUseCase};
use crate::application::dto::ExecuteCommandDto;
use crate::domain::models::value_objects::{
    BrightnessValue, ColorTemperatureValue, ColorValues, Command
};
use crate::domain::models::value_objects::{
    AcPowerState, AcValues, AcMode, AcFanSpeed,
};
use crate::domain::repositories::IDeviceRepository;
use crate::presentation::cli::{Args, Commands};
pub struct Dispatcher<'a> {
    use_case: &'a dyn IControlDeviceUseCase,
    resolver: &'a AliasResolver,
}

impl<'a> Dispatcher<'a> {
    pub fn new(
        use_case: &'a dyn IControlDeviceUseCase, 
        resolver: &'a AliasResolver 
    ) -> Self {
        Self {
            use_case,
            resolver
        }
    }

    pub async fn dispatch(&self) -> Result<()> {
        let args: Args = Args::try_parse().map_err(|e| {
            tracing::error!("{e}");
            std::process::exit(e.use_stderr() as i32);
        })?;
    
        match args.command {
            Commands::DeviceList => {
                let devices = self.use_case.fetch_devices().await?;
                devices.into_iter().for_each(|v| println!("{v:?}"));
            }
            Commands::Exec {
                device,
                command,
                values,
                customize,
            } => {
                tracing::debug!("{device:?} {command:?} {values:?}");

                let device_id = self.resolver.resolve(device.as_str()).to_string();
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
                    "ac" => {
                        let values:[String; 4] = values
                            .ok_or_else(|| anyhow!("value does not exist"))?
                            .try_into()
                            .map_err(|_| anyhow!("invalid number of elements."))?;

                        let temperature = values[0].parse::<u8>()?;
                        let mode = values[1].to_lowercase().parse::<AcMode>()?;
                        let fan_speed = values[2].to_lowercase().parse::<AcFanSpeed>()?;
                        let power_state = values[3].to_lowercase().parse::<AcPowerState>()?;

                        let values = AcValues{temperature, mode, fan_speed, power_state};
                        Command::AcSetAll(values)
                    }
                    other => Command::Custom {
                        command_type: if customize {"customize".into()} else {"command".into()},
                        command,
                        parameter: "default".into(),
                    },
                };

                let _ = self.use_case
                    .execute(ExecuteCommandDto::new(device_id, command))
                    .await?;
            }
        }

        Ok(())
    }
}


