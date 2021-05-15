use std::path::PathBuf;
use clap::clap_app;
use crate::fileio::FileIO;
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
                (@arg FILE: +required "File to add")
            )
            // TODO ::: Considiering adding short version of remove which is "rm"
            (@subcommand remove =>
                (about: "Remove file from rif")
                (@arg FILE: +required "File to remove")
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
            )
            (@subcommand list =>
                (about: "Diplay all files from rif file")
            )
        ).get_matches()
    }

    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                let path = PathBuf::from(file);
                let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
                rif_list.add_file(&path)?;
                FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            utils::check_rif_file()?;
            if let Some(file) = sub_match.value_of("FILE") {
                if sub_match.is_present("") {

                }
                let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
                let path = PathBuf::from(file);
                rif_list.remove_file(&path)?;
                FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
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

                    let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
                    rif_list.add_reference(&path, &refs)?;
                    FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
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

                    let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
                    rif_list.remove_reference(&path, &refs)?;
                    FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
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
                let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;

                if sub_match.is_present("force") {
                    rif_list.update_filestamp_force(&path)?;
                } else {
                    rif_list.update_filestamp(&path)?;
                }

                FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
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
                let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
                rif_list.discard_change(&path)?;
                FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(_sub_match) = matches.subcommand_matches("list") {
            utils::check_rif_file()?;
            let rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
            println!("\n{:#?}", rif_list);
        } 
        Ok(())
    }

    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("check") {
            utils::check_rif_file()?;
            let mut rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;

            // Automatically update all files that has been modified
            if sub_match.is_present("update") {
                for item in rif_list.get_modified_files()?.iter() {
                    rif_list.update_filestamp(item)?;
                }
            }

            let mut checker = Checker::new();
            checker.add_rif_list(&rif_list)?;
            checker.check(&mut rif_list)?;
            FileIO::save_with_str(RIF_LIST_FILE, rif_list)?;
        } 
        Ok(())
    }

    fn subcommand_new(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("new") {
            let new_rif_list = RifList::new();
            FileIO::save(&std::env::current_dir()?.join(PathBuf::from(RIF_LIST_FILE)), new_rif_list)?;
        } 
        Ok(())
    }

    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("sanity") {
            utils::check_rif_file()?;
            // NOTE ::: You don't have to manually call sanity check
            // Because read operation always check file sanity after reading a file
            // and return erros if sanity was not assured.
            let mut raw_rif_list = FileIO::read_with_str_as_raw(RIF_LIST_FILE)?;

            if sub_match.is_present("fix") {
                println!("FIFIFIX");
                raw_rif_list.sanity_fix()?;
                FileIO::save_with_str(RIF_LIST_FILE, raw_rif_list)?;
            } else {
                raw_rif_list.sanity_check()?;
            }
        } 
        Ok(())
    }

    fn subcommand_status(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("status") {
            utils::check_rif_file()?;
            let rif_list = FileIO::read_with_str(RIF_LIST_FILE)?;
            rif_list.track_files()?;
            println!("\nCurrent rif status:");
            print!("{}", rif_list);
        } 
        Ok(())
    }
}
