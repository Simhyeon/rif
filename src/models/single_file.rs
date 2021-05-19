use std::collections::HashSet;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use crate::models::enums::FileStatus;
use crate::utils;

/// Struct that contains information about single file in rif
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleFile {
    /// Name of the file, it is not full path
    name: String,
    /// Current status of the file
    pub status: FileStatus,
    /// Current last modified time stored in rif.
    ///
    /// This is not same with system's last modified time
    pub last_modified : NaiveDateTime,
    /// Timestamp of the file
    ///
    /// This is a critera to compare file status
    pub timestamp: NaiveDateTime,
    /// Files set that contains referencing files
    pub references: HashSet<PathBuf>,
}

impl SingleFile {
    // Mostly for debugging purpose
    pub fn new(name: PathBuf) -> Self {
        Self {  
            name : name.file_name().unwrap().to_str().unwrap().to_owned(),
            status: FileStatus::Fresh,
            last_modified: utils::get_current_unix_time(),
            timestamp: utils::get_current_unix_time(),
            references: HashSet::new()
        }
    }
}

