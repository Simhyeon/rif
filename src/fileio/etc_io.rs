use std::path::PathBuf;
use crate::models::rif_error::RifError;
use std::collections::HashSet;
use crate::consts::*;
use crate::utils;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_black_list() -> Result<HashSet<PathBuf>, RifError> {

    let mut black_list: HashSet<PathBuf> = HashSet::new();

    // Const files that should be ignored
    black_list.extend(BLACK_LIST.to_vec().iter().map(|a| PathBuf::from(*a)).collect::<HashSet<PathBuf>>());

    // Rif ignore files
    let rif_ignore = read_rif_ignore()?;
    black_list.extend(rif_ignore);

    Ok(black_list)
}

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