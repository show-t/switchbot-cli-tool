use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DeviceId(pub String);

#[derive(Debug, Clone)]
pub enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(u8),
    SetColor(String),
    Custom {
        name: String,
        params: Option<serde_json::Value>
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub device_type: String,
    pub is_infrared: bool,
    pub hub_device_id: String,
}

