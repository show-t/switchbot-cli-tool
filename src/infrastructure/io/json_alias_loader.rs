use std::collections::HashMap;
use std::fs;

use anyhow::{Result, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeviceAliasMap(pub HashMap<String, String>);

pub struct JsonAliasLoader;

impl JsonAliasLoader {
    pub fn load(path: &str) -> Result<DeviceAliasMap> {
        let contents = fs::read_to_string(path).with_context(|| format!("Failed to read alias file: {}", path))?;
        let map: HashMap<String, String> = serde_json::from_str(&contents).with_context(|| "Failed to parse alias JSON")?;
        
        Ok(DeviceAliasMap(map))
    }
}