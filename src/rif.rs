pub mod rel;
pub mod config;
pub mod history;
pub mod hook;
pub mod meta;

use crate::checker::Checker;
use crate::models::{LoopBranch, ListType};
use crate::utils;
use std::collections::HashSet;
use config::Config;
use rel::Relations;
use history::History;
use meta::Meta;
use crate::RifError;
use std::path::{Path, PathBuf};
use crate::consts::*;

/// Rif struct stores all iformation necessary for rif operations
pub struct Rif {
    config: Config,
    history: History,
    relation: Relations,
    meta: Meta,
    black_list: HashSet<PathBuf>,
    root_path: Option<PathBuf>,
}

impl Rif {
    /// Create new rif struct
    pub fn new(path: Option<impl AsRef<Path>>) -> Result<Self, RifError> {
        let config = Config::read_from_file(path.as_ref())?;
        let black_list = utils::get_black_list(config.git_ignore)?;
        Ok(Self {
            config,
            history: History::read_from_file(path.as_ref())?,
            relation: Relations::read_from_file(path.as_ref())?,
            meta: Meta::read_from_file(path.as_ref())?,
            black_list,
            root_path: path.map(|p| p.as_ref().clone().to_owned()),
        })
    }

    // ==========
    // External methods start

    /// Initiate given directory
    ///
    /// If directory is not supplied, initiates current working directory
    pub fn init(path: Option<impl AsRef<Path>>,create_rif_ignore: bool) -> Result<(), RifError> {
        let path = if let Some(path) = path {
            path.as_ref().to_owned()
        } else { std::env::current_dir()? };

        // Already initiated
        if path.join(RIF_DIECTORY).exists() {
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
        // Rif meta
        let new_meta = Meta::new();
        new_meta.save_to_file(Some(&path))?;
        println!("Initiated a rif directory \"{}\"", std::env::current_dir()?.display());

        // Also create rifignore file
        if create_rif_ignore {
            std::fs::write(".rifignore",".git")?;
        }
        Ok(())
    }

    /// Add new file 
    ///
    /// Files that modified, newly created, deleted files can be added but non modiifed files can
    /// alos be added with force option
    pub fn add(&mut self, files: &Vec<impl AsRef<Path>>, force: bool) -> Result<(), RifError> {
        for file in files {
            let mut path = file.as_ref().to_owned();

            // If file doesn't exist, simply ignore
            if !path.exists() {
                continue;
            }

            // If given value is "." which means that input value has not been expanded.
            // substitute with current working directory
            if path.to_str().unwrap() == "." {
                path = std::env::current_dir()?;
                self.add_directory(&path)?;
                continue;
            } else if path.is_dir() {
                self.add_directory(&path)?;
                continue;
            }

            // Don't do anything if file is in blacklist
            if self.is_in_black_list(&path) {
                continue;
            }

            // First, file is already inside
            // Second, file is new
            if self.relation.files.contains_key(&path) {
                self.add_old_file(&path, force)?;
            } else {
                self.add_new_file(&path)?;
            }
        } // for loop end

        // Update relation file
        self.relation.save_to_file(self.root_path.as_ref())?;
        self.meta.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Revert added files
    ///
    /// No files arugment revert all added files
    pub fn revert(&mut self, files: Option<&Vec<impl AsRef<Path>>>) -> Result<(), RifError> {
        if let Some(files) = files {
            for file in files {
                let path = file.as_ref();
                // Removes single item
                self.meta.remove_add_queue(&path);
            } // for loop end
        } else {
            // No argument, revert everything
            self.meta.clear();
        }

        self.meta.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Commit addition to rif struct and check impact
    ///
    /// Message is saved inside history file
    pub fn commit(&mut self, message: Option<&str>) -> Result<(), RifError> {

        // Literaly, commit needs to resolve all deleted files
        if self.relation.get_deleted_files().len() != self.meta.to_be_deleted.len() {
            return Err(RifError::CommitFail("Commit without deleted files are illegal. Rejected".to_owned()))
        }

        // delete
        for file in self.meta.to_be_deleted.clone().iter() {
            self.remove_file(file)?;
        }

        // Register new files
        for file in self.meta.to_be_registerd.clone().into_iter() {
            self.register_new_file(&file, message)?;
        }

        // force updates
        for file in self.meta.to_be_forced.iter() {
            self.relation.update_filestamp_force(&file)?;
        }

        // updates
        for file in self.meta.to_be_added.iter() {
            self.relation.update_filestamp(&file)?;

            // Add message to history
            if let Some(msg) = message {
                self.history.add_history(&file, msg)?;
                self.history.save_to_file(self.root_path.as_ref())?;
            }
        }

        // Check if added files are not empty
        if self.meta.to_be_added_later().count() != 0 {
            self.check_exec()?;
        }

        // Clear meta
        self.meta.clear();

        // Save files
        self.meta.save_to_file(self.root_path.as_ref())?;
        self.relation.save_to_file(self.root_path.as_ref())?;
        self.history.save_to_file(self.root_path.as_ref())?;

        Ok(())
    }
    
    /// Discard file change and updated filestamp
    ///
    /// This cannot be reverted so multiple files are not supported
    pub fn discard(&mut self, file: impl AsRef<Path>) -> Result<(), RifError> {
        self.relation.discard_change(file.as_ref())?;
        self.relation.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    // TODO
    // Needs serious consideration
    // Whether new_name exists in real directory
    // Whether sany check is activated so that insane rename should be not executed
    /// Rename rif file 
    ///
    /// If file exist, change the file name in filesystem
    pub fn rename(&mut self, source_name: &str, new_name: &str) -> Result<(), RifError> {
        let source_name = Path::new(source_name);
        let new_name = Path::new(new_name);
        if let Some(_) = self.relation.files.get(new_name) {
            return Err(RifError::RenameFail(format!("Rename target: \"{}\" already exists", new_name.display())));
        }

        // Rename file if it exsits and inside relation files
        if source_name.exists() && self.relation.files.contains_key(source_name) {
            if !new_name.exists() {
                std::fs::rename(source_name, new_name)?;
            } else {
                return Err(RifError::RenameFail("New name already exists".to_owned()));
            }
        }

        self.relation.rename_file(source_name, new_name)?;
        self.relation.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Remove file from rif
    pub fn remove(&mut self, files: &Vec<impl AsRef<Path>>) -> Result<(), RifError> {
        for file in files {
            self.remove_file(file.as_ref())?;
        }
        self.relation.save_to_file(self.root_path.as_ref())?;
        self.history.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Set reference of file
    pub fn set(&mut self, file: &Path, refs : &Vec<impl AsRef<Path>>) -> Result<(), RifError> {
        let refs: HashSet<PathBuf> = refs.iter().map(|a| a.as_ref().to_owned()).collect();

        self.relation.add_reference(file, &refs)?;
        self.relation.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Unset reference of file
    pub fn unset(&mut self, file: &Path, refs : &Vec<impl AsRef<Path>>) -> Result<(), RifError> {
        let refs: HashSet<PathBuf> = refs.iter().map(|a| a.as_ref().to_owned()).collect();

        self.relation.remove_reference(file, &refs)?;
        self.relation.save_to_file(self.root_path.as_ref())?;
        Ok(())
    }

    /// Show current status of rif project
    pub fn status(&mut self, ignore: bool, verbose: bool) -> Result<(), RifError> {
        // Remove deleted files from to be added.
        self.meta.remove_non_exsitent();

        let mut to_be_added_later = self.meta.to_be_added_later().peekable();

        if let Some(_) = to_be_added_later.peek() {
            println!("# Changes to be commited :");
            for item in &self.meta.to_be_registerd {
                let format = format!("    new file : {}", &item.to_str().unwrap());
                println!("{}", utils::green(&format));
            }
            for item in &self.meta.to_be_added {
                let format = format!("    modified : {}", &item.to_str().unwrap());
                println!("{}", utils::green(&format));
            }
            for item in &self.meta.to_be_forced {
                let format = format!("    forced   : {}", &item.to_str().unwrap());
                println!("{}", utils::green(&format));
            }
            for item in &self.meta.to_be_deleted {
                let format = format!("    deleted  : {}", &item.to_str().unwrap());
                println!("{}", utils::green(&format));
            }
            println!("");
        }
    
        println!("# Changed files :");
        self.relation.track_modified_files(self.meta.to_be_added_later())?;

        // Ignore untracked files
        if !ignore {
            // Default black list only includes .rif file for now
            // Currently only check relative paths,or say, stripped path
            println!("\n# Untracked files :");
            self.relation.track_unregistered_files(&self.black_list, &self.meta.to_be_registerd)?;
        }

        if verbose {
            println!("\n# Current rif status:\n---");
            print!("{}", self.relation);
        }

        // Save meta file
        self.meta.save_to_file(self.root_path.as_ref())?;

        Ok(())
    }

    /// Show file informations of rif project
    pub fn list(&self, file : Option<impl AsRef<Path>>, list_type: ListType, depth: Option<usize>) -> Result<(), RifError> {
        if let Some(file) = file {
            // Print relation tree
            self.relation.display_file_depth(file.as_ref(), 0)?;

            // Also print update 
            println!("\n# History : ");
            self.history.print_history(file.as_ref())?;
        }  else { // No file was given
            match list_type {
                ListType::All => {
                    self.relation.display_depth(depth.unwrap_or(0))?;
                }
                ListType::Stale => {
                    self.relation.display_stale_files(depth.unwrap_or(0))?;
                }
                // Updated is not yet added
                _ => (),
            }
        }
        Ok(())
    }

    /// Show data of rif project
    pub fn data(&self, data_type: Option<&str>, compact: bool) -> Result<(), RifError> {
        if let Some(data_type) = data_type {
            match data_type {
                "meta"  => {
                    println!("{:#?}", self.meta);
                }
                "history" => {
                    println!("{:#?}", self.history);
                }
                _ => () // This doesn't happen
            }
        } else {
            if compact {
                println!("{:?}", self.relation);
            } else {
                println!("{:#?}", self.relation);
            }
        }

        Ok(())
    }

    /// Show files that depend on given file
    pub fn depend(&self, file: &Path)  -> Result<(), RifError> {
        let dependes = self.relation.find_depends(file)?;
        println!("Files that depends on \"{}\"", file.display());
        println!("=====");
        for item in dependes {
            println!("{}", utils::green(&item.display().to_string()));
        }
        Ok(())
    }

    /// Check file references
    pub fn check(&mut self) -> Result<(), RifError> {
        if self.relation.get_deleted_files().len() != 0 {
            return Err(RifError::CheckerError("Check with deleted files are illegal. Rejected".to_owned()));
        }

        self.check_exec()?;
        Ok(())
    }

    /// Check sanity of rif proeject
    pub fn sanity(&mut self, fix: bool) -> Result<(), RifError> {
        // NOTE ::: You don't have to manually call sanity check
        // Because read operation always check file sanity after reading a file
        // and return erros if sanity was not assured.
        if fix {
            self.relation.sanity_fix()?;
            self.relation.save_to_file(self.root_path.as_ref())?;
            println!("Sucessfully fixed the rif file");
        } else {
            self.relation.sanity_check()?;
            println!("Sucessfully checked the rif file");
        }
        Ok(())
    }

    // External methods end

    // MISC methods start
    //
    
    /// Check file relations(impact of changes)
    fn check_exec(&mut self) -> Result<(), RifError> {
        // Check relations(impact)
        let mut checker = Checker::with_relations(&self.relation)?;
        let changed_files = checker.check(&mut self.relation)?;

        if changed_files.len() != 0 && self.config.hook.trigger {
            println!("\nHook Output");
            self.config.hook.execute(changed_files)?;
        }

        Ok(())
    }
    
    /// Check if given path is inside black_list
    fn is_in_black_list(&self, path: &Path) -> bool {
        // File is in rif ignore
        if let Some(_) = self.black_list.get(path) {
            // If File is not configurable
            // It's not allowed by the program
            // else it's allowd by the program 
            if BLACK_LIST.to_vec().contains(&path.to_str().unwrap()) {
                eprintln!("File : \"{}\" is not allowed", path.display());
            } else {
                println!("\"{}\" is in rifignore file, which is ignored.", path.display());
            }
            return true;
        } 

        false
    }

    /// Add new file to rif 
    fn add_new_file(&mut self, file: &Path) -> Result<(), RifError> {
        self.meta.to_be_registerd.insert(file.to_owned());
        Ok(())
    }

    /// Remove new file to rif 
    fn remove_file(&mut self, file: &Path) -> Result<(), RifError> {
        self.relation.remove_file(file)?;
        self.history.remove_file(file)?;
        Ok(())
    }

    /// Register a new file
    fn register_new_file(&mut self, file: &Path, message: Option<&str>) -> Result<(), RifError> {
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
        if file.is_dir() {
            utils::walk_directory_recursive(file, &mut closure)?;
        } else { 
            let file = utils::relativize_path(file)?;

            // TODO 
            // THis returns bools, is it not needed?
            // File was not added e.g. file already exists
            self.relation.add_file(&file)?;
            self.history.add_history(&file, message.unwrap_or(""))?;
        }
        Ok(())
    }
    
    /// Add directory
    fn add_directory(&mut self, dir: &Path) -> Result<(), RifError> {
        let tracked = self.relation.files.keys().cloned().collect::<Vec<PathBuf>>();
        let modified = self.relation.get_modified_files()?.clone();
        let mut deleted = self.relation.get_deleted_files().clone();
        let mut to_be_deleted = HashSet::new();
        let mut to_be_added = HashSet::new();
        let mut to_be_registerd = HashSet::new();
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

            // If directory go inside and don't add the directory
            if striped_path.is_dir() {
                // Only retain deleted files that are not specified in directory
                // or say, paths that don't start with entry path
                deleted.retain(|path| {
                    let is_inside = path.starts_with(&entry_path);
                    // Add to to_be_deleted
                    if is_inside {
                        to_be_deleted.insert(path.to_owned());
                    }
                    // Only retain files that is not inside the directory
                    !is_inside
                });
                return Ok(LoopBranch::Continue);
            } 

            if modified.contains(&striped_path) {
                // Is modified file
                to_be_added.insert(striped_path);
            } else if !tracked.contains(&striped_path) {
                // NOt in a tracking file
                to_be_registerd.insert(striped_path);
            }
            Ok(LoopBranch::Continue)
        }; // Closure end here 

        utils::walk_directory_recursive(dir, &mut closure)?;

        self.meta.to_be_registerd.extend(to_be_registerd);
        self.meta.to_be_added.extend(to_be_added);
        self.meta.to_be_deleted.extend(to_be_deleted);

        Ok(())
    }

    /// Add old file
    fn add_old_file(&mut self, file: &Path, force: bool) -> Result<(), RifError> {
        if file.exists() {
            self.meta.queue_added(file, force);
        } else {
            self.meta.queue_deleted(file);
        }
        Ok(())
    }

    // MISC methods end
}
