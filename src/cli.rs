use std::collections::HashSet;
use std::path::{PathBuf, Path};

use colored::*;
use clap::clap_app;
use crate::checker::Checker;
use crate::rif::{config::Config, history::History, rel::Relations};
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
        Cli::subcommand_commit(args)?;
        Cli::subcommand_remove(args)?;
        Cli::subcommand_rename(args)?;
        Cli::subcommand_set(args)?;
        Cli::subcommand_unset(args)?;
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
                (@arg force: -f --force "Force add")
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
            (@subcommand rm =>
                (about: "Remove file from rif")
                (@arg FILE: ... +required "File to remove")
            )
            (@subcommand mv =>
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
            (@subcommand commit =>
                (about: "Commit addition of files")
                (@arg message: -m --message +takes_value conflicts_with[discard] "Message to add in update")
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
            (@subcommand ls =>
                (about: "Diplay all files from rif file")
                (@arg FILE: "File to list")
                (@arg depth: -d --depth +takes_value "Maximum depth for display tree(unsigned integer). 0 means print a whole tree")
            )
        ).get_matches()
    }

    /// Check if `init` subcommand was given and parse subcommand options
    fn subcommand_init(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("init") {
            // NOTE !!!
            // This is necessary because, simple none cannot infer type, thus
            // type annotation is necessary
            let current_dir :Option<&Path> = None;
            Rif::init(current_dir, sub_match.is_present("rifignore"))?;
        } 
        Ok(())
    }

    /// Check if `add` subcommand was given and parse subcommand options
    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            if let Some(files) = sub_match.values_of("FILE") {
                let files = files.into_iter().map(|s| Path::new(s)).collect();
                let force = sub_match.is_present("force");

                let rif_path = utils::get_rif_directory()?;
                let mut rif = Rif::new(Some(&rif_path))?;
                rif.add(&files, force)?;
            } 
        } 
        Ok(())
    }

    /// Check if `commit` subcommand was given and parse subcommand options
    fn subcommand_commit(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("update") {
            let message = sub_match.value_of("message");

            let rif_path = utils::get_rif_directory()?;
            let mut rif = Rif::new(Some(&rif_path))?;
            rif.commit(message)?;
        } 
        Ok(())
    }

    /// Check if `remove` subcommand was given and parse subcommand options
    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("rm") {
            if let Some(files) = sub_match.values_of("FILE") {
                let files = files.into_iter().map(|s| Path::new(s)).collect();
                let rif_path = utils::get_rif_directory()?;

                let mut rif = Rif::new(Some(&rif_path))?;
                rif.remove(&files)?;
            } 
        }
        Ok(())
    }

    /// Check if `rename` subcommand was given and parse subcommand options
    fn subcommand_rename(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("mv") {
            if let Some(source_name) = sub_match.value_of("FILE") {
                if let Some(new_name) = sub_match.value_of("NEWNAME") {
                    let rif_path = utils::get_rif_directory()?;
                    let mut rif = Rif::new(Some(&rif_path))?;
                    rif.rename(source_name, new_name)?;
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

    /// Check if `discard` subcommand was given and parse subcommand options
    // Discard simply updates filestamp in rif time without updating filestamp in rif file
    fn subcommand_discard(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("discard") {
            if let Some(file) = sub_match.value_of("FILE") {
                let rif_path = utils::get_rif_directory()?;
                let mut rif = Rif::new(Some(&rif_path))?;
                rif.discard(file)?;
            } 
        } 
        Ok(())
    }

    /// Check if `list` subcommand was given and parse subcommand options
    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("ls") {
            let file = sub_match.value_of("FILE");
            let depth = sub_match
                .value_of("depth")
                .map(|num| {
                    num.parse::<usize>().expect("Depth value should be an unsigned integer")
                });

            let rif_path = utils::get_rif_directory()?;
            let rif = Rif::new(Some(&rif_path))?;
            rif.list(file,depth)?;
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
                config.hook.execute(changed_files)?;
            }
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
            let ignore = sub_match.is_present("ignore");
            let verbose = sub_match.is_present("verbose");

            let rif_path = utils::get_rif_directory()?;
            let rif = Rif::new(Some(&rif_path))?;
            rif.status(ignore, verbose)?;
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
