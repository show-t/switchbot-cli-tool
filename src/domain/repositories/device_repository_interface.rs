use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::entities::Device;
use crate::domain::models::value_objects::{Command, DeviceId};

#[async_trait]
pub trait IDeviceRepository {
    async fn get_device(&self, id: &DeviceId) -> Result<Device>;
    async fn send_command(&self, id: &DeviceId, command: &Command) -> Result<()>;
    async fn get_device_list(&self) -> Result<Vec<Device>>;
}
