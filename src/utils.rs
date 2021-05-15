use std::path::{PathBuf, Path};
use std::fs::metadata;
use walkdir::{WalkDir, DirEntry};
use chrono::NaiveDateTime;
use filetime::FileTime;
use crate::models::RifError;
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

pub fn walk_directory_recursive(path: &Path, f: &dyn Fn(DirEntry) -> Result<(), RifError>) -> Result<(), RifError> {
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let md = metadata(entry.path()).unwrap();
        if entry.path() != path  && !md.is_dir() {
            f(entry)?;
        }
    }

    Ok(())
}

fn walkdir_print_stripped(path : walkdir::DirEntry) -> Result<(), RifError> {
    if let Ok( striped_path ) =  path.path().strip_prefix(std::env::current_dir()?) {
        println!("{}", striped_path.display())
    }

    Ok(())
}

pub fn check_rif_file() -> Result<(), RifError> {
    if !PathBuf::from(RIF_LIST_FILE).exists() {
        return Err(RifError::Ext(format!("<{}> doesn't exist in current working directory", RIF_LIST_FILE)));
    }
    Ok(())
}
