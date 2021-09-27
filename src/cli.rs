use std::collections::HashSet;
use std::path::{PathBuf, Path};

use colored::*;
use clap::clap_app;
use crate::checker::Checker;
use crate::rif::{hook::Hook, config::Config, history::History, rel::Relations};
use crate::RifError;
use crate::Rif;
use crate::utils;

/// Struct to parse command line arguments and execute proper operations
pub struct Cli{}
impl Cli {
    /// Parse program arguments and feed parsed values to subcommand parser
    pub fn parse() -> Result<(), RifError>{
        let cli_args = Cli::args_builder();
        Cli::parse_subcommands(&cli_args)?;
        Ok(())
    }

    /// Parse subcommands
    ///
    /// # Args
    /// * `args` - parsed arguments
    fn parse_subcommands(args: &clap::ArgMatches) -> Result<(), RifError> {
        Cli::subcommand_init(args)?;
        Cli::subcommand_add(args)?;
        Cli::subcommand_remove(args)?;
        Cli::subcommand_rename(args)?;
        Cli::subcommand_set(args)?;
        Cli::subcommand_unset(args)?;
        Cli::subcommand_update(args)?;
        Cli::subcommand_discard(args)?;
        Cli::subcommand_list(args)?;
        Cli::subcommand_check(args)?;
        Cli::subcommand_sanity(args)?;
        Cli::subcommand_status(args)?;
        Cli::subcommand_depend(args)?;
        Cli::subcommand_data(args)?;
        Ok(())
    }

    /// Creates argument matcher
    ///
    /// Argument matcher consists of clap macros
    fn args_builder() -> clap::ArgMatches {
        clap_app!(Rif =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Rif is a program to track impact of file changes")
            (@setting ArgRequiredElseHelp)
            (@subcommand add =>
                (about: "Add file to rif")
                (@arg FILE: ... +required "File to add")
            )
            (@subcommand data =>
                (about: "Print data as json format")
                (@arg TYPE: "Type of data to print, default is rif. Available options : <history>")
                (@arg compact: -c --compact "Print compact json without formatting")
            )
            (@subcommand depend =>
                (about: "Find files that depend on the file")
                (@arg FILE: ... +required "File to find dependencies")
            )
            (@subcommand remove =>
                (about: "Remove file from rif")
                (@arg FILE: ... +required "File to remove")
            )
            (@subcommand rename =>
                (about: "Rename a file")
                (@arg FILE: +required "File to rename")
                (@arg NEWNAME: +required "New file name")
            )
            (@subcommand set =>
                (about: "Set references to file")
                (@arg FILE: +required "File to change")
                (@arg REFS: ... +required "Files to set")
            )
            (@subcommand unset =>
                (about: "Unset references from file")
                (@arg FILE: +required "File to change")
                (@arg REFS: ... +required "Files to set")
            )
            (@subcommand update =>
                (about: "Update file's timestamp")
                (@arg FILE: +required "File to update")
                (@arg force: -f --force "Force update on file")
                (@arg check: -c --check "Check after update")
                (@arg message: -m --message +takes_value conflicts_with[force] "Message to add in update")
            )
            (@subcommand discard =>
                (about: "Discard file changes")
                (@arg FILE: +required "File to discard changes")
            )
            (@subcommand check =>
                (about: "Check file references")
                (@arg update: -u --update "Update all modified files before check")
            )
            (@subcommand sanity =>
                (about: "Check sanity of rif file")
                (@arg fix: --fix "Fix rif sanity")
            )
            (@subcommand init =>
                (about: "Initiate working directory")
                (@arg default: -d --default "Creates defult rifignore file")
            )
            (@subcommand status =>
                (about: "Show current status of rif")
                (@arg ignore: -i --ignore "Ignore untracked files")
                (@arg verbose: -v --verbose "Also print out list")
            )
            (@subcommand list =>
                (about: "Diplay all files from rif file")
                (@arg FILE: "File to list")
                (@arg depth: -d --depth +takes_value "Maximum depth for display tree(unsigned integer). 0 means print a whole tree")
            )
        ).get_matches()
    }

