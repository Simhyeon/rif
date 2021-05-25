use std::process::Command;

use serde::{ Deserialize, Serialize };

use crate::consts::*;
use crate::models::rif_error::RifError;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{ 
    pub histoy_capacity: usize,
    // Check hook status
    // Check hook script 
    // Argument options(Which argument to pass) -> Only get fresh vector, Only get stale vector, Get json struct
    // Set check after update, or --check flag as default
}

impl Config {
    pub fn new() -> Self {
        Self {
            histoy_capacity: 30,
        }
    }

    /// Read config from a file
    pub fn read_from_file() -> Result<Self, RifError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(RIF_CONFIG)?)?)
    }

    /// Save config into a file
    pub fn save_to_file(&self) -> Result<(), RifError> {
        let rif_config = serde_json::to_string(self)?;
        std::fs::write(RIF_CONFIG, rif_config)?;

        Ok(())
    }
}
