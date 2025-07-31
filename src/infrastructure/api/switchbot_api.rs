use std::fmt::format;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Result, bail};
use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose;
use hmac::{Hmac, Mac};
use rand::Rng;
use rand::distributions::Alphanumeric;
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;

mod dto;
use dto::{CommandResponse, DeviceListResponse};

use crate::domain::models::entities::Device;
use crate::domain::models::value_objects::Command;
use crate::domain::models::value_objects::DeviceId;
use crate::domain::repositories::IDeviceRepository;
use crate::infrastructure::api::switchbot_api::dto::CommandRequestBody;

#[derive(Debug)]
pub struct SwitchBotApi {
    pub host: String,
    pub token: String,
    pub secret: String,
    pub client: Client,
}

impl SwitchBotApi {
    pub fn new(host: String, token: String, secret: String) -> Self {
        Self {
            host,
            token,
            secret,
            client: Client::new(),
        }
    }

    fn generate_signature(&self) -> Result<(String, String, String)> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();

        let nonce: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let payload = format!("{}{}{}", self.token, &timestamp, &nonce);

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())?;
        mac.update(payload.as_bytes());

        let result = mac.finalize().into_bytes();
        let sign = general_purpose::STANDARD.encode(result);

        Ok((timestamp, nonce, sign))
    }

    fn auth_headers(&self) -> Result<reqwest::header::HeaderMap> {
        use reqwest::header::AUTHORIZATION;
        use reqwest::header::CONTENT_TYPE;
        use reqwest::header::HeaderMap;
        use reqwest::header::HeaderValue;

        let (timestamp, nonce, sign) = self.generate_signature()?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.token)?);
        headers.insert("sign", HeaderValue::from_str(&sign)?);
        headers.insert("t", HeaderValue::from_str(&timestamp)?);
        headers.insert("nonce", HeaderValue::from_str(&nonce)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        Ok(headers)
    }

    fn to_device_list(&self, res: DeviceListResponse) -> Vec<Device> {
        let mut devices: Vec<Device> = res.body.device_list.into_iter().map(Device::from).collect();

        if let Some(infrared_list) = res.body.infrared_remote_list {
            let mut ir_devices = infrared_list.into_iter().map(Device::from).collect();
            devices.append(&mut ir_devices);
        }

        devices
    }
}

#[async_trait]
impl IDeviceRepository for SwitchBotApi {
    async fn get_device(&self, id: &DeviceId) -> Result<Device> {
        // 仮実装
        Ok(Device {
            id: id.clone(),
            name: "dummy".to_string(),
            device_type: "Bot".to_string(),
            is_infrared: false,
            hub_device_id: "hoge".to_string(),
        })
    }

    async fn send_command(&self, id: &DeviceId, command: &Command) -> Result<()> {
        let url = self.host.clone() + "/devices/" + &id.value()?.to_string() + "/commands";
        tracing::debug!("{:?}", url);

        let body = match command {
            Command::TurnOn => CommandRequestBody {
                command_type: "command".into(),
                command: "turnOn".into(),
                parameter: "default".into(),
            },
            Command::TurnOff => CommandRequestBody {
                command_type: "command".into(),
                command: "turnOff".into(),
                parameter: "default".into(),
            },
            Command::SetBrightness(value) => CommandRequestBody {
                command_type: "command".into(),
                command: "setBrightness".into(),
                parameter: Value::Number(value.get().into()),
            },
            Command::SetColor(values) => {
                let (r, g, b) = values.get();
                CommandRequestBody {
                    command_type: "command".into(),
                    command: "setColor".into(),
                    parameter: Value::String(format!("{}:{}:{}", r, g, b)),
                }
            }
            Command::SetColorTemperature(value) => CommandRequestBody {
                command_type: "command".into(),
                command: "setColorTemperature".into(),
                parameter: Value::Number(value.get().into()),
            },
            Command::AcSetAll(values) => CommandRequestBody {
                command_type: "command".into(),
                command: "setAll".into(),
                parameter: Value::String(format!(
                    "{},{},{},{}", 
                    values.temperature,
                    values.mode as u8,
                    values.fan_speed as u8,
                    values.power_state.to_string(),
                )),
            },
            Command::Custom { command_type, command, parameter} => CommandRequestBody { 
                command_type: command_type.clone(), 
                command: command.clone(),
                parameter: Value::String(parameter.clone())
            },
        };

        let req = self
            .client
            .post(&url)
            .headers(self.auth_headers()?)
            .json(&body);

        let res = req.send().await?;

        if !res.status().is_success() {
            bail!("API Error: {}", res.status())
        }

        let res: CommandResponse = res.json().await?;
        tracing::debug!("{res:?}");
        Ok(())
    }

    async fn get_device_list(&self) -> Result<Vec<Device>> {
        let url = self.host.clone() + "/devices";
        tracing::debug!("{:?}", url);

        let req = self
            .client
            .get(url)
            .headers(self.auth_headers()?);

        let res = req.send().await?;

        if !res.status().is_success() {
            bail!("Request failed with status: {}", res.status())
        }

        let res: DeviceListResponse = res.json().await?;
        let devices = self.to_device_list(res);

        Ok(devices)
    }
}
