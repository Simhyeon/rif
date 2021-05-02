use std::path::{ PathBuf , Path};
use serde::{ Serialize, Deserialize };
use chrono::DateTime;

#[derive(Debug)]
pub enum RifError {
    AddFail(String),
    DelFail(String),
    IOERROR(std::io::Error),
}

impl From<std::io::Error> for RifError {
    fn from(err : std::io::Error) -> Self {
        Self::IOERROR(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RifList {
    pub files: Vec<SingleFile>,
}

impl RifList {
    pub fn add_file(&mut self, path: PathBuf) -> Result<(), RifError> {
        // If file exists then executes.
        if path.exists() {
            let path = if let Some(content) = path.to_str() {
                content
            } else {
                // Cannot extract path into string, which is hardly but able. I guess
                return Err(RifError::AddFail("Invalid file path".to_owned()));
            };
            self.files.push(SingleFile::new(path));
        } else {
            return Err(RifError::AddFail("Invalid file path: Path doesn't exist".to_owned()));
        }

        // DEBUG 
        println!("{:?}", self.files);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleFile {
    path: String,
    status: FileStatus,
    timestamp: DateTime<chrono::Utc>,
    references: Vec<String>,
}

impl SingleFile {
    pub fn new(path: &str) -> Self {
        Self {  
            path : path.to_owned(),
            status: FileStatus::Fresh,
            timestamp: chrono::Utc::now(),
            references: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileStatus {
    Stale,
    Fresh
}
