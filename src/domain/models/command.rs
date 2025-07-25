use serde::Serialize;

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