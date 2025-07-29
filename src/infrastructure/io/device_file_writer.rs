use std::fs::File;
use std::io::Write;

use anyhow::Result;

use crate::domain::models::Device;

pub struct DeviceFileWriter;

impl DeviceFileWriter {
    pub fn write_to_json(devices: &[Device], path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(devices)?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
