use std::path::PathBuf;
use chrono::NaiveDateTime;
use filetime::FileTime;
use crate::models::RifError;

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
