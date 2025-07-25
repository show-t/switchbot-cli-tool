use crate::domain::models::Device;
use crate::domain::models::value_objects::Command;

use anyhow::Result;

#[derive(Debug)]
pub struct ExecuteCommandDto {
    pub device_id: String,
    pub command: Command,
}

#[derive(Debug)]
pub struct DeviceResponseDto {
    pub device_id: Result<String>,
    pub device_name: String,
    pub device_type: String, 
}

impl From<Device> for DeviceResponseDto {
    fn from(device: Device) -> Self {
        Self {
            device_id: device.id.value(),
            device_name: device.name,
            device_type: device.device_type,
        }
    }
}