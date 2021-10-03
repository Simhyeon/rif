use crate::models::LoopBranch;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs::metadata;
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use std::process::Command;

#[cfg(feature = "color")]
use colored::*;
use chrono::NaiveDateTime;
use filetime::FileTime;
use crate::consts::*;
use crate::RifError;

/// Get file's system timestamp in unix time
///
/// # Args
///
/// * `path` - File path to get system timestamp
pub fn get_file_unix_time(path: &Path) -> Result<NaiveDateTime, RifError> {
    let metadata = std::fs::metadata(path)?;
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
pub(crate) fn walk_directory_recursive(path: &Path, f: &mut dyn FnMut(PathBuf) -> Result<LoopBranch, RifError>) -> Result<(), RifError> {
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

/// Get rif file
///
/// Return error if rif file is not in current or ancestors' directory
/// This returns root directory that contains .rif directory, not .rif directory itself
pub fn get_rif_directory() -> Result<PathBuf, RifError> {
    for path in std::env::current_dir()?.ancestors() {
        if path.join(RIF_DIECTORY).is_dir() {
            return Ok(path.to_owned());
        }
    }
    Err(RifError::ConfigError("Not a rif directory".to_owned()))
}

/// Get black list
///
/// This read rig ignore file contents and merge with const black list contents
pub fn get_black_list(use_gitignore: bool) -> Result<HashSet<PathBuf>, RifError> {
    let mut black_list: HashSet<PathBuf> = HashSet::new();
    let rif_ignore = read_rif_ignore()?;

    // Const files that should be ignored
    black_list.extend(BLACK_LIST.to_vec().iter().map(|a| PathBuf::from(*a)).collect::<HashSet<PathBuf>>());
    // Rif ignore files
    black_list.extend(rif_ignore);

    // Include git ignore files
    if use_gitignore{
        black_list.extend(read_git_ignore()?);
        black_list.insert(PathBuf::from(".gitignore"));
    } 

    Ok(black_list)
}

/// Read rif ignore file
///
/// Do io operation and converts into hashset
fn read_rif_ignore() -> Result<HashSet<PathBuf>, RifError> {
    if let Ok(file) = File::open(RIF_IGNORE_FILE) {
        let buffer = BufReader::new(file);
        let ignore: Result<HashSet<PathBuf>, RifError> = buffer
            .lines()
            .map(|op| -> Result<PathBuf, RifError> {Ok(PathBuf::from(op?))})
            .collect();

        Ok(ignore?)
    } else {
        // It is perfectly normal that rifignore file doesn't exist
        Ok(HashSet::new())
    }
}

/// Read git ignore file
fn read_git_ignore() -> Result<HashSet<PathBuf>, RifError> {
    if let Ok(file) = File::open(".gitignore") {
        let buffer = BufReader::new(file);
        let ignore: Result<HashSet<PathBuf>, RifError> = buffer
            .lines()
            .map(|op| -> Result<PathBuf, RifError> {Ok(PathBuf::from(op?))})
            .collect();

        Ok(ignore?)
    } else {
        // It is perfectly normal that gitignore file doesn't exist
        Ok(HashSet::new())
    }
}

// Path Getters

pub fn get_rel_path(path : Option<impl AsRef<Path>>) -> Result<PathBuf, RifError> {
    let path = if let Some(path) = path {
        path.as_ref().to_owned()
    } else {  
        std::env::current_dir()?
    };
    Ok(path.join(RIF_DIECTORY).join(RIF_REL_FILE))
}

pub fn get_config_path(path : Option<impl AsRef<Path>>) -> Result<PathBuf, RifError> {
    let path = if let Some(path) = path {
        path.as_ref().to_owned()
    } else {  
        std::env::current_dir()?
    };
    Ok(path.join(RIF_DIECTORY).join(RIF_CONFIG))
}

pub fn get_history_path(path : Option<impl AsRef<Path>>) -> Result<PathBuf, RifError> {
    let path = if let Some(path) = path {
        path.as_ref().to_owned()
    } else {  
        std::env::current_dir()?
    };
    Ok(path.join(RIF_DIECTORY).join(RIF_HIST_FILE))
}

pub fn get_meta_path(path : Option<impl AsRef<Path>>) -> Result<PathBuf, RifError> {
    let path = if let Some(path) = path {
        path.as_ref().to_owned()
    } else {  
        std::env::current_dir()?
    };
    Ok(path.join(RIF_DIECTORY).join(RIF_META))
}

pub fn green(string : &str) -> Box<dyn std::fmt::Display> {
    if cfg!(feature = "color") {
        #[cfg(feature = "color")]
        return Box::new(string.green().to_owned());
    }
    Box::new(string.to_owned())
}

pub fn blue(string : &str) -> Box<dyn std::fmt::Display> {
    if cfg!(feature = "color") {
        #[cfg(feature = "color")]
        return Box::new(string.blue().to_owned());
    }
    Box::new(string.to_owned())
}

pub fn red(string : &str) -> Box<dyn std::fmt::Display> {
    if cfg!(feature = "color") {
        #[cfg(feature = "color")]
        return Box::new(string.red().to_owned());
    }
    Box::new(string.to_owned())
}

pub fn yellow(string : &str) -> Box<dyn std::fmt::Display> {
    if cfg!(feature = "color") {
        #[cfg(feature = "color")]
        return Box::new(string.yellow().to_owned());
    }
    Box::new(string.to_owned())
}

pub fn cmd(cmd: &str, args: Vec<String>) -> Result<(), RifError> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .args(args)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(cmd)
            .args(args)
            .output()
            .expect("failed to execute process")
    };

    // This can be not valid, since it is lossy, but hardly 
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if stdout.len() != 0 { println!("{}",stdout); }
    if stderr.len() != 0 { eprintln!("{}",stderr); }

    Ok(())
}
