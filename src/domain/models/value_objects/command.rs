use serde_json::Value;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, Clone)]
pub enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(BrightnessValue),
    // SetColor(u8, u8, u8),
    Custom {
        name: String,
        params: Vec<Option<Value>>
    },
}

#[derive(Debug, Clone)]
pub struct BrightnessValue(u8);
impl BrightnessValue {
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for BrightnessValue {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        (1..=100)
            .contains(&value)
            .then(||Self(value))
            .ok_or_else(|| anyhow!("Value must be between 1 and 100"))        
    }    
}