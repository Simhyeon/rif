use std::path::PathBuf;

use crate::models::{
    rif_error::RifError
    rif_list::RifList, 
};

// Read file and construct into RifList struct
// Return error on errorneous process
pub fn read(file_name: &PathBuf) -> Result<RifList, RifError> {
    let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(file_name)?)?;
    rif_list.sanity_check()?;

    //println!("Successfully read file");
    Ok(rif_list)
}
// Same method but gets str
pub fn read_with_str(file_name: &str) -> Result<RifList, RifError> {
    let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(file_name)?)?;
    rif_list.sanity_check()?;

    //println!("Successfully read file");
    Ok(rif_list)
}

// This method doesn't check sanity of file
pub fn read_with_str_as_raw(file_name: &str) -> Result<RifList, RifError> {
    let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(file_name)?)?;

    //println!("Successfully read file");
    Ok(rif_list)
}

// Save/Update rif list into a file
pub fn save(file_name: &PathBuf, rif_list: RifList) -> Result<(), RifError> {
    let rif_content = serde_json::to_string(&rif_list)?;
    std::fs::write(file_name, rif_content)?;
    Ok(())
}

pub fn save_with_str(file_name: &str, rif_list: RifList) -> Result<(), RifError> {
    let rif_content = serde_json::to_string(&rif_list)?;
    std::fs::write(file_name, rif_content)?;
    Ok(())
}