    /// Check if `add` subcommand was given and parse subcommand options
    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            if let Some(files) = sub_match.values_of("FILE") {
                let rif_path = utils::get_rif_directory()?;
                let mut rif = Rif::new(Some(&rif_path))?;
                rif.add(files.into_iter().collect::<Vec<&str>>())?;
            } 
        } 
        Ok(())
    }

    /// Check if `remove` subcommand was given and parse subcommand options
    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            let rif_path = utils::get_rif_directory()?;

            if let Some(files) = sub_match.values_of("FILE") {
                let mut relations = Relations::read_from_file(Some(&rif_path))?;
                for file in files {
                    let path = PathBuf::from(file);
                    if relations.remove_file(&path)? {
                        println!("Removed file: {}", file);
                    }
                }
                relations.save_to_file(Some(&rif_path))?;
            } 
        }
        Ok(())
    }

    /// Check if `rename` subcommand was given and parse subcommand options
    fn subcommand_rename(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("rename") {
            let rif_path = utils::get_rif_directory()?;

            // TODO
            // Check this process
            // Rename's target might already in rif_file
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(new_name) = sub_match.value_of("NEWNAME") {
                    let mut raw_relations = Relations::read_as_raw(Some(&rif_path))?;
                    if let Some(_) = raw_relations.files.get(&PathBuf::from(new_name)) {
                        return Err(RifError::RenameFail(format!("Rename target: \"{}\" already exists", new_name)));
                    }

                    let file_path = Path::new(file);
                    // Rename file if it exsits
                    if file_path.exists() {
                        std::fs::rename(file_path, new_name)?;
                    }
                    raw_relations.rename_file(&PathBuf::from(file), &PathBuf::from(new_name))?;
                    raw_relations.save_to_file(Some(&rif_path))?;
                    println!("Sucessfully renamed \"{}\" to \"{}\"", file, new_name);
                }
            } 
        }
        Ok(())
    }

    /// Check if `set` subcommand was given and parse subcommand options
    fn subcommand_set(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("set") {
            let rif_path = utils::get_rif_directory()?;

            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();
                    let mut relations = Relations::read_from_file(Some(&rif_path))?;

                    relations.add_reference(&path, &refs)?;
                    relations.save_to_file(Some(&rif_path))?;
                    println!("Added references to: {}", file);
                }
            } 
        } 
        Ok(())
    }

    /// Check if `unset` subcommand was given and parse subcommand options
    fn subcommand_unset(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("unset") {
            let rif_path = utils::get_rif_directory()?;

            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();
                    let mut relations = Relations::read_from_file(Some(&rif_path))?;

                    relations.remove_reference(&path, &refs)?;
                    relations.save_to_file(Some(&rif_path))?;
                    println!("Removed references from: {}", file);
                }
            } 
        } 
        Ok(())
    }

    /// Check if `update` subcommand was given and parse subcommand options
    fn subcommand_update(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("update") {
            let rif_path = utils::get_rif_directory()?;

            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut relations = Relations::read_from_file(Some(&rif_path))?;

                if sub_match.is_present("force") {
                    relations.update_filestamp_force(&path)?;
                } else {
                    relations.update_filestamp(&path)?;

                    if let Some(msg) = sub_match.value_of("message") {
                        let mut history = History::read_from_file(Some(&rif_path))?;
                        history.add_history(&path, msg)?;
                        history.save_to_file(Some(&rif_path))?;
                    }
                }
                println!("Updated file: {}", file);

                if sub_match.is_present("check") {
                    let mut checker = Checker::with_relations(&relations)?;
                    let changed_files = checker.check(&mut relations)?;
                    println!("Rif check complete");

                    if changed_files.len() != 0 {
                        println!("\n///Hook Output///");
                        let config = Config::read_from_file(Some(&rif_path))?;
                        Hook::new(config.hook).execute(changed_files)?;
                    }
                }
                relations.save_to_file(Some(&rif_path))?;
            } 
        } 
        Ok(())
    }

    /// Check if `discard` subcommand was given and parse subcommand options
    // Discard simply updates filestamp in rif time without updating filestamp in rif file
    fn subcommand_discard(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("discard") {
            let rif_path = utils::get_rif_directory()?;

            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut relations = Relations::read_from_file(Some(&rif_path))?;

                relations.discard_change(&path)?;
                relations.save_to_file(Some(&rif_path))?;
                println!("File modification ignored for file: {}", file);
            } 
        } 
        Ok(())
    }

    /// Check if `list` subcommand was given and parse subcommand options
    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("list") {
            let rif_path = utils::get_rif_directory()?;

            let relations = Relations::read_from_file(Some(&rif_path))?;
            // If list command was given file argument, 
            // then print only the item not the whole list
            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);

                // Print relation tree
                relations.display_file_depth(&path, 0)?;

                // Also print update 
                println!("\n# History : ");
                let rif_history = History::read_from_file(Some(&rif_path))?;
                rif_history.print_history(&path)?;

                return Ok(());
            } else if let Some(depth) = sub_match.value_of("depth") {
                if let Ok(depth) = depth.parse::<u8>() {
                    relations.display_depth(depth)?;
                } else {
                    return Err(RifError::Ext(String::from("Failed to parse depth because is is either not unsinged integer or invalid number")));
                }
            } else {
                print!("{}", relations);
            }
        } 
        Ok(())
    }

    /// Check if `check` subcommand was given and parse subcommand options
    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("check") {
            let rif_path = utils::get_rif_directory()?;
            let mut relations = Relations::read_from_file(Some(&rif_path))?;

            // Automatically update all files that has been modified
            if sub_match.is_present("update") {
                for item in relations.get_modified_files()?.iter() {
                    relations.update_filestamp(item)?;
                }
            }

            let mut checker = Checker::with_relations(&relations)?;
            let changed_files = checker.check(&mut relations)?;
            relations.save_to_file(Some(&rif_path))?;
            println!("Rif check complete");

            if changed_files.len() != 0 {
                println!("\n///Hook Output///");
                let config = Config::read_from_file(Some(&rif_path))?;
                Hook::new(config.hook).execute(changed_files)?;
            }
        } 
        Ok(())
    }

    /// Check if `init` subcommand was given and parse subcommand options
    fn subcommand_init(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("init") {
            // NOTE !!!
            // This is necessary because, simple none cannot infer type, thus
            // type annotation is necessary
            let none :Option<&Path> = None;
            Rif::init(none, sub_match.is_present("rifignore"))?;
        } 
        Ok(())
    }

    /// Check if `sanity` subcommand was given and parse subcommand options
    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("sanity") {
            let rif_path = utils::get_rif_directory()?;

            // NOTE ::: You don't have to manually call sanity check
            // Because read operation always check file sanity after reading a file
            // and return erros if sanity was not assured.
            let mut raw_relations = Relations::read_as_raw(Some(&rif_path))?;

            if sub_match.is_present("fix") {
                raw_relations.sanity_fix()?;
                raw_relations.save_to_file(Some(&rif_path))?;
                println!("Sucessfully fixed the rif file");
            } else {
                raw_relations.sanity_check()?;
                println!("Sucessfully checked the rif file");
            }
        } 
        Ok(())
    }

    /// Check if `status` subcommand was given and parse subcommand options
    fn subcommand_status(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("status") {
            let rif_path = utils::get_rif_directory()?;

            let relations = Relations::read_from_file(Some(&rif_path))?;
            println!("# Modified files :");
            relations.track_modified_files()?;

            // Ignore untracked files
            if !sub_match.is_present("ignore") {

                let config = Config::read_from_file(Some(&rif_path))?;
                // Default black list only includes .rif file for now
                // Currently only check relative paths,or say, stripped path
                let black_list = utils::get_black_list(config.git_ignore)?;
                println!("\n# Untracked files :");
                relations.track_unregistered_files(&black_list)?;
            }

            if sub_match.is_present("verbose") {
                println!("\n# Current rif status:\n---");
                print!("{}", relations);
            }
        } 
        Ok(())
    }
    
    fn subcommand_depend(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("depend") {
            let rif_path = utils::get_rif_directory()?;
            if let Some(file) = sub_match.value_of("FILE") {
                let relations = Relations::read_from_file(Some(&rif_path))?;
                let dependes = relations.find_depends(&PathBuf::from(file))?;
                println!("Files that depends on \"{}\"", file);
                println!("=====");
                for item in dependes {
                    println!("{}", item.display().to_string().green());
                }
            }
        }

        Ok(())
    }

    fn subcommand_data(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("data") {
            let rif_path = utils::get_rif_directory()?;
            if let Some(file) = sub_match.value_of("TYPE") {
                match file {
                    "history" => {
                        let rif_history = History::read_from_file(Some(&rif_path))?;
                        println!("{:#?}", rif_history);
                    }
                    _ => () // This doesn't happen
                }
            } else {
                let relations = Relations::read_from_file(Some(&rif_path))?;
                if sub_match.is_present("compact") {
                    println!("{:?}", relations);
                } else {
                    println!("{:#?}", relations);
                }
            }
        }

        Ok(())
    }
}
