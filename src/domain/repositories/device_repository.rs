use async_trait::async_trait;
use anyhow::Result;

use crate::domain::models::Device;
use crate::domain::models::value_objects::{DeviceId, Command};

#[async_trait]
pub trait DeviceRepository {
    async fn get_device(&self, id: &DeviceId) -> Result<Device>;
    async fn send_command(&self, id: &DeviceId, command: &Command) -> Result<()>;
    async fn get_device_list(&self) -> Result<Vec<Device>>;
}