use std::collections::HashSet;
use std::fs::metadata;
use std::path::{PathBuf, Path};

use colored::*;
use clap::clap_app;
use crate::checker::Checker;
use crate::consts::*;
use crate::rif::{hook::Hook, config::Config, history::History, rel::Relations};
use crate::RifError;
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
                (@arg set: ... -s --set +takes_value "Files to add reference to")
                (@arg batch: -b --batch "Batch set references to all files given as arguments")
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
            utils::check_rif_file()?;

            if let Some(files) = sub_match.values_of("FILE") {
                let mut relations = Relations::read()?;
                // Easily fallable mistake prevention
                // When user is trying set multiple files references, user need to explicit
                if files.len() > 1 && sub_match.is_present("set") && !sub_match.is_present("batch") {
                    return Err(RifError::CliError(String::from("You need to give batch option <-b or --batch> for batch set of references")));
                }

                let config = Config::read_from_file()?;
                let black_list = utils::get_black_list(config.git_ignore)?;
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
                    } else {
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
                        if let Some(_) = black_list.get(&striped_path) {
                            if striped_path.is_dir() {
                                return Ok(LoopBranch::Exit);
                            } 
                            else {
                                return Ok(LoopBranch::Continue);
                            }
                        }

                        if !relations.add_file(&striped_path)? { return Ok(LoopBranch::Continue); }
                        println!("Added file: {}", &striped_path.display());

                        // Set option
                        if let Some(files) = sub_match.values_of("set") {
                            let set: HashSet<PathBuf> = files.map(|a| PathBuf::from(a)).collect();
                            relations.add_reference(&striped_path, &set)?;
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
                        if !relations.add_file(&path)? { continue; }
                        println!("Added file: {}", path.display());

                        // Set option
                        if let Some(files) = sub_match.values_of("set") {
                            let set: HashSet<PathBuf> = files.map(|a| PathBuf::from(a)).collect();
                            relations.add_reference(&path, &set)?;
                            println!("Added references to: {}", path.display());
                        }
                    }
                } // for loop end
                Relations::save(relations)?;
            } 
        }  // if let end
        Ok(())
    }

    /// Check if `remove` subcommand was given and parse subcommand options
    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            utils::check_rif_file()?;

            if let Some(files) = sub_match.values_of("FILE") {
                let mut relations = Relations::read()?;
                for file in files {
                    let path = PathBuf::from(file);
                    if relations.remove_file(&path)? {
                        println!("Removed file: {}", file);
                    }
                }
                Relations::save( relations)?;
            } 
        }
        Ok(())
    }

    /// Check if `rename` subcommand was given and parse subcommand options
    fn subcommand_rename(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("rename") {
            utils::check_rif_file()?;

            // TODO
            // Check this process
            // Rename's target might already in rif_file
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(new_name) = sub_match.value_of("NEWNAME") {
                    let mut raw_relations = Relations::read_as_raw()?;
                    if let Some(_) = raw_relations.files.get(&PathBuf::from(new_name)) {
                        return Err(RifError::RenameFail(format!("Rename target: \"{}\" already exists", new_name)));
                    }

                    let file_path = Path::new(file);
                    // Rename file if it exsits
                    if file_path.exists() {
                        std::fs::rename(file_path, new_name)?;
                    }
                    raw_relations.rename_file(&PathBuf::from(file), &PathBuf::from(new_name))?;
                    Relations::save(raw_relations)?;
                    println!("Sucessfully renamed \"{}\" to \"{}\"", file, new_name);
                }
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
                    let mut relations = Relations::read()?;

                    relations.add_reference(&path, &refs)?;
                    Relations::save(relations)?;
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
                    let mut relations = Relations::read()?;

                    relations.remove_reference(&path, &refs)?;
                    Relations::save(relations)?;
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
                let mut relations = Relations::read()?;

                if sub_match.is_present("force") {
                    relations.update_filestamp_force(&path)?;
                } else {
                    relations.update_filestamp(&path)?;

                    if let Some(msg) = sub_match.value_of("message") {
                        let mut history = History::read_from_file()?;
                        history.add_history(&path, msg)?;
                        history.save_to_file()?;
                    }
                }
                println!("Updated file: {}", file);

                if sub_match.is_present("check") {
                    let mut checker = Checker::with_relations(&relations)?;
                    let changed_files = checker.check(&mut relations)?;
                    println!("Rif check complete");

                    if changed_files.len() != 0 {
                        println!("\n///Hook Output///");
                        let config = Config::read_from_file()?;
                        Hook::new(config.hook).execute(changed_files)?;
                    }
                }

                Relations::save(relations)?;
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
                let mut relations = Relations::read()?;

                relations.discard_change(&path)?;
                Relations::save(relations)?;
                println!("File modification ignored for file: {}", file);
            } 
        } 
        Ok(())
    }

    /// Check if `list` subcommand was given and parse subcommand options
    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("list") {
            utils::check_rif_file()?;

            let relations = Relations::read()?;
            // If list command was given file argument, 
            // then print only the item not the whole list
            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);

                // Print relation tree
                relations.display_file_depth(&path, 0)?;

                // Also print update 
                println!("\n# History : ");
                let rif_history = History::read_from_file()?;
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
            utils::check_rif_file()?;
            let mut relations = Relations::read()?;

            // Automatically update all files that has been modified
            if sub_match.is_present("update") {
                for item in relations.get_modified_files()?.iter() {
                    relations.update_filestamp(item)?;
                }
            }

            let mut checker = Checker::with_relations(&relations)?;
            let changed_files = checker.check(&mut relations)?;
            Relations::save(relations)?;
            println!("Rif check complete");

            if changed_files.len() != 0 {
                println!("\n///Hook Output///");
                let config = Config::read_from_file()?;
                Hook::new(config.hook).execute(changed_files)?;
            }
        } 
        Ok(())
    }

    /// Check if `init` subcommand was given and parse subcommand options
    fn subcommand_init(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("init") {
            if PathBuf::from(RIF_DIECTORY).exists() {
                return Err(RifError::RifIoError(String::from("Directory is already initiated")));
            }
            // Root directory
            std::fs::create_dir(RIF_DIECTORY)?;

            // Rif List (Named as )
            let new_relations = Relations::new();
            Relations::save(new_relations)?;
            // Rif history
            let new_rif_history = History::new();
            new_rif_history.save_to_file()?;
            // Rif Config
            let new_config = Config::new();
            new_config.save_to_file()?;
            // User feedback
            println!("Initiated rif project");

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
            let mut raw_relations = Relations::read_as_raw()?;

            if sub_match.is_present("fix") {
                raw_relations.sanity_fix()?;
                Relations::save(raw_relations)?;
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
            utils::check_rif_file()?;

            let relations = Relations::read()?;
            println!("# Modified files :");
            relations.track_modified_files()?;

            // Ignore untracked files
            if !sub_match.is_present("ignore") {

                let config = Config::read_from_file()?;
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
            if let Some(file) = sub_match.value_of("FILE") {
                let relations = Relations::read()?;
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
            if let Some(file) = sub_match.value_of("TYPE") {
                match file {
                    "history" => {
                        let rif_history = History::read_from_file()?;
                        println!("{:#?}", rif_history);
                    }
                    _ => () // This doesn't happen
                }
            } else {
                let relations = Relations::read()?;
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
