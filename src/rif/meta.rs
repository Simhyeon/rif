use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use crate::RifError;
use crate::utils;

/// Meta information related to rif directory
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Meta {
    pub to_be_forced: HashSet<PathBuf>,
    pub to_be_added: HashSet<PathBuf>,
}

impl Meta {
    pub fn read_from_file(path: Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let path = utils::get_meta_path(path)?;
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

    pub fn save_to_file(&self, path: Option<impl AsRef<Path>>) -> Result<(), RifError> {
        let result = bincode::serialize(self);
        let path =utils::get_meta_path(path)?;
        if let Err(err) = result {
            Err(RifError::BincodeError(err))
        } else {
            std::fs::write(path, result.unwrap())?;
            Ok(())
        }
    }
}
