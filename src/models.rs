use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use itertools::Itertools;
use crate::utils;
use colored::*;

#[derive(Debug)]
pub enum RifError {
    AddFail(String),
    Ext(String),
    RifIoError(String),
    CliError(String),
    GetFail(String),
    InvalidFormat(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    CheckerError(String),
}

impl std::fmt::Display for RifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RifError::AddFail(content) => write!(f, "{}", content),
            RifError::Ext(content) => write!(f, "{}", content),
            RifError::GetFail(content) => write!(f, "{}", content),
            RifError::InvalidFormat(content) => write!(f, "{}", content),
            RifError::IoError(content) => write!(f, "{}", content),
            RifError::CliError(content) => write!(f, "{}", content),
            RifError::RifIoError(content) => write!(f, "{}", content),
            RifError::SerdeError(content) => write!(f, "{}", content),
            RifError::CheckerError(content) => write!(f, "{}", content),
        }
    }
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

impl std::fmt::Display for RifList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for path in self.files.keys().sorted() {
            let single_file = self.files.get(path).unwrap();
            let current_time = single_file.timestamp;
            let mut file_output = String::new();
            file_output.push_str(
                &format!("{} {}", path.display(), single_file.status)
            );
            if single_file.references.len() != 0 {
                file_output.push_str( &format!("\n| [refs]\n"));
            }
            for ref_item in single_file.references.iter() {
                file_output.push_str(&format!("| - {} {}", ref_item.display(), self.files.get(ref_item).unwrap().status));
                if current_time < self.files.get(ref_item).unwrap().timestamp {
                    file_output.push_str(&format!(" {}", "Updated".yellow()));
                }
                file_output.push_str("\n");
            }
            output.push_str(&format!("{}\n", file_output));
        }
        write!(f, "{}", output)
    }
}


