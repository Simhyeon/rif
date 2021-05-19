use std::fs::metadata;
use std::path::{PathBuf, Path};

use chrono::NaiveDateTime;
use filetime::FileTime;
use crate::consts::*;
use crate::models::rif_error::RifError;

/// Get file's system timestamp in unix time
///
/// # Args
///
/// * `path` - File path to get system timestamp
pub fn get_file_unix_time(path: &PathBuf) -> Result<NaiveDateTime, RifError> {
    let metadata = std::fs::metadata(&path)?;
    // File
    let mtime = FileTime::from_last_modification_time(&metadata);
    // Convert to unix_time
    let unix_time = chrono::NaiveDateTime::from_timestamp(mtime.unix_seconds(), 0);
    Ok(unix_time)
}

/// Get current time in unix time
pub fn get_current_unix_time() -> NaiveDateTime {
    let now = chrono::Utc::now().timestamp();
    let unix_time = chrono::NaiveDateTime::from_timestamp(now, 0);
    unix_time
}

/// Recursively walk directories and call a given function
///
/// Function is called on all paths including files and directories
/// but given path directory.
///
/// # Args
///
/// * `path` - File path to start directory walking
/// * `f` - Function refernce to be triggered on every path entry
pub fn walk_directory_recursive(path: &Path, f: &mut dyn FnMut(PathBuf) -> Result<LoopBranch, RifError>) -> Result<(), RifError> {
    for entry in std::fs::read_dir(path)? {
        let entry_path: PathBuf = strip_path(&entry?.path(), None)?;
        let md = metadata(entry_path.clone()).unwrap();

        // TODO Remove this check becuase std walk_dir doesn't include self path
        if entry_path != path { // prevent infinite loop
            // if not a directory, or is a file
            // else, is a directory, recursive call a function
            if !md.is_dir() {
                if let LoopBranch::Exit = f(entry_path)? {
                    return Ok(());
                }
            } else {
                if let LoopBranch::Continue = f(entry_path.clone())? {
                    walk_directory_recursive(&entry_path, f)?;
                }
            }
        }  
    }

    Ok(())
} // function end

/// Walk directories and call a given function
///
/// Function is called on all paths including files and directories
/// but given path directory.
///
/// # Args
///
/// * `path` - File path to start directory walking
/// * `f` - Function refernce to be triggered on every path entry
pub fn walk_directory(path: &Path, f: &mut dyn FnMut(PathBuf) -> Result<(), RifError>) -> Result<(), RifError> {
    for entry in std::fs::read_dir(path)? {
        f(entry?.path())?;
    }
    Ok(())
}

/// Strip a target path with a given base path
///
/// If no strip path is given, then strip a current working directory from a given path.
///
/// # Args
///
/// * `path` - Target path to strip
/// * `base_path` - Path to strip from target, default is current working directory
///
/// # Example
///
/// ```
/// // Current working directory is /home/user/test
/// let target_path = PathBuf::from("/home/user/test/target");
/// let stripped = strip_path(&target_path, None);
/// assert_eq!(stripped, PathBuf::from("target"));
/// 
/// let stripped2 = strip_path(&target_path, Some(PathBuf::from("/home/user")));
/// assert_eq!(stripped, PathBuf::from("test/target"));
/// ```
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
            Ok(path.to_path_buf())
        }
    }
}

/// Convert a path into a relative path
///
/// This function yields error when absolute path doesn't start with current working directory.
/// # Args
///
/// * `path` - File path to make as relative path
///
/// # Example
/// ```
/// // Current working directory is /home/user/test/example
/// let absolute = relativize_path(PathBuf::from("/home/user/test/example"));
/// assert_eq!(absolute, PathBuf::from("example"));
///
/// let dotslash = relativize_path(PathBuf::from("./test/example"));
/// assert_eq!(absolute, PathBuf::from("example"));
/// ```
pub fn relativize_path(path: &Path) -> Result<PathBuf, RifError> {
    let path_buf: PathBuf;

    if path.starts_with("./") {
        path_buf = strip_path(path, Some(PathBuf::from("./")))?;
    } else if path.starts_with(&std::env::current_dir()?){
        path_buf = strip_path(path, Some(std::env::current_dir()?))?;
    } else if !std::env::current_dir()?.join(path).exists() {
        return Err(RifError::RifIoError( format!("Only files inside of rif directory can be added\nFile \"{}\" is not.", path.display())));
    } else {
        return Ok(path.to_path_buf());
    }

    Ok(path_buf)
}

/// Check if rif file exists
///
/// Return error if rif file is not in current working directory
pub fn check_rif_file() -> Result<(), RifError> {
    if !PathBuf::from(RIF_LIST_FILE).exists() {
        return Err(RifError::RifIoError(format!("\"{}\" doesn't exist in current working directory", RIF_LIST_FILE)));
    }
    Ok(())
}

/// Loop diversion enumerator
///
/// Used with walk_directory_recursive method, so that given function can decide when to stop recursion.
pub enum LoopBranch {
    Exit,
    Continue,
}
