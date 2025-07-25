use anyhow::Result;

use crate::domain::models::Device;
use crate::domain::models::value_objects::{DeviceId, Command};
use crate::domain::repositories::DeviceRepository;

pub struct ControlDeviceUseCase<'a, R: DeviceRepository> {
    pub repo: &'a R,
}

impl <'a, R: DeviceRepository> ControlDeviceUseCase<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, device_id: DeviceId, command: Command) -> Result<()>{
        self.repo.send_command(&device_id, &command).await
    }

    pub async fn fetch_devices(&self) -> Result<Vec<Device>> {
        let devices = self.repo.get_device_list().await?;

        for device in &devices {
            println!(
                "ID: {:?}, Name: {:?}, Type: {:?}",
                device.id, device.name, device.device_type
            );
        }

        Ok(devices)
    }
}

