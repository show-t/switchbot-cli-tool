use anyhow::Result;
use async_trait::async_trait;

use crate::application::dto::{DeviceResponseDto, ExecuteCommandDto};
use crate::application::services::export_devices::export_devices_to_file;
use crate::domain::models::value_objects::DeviceId;
use crate::domain::repositories::IDeviceRepository;

#[async_trait]
pub trait IControlDeviceUseCase {
    async fn execute(&self, dto: ExecuteCommandDto) -> Result<()>;
    async fn fetch_devices(&self) -> Result<Vec<DeviceResponseDto>>;
}

#[derive(Debug)]
pub struct ControlDeviceUseCase<'a, R: IDeviceRepository> {
    repo: &'a R,
}

impl<'a, R: IDeviceRepository> ControlDeviceUseCase<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> IControlDeviceUseCase for ControlDeviceUseCase<'_, R> 
    where 
        R: IDeviceRepository + Sync + Send,
{
    async fn execute(&self, dto: ExecuteCommandDto) -> Result<()> {
        let device_id = DeviceId::new(dto.device_id);
        tracing::debug!("{:?} {:?}", device_id, dto.command);
        self.repo.send_command(&device_id, &dto.command).await
    }

    async fn fetch_devices(&self) -> Result<Vec<DeviceResponseDto>> {
        let devices = self.repo.get_device_list().await?;
        let _ = export_devices_to_file(&devices, "output/devices.json");

        let dto: Vec<DeviceResponseDto> = devices.into_iter().map(|v| v.into()).collect();
        Ok(dto)
    }
}
