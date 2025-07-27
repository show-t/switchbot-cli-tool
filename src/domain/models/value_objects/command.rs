use serde_json::Value;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, Clone)]
pub enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(BrightnessValue),
    SetColor(ColorValues),
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

#[derive(Debug, Clone)]
pub struct ColorValues(u8, u8, u8);
impl ColorValues {
    pub fn get(&self) -> (u8, u8, u8) {
        (self.get_r(), self.get_g(), self.get_b())
    }

    pub fn get_r(&self) -> u8 {
        self.0
    }

    pub fn get_g(&self) -> u8 {
        self.1
    }

    pub fn get_b(&self) -> u8 {
        self.2 
    }
}

impl TryFrom<(u8, u8, u8)> for ColorValues {
    type Error = Error;

    fn try_from(values: (u8, u8, u8)) -> Result<Self> {
        let (r, g, b) = values;
        [r, g, b]
            .iter()
            .all(|&v|(0..=255).contains(&v))
            .then(||Self(r, g, b))
            .ok_or_else(|| anyhow!("color values must be between 0 and 255"))      
    }
}