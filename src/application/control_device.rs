use anyhow::Result;

use crate::domain::models::value_objects::DeviceId;
use crate::domain::repositories::DeviceRepository;
use crate::application::export_devices::export_devices_to_file;
use crate::application::dto::{DeviceResponseDto, ExecuteCommandDto};

pub struct ControlDeviceUseCase<'a, R: DeviceRepository> {
    pub repo: &'a R,
}

impl <'a, R: DeviceRepository> ControlDeviceUseCase<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, dto: ExecuteCommandDto) -> Result<()>{
        let device_id = DeviceId::new(dto.device_id);
        self.repo.send_command(&device_id, &dto.command).await
    }

    pub async fn fetch_devices(&self) -> Result<Vec<DeviceResponseDto>> {
        let devices = self.repo.get_device_list().await?;
        let _ = export_devices_to_file(&devices, "output/devices.json");

        let dto: Vec<DeviceResponseDto> = devices.into_iter().map(|v| v.into()).collect();
        Ok(dto)
    }
}

