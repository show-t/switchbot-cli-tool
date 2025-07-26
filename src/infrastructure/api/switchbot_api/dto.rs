use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::models::Device;
use crate::domain::models::value_objects::DeviceId;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SwitchbotApiResponse<T> {
    pub(super) status_code: i32,
    pub(super) message: String,
    pub(super) body: T,
}

pub(super) type DeviceListResponse = SwitchbotApiResponse<DeviceListResponseBody>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeviceListResponseBody {
    pub(super) device_list: Vec<DeviceDto>,
    pub(super) infrared_remote_list: Option<Vec<IrRemoteDto>>,
} 

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeviceDto {
    device_id: String,
    device_name: String,
    device_type: String,
    hub_device_id: String,
}
impl From<DeviceDto> for Device {
    fn from(dto: DeviceDto) -> Self {
        Device {
            id: DeviceId::new(dto.device_id),
            name: dto.device_name,
            device_type: dto.device_type,
            is_infrared: false,
            hub_device_id: dto.hub_device_id,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IrRemoteDto {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}
impl From<IrRemoteDto> for Device {
    fn from(dto: IrRemoteDto) -> Self {
        Device {
            id: DeviceId::new(dto.device_id),
            name: dto.device_name,
            device_type: dto.remote_type,
            is_infrared: true,
            hub_device_id: dto.hub_device_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommandRequestBody {
    pub(super) command_type: String,
    pub(super) command: String,
    //pub(super) parameter: Option<Value>,
    pub(super) parameter: String,
}

pub(super) type CommandResponse = SwitchbotApiResponse<CommandResponseBody>;
pub(super) type CommandResponseBody = Value;
