use crate::infrastructure::io::DeviceFileWriter;
use crate::domain::models::Device;
use anyhow::Result;

pub fn export_devices_to_file(devices: &[Device], path: &str) -> Result<()> {
    DeviceFileWriter::write_to_json(devices, path)
}