use std::path::{ PathBuf , Path};
use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Debug)]
pub enum RifError {
    AddFail(String),
    RemFail(String),
    GetFail(String),
    IOERROR(std::io::Error),
    ExtError(String),
}

impl From<std::io::Error> for RifError {
    fn from(err : std::io::Error) -> Self {
        Self::IOERROR(err)
    }
}

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
                .ok_or(RifError::ExtError("Failed to get file name from path".to_owned()))?
                .to_str()
                .ok_or(RifError::ExtError("Failed to convert file name into string".to_owned()))?
                .to_owned();
            self.files.insert(file_path.clone(), SingleFile::new(file_name));
        } else {
            return Err(RifError::AddFail("Invalid file path: Path doesn't exist".to_owned()));
        }

        // DEBUG 
        println!("{:#?}", self.files);

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
        if let Some(file) = self.files.get_mut(file_path) {
            file.references.push(ref_file.to_str().ok_or(RifError::GetFail("Invalid file name".to_owned()))?.to_owned());
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

    // TODO 
    // This should consruct file corelation tree
    // And find out if file is stale or fresh(or say up-to-date)
    pub fn update_file_statuses(&mut self) -> Result<(), RifError> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleFile {
    name: String,
    status: FileStatus,
    // Important - This is a unix time represented as naive date time
    timestamp: NaiveDateTime,
    references: Vec<String>,
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
