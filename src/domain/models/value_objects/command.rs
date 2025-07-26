use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Command {
    TurnOn,
    TurnOff,
    // SetBrightness(u8),
    // SetColor(u8, u8, u8),
    Custom {
        name: String,
        params: Vec<Option<Value>>
    },
}