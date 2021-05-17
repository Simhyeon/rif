pub mod rif_io {
    use std::path::PathBuf;
    use crate::models::{RifList, RifError};
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
}

pub mod etc_io {
    use std::path::PathBuf;
    use crate::models::RifError;
    use std::collections::HashSet;
    use crate::consts::*;
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
};


    pub fn read_rif_ignore() -> Result<HashSet<PathBuf>, RifError> {
        if let Ok(file) = File::open(RIF_IGNORE_FILE) {
            let buffer = BufReader::new(file);
            let ignore: Result<HashSet<PathBuf>, RifError> = buffer
                .lines()
                .map(|op| -> Result<PathBuf, RifError> {Ok(PathBuf::from(op?))})
                .collect();

            Ok(ignore?)
        } else {
            // It is perfectly normal that rifignore file doesn't exist
            Ok(HashSet::new())
        }
    }
}
