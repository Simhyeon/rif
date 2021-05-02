use std::path::PathBuf;
use crate::models::RifList;

pub struct Reader; 
pub struct Writer;
pub struct Sanitizer;

impl Reader {
    // Read file and construct into RifList struct
    // Return error on errorneous process
    pub fn read(file_name: PathBuf) -> Result<RifList, std::io::Error> {
        let rif_list = serde_json::from_str(&std::fs::read_to_string(file_name)?)?;
        Ok(rif_list)
    }
}

impl Writer {
    // Save/Update rif list into a file
    pub fn save(file_name: PathBuf, rif_list: RifList) -> Result<(), std::io::Error> {
        let rif_content = serde_json::to_string(&rif_list)?;
        std::fs::write(file_name, rif_content)?;
        Ok(())
    }
}

impl Sanitizer {
    // Sanitize rif file whether files and references exist
    pub fn sanitize(rif_list: RifList) -> Result<(), std::io::Error> {
        Ok(())
    }
}
