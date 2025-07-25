use anyhow::{Result, bail};
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use rand::{Rng, distributions::Alphanumeric};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::models::Device;
use crate::domain::models::value_objects::{DeviceId, Command};
use crate::domain::repositories::DeviceRepository;

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
        use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceListResponse {
    _status_code: i32,
    _message: String,
    body: DeviceListBody,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceListBody {
    device_list: Vec<DeviceDto>,
    infrared_remote_list: Option<Vec<IrRemoteDto>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceDto {
    device_id: String,
    device_name: String,
    device_type: String,
    hub_device_id: String,
}

impl From<DeviceDto> for Device {
    fn from(dto: DeviceDto) -> Self {
        Device {
            id: DeviceId(dto.device_id),
            name: dto.device_name,
            device_type: dto.device_type,
            is_infrared: false,
            hub_device_id: dto.hub_device_id,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IrRemoteDto {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}
impl From<IrRemoteDto> for Device {
    fn from(dto: IrRemoteDto) -> Self {
        Device {
            id: DeviceId(dto.device_id),
            name: dto.device_name,
            device_type: dto.remote_type,
            is_infrared: true,
            hub_device_id: dto.hub_device_id,
        }
    }
}

#[derive(Serialize)]
struct CommandBody {
    command: String,
    parameter: String,
    command_type: String,
}

#[async_trait]
impl DeviceRepository for SwitchBotApi {
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
        let url = self.host.clone() + "/devices/" + &id.0.to_string() + "/commands";

        let (command_str, parameter, parameter_type) = match command {
            Command::TurnOn => ("turnOn".into(), "default".into(), "command".into()),
            Command::TurnOff => ("turnOff".into(), "default".into(), "command".into()),
            Command::SetBrightness(level) => {
                ("setBrightness".into(), level.to_string(), "command".into())
            }
            Command::SetColor(color) => ("setColor".into(), color.clone(), "command".into()),
            Command::Custom { name, params } => {
                let param = params
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "default".to_string());
                (name.clone(), param, "command".to_string())
            }
        };

        let body = CommandBody {
            command: command_str,
            parameter,
            command_type: parameter_type,
        };

        let res = self.client
            .post(&url)
            .headers(self.auth_headers()?)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            bail!("API Error: {}", res.status())
        }

        Ok(())
    }

    async fn get_device_list(&self) -> Result<Vec<Device>> {
        let url = self.host.clone() + "/devices";
        let client = reqwest::Client::new();
        let res = client.get(url).headers(self.auth_headers()?).send().await?;

        if !res.status().is_success() {
            bail!("Request failed with status: {}", res.status())
        }

        let parsed: DeviceListResponse = res.json().await?;
        let devices = self.to_device_list(parsed);

        Ok(devices)
    }
}
