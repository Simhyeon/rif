use serde::{ Deserialize, Serialize };

use crate::consts::*;
use crate::models::enums::HookArgument;
use crate::models::rif_error::RifError;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{ 
    pub histoy_capacity: usize,
    pub hook: HookConfig,
    // Set check after update, or --check flag as default
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HookConfig {
    pub trigger: bool,
    pub hook_command: Option<String>,
    pub hook_argument: HookArgument,
}

impl Config {
    pub fn new() -> Self {
        Self {
            histoy_capacity: 30,
            hook : HookConfig {
                trigger: false,
                hook_command: None,
                hook_argument: HookArgument::None,
            },
        }
    }

    /// Read config from a file
    pub fn read_from_file() -> Result<Self, RifError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(RIF_CONFIG)?)?)
    }

    /// Save config into a file
    pub fn save_to_file(&self) -> Result<(), RifError> {
        let rif_config = serde_json::to_string_pretty(self)?;
        std::fs::write(RIF_CONFIG, rif_config)?;

        Ok(())
    }
}
