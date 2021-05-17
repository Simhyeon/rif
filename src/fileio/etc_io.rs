use std::path::PathBuf;
use crate::models::rif_error::RifError;
use std::collections::HashSet;
use crate::consts::*;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};


pub fn read_rif_ignore() -> Result<HashSet<PathBuf>, RifError> {
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
