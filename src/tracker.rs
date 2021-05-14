use std::path::PathBuf;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use crate::models::{RifList, RifError};
use serde::{Deserialize, Serialize};
use crate::utils;
use crate::consts::*;


pub struct Tracker;

impl Tracker {
    pub fn track_files() -> Result<(), RifError> {
        let rif_time = Tracker::read_tracker_file()?;
        let mut modified: Vec<&PathBuf> = vec![];
        for (path, time) in rif_time.times.iter() {
            let last_modified = utils::get_file_unix_time(path)?;
            // When file's timestamp is newer than the timestamp stored in rif time file
            if last_modified > *time {
                modified.push(path);
            }
        }

        let mut dipslay_text: String = String::new();
        for file in modified.iter() {
            dipslay_text.push_str(&format!("{:#?}", file));
        }

        println!("{}", dipslay_text);
        Ok(())
    }

    fn read_tracker_file() -> Result<RifTime, RifError> {
        let rif_time: RifTime = serde_json::from_str(&std::fs::read_to_string(TRACKER_FILE_NAME)?)?;
        Ok(rif_time)
    }

    // Create new tracker file
    pub fn save_new_file() -> Result<(), RifError> {
        let new_rif_time = RifTime::new();
        let rif_time_content = serde_json::to_string(&new_rif_time)?;
        std::fs::write(TRACKER_FILE_NAME, rif_time_content)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct RifTime {
    times: HashMap<PathBuf, NaiveDateTime>
}

impl RifTime {
    pub fn new() -> Self {
        Self {  
            times: HashMap::new(),
        }
    }
}
