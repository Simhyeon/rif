use std::collections::{ HashMap, HashSet };
use std::path::PathBuf;

use colored::*;
use itertools::Itertools;
use serde::{ Serialize, Deserialize };
use crate::models::{ 
    enums::{ SanityType, RefStatus, FileStatus },
    rif_error::RifError,
    single_file::SingleFile,
};
use crate::utils::{self, LoopBranch};

/// RifList is a struct that stores all information about rif 
#[derive(Serialize, Deserialize, Debug)]
pub struct RifList {
    pub files: HashMap<PathBuf, SingleFile>,
}

impl std::fmt::Display for RifList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for path in self.files.keys().sorted() {
            let file_output = self.display_file(path);
            output.push_str(&format!("{}\n", file_output));
        }
        write!(f, "{}", output)
    }
}

impl RifList {
    pub fn new() -> Self {
        Self {  
            files: HashMap::new(),
        }
    }

    /// Display a single file as human readable form
    ///
    /// # Args
    ///
    /// * `path` - A file path(name) to display
    ///
    /// # Display format
    ///
    /// \> <FILE_NAME> <STATUS>
    /// | [REFS]
    /// - > <FILE> <STATUS>
    pub fn display_file(&self, path: &PathBuf) -> String {
        let single_file = self.files.get(path).unwrap();
        let current_time = single_file.timestamp;
        let mut file_output = String::new();

        file_output.push_str(
            &format!("> {} {}", path.to_str().unwrap().green(), single_file.status)
        );

        for ref_item in single_file.references.iter() {
            file_output.push_str(&format!("\n- > {} {}", ref_item.display(), self.files.get(ref_item).unwrap().status));
            if current_time < self.files.get(ref_item).unwrap().timestamp {
                file_output.push_str(&format!(" {}", "Updated".yellow()));
            }
            if let FileStatus::Stale = single_file.status {
                if current_time < self.files.get(ref_item).unwrap().timestamp {
                    print!(" {}", "Updated".yellow());
                }
            }
            file_output.push_str("\n");
        }

        file_output
    }

    /// Print rif relation tree with given nested level
    ///
    /// # Args
    ///
    /// * `depth` - Desired depth value to display
    pub fn display_file_depth(&self, depth: u8) -> Result<(), RifError> {
        for path in self.files.keys().cloned().sorted() {
            // This should always work theoritically
            let single_file = self.files.get(&path).unwrap();
            print!("> {} {}\n", path.to_str().unwrap().green(), single_file.status);
            if single_file.references.len() != 0 && depth != 1 {
                self.display_file_recursive(&path, std::cmp::max(1, depth) - 1, 1)?;
                print!("\n");
            }
        }
        Ok(())
    }

    /// Display recursively
    ///
    /// Internal function used by display_file_depth
    /// # Args
    ///
    /// * `path` - A file path(name) to display
    /// * `current_depth` - Current depth in recursion
    fn display_file_recursive(&self, path: &PathBuf, current_depth: u8, indent_level: u8) -> Result<(), RifError> {
        let parent_file = self.files.get(path).unwrap();
        let current_time = parent_file.timestamp;

        for ref_item_key in parent_file.references.iter() {
            let ref_item = self.files.get(ref_item_key).unwrap();
            for _ in 0..indent_level {
                print!("  ");
            }
            print!("- > {} {}", ref_item_key.display(), ref_item.status);
            if let FileStatus::Stale = parent_file.status {
                if current_time < ref_item.timestamp {
                    print!(" {}", "Updated".yellow());
                }
            }
            print!("\n");
            if ref_item.references.len() != 0 && current_depth != 1 {
                // if given value is 0, then it prints whole tree
                self.display_file_recursive(ref_item_key, std::cmp::max(1, current_depth) - 1, indent_level + 1)?;
            }
        }

        Ok(())
    }

    /// Add file to rif list
    ///
    /// Fails when path doesn't exist. 
    /// Sanity is checked after addition.
    /// # Args
    ///
    /// * `file_path` - A file path to add 
    pub fn add_file(&mut self, file_path: &PathBuf) -> Result<bool, RifError> {
        if file_path.is_dir() { return Ok(false); }

        // If file exists then executes.
        if file_path.exists() {
            if let None = self.files.get(file_path) {
                self.files.insert(file_path.clone(), SingleFile::new(file_path.to_path_buf()));
            } else {
                return Ok(false);
            }
        } else {
            return Err(RifError::AddFail("Invalid file path: Path doesn't exist".to_owned()));
        }

        self.sanity_check_file(file_path, SanityType::Direct)?;

        Ok(true)
    }

