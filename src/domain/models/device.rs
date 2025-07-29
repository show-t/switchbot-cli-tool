use serde::Serialize;

use super::value_objects::DeviceId;

#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub device_type: String,
    pub is_infrared: bool,
    pub hub_device_id: String,
}
