use serde::Serialize;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Result<String> {
        Ok(self.0.clone())
    }
}