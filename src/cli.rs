use std::collections::HashSet;
use std::fs::metadata;
use std::path::PathBuf;

use clap::clap_app;
use crate::checker::Checker;
use crate::consts::*;
use crate::fileio::{rif_io, etc_io};
use crate::models::{ 
    rif_error::RifError, 
    rif_list::RifList 
};
use crate::utils::{self, LoopBranch};

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
        Cli::subcommand_new(args)?;
        Cli::subcommand_add(args)?;
        Cli::subcommand_remove(args)?;
        Cli::subcommand_set(args)?;
        Cli::subcommand_unset(args)?;
        Cli::subcommand_update(args)?;
        Cli::subcommand_discard(args)?;
        Cli::subcommand_list(args)?;
        Cli::subcommand_check(args)?;
        Cli::subcommand_sanity(args)?;
        Cli::subcommand_status(args)?;
        Ok(())
    }

    /// Creates argument matcher
    ///
    /// Argument matcher consists of clap macros
    fn args_builder() -> clap::ArgMatches {
        clap_app!(Rif =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Rif is a program to track file refernces")
            (@setting ArgRequiredElseHelp)
            (@subcommand add =>
                (about: "Add file to rif")
                (@arg FILE: ... +required "File to add")
                (@arg set: ... -s --set "Files to add reference to")
                (@arg batch: -b --batch "Batch set references to all files given as arguments")
            )
            (@subcommand remove =>
                (about: "Remove file from rif")
                (@arg FILE: ... +required "File to remove")
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
            (@subcommand new =>
                (about: "Create a new rif file in current working directory")
                (@arg default: -d --default "Creates defult rifignore file")
            )
            (@subcommand status =>
                (about: "Show current status of rif")
                (@arg ignore: -i --ignore "Ignore untracked files")
                (@arg verbose: -v --verbose "Ignore untracked files")
            )
            (@subcommand list =>
                (about: "Diplay all files from rif file")
                (@arg FILE: "File to list")
            )
        ).get_matches()
    }

    /// Check if `add` subcommand was given and parse subcommand options
    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            utils::check_rif_file()?;

            if let Some(files) = sub_match.values_of("FILE") {
                let mut rif_list = rif_io::read()?;

                // Easily fallable mistake prevention
                // When user is trying set multiple files references, user need to explicit
                if files.len() > 1 && sub_match.is_present("set") && !sub_match.is_present("batch") {
                    return Err(RifError::CliError(String::from("You need to give batch option <-b or --batch> for batch set of references")));
                }

                let black_list = etc_io::get_black_list()?;
                let argc = files.len();

                for file in files {
                    let mut path = PathBuf::from(file);

                    // File is in rif ignore
                    if let Some(_) = black_list.get(&path) {
                        // If argument's count is 1 display specific logs
                        // to help user to grasp what is happening
                        if argc == 1 {
                            // If File is not configurable
                            // It's not allowed by the program
                            // else it's allowd by the program 
                            if BLACK_LIST.to_vec().contains(&file) {
                                println!("File : \"{}\" is not allowed", file);
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
                    }

                    // Closure to recursively get inside directory and add files
                    let mut closure = |entry_path : PathBuf| -> Result<LoopBranch, RifError> {
                        let striped_path = utils::relativize_path(&entry_path)?;

                        // Early return if file or directory is in black_list
                        // Need to check the black_list once more because closure checks nested
                        // directory that is not checked in outer for loop
                        if let Some(_) = black_list.get(&striped_path) {
                            if striped_path.is_dir() {
                                return Ok(LoopBranch::Exit);
                            } 
                            else {
                                return Ok(LoopBranch::Continue);
                            }
                        }

                        if !rif_list.add_file(&striped_path)? { return Ok(LoopBranch::Continue); }
                        println!("Added file: {}", &striped_path.display());

                        // Set option
                        if let Some(files) = sub_match.values_of("set") {
                            let set: HashSet<PathBuf> = files.map(|a| PathBuf::from(a)).collect();
                            rif_list.add_reference(&striped_path, &set)?;
                            println!("Added references to: {}", &striped_path.display());
                        }

                        Ok(LoopBranch::Continue)
                    }; // Closure end here 

                    // if path is a directory then recusively get into it
                    // if path is a file then simply add a file
                    if metadata(file)?.is_dir() {
                        utils::walk_directory_recursive( &path, &mut closure)?;
                    } else { 
                        path = utils::relativize_path(&path)?;

                        // File was not added e.g. file already exists
                        if !rif_list.add_file(&path)? { continue; }
                        println!("Added file: {}", path.display());

                        // Set option
                        if let Some(files) = sub_match.values_of("set") {
                            let set: HashSet<PathBuf> = files.map(|a| PathBuf::from(a)).collect();
                            rif_list.add_reference(&path, &set)?;
                            println!("Added references to: {}", path.display());
                        }
                    }
                } // for loop end
                rif_io::save(rif_list)?;
            } 
        }  // if let end
        Ok(())
    }

    /// Check if `remove` subcommand was given and parse subcommand options
    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            utils::check_rif_file()?;

            if let Some(files) = sub_match.values_of("FILE") {
                let mut rif_list = rif_io::read()?;
                for file in files {
                    let path = PathBuf::from(file);
                    if rif_list.remove_file(&path)? {
                        println!("Removed file: {}", file);
                    }
                }
                rif_io::save( rif_list)?;
            } 
        }
        Ok(())
    }

    /// Check if `set` subcommand was given and parse subcommand options
    fn subcommand_set(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("set") {
            utils::check_rif_file()?;

            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();
                    let mut rif_list = rif_io::read()?;

                    rif_list.add_reference(&path, &refs)?;
                    rif_io::save(rif_list)?;
                    println!("Added references to: {}", file);
                }
            } 
        } 
        Ok(())
    }

    /// Check if `unset` subcommand was given and parse subcommand options
    fn subcommand_unset(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("unset") {
            utils::check_rif_file()?;

            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();
                    let mut rif_list = rif_io::read()?;

                    rif_list.remove_reference(&path, &refs)?;
                    rif_io::save(rif_list)?;
                    println!("Removed references from: {}", file);
                }
            } 
        } 
        Ok(())
    }

    /// Check if `update` subcommand was given and parse subcommand options
    fn subcommand_update(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("update") {
            utils::check_rif_file()?;

            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut rif_list = rif_io::read()?;

                if sub_match.is_present("force") {
                    rif_list.update_filestamp_force(&path)?;
                } else {
                    rif_list.update_filestamp(&path)?;
                }
                println!("Updated file: {}", file);

                if sub_match.is_present("check") {
                    let mut checker = Checker::with_rif_list(&rif_list)?;
                    checker.check(&mut rif_list)?;
                    println!("Rif check complete");
                }

                rif_io::save(rif_list)?;
            } 
        } 
        Ok(())
    }

    /// Check if `discard` subcommand was given and parse subcommand options
    // Discard simply updates filestamp in rif time without updating filestamp in rif file
    fn subcommand_discard(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("discard") {
            utils::check_rif_file()?;

            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut rif_list = rif_io::read()?;

                rif_list.discard_change(&path)?;
                rif_io::save(rif_list)?;
                println!("File modification ignored for file: {}", file);
            } 
        } 
        Ok(())
    }

    /// Check if `list` subcommand was given and parse subcommand options
    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("list") {
            utils::check_rif_file()?;

            let rif_list = rif_io::read()?;

            // If list command was given file argument, 
            // then print only the item not the whole list
            if let Some(file) = sub_match.value_of("FILE") {
                println!("\n{}", rif_list.display_file(&PathBuf::from(file)));
                return Ok(());
            }
            print!("{}", rif_list);
        } 
        Ok(())
    }

    /// Check if `check` subcommand was given and parse subcommand options
    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("check") {
            utils::check_rif_file()?;
            let mut rif_list = rif_io::read()?;

            // Automatically update all files that has been modified
            if sub_match.is_present("update") {
                for item in rif_list.get_modified_files()?.iter() {
                    rif_list.update_filestamp(item)?;
                }
            }

            let mut checker = Checker::with_rif_list(&rif_list)?;
            checker.check(&mut rif_list)?;
            rif_io::save(rif_list)?;
            println!("Rif check complete");
        } 
        Ok(())
    }

    /// Check if `new` subcommand was given and parse subcommand options
    fn subcommand_new(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("new") {
            let new_rif_list = RifList::new();
            rif_io::save(new_rif_list)?;
            println!("Created new rif file in {}", std::env::current_dir()?.display());

            if sub_match.is_present("default") {
                std::fs::write(".rifignore", "target
build
target
.git")?;
                println!("Created new rig ignore file");
            } // if end
        } 
        Ok(())
    }

    /// Check if `sanity` subcommand was given and parse subcommand options
    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("sanity") {
            utils::check_rif_file()?;

            // NOTE ::: You don't have to manually call sanity check
            // Because read operation always check file sanity after reading a file
            // and return erros if sanity was not assured.
            let mut raw_rif_list = rif_io::read_as_raw()?;

            if sub_match.is_present("fix") {
                raw_rif_list.sanity_fix()?;
                rif_io::save(raw_rif_list)?;
            } else {
                raw_rif_list.sanity_check()?;
            }
            println!("Rif is in valid format");
        } 
        Ok(())
    }

    /// Check if `status` subcommand was given and parse subcommand options
    fn subcommand_status(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("status") {
            utils::check_rif_file()?;

            let rif_list = rif_io::read()?;
            println!("# Modified files :");
            rif_list.track_modified_files()?;

            // Ignore untracked files
            if !sub_match.is_present("ignore") {
                // Default black list only includes .rif file for now
                // Currently only check relative paths,or say, stripped path
                let black_list = etc_io::get_black_list()?;
                println!("\n# Untracked files :");
                rif_list.track_unregistered_files(&black_list)?;
            }

            if sub_match.is_present("verbose") {
                println!("\n# Current rif status:\n---");
                print!("{}", rif_list);
            }
        } 
        Ok(())
    }
}
