use serde::{ Serialize , Deserialize};
use std::path::PathBuf;
use std::collections::HashMap;
use crate::consts::*;
use crate::models::rif_error::RifError;

/// Struct history of rif update messags
///
/// Hisotry stores vector of messages thus can be very large theoritically
#[derive(Serialize, Deserialize)]
pub struct History {
    hist_map : HashMap::<PathBuf, Vec<String>>,
}

impl History {
    pub fn new() -> Self {
        Self {
            hist_map: HashMap::new(),
        }
    }

    /// Read history struct from a file
    pub fn read_from_file() -> Result<Self, RifError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(RIF_UPDATE_HISTORY)?)?)
    }

    /// Save history struct into a file
    pub fn save_to_file(&self) -> Result<(), RifError> {
        let rif_history = serde_json::to_string(self)?;
        std::fs::write(RIF_UPDATE_HISTORY, rif_history)?;

        Ok(())
    }

    /// Add new history
    ///
    /// # Args
    ///
    /// * `path` - Target file name
    /// * `msg` - Message to add
    pub fn add_history(&mut self, path: &PathBuf, msg: &str) -> Result<(), RifError> {
        // Wehter history already exists or not
        if let Some(unit) = self.hist_map.get_mut(path) {
            unit.push(msg.to_owned());
        } else {
            self.hist_map.insert(path.to_path_buf(), vec![msg.to_owned()]);
        }

        Ok(())
    }

    /// Print all histroy of given file
    ///
    /// # Args
    ///
    /// * `path` - Target file name
    pub fn print_history(&self, path: &PathBuf) -> Result<(), RifError> {
        if let Some(hist) = self.hist_map.get(path) {
            // Iterator should be reverse to print the newest first.
            for item in hist.iter().rev() {
                println!("  ^ {}", item);
            }
            Ok(())
        } else {
            Err(RifError::GetFail(format!("Failed get histroy with given path : {}", path.display())))
        }
    }
}
