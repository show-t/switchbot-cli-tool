use anyhow::{Error, Result, anyhow};
use num_enum::TryFromPrimitive;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Command {
    TurnOn,
    TurnOff,
    SetBrightness(BrightnessValue),
    SetColor(ColorValues),
    SetColorTemperature(ColorTemperatureValue),
    AcSetAll(AcValues),
    Custom {
        name: String,
        params: Vec<Option<Value>>,
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
        let inf = 1;
        let sup = 100;

        (inf..=sup)
            .contains(&value)
            .then(|| Self(value))
            .ok_or_else(|| anyhow!("Value must be between {inf} and {sup}"))
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
            .all(|&v| (0..=255).contains(&v))
            .then(|| Self(r, g, b))
            .ok_or_else(|| anyhow!("color values must be between 0 and 255"))
    }
}

#[derive(Debug, Clone)]
pub struct ColorTemperatureValue(u16);
impl ColorTemperatureValue {
    pub fn get(&self) -> u16 {
        self.0
    }
}
impl TryFrom<u16> for ColorTemperatureValue {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        let inf = 2700;
        let sup = 6500;

        (inf..=sup)
            .contains(&value)
            .then(|| Self(value))
            .ok_or_else(|| anyhow!("Value must be between {inf} and {sup}"))
    }
}

#[derive(Debug, Clone)]
pub struct AcValues {
    pub temperature: u8,
    pub mode: AcMode,
    pub fan_speed: AcFanSpeed,
    pub power_state: AcPowerState,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum AcMode {
    AUTO = 1,
    COOL = 2,
    DRY = 3,
    FAN = 4,
    HEAT = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum AcFanSpeed {
    AUTO = 1,
    LOW = 2,
    MEDIUM = 3,
    HIGH = 4,
}

#[derive(Debug, Clone)]
pub struct AcPowerState(pub bool);
impl AcPowerState {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "on" | "1" => Some(Self(true)),
            "off" | "0" => Some(Self(false)),
            _ => None, 
        }
    }

    pub fn as_str(&self) -> &'static str {
        if self.0 {"on"} else {"off"}
    }
}