use std::path::{PathBuf, Path};
use std::fs::metadata;
use walkdir::{WalkDir, DirEntry};
use chrono::NaiveDateTime;
use filetime::FileTime;
use crate::models::rif_error::RifError;
use crate::consts::*;

// Get file name and return filestamp as unix time(Epoch time)
pub fn get_file_unix_time(path: &PathBuf) -> Result<NaiveDateTime, RifError> {
    let metadata = std::fs::metadata(&path)?;
    // File
    let mtime = FileTime::from_last_modification_time(&metadata);
    // Convert to unix_time
    let unix_time = chrono::NaiveDateTime::from_timestamp(mtime.unix_seconds(), 0);
    Ok(unix_time)
}

pub fn get_current_unix_time() -> NaiveDateTime {
    let now = chrono::Utc::now().timestamp();
    let unix_time = chrono::NaiveDateTime::from_timestamp(now, 0);

    unix_time
}

pub fn walk_directory_recursive(path: &Path, f: &mut dyn FnMut(DirEntry) -> Result<(), RifError>) -> Result<(), RifError> {
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let md = metadata(entry.path()).unwrap();
        if entry.path() != path  && !md.is_dir() {
            f(entry)?;
        }
    }

    Ok(())
}

pub fn strip_path(path: &Path, base_path: Option<PathBuf>) -> Result<PathBuf, RifError> {
    if let Some(base_path) = base_path {
        if let Ok( striped_path ) =  path.strip_prefix(base_path) {
            Ok(striped_path.to_owned())
        } else {
            Err(RifError::Ext(String::from("Failed to get stripped path")))
        }
    } else {
        if let Ok( striped_path ) =  path.strip_prefix(std::env::current_dir()?) {
            Ok(striped_path.to_owned())
        } else {
            Err(RifError::Ext(String::from("Failed to get stripped path")))
        }
    }
}

pub fn relativize_path(path: &Path) -> Result<PathBuf, RifError> {
    let path_buf: PathBuf;

    if path.starts_with("./") {
        path_buf = strip_path(path, Some(PathBuf::from("./")))?;
    } else if path.starts_with(&std::env::current_dir()?){
        path_buf = strip_path(path, Some(std::env::current_dir()?))?;
    } else if !std::env::current_dir()?.join(path).exists() {
        return Err(RifError::RifIoError(format!("Only files inside of rif directory can be added\nFile \"{}\" is not.", path.display())));
    } else {
        return Ok(path.to_path_buf());
    }

    Ok(path_buf)
}

pub fn check_rif_file() -> Result<(), RifError> {
    if !PathBuf::from(RIF_LIST_FILE).exists() {
        return Err(RifError::RifIoError(format!("\"{}\" doesn't exist in current working directory", RIF_LIST_FILE)));
    }
    Ok(())
}
