use std::path::{ PathBuf , Path};
use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Debug)]
pub enum RifError {
    AddFail(String),
    RemFail(String),
    GetFail(String),
    InvalidFormat(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    CheckerError(String),
}

// =====
// From conversion for Rif Error
impl From<std::io::Error> for RifError {
    fn from(err : std::io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<serde_json::Error> for RifError {
    fn from(err : serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}
// =====

#[derive(Serialize, Deserialize, Debug)]
pub struct RifList {
    pub files: HashMap<PathBuf, SingleFile>,
}

impl RifList {
    pub fn add_file(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        // If file exists then executes.
        if file_path.exists() {
            let file_name = 
                file_path.file_name()
                .ok_or(RifError::AddFail("Failed to get file name from path".to_owned()))?
                .to_str()
                .ok_or(RifError::AddFail("Failed to convert file name into string".to_owned()))?
                .to_owned();
            self.files.insert(file_path.clone(), SingleFile::new(file_name));
        } else {
            return Err(RifError::AddFail("Invalid file path: Path doesn't exist".to_owned()));
        }

        // DEBUG 
        println!("{:#?}", self.files);

        self.sanity_check_file(file_path, SanityType::Direct)?;

        Ok(())
    }
    pub fn remove_file(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if let None = self.files.remove(file_path) {
            return Err(RifError::RemFail("Failed to remove file from rif list".to_owned()));
        }

        // DEBUG 
        println!("{:#?}", self.files);

        Ok(())
    }

    pub fn add_reference(&mut self, file_path: &PathBuf, ref_file: &PathBuf) -> Result<(), RifError> {
        // If file doesn't exist, return error
        if !ref_file.exists() {
            return Err(RifError::AddFail(format!("No such reference file exists : {:#?}", ref_file)));
        }

        if let Some(file) = self.files.get_mut(file_path) {
            file.references.push(ref_file.clone());
            self.sanity_check_file(file_path, SanityType::Direct)?;
            return Ok(());
        } else {
            return Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()));
        }
    }

    pub fn set_file_status(&mut self, file_path: &PathBuf, file_status: FileStatus) -> Result<(), RifError> {
        if let Some(file) = self.files.get_mut(file_path) {
            file.status = file_status;
        } else {
            return Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()));
        }
        Ok(())
    }

    pub fn sanity_check(&self) -> Result<(), RifError> {
        for path in self.files.keys() {
            self.sanity_check_file(path, SanityType::Indirect)?;
        }
        Ok(())
    }

    // Internal function for single sanity check file
    fn sanity_check_file(&self, target_path: &PathBuf, sanity_type: SanityType) -> Result<(), RifError> {
        // NOTE ::: Question - Why sanity type exists?
        // Answer - |
        // Because indirect reference checking goes through all files and references which is not
        // possible in such scenario that only some files were added into rif_list
        // but direct checking is also needed to early return from easily detectable invalid case.
        // Thus there exists two types and indirect check also does direct checks
        
        // Check direct self reference.
        let first_item = self.files.get(target_path).unwrap().references.iter().filter(|a| *a == target_path).next();
        // If the first exists, then target_path has same file in its reference
        if let Some(_) = first_item {
            return Err(RifError::InvalidFormat(format!("File {:?} is referencing itself which is not allowd", target_path)));
        }

        // Also check indirect self references.
        if let SanityType::Indirect = sanity_type {
            // Recursively check into children and check if the item is same with target_path
            let current_file = self.files.get(target_path).unwrap();

            // File has no references, then early return
            if current_file.references.len() == 0 {
                return Ok(());
            }

            // Recursively check
            let mut ref_status = RefStatus::Valid;
            for child in self.files.get(target_path).unwrap().references.iter() {
                self.recursive_check(target_path, child, &mut ref_status);
            }

            if let RefStatus::Invalid = ref_status {
                return Err(RifError::InvalidFormat(format!("Infinite reference loop detected\nLast loop was {:#?}", target_path)));
            }
        }

        Ok(())
    }

    fn recursive_check(&self, origin_path: &PathBuf, current_path: &PathBuf, ref_status: &mut RefStatus) {
        if origin_path == current_path {
            *ref_status = RefStatus::Invalid;
        } else if let RefStatus::Valid = ref_status {
            for child in self.files.get(current_path).unwrap().references.iter() {
                self.recursive_check(origin_path, child, ref_status);
            }
        }
    }
}

pub enum SanityType {
    Direct,
    Indirect
}

enum RefStatus {
    Invalid,
    Valid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleFile {
    name: String,
    pub status: FileStatus,
    // Important - This is a unix time represented as naive date time
    pub timestamp: NaiveDateTime,
    pub references: Vec<PathBuf>,
}

impl SingleFile {
    // Mostly for debugging purpose
    pub fn new(name: String) -> Self {
        Self {  
            name,
            status: FileStatus::Fresh,
            timestamp: 
                NaiveDateTime::new(
                    NaiveDate::from_ymd(2020, 5, 6), 
                    NaiveTime::from_hms(1, 2, 34)
                ),
            references: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileStatus {
    Stale,
    Fresh,
    Neutral // Reserved for future usages. Might be deleted after all.
}
