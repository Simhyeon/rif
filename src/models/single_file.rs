use std::collections::HashSet;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use crate::models::enums::FileStatus;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleFile {
    name: String,
    pub status: FileStatus,
    // Important - This is a unix time represented as naive date time
    pub last_modified : NaiveDateTime,
    pub timestamp: NaiveDateTime,
    pub references: HashSet<PathBuf>,
}

impl SingleFile {
    // Mostly for debugging purpose
    pub fn new(name: String) -> Self {
        Self {  
            name,
            status: FileStatus::Fresh,
            last_modified: utils::get_current_unix_time(),
            timestamp: utils::get_current_unix_time(),
            references: HashSet::new()
        }
    }
}

