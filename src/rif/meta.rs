use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use crate::RifError;
use crate::utils;

/// Meta information related to rif directory
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Meta {
    pub to_be_forced: HashSet<PathBuf>,
    pub to_be_added: HashSet<PathBuf>,
    pub to_be_registerd: HashSet<PathBuf>,
    pub to_be_deleted: HashSet<PathBuf>,
}

impl<'a> Meta {
    pub fn new() -> Self {
        Self {
            to_be_added: HashSet::new(),
            to_be_forced: HashSet::new(),
            to_be_registerd: HashSet::new(),
            to_be_deleted : HashSet::new(),
        }
    }

    /// Add to_be_added
    pub fn queue_added(&mut self, file : &Path, force: bool) {
        if !self.to_be_added.contains(file) && !self.to_be_forced.contains(file) {
            if force {
                self.to_be_forced.insert(file.to_owned());
            } else {
                self.to_be_added.insert(file.to_owned());
            }
        }
    }

    /// Add to_be_added
    pub fn queue_deleted(&mut self, file : &Path) {
        self.remove_add_queue(file);
        self.to_be_deleted.insert(file.to_owned());
    }

    pub fn remove_add_queue(&mut self, file : &Path) {
        if self.to_be_added.contains(file) || self.to_be_forced.contains(file) {
            self.to_be_forced.remove(file);
            self.to_be_added.remove(file);
        }
    }

    pub fn remove_non_exsitent(&mut self) {
        self.to_be_added.retain(|path| path.exists());
        self.to_be_forced.retain(|path| path.exists());
        self.to_be_registerd.retain(|path| path.exists());
    }

    pub fn clear(&mut self) {
        self.to_be_added.clear();
        self.to_be_forced.clear();
        self.to_be_deleted.clear();
        self.to_be_registerd.clear();
    }

    pub fn to_be_added_later(&'a self) -> impl Iterator<Item = &PathBuf> + 'a {
        self.to_be_added.iter().chain(self.to_be_forced.iter()).chain(self.to_be_registerd.iter()).chain(self.to_be_deleted.iter())
    }

    pub fn read_from_file(path: Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let path = utils::get_meta_path(path)?;
        let result = bincode::deserialize::<Self>(&std::fs::read(path)?);
        match result {
            Err(err) => {
                Err(RifError::BincodeError(err))
            }
            Ok(history) => {
                Ok(history)
            }
        }
    }

    pub fn save_to_file(&self, path: Option<impl AsRef<Path>>) -> Result<(), RifError> {
        let result = bincode::serialize(self);
        let path = utils::get_meta_path(path)?;
        if let Err(err) = result {
            Err(RifError::BincodeError(err))
        } else {
            std::fs::write(path, result.unwrap())?;
            Ok(())
        }
    }
}
