pub mod rel;
pub mod config;
pub mod history;
pub mod hook;

use crate::models::LoopBranch;
use crate::utils;
use std::fs::metadata;
use std::collections::HashSet;
use config::Config;
use rel::Relations;
use history::History;
use crate::RifError;
use std::path::{Path, PathBuf};
use crate::consts::*;

pub struct Rif {
    config: Config,
    history: History,
    relation: Relations,
    black_list: HashSet<PathBuf>,
    root_path: Option<PathBuf>,
}

impl Rif {
    pub fn new(path: Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let config = Config::read_from_file(path.as_ref())?;
        let black_list = utils::get_black_list(config.git_ignore)?;
        Ok(Self {
            config,
            history: History::read_from_file(path.as_ref())?,
            relation: Relations::read_from_file(path.as_ref())?,
            black_list,
            root_path: path.map(|p| p.as_ref().clone().to_owned()),
        })
    }

    pub fn init(path: Option<impl AsRef<Path>>,create_rif_ignore: bool) -> Result<(), RifError> {
        let path = if let Some(path) = path {
            path.as_ref().to_owned()
        } else { std::env::current_dir()? };

        // Already initiated
        if path.exists() {
            return Err(RifError::RifIoError(String::from("Directory is already initiated")));
        }

        // Crate root directory
        std::fs::create_dir(path.join(RIF_DIECTORY))?;

        // Rif relation
        let new_relations = Relations::new();
        new_relations.save_to_file(Some(&path))?;
        // Rif history
        let new_rif_history = History::new();
        new_rif_history.save_to_file(Some(&path))?;
        // Rif Config
        let new_config = Config::new();
        new_config.save_to_file(Some(&path))?;
        // User feedback
        println!("Initiated a rif directory \"{}\"", std::env::current_dir()?.display());

        // Also crate rifignore file
        if create_rif_ignore {
            std::fs::write(".rifignore", "target
build
target
.git")?;
        }
        Ok(())
    }

    pub fn add(&mut self, files: Vec<&str>) -> Result<(), RifError> {
        let argc = files.len();

        for file in files {
            let mut path = PathBuf::from(file);

            // File is in rif ignore
            if let Some(_) = self.black_list.get(&path) {
                // If argument's count is 1 display specific logs
                // to help user to grasp what is happening
                if argc == 1 {
                    // If File is not configurable
                    // It's not allowed by the program
                    // else it's allowd by the program 
                    if BLACK_LIST.to_vec().contains(&file) {
                        eprintln!("File : \"{}\" is not allowed", file);
                    } else {
                        println!("\"{}\" is in rifignore file, which is ignored.", file)
                    }
                }
                continue;
            }

            // If given value is "." which means that input value has not been expanded.
            // substitute with current working directory
            if path.to_str().unwrap() == "." {
                path = std::env::current_dir()?;
            } else {
                // TODO 
                // Simply remove it?
                // Enable directory to be passed as argument
                // If given path is a directory
                if path.is_dir() && argc == 1 {
                    println!("Directory cannot be added to rif.");
                }
            }

            // Closure to recursively get inside directory and add files
            let mut closure = |entry_path : PathBuf| -> Result<LoopBranch, RifError> {
                let striped_path = utils::relativize_path(&entry_path)?;

                // Early return if file or directory is in black_list
                // Need to check the black_list once more because closure checks nested
                // directory that is not checked in outer for loop
                if let Some(_) = self.black_list.get(&striped_path) {
                    if striped_path.is_dir() {
                        return Ok(LoopBranch::Exit);
                    } 
                    else {
                        return Ok(LoopBranch::Continue);
                    }
                }

                if !self.relation.add_file(&striped_path)? { return Ok(LoopBranch::Continue); }
                Ok(LoopBranch::Continue)
            }; // Closure end here 

            // if path is a directory then recusively get into it
            // if path is a file then simply add a file
            if metadata(file)?.is_dir() {
                utils::walk_directory_recursive( &path, &mut closure)?;
            } else { 
                path = utils::relativize_path(&path)?;

                // File was not added e.g. file already exists
                if !self.relation.add_file(&path)? { continue; }
            }
        } // for loop end
        self.relation.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    pub fn commit(&self) -> Result<(), RifError> {
        Ok(())
    }

    pub fn rename(&self) -> Result<(), RifError> {
        Ok(())
    }

    pub fn remove(&self) -> Result<(), RifError> {
        Ok(())
    }

    pub fn set(&self) -> Result<(), RifError> {
        Ok(())
    }

    pub fn unset(&self) -> Result<(), RifError> {
        Ok(())
    }

    pub fn status(&self) -> Result<(), RifError> {
        Ok(())
    }
}
