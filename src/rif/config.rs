use std::path::Path;
use serde::{ Deserialize, Serialize };

use crate::utils;
use crate::rif::hook::{HookArgument, Hook};
use crate::RifError;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config{ 
    pub hook: Hook,
    pub git_ignore: bool,
    // Set check after update, or --check flag as default
}

impl Config {
    pub fn new() -> Self {
        Self {
            hook : Hook {
                trigger: false,
                command: None,
                arg_type: HookArgument::None,
            },
            // Default is true
            git_ignore: true,
        }
    }

    /// Read config from a file
    pub fn read_from_file(path : Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let path = utils::get_config_path(path)?;
        Ok(serde_json::from_str( &std::fs::read_to_string(path)?)?)
    }

    /// Save config into a file
    pub fn save_to_file(&self, path: Option<impl AsRef<Path>>) -> Result<(), RifError> {
        let path = utils::get_config_path(path)?;
        let rif_config = serde_json::to_string_pretty(self)?;
        std::fs::write(path, rif_config)?;

        Ok(())
    }
}
