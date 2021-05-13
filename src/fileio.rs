use std::path::PathBuf;
use crate::models::{RifList, RifError};

pub struct FileIO;

impl FileIO {
    // Read file and construct into RifList struct
    // Return error on errorneous process
    pub fn read(file_name: PathBuf) -> Result<RifList, RifError> {
        let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(file_name)?)?;
        rif_list.sanity_check()?;

        println!("Successfully read file");
        Ok(rif_list)
    }
    // Save/Update rif list into a file
    pub fn save(file_name: PathBuf, rif_list: RifList) -> Result<(), RifError> {
        let rif_content = serde_json::to_string(&rif_list)?;
        std::fs::write(file_name, rif_content)?;
        Ok(())
    }
}