    /// Remove file from rif
    ///
    /// Path validity doesn't matter.
    /// # Args
    ///
    /// * `file_path` - File path(name) to remove from rif.
    pub fn remove_file(&mut self, file_path: &PathBuf) -> Result<bool, RifError> {
        if let None = self.files.remove(file_path) {
            return Ok(false);
        }

        for (_, file) in self.files.iter_mut() {
            file.references.remove(file_path);
        }

        Ok(true)
    }

    /// Update filestamp of file
    ///
    /// Update file's timestamp and last modified time into file's system last modified time.
    /// # Args
    ///
    /// * `file_path` - File path(name) to update timestamp
    pub fn update_filestamp(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_file_unix_time(file_path)?;
                file.timestamp = unix_time; 
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }

        Ok(())
    }

    /// Forcefully update filestamp of file
    ///
    /// Update file's timestamp into current unix time.
    /// # Args
    ///
    /// * `file_path` - File path(name) to update timestamp
    pub fn update_filestamp_force(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_current_unix_time();
                file.timestamp = unix_time; 
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }

        Ok(())
    }

    /// Discard file modification
    ///
    /// Retain file's timestamp and only update last modified time.
    /// # Args
    ///
    /// * `file_path` - File path(name) to discard modification
    pub fn discard_change(&mut self, file_path: &PathBuf) -> Result<(), RifError> {
        if file_path.exists() {
            if let Some(file) = self.files.get_mut(file_path) {
                let unix_time = utils::get_current_unix_time();
                // Only Update last_modified
                file.last_modified = unix_time;
            } else {
                return Err(RifError::GetFail(String::from("Failed to get file from rif_list")));
            }
        } else {
            return Err(RifError::GetFail(String::from("File doesn't exist")));
        }

        Ok(())
    }

    /// Set references to a file
    ///
    /// This is union operation thus existing files are not affected.
    /// References should be existent in rif list to be added to a file.
    /// # Args
    ///
    /// * `file_path` - File path(name) to add references
    /// * `ref_files` - File pahts to set as references
    pub fn add_reference(&mut self, file_path: &PathBuf, ref_files: &HashSet<PathBuf>) -> Result<(), RifError> {
        // If file doesn't exist, return error
        for file in ref_files.iter() {
            if !file.exists() {
                return Err(RifError::AddFail(format!("No such reference file exists : {}", file.display())));
            }
            if let None = self.files.get(file) {
                return Err(RifError::AddFail(format!("No such reference file exists in rif : {}", file.display())));
            }
        }

        if let Some(file) = self.files.get_mut(file_path) {
            file.references = file.references.union(ref_files).cloned().collect();
            self.sanity_check()?;
            Ok(())
        } else {
            Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()))
        }
    }

    /// Unset references from a file
    ///
    /// This is minus operation thus references don't have to be valid path.
    /// # Args
    ///
    /// * `file_path` - File path(name) to discard modification
    /// * `ref_files` - File pahts to unset as references
    pub fn remove_reference(&mut self, file_path: &PathBuf, ref_files: &HashSet<PathBuf>) -> Result<(), RifError> {
        // Remove doesn't check existences
        // Becuase artifacts cannot be fixed easily if it is
        if let Some(file) = self.files.get_mut(file_path) {
            file.references = &file.references - ref_files;
            self.sanity_check()?;
            Ok(())
        } else {
            Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()))
        }
    }

    /// Set status for a file
    ///
    /// # Args
    ///
    /// * `file_path` - File path(name) to set a status
    /// * `file_status` - File status to set for the file
    pub fn set_file_status(&mut self, file_path: &PathBuf, file_status: FileStatus) -> Result<(), RifError> {
        if let Some(file) = self.files.get_mut(file_path) {
            file.status = file_status;
        } else {
            return Err(RifError::GetFail("Failed to set status of a file : Non existant.".to_owned()));
        }

        Ok(())
    }

    /// Check sanity of rif list
    ///
    /// Sanity is assured when: file is not referencing itself,
    /// file referencing conclues to infinite loop
    pub fn sanity_check(&self) -> Result<(), RifError> {
        for path in self.files.keys() {
            self.sanity_check_file(path, SanityType::Indirect)?;
        }
        Ok(())
    }

    /// Internal function for single file sanity checking
    ///
    /// This method is used by sanity_check method.
    /// If sanity type is indirect, only self referncing is checked
    /// else if sanity type is direct, infnite loop is also checked.
    /// Infinite loop is detected by following references recursively.
    /// If recursion finds out repeating file_path then it is infinite loop.
    /// # Args
    ///
    /// * `target_path` - File path to check sanity
    /// * `sanity_type` - Sanity checking type 
    fn sanity_check_file(&self, target_path: &PathBuf, sanity_type: SanityType) -> Result<(), RifError> {
        // NOTE ::: Question - Why sanity type exists?
        // Answer - |
        // Because indirect reference checking goes through all files and references which is not
        // possible in such scenario that only some files were added into rif_list
        // but direct checking is also needed to early return from easily detectable invalid case.
        // Thus there exists two types and indirect check also does direct checks
        
        // Check if file exists in the first place
        if !target_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", target_path.display())));
        }

        // Check direct self reference.
        let first_item = self.files.get(target_path).unwrap().references.iter().filter(|a| *a == target_path).next();
        // If the first exists, then target_path has same file in its reference
        if let Some(_) = first_item {
            return Err(RifError::InvalidFormat(format!("File \"{}\" is referencing itself which is not allowd", target_path.display())));
        }

        // Also check indirect self references.
        if let SanityType::Indirect = sanity_type {
            // Recursively check into children and check if the item is same with target_path
            let current_file = self.files.get(target_path).unwrap();

            // File has no references, then early return
            if current_file.references.len() == 0 {
                return Ok(());
            }

            // Variable for cached status
            let mut ref_status = RefStatus::Valid;

            // Recursively check
            for child in self.files.get(target_path).unwrap().references.iter() {
                self.recursive_check(target_path, child, &mut ref_status)?;
            }

            if let RefStatus::Invalid = ref_status {
                return Err(RifError::InvalidFormat(format!("Infinite reference loop detected. Last loop was \"{}\"", target_path.display())));
            }
        }

        Ok(())
    }

    /// Internal function for sanity checking
    ///
    /// This method recursively follows file references and find if referencing file is same with parent or ifinite loop is detected.
    /// # Args
    ///
    /// * `origin_path` - Base comparator of recursion. If origin path is detected it is invalid.
    /// * `current_path` - Current recursion path.
    /// * `ref_status` - Current status of references; It is either invalid or valid.
    ///
    /// # Example
    ///
    /// Assume file references are as followed
    /// `base -> child -> granchild -> base`
    ///
    /// On recursion where origin_path is base and current_path is grandchild, it will set ref_status as invalid.
    /// 
    fn recursive_check(&self, origin_path: &PathBuf, current_path: &PathBuf, ref_status: &mut RefStatus) -> Result<(), RifError> {
        // if current path is not existent return erro
        if !current_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", current_path.display())));
        }

        if origin_path == current_path {
            *ref_status = RefStatus::Invalid;
        } else if let RefStatus::Valid = ref_status {
            for child in self.files.get(current_path).unwrap().references.iter() {
                // Current path is same with child which means self referencing 
                if current_path == child {
                    return Err(RifError::InvalidFormat(format!("File \"{}\" is referencing itself which is not allowd", current_path.display())));
                }

                self.recursive_check(origin_path, child, ref_status)?;
            }
        }

        Ok(())
    }

    /// Fix invalid format so that sanity can be retained.
    ///
    /// Repeatedly find invalid referecning and fix until sanity check succeeds.
    pub fn sanity_fix(&mut self) -> Result<(), RifError> {
        while let Err(_) = self.sanity_check() {
            for path in self.files.keys().cloned() {
                if let Ok(Some((parent, child))) = self.sanity_get_invalid(&path) {
                    println!("Found invalid {} - {}", parent.display(), child.display());
                    self.files.get_mut(&parent).unwrap().references.remove(&child);
                    break;
                } else {
                    continue;
                }
            }
        }

        Ok(())
    }

    /// Find a invalid reference from rif list 
    ///
    /// # Args
    ///
    /// * `target_path` - Target file to start sanity checking
    fn sanity_get_invalid(&self, target_path: &PathBuf) -> Result<Option<(PathBuf, PathBuf)>, RifError> {
        // Check if file exists in the first place
        // This is mandatory because sanity_fix doesn't check sanity when they read file into
        // struct. Thus path might not really exist.
        if !target_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", target_path.display())));
        }

        // Check direct self reference.
        let first_item = self.files.get(target_path).unwrap().references.iter().filter(|a| *a == target_path).next();
        // If the first exists, then target_path has same file in its reference
        if let Some(path) = first_item {
            return Ok(Some((path.clone(), path.clone())));
        }

        // Recursively check into children and check if the item is same with target_path
        let current_file = self.files.get(target_path).unwrap();

        // File has no references, then early return
        if current_file.references.len() == 0 {
            return Ok(None);
        }

        // Variable for status cache
        let mut ref_status = RefStatus::Valid;

        // Recursively check
        for child in self.files.get(target_path).unwrap().references.iter() {
            return Ok(self.recursive_find_invalid(target_path, child, &mut ref_status)?);
        }

        Ok(None)
    }

    /// Internal function find invalid reference
    ///
    /// This function is used by sanity_get_valid
    /// Basic logic is very similar to that of recursive_check but it returns invalid reference to caller.
    /// # Args
    ///
    /// * `origin_path` - Base comparator of recursion. If origin path is detected it is invalid.
    /// * `current_path` - Current recursion path.
    /// * `ref_status` - Current status of references; It is either invalid or valid.
    fn recursive_find_invalid(&self, origin_path: &PathBuf, current_path: &PathBuf, ref_status: &mut RefStatus) -> Result<Option<(PathBuf, PathBuf)>, RifError> {
        // if current path is not existent return error
        if !current_path.exists() {
            return Err(RifError::GetFail(format!("File {} doesn't exist", current_path.display())));
        }

        if origin_path == current_path {
            return Ok(Some((current_path.clone(), origin_path.clone())));
        } else if let RefStatus::Valid = ref_status {
            for child in self.files.get(current_path).unwrap().references.iter() {
                // Current path is same with child which means self referencing 
                if current_path == child {
                    println!("LOG ::: Child is referencing itself");
                    return Ok(Some((child.clone(), child.clone())));
                }

                return Ok(self.recursive_find_invalid(origin_path, child, ref_status)?);
            }
        }

        Ok(None)
    }

    /// Track and print modified files 
    ///
    /// Modfication is determined by comparing rif's last modified and system's modifid time.
    /// If rif's last modified time is oldere than system's modified time, it is considered as modified.
    pub fn track_modified_files(&self) -> Result<(), RifError> {
        let mut display_text: String = String::new();
        let mut modified: Vec<&PathBuf> = vec![];

        for (path, file) in self.files.iter() {
            let system_time = utils::get_file_unix_time(path)?;
            if file.last_modified < system_time {
                modified.push(path);
            }
        }

        if modified.len() != 0 {
            for file in modified.iter() {
                display_text.push_str(&format!("    {}\n", file.display().to_string().red()));
            }
        } else {
            display_text.push_str(&format!("    All files are up to date.\n"));
        }

        print!("{}", display_text);
        Ok(())
    }

    /// Get list of modified files
    ///
    /// Logic is very similar to track_modified_files but it returns list of modified files.
    pub fn get_modified_files(&self) -> Result<Vec<PathBuf>, RifError> {
        let mut modified: Vec<PathBuf> = vec![];

        for (path, file) in self.files.iter() {
            let system_time = utils::get_file_unix_time(path)?;
            if file.last_modified < system_time {
                modified.push(path.clone());
            }
        }

        Ok(modified)
    }

    /// Track and display unregistered files
    ///
    /// Unregistered file is a file which exists in under directory where rif file resides.
    ///
    /// # Args
    ///
    /// * `black_list ` - Blacklists to to ignore when tracking unregistered files
    pub fn track_unregistered_files(&self, black_list: &HashSet<PathBuf>) -> Result<(), RifError> {
        utils::walk_directory_recursive(&std::env::current_dir()?, &mut | walk_path | -> Result<LoopBranch, RifError> {
            let stripped = utils::strip_path(&walk_path, None)?;
            // Path is not in black list else, is in the black list
            if !black_list.contains(&stripped) {
                if !stripped.is_dir() {
                    // File is not in tracked files
                    if let None = self.files.get(&walk_path) {
                        println!("    {}", stripped.display().to_string().red());
                    }
                }
                Ok(LoopBranch::Continue)
            } else {
                // If path is directory than ignore further cases
                if stripped.is_dir() {
                    Ok(LoopBranch::Exit)
                } 
                // if path is a file then check other files that is located in same directory as of 
                // the given file.
                else {
                    Ok(LoopBranch::Continue)
                }
            }
        })?;

        Ok(())
    }
}
