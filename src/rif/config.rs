use serde::{ Deserialize, Serialize };

use crate::consts::*;
use crate::rif::hook::{HookConfig,HookArgument};
use crate::RifError;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{ 
    pub hook: HookConfig,
    pub git_ignore: bool,
    // Set check after update, or --check flag as default
}

impl Config {
    pub fn new() -> Self {
        Self {
            hook : HookConfig {
                trigger: false,
                hook_command: None,
                hook_argument: HookArgument::None,
            },
            git_ignore: false,
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