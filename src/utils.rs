use std::path::PathBuf;
use chrono::NaiveDateTime;
use filetime::FileTime;

// Get file name and return filestamp as unix time(Epoch time)
pub fn get_file_unix_time(path: PathBuf) -> Option<NaiveDateTime> {
    let metadata = 
        if let Ok(data) = std::fs::metadata(&path) {
            data
        } else {
            return None;
        };
    // Get last modification time 
    let mtime = FileTime::from_last_modification_time(&metadata);
    // Convert to unix_time
    let unix_time = chrono::NaiveDateTime::from_timestamp(mtime.unix_seconds(), 0);
    Some(unix_time)
}
