use std::path::{PathBuf, Path};
use std::fs::metadata;
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

/// Call a given closure on all directories under given path
/// this function call the closure on all paths including files and directories
/// but given path directory.
pub fn walk_directory_recursive(path: &Path, f: &mut dyn FnMut(PathBuf) -> Result<LoopBranch, RifError>) -> Result<(), RifError> {
    for entry in std::fs::read_dir(path)? {
        let entry_path: PathBuf = strip_path(&entry?.path(), None)?;
        let md = metadata(entry_path.clone()).unwrap();
        if entry_path != path { // prevent infinite loop
            // Not a directory
            if !md.is_dir() {
                if let LoopBranch::Exit = f(entry_path)? {
                    return Ok(());
                }
            } 
            // Directory, recursive call
            else {
                if let LoopBranch::Continue = f(entry_path.clone())? {
                    walk_directory_recursive(&entry_path, f)?;
                }
            }
        }  
    }

    Ok(())
}

pub fn walk_directory(path: &Path, f: &mut dyn FnMut(PathBuf) -> Result<(), RifError>) -> Result<(), RifError> {
    for entry in std::fs::read_dir(path)? {
        f(entry?.path())?;
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
            // This was formerlyl an error
            // Err(RifError::Ext(String::from("Failed to get stripped path")))
            Ok(path.to_path_buf())
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

pub enum LoopBranch {
    Exit,
    Continue,
}
