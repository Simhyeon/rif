use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    collections::HashSet
};

use crate::consts::*;
use crate::models::rif_error::RifError;

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
