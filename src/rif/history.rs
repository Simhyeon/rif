use serde::{ Serialize , Deserialize};
use crate::utils;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::RifError;

/// Struct history of rif update messags
///
/// Hisotry stores vector of messages thus can be very large theoritically
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn read_from_file(path: Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let path =utils::get_history_path(path)?;
        let result = bincode::deserialize::<Self>(&std::fs::read(path)?);
        match result {
            Err(err) => {
                Err(RifError::BincodeError(err))
            }
            Ok(history) => {
                Ok(history)
            }
        }
    }

    /// Save history struct into a file
    pub fn save_to_file(&self, path: Option<impl AsRef<Path>>) -> Result<(), RifError> {
        let result = bincode::serialize(self);
        let path =utils::get_history_path(path)?;
        if let Err(err) = result {
            Err(RifError::BincodeError(err))
        } else {
            std::fs::write(path, result.unwrap())?;
            Ok(())
        }
    }

    /// Add new history
    ///
    /// After history's length go over config's capacity it automatically removes old update messages
    /// # Args
    ///
    /// * `path` - Target file name
    /// * `msg` - Message to add
    pub fn add_history(&mut self, path: &Path, msg: &str) -> Result<(), RifError> {
        // Whether history already exists or not
        if let Some(unit) = self.hist_map.get_mut(path) {
            unit.push(msg.to_owned());
        } else {
            self.hist_map.insert(path.to_path_buf(), vec![msg.to_owned()]);
        }

        Ok(())
    }

    /// Removea file from history
    pub fn remove_file(&mut self, file: &Path) -> Result<(), RifError> {
        self.hist_map.remove(file);
        Ok(())
    }

    /// Print all histroy of given file
    ///
    /// # Args
    ///
    /// * `path` - Target file name
    pub fn print_history(&self, path: &Path) -> Result<(), RifError> {
        if let Some(hist) = self.hist_map.get(path) {
            // Iterator should be reverse to print the newest first.
            for item in hist.iter().rev() {
                println!("  | {}", item);
            }
        }
        else {
            println!("{}", utils::red("  No history"));
            // REF
            // No history was considered error before, however I don't think it should be
            //Err(RifError::GetFail(format!("Failed get histroy with given path : {}", path.display())))
        }
        
        Ok(())
    }
}
