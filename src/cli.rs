use std::path::PathBuf;
use std::fs::metadata;
use clap::clap_app;
use crate::fileio::{rif_io, etc_io};
use crate::models::{RifError, RifList};
use crate::checker::Checker;
use crate::consts::*;
use std::collections::HashSet;
use crate::utils;

pub struct Cli{}

// REFACTOR :::
// Make subcommand parse into a function 
// and the function include multiple subcommand_name functions
// No need for enum

impl Cli {
    pub fn parse() -> Result<(), RifError>{

        let cli_args = Cli::args_builder();
        Cli::parse_subcommands(&cli_args)?;

        Ok(())
    //end Parse
    }

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
                (@arg batch: -b --batch "Files to add reference to")
            )
            // TODO ::: Considiering adding short version of remove which is "rm"
            (@subcommand remove =>
                (about: "Remove file from rif")
                (@arg FILE: ... +required "File to remove")
            )
            (@subcommand set =>
                (about: "Set references to file")
                // THis should be an array not a single item
                (@arg FILE: +required "File to change")
                (@arg REFS: ... +required "Files to set")
            )
            (@subcommand unset =>
                (about: "Unset references from file")
                // THis should be an array not a single item
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
            )
            (@subcommand status =>
                (about: "Show current status of rif")
                (@arg ignore: -i --ignore "Ignore untracked files")
            )
            (@subcommand list =>
                (about: "Diplay all files from rif file")
            )
        ).get_matches()
    }

    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            utils::check_rif_file()?;
            if let Some(files) = sub_match.values_of("FILE") {
                let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;

                // Easily fallable mistake prevention
                // When user is trying set multiple files references, user need to explicit
                if files.len() > 1 && sub_match.is_present("set") && !sub_match.is_present("batch") {
                    return Err(RifError::Ext(String::from("You need to give batch option <-b or --batch> for batch set of references")));
                }

                // Rifignore + const array of ignored files which are [.rif, .rifignore]
                let mut rifignore = etc_io::read_rif_ignore()?;
                rifignore.extend(BLACK_LIST.to_vec().iter().map(|a| PathBuf::from(*a)).collect::<HashSet<PathBuf>>());

                let argc = files.len();

                for file in files {
                    let path = PathBuf::from(file);

                    // File is in rif ignore
                    if let Some(_) = rifignore.get(&path) {
                        if argc == 1 { 
                            // File is not configurable
                            // It's not allowed by the program
                            if BLACK_LIST.to_vec().contains(&file) {
                                println!("File : \"{}\" is not allowed", file);
                            } 
                            // File is configurable
                            // Reove a file from rifginore to allow addition
                            else {
                                println!("\"{}\" is in rifignore file, which is ignored.", file) 
                            }
                        }
                        continue;
                    }

                    // if file is directory
                    // Do nothing for now 
                    // TODO ::: Add directory logic
                    if metadata(file)?.is_dir() {
                        if argc == 1 {
                            println!("Directory cannot be added to rif");
                        }
                        continue;
                    }

                    // File was not added
                    // e.g. file already exists
                    if !rif_list.add_file(&path)? {
                        continue;
                    }

                    println!("Added file: {}", file);

                    if let Some(files) = sub_match.values_of("set") {
                        let set: HashSet<PathBuf> = files.map(|a| PathBuf::from(a)).collect();
                        rif_list.add_reference(&path, &set)?;
                        println!("Added references to: {}", file);
                    }
                }
                rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            utils::check_rif_file()?;
            if let Some(files) = sub_match.values_of("FILE") {
                let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;
                for file in files {
                    let path = PathBuf::from(file);
                    if rif_list.remove_file(&path)? {
                        println!("Removed file: {}", file);
                    }
                }
                rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_set(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("set") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();

                    let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;
                    rif_list.add_reference(&path, &refs)?;
                    rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
                    println!("Added references to: {}", file);
                }
            } 
        } 
        Ok(())
    }

    fn subcommand_unset(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("unset") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let path = PathBuf::from(file);
                    let refs_vec: Vec<&str> = refs.collect();
                    let refs: HashSet<PathBuf> = refs_vec.iter().map(|a| PathBuf::from(a)).collect();

                    let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;
                    rif_list.remove_reference(&path, &refs)?;
                    rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
                    println!("Removed references from: {}", file);
                }
            } 
        } 
        Ok(())
    }

    fn subcommand_update(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("update") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;

                if sub_match.is_present("force") {
                    rif_list.update_filestamp_force(&path)?;
                } else {
                    rif_list.update_filestamp(&path)?;
                }

                println!("Updated file: {}", file);

                if sub_match.is_present("check") {
                    let mut checker = Checker::new();
                    checker.add_rif_list(&rif_list)?;
                    checker.check(&mut rif_list)?;
                    println!("Rif check complete");
                }

                rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
            } 
        } 
        Ok(())
    }

    // Discard simply updates filestamp in rif time without updating filestamp in rif file
    fn subcommand_discard(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("discard") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;
                rif_list.discard_change(&path)?;
                rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
                println!("File modification ignored for file: {}", file);
            } 
        } 
        Ok(())
    }

    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(_sub_match) = matches.subcommand_matches("list") {
            utils::check_rif_file()?;
            let rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;
            println!("\n{:#?}", rif_list);
        } 
        Ok(())
    }

    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("check") {
            utils::check_rif_file()?;
            let mut rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;

            // Automatically update all files that has been modified
            if sub_match.is_present("update") {
                for item in rif_list.get_modified_files()?.iter() {
                    rif_list.update_filestamp(item)?;
                }
            }

            let mut checker = Checker::new();
            checker.add_rif_list(&rif_list)?;
            checker.check(&mut rif_list)?;
            rif_io::save_with_str(RIF_LIST_FILE, rif_list)?;
            println!("Rif check complete");
        } 
        Ok(())
    }

    fn subcommand_new(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("new") {
            let new_rif_list = RifList::new();
            rif_io::save(&std::env::current_dir()?.join(PathBuf::from(RIF_LIST_FILE)), new_rif_list)?;
            println!("Created new rif file in {}", std::env::current_dir()?.display());
        } 
        Ok(())
    }

    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("sanity") {
            utils::check_rif_file()?;
            // NOTE ::: You don't have to manually call sanity check
            // Because read operation always check file sanity after reading a file
            // and return erros if sanity was not assured.
            let mut raw_rif_list = rif_io::read_with_str_as_raw(RIF_LIST_FILE)?;

            if sub_match.is_present("fix") {
                raw_rif_list.sanity_fix()?;
                rif_io::save_with_str(RIF_LIST_FILE, raw_rif_list)?;
            } else {
                raw_rif_list.sanity_check()?;
            }
            println!("Rif is in valid format");
        } 
        Ok(())
    }

    fn subcommand_status(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("status") {
            utils::check_rif_file()?;
            let rif_list = rif_io::read_with_str(RIF_LIST_FILE)?;

            println!("# Modified files :");
            rif_list.track_modified_files()?;

            // Ignore untracked files
            if !sub_match.is_present("ignore") {
                // Default black list only includes .rif file for now
                // Currently only check relative paths,or say, stripped path
                println!("\n# Untracked files :");
                let mut black_list: HashSet<PathBuf> = HashSet::new();
                let rif_files: HashSet<PathBuf> = rif_list.files.keys().cloned().collect();
                black_list.extend(rif_files);
                black_list.extend(etc_io::read_rif_ignore()?);
                black_list.insert(PathBuf::from(".rif"));
                black_list.insert(PathBuf::from(".rifignore"));

                rif_list.track_unregistered_files(&black_list)?;
            }

            println!("\n# Current rif status:\n---");
            print!("{}", rif_list);
        } 
        Ok(())
    }
}
