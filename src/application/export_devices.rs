use anyhow::Result;

use crate::domain::models::Device;
use crate::infrastructure::io::DeviceFileWriter;

pub fn export_devices_to_file(devices: &[Device], path: &str) -> Result<()> {
    DeviceFileWriter::write_to_json(devices, path)
}