impl RifList {
    pub fn new() -> Self {
        Self {  
            files: HashMap::new(),
        }
    }
    pub fn add_file(&mut self, file_path: &PathBuf) -> Result<bool, RifError> {
        // If file exists then executes.
        if file_path.exists() {
            // TODO check this logic
            // This file name is a base name not a full path
            let file_name = 
                file_path.file_name()
                .ok_or(RifError::AddFail("Failed to get file name from path\nDid you give a directory name?".to_owned()))?
                .to_str()
                .ok_or(RifError::AddFail("Failed to convert file name into string".to_owned()))?
                .to_owned();
            if let None = self.files.get(file_path) {
                self.files.insert(file_path.clone(), SingleFile::new(file_name));
            } else {
                return Ok(false);
            }
        } else {
            return Err(RifError::AddFail("Invalid file path: Path doesn't exist".to_owned()));
        }

        self.sanity_check_file(file_path, SanityType::Direct)?;

        Ok(true)
    }
    pub fn remove_file(&mut self, file_path: &PathBuf) -> Result<bool, RifError> {
        if let None = self.files.remove(file_path) {
            return Ok(false);
        }

        for (_, file) in self.files.iter_mut() {
            file.references.remove(file_path);
        }

        Ok(true)
    }
    pub fn update_filestamp(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_file_unix_time(file_path)?;
                file.timestamp = unix_time; 
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }
        Ok(())
    }

    pub fn update_filestamp_force(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_current_unix_time();
                file.timestamp = unix_time; 
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }
        Ok(())
    }

    pub fn discard_change(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_current_unix_time();
                // Only Update last_modified
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }
        Ok(())
    }

    pub fn add_reference(&mut self, file_path: &PathBuf, ref_files: &HashSet<PathBuf>) -> Result<(), RifError> {
        // If file doesn't exist, return error
        for file in ref_files.iter() {
            if !file.exists() {
                return Err(RifError::AddFail(format!("No such reference file exists : {}", file.display())));
            }
            if let None = self.files.get(file) {
                return Err(RifError::AddFail(format!("No such reference file exists in rif : {}", file.display())));
            }
        }

        if let Some(file) = self.files.get_mut(file_path) {
            file.references = file.references.union(ref_files).cloned().collect();
            self.sanity_check()?;
            return Ok(());
        } else {
            return Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()));
        }
    }

    pub fn remove_reference(&mut self, file_path: &PathBuf, ref_files: &HashSet<PathBuf>) -> Result<(), RifError> {
        // Remove doesn't check existences
        // Becuase artifacts cannot be fixed easily if it is
        
        if let Some(file) = self.files.get_mut(file_path) {
            file.references = &file.references - ref_files;
            self.sanity_check()?;
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
        
        // Check if file exists in the first place
        if !target_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", target_path.display())));
        }

        // Check direct self reference.
        let first_item = self.files.get(target_path).unwrap().references.iter().filter(|a| *a == target_path).next();
        // If the first exists, then target_path has same file in its reference
        if let Some(_) = first_item {
            return Err(RifError::InvalidFormat(format!("File \"{}\" is referencing itself which is not allowd", target_path.display())));
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
                self.recursive_check(target_path, child, &mut ref_status)?;
            }

            if let RefStatus::Invalid = ref_status {
                return Err(RifError::InvalidFormat(format!("Infinite reference loop detected. Last loop was \"{}\"", target_path.display())));
            }
        }

        Ok(())
    }

    fn recursive_check(&self, origin_path: &PathBuf, current_path: &PathBuf, ref_status: &mut RefStatus) -> Result<(), RifError> {

        // if current path is not existent return erro
        if !current_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", current_path.display())));
        }

        if origin_path == current_path {
            *ref_status = RefStatus::Invalid;
        } else if let RefStatus::Valid = ref_status {
            for child in self.files.get(current_path).unwrap().references.iter() {

                // Current path is same with child which means self referencing 
                if current_path == child {
                    return Err(RifError::InvalidFormat(format!("File \"{}\" is referencing itself which is not allowd", current_path.display())));
                }

                self.recursive_check(origin_path, child, ref_status)?;
            }
        }

        Ok(())
    }

    pub fn sanity_fix(&mut self) -> Result<(), RifError> {
        while let Err(_) = self.sanity_check() {
            for path in self.files.keys().cloned() {
                if let Ok(Some((parent, child))) = self.sanity_get_invalid(&path) {
                    println!("Found invalid {} - {}", parent.display(), child.display());
                    self.files.get_mut(&parent).unwrap().references.remove(&child);
                    break;
                } else {
                    continue;
                }
            }
        }
        //println!("Successfully fixed rif sanity");
        Ok(())
    }

    // Get invalid references from rif list 
    fn sanity_get_invalid(&self, target_path: &PathBuf) -> Result<Option<(PathBuf, PathBuf)>, RifError> {
        println!("Get invalid");
        
        // Check if file exists in the first place
        if !target_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", target_path.display())));
        }

        // Check direct self reference.
        let first_item = self.files.get(target_path).unwrap().references.iter().filter(|a| *a == target_path).next();
        // If the first exists, then target_path has same file in its reference
        if let Some(path) = first_item {
            return Ok(Some((path.clone(), path.clone())));
        }

        // Recursively check into children and check if the item is same with target_path
        let current_file = self.files.get(target_path).unwrap();

        // File has no references, then early return
        if current_file.references.len() == 0 {
            return Ok(None);
        }

        // Recursively check
        let mut ref_status = RefStatus::Valid;
        for child in self.files.get(target_path).unwrap().references.iter() {
            return Ok(self.recursive_find_invalid(target_path, child, &mut ref_status)?);
        }

        Ok(None)
    }

    fn recursive_find_invalid(&self, origin_path: &PathBuf, current_path: &PathBuf, ref_status: &mut RefStatus) -> Result<Option<(PathBuf, PathBuf)>, RifError> {

        // if current path is not existent return erro
        if !current_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", current_path.display())));
        }

        if origin_path == current_path {
            return Ok(Some((current_path.clone(), origin_path.clone())));
        } else if let RefStatus::Valid = ref_status {
            for child in self.files.get(current_path).unwrap().references.iter() {

                // Current path is same with child which means self referencing 
                if current_path == child {
                    println!("LOG ::: Child is referencing itself");
                    return Ok(Some((child.clone(), child.clone())));
                }

                return Ok(self.recursive_find_invalid(origin_path, child, ref_status)?);
            }
        }

        Ok(None)
    }

    pub fn track_modified_files(&self) -> Result<(), RifError> {
        let mut modified: Vec<&PathBuf> = vec![];

        for (path, file) in self.files.iter() {
            let system_time = utils::get_file_unix_time(path)?;
            if file.last_modified < system_time {
                modified.push(path);
            }
        }

        let mut display_text: String = String::new();
        if modified.len() != 0 {
            for file in modified.iter() {
                display_text.push_str(&format!("    {}\n", file.display().to_string().red()));
            }
        } else {
            display_text.push_str(&format!("    All files are up to date.\n"));
        }

        print!("{}", display_text);
        Ok(())
    }

    pub fn get_modified_files(&self) -> Result<Vec<PathBuf>, RifError> {
        let mut modified: Vec<PathBuf> = vec![];

        for (path, file) in self.files.iter() {
            let system_time = utils::get_file_unix_time(path)?;
            if file.last_modified < system_time {
                modified.push(path.clone());
            }
        }
        Ok(modified)
    }

    pub fn track_unregistered_files(&self, black_list: &HashSet<PathBuf>) -> Result<(), RifError> {
        utils::walk_directory_recursive(&std::env::current_dir()?, &| walk_path | -> Result<(), RifError> {
            // File is not in black list
            let stripped = utils::strip_path(walk_path.path(), None)?;
            if !black_list.contains(&stripped) {
                // File is not in tracked files
                if let None = self.files.get(walk_path.path()) {
                    println!("    {}", stripped.display().to_string().red());
                }
            }
            Ok(())
        })?;
        Ok(())
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FileStatus {
    Stale,
    Fresh,
    Neutral // Reserved for future usages. Might be deleted after all.
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatus::Stale => write!(f, "{}", "{Stale}".red()),
            FileStatus::Fresh => write!(f, "{}", "{Fresh}".blue()),
            FileStatus::Neutral => write!(f, "{}", "{Neutral}".green()),
        }
    }
}
