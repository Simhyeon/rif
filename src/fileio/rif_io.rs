use crate::consts::*;
use crate::models::{
    rif_error::RifError,
    rif_list::RifList, 
};

/// Read rif file and return rif list
pub fn read() -> Result<RifList, RifError> {
    let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(RIF_LIST_FILE)?)?;
    rif_list.sanity_check()?;
    Ok(rif_list)
}

/// Read rif file without sanity check
pub fn read_as_raw() -> Result<RifList, RifError> {
    let rif_list: RifList = serde_json::from_str(&std::fs::read_to_string(RIF_LIST_FILE)?)?;
    Ok(rif_list)
}

/// Save rif list into rif file
pub fn save(rif_list: RifList) -> Result<(), RifError> {
    let rif_content = serde_json::to_string(&rif_list)?;
    std::fs::write(RIF_LIST_FILE, rif_content)?;
    Ok(())
}
