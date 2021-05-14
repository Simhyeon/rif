use std::path::PathBuf;
use clap::clap_app;
use crate::fileio::FileIO;
use crate::models::{RifError, RifList};
use crate::checker::Checker;
use crate::consts::*;

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
        Cli::subcommand_add(args)?;
        Cli::subcommand_remove(args)?;
        Cli::subcommand_set(args)?;
        Cli::subcommand_update(args)?;
        Cli::subcommand_discard(args)?;
        Cli::subcommand_list(args)?;
        Cli::subcommand_check(args)?;
        Cli::subcommand_new(args)?;
        Cli::subcommand_sanity(args)?;

        Ok(())
    }

    fn args_builder() -> clap::ArgMatches {
        clap_app!(myapp =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Manipulate rif file")
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
                (@arg FILE: +required "File to remove")
            )
            (@subcommand update =>
                (about: "Update file's timestamp")
                (@arg FILE: +required "File to update")
            )
            (@subcommand discard =>
                (about: "Discard file changes")
                (@arg FILE: +required "File to discard changes")
            )
            (@subcommand check =>
                (about: "Check file references")
            )
            (@subcommand sanity =>
                (about: "Check sanity of rif file")
            )
            (@subcommand new =>
                (about: "Create new rif files in current working directory")
            )
            (@subcommand list =>
                (about: "Diplay all files from rif file")
                (@arg RIF: +required "rif file to check sanity")
            )
        ).get_matches()
    }

    fn subcommand_add(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("add") {
            if let Some(file) = sub_match.value_of("FILE") {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.add_file(&PathBuf::from(file))?;
                FileIO::save_with_str(RIF_LIST_NAME, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_remove(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("remove") {
            if let Some(file) = sub_match.value_of("FILE") {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                // TODO :: Remove file should be applied to file references
                // Including file itself and also references.
                // rif_list.remove_file(file)?;
                FileIO::save_with_str(RIF_LIST_NAME, rif_list)?;
            } 
        } 
        Ok(())
    }

    fn subcommand_set(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(_sub_match) = matches.subcommand_matches("set") {
            //if let Some(file) = sub_match.value_of("FILE") {
                //return CliAction::Remove(PathBuf::from(file));
            //} 
        } 
        Ok(())
    }

    fn subcommand_update(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("update") {
            if let Some(file) = sub_match.value_of("FILE") {

                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.update_filestamp(&PathBuf::from(file))?;
            } 
        } 
        Ok(())
    }

    fn subcommand_discard(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("discard") {
            if let Some(file) = sub_match.value_of("FILE") {
                //return CliAction::Discard(PathBuf::from(file));
            } 
        } 
        Ok(())
    }

    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(_sub_match) = matches.subcommand_matches("update") {
            //return CliAction::List;
        } 
        Ok(())
    }

    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("check") {
            let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
            let mut checker = Checker::new();
            checker.add_rif_list(&rif_list)?;
            checker.check(&mut rif_list)?;
            FileIO::save_with_str(RIF_LIST_NAME, rif_list)?;
        } 
        Ok(())
    }

    fn subcommand_new(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("new") {
            let new_rif_list = RifList::new();
            FileIO::save(&std::env::current_dir()?.join(PathBuf::from(RIF_LIST_NAME)), new_rif_list)?;
            crate::tracker::Tracker::save_new_file()?;
        } 
        Ok(())
    }

    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_sub_match) = matches.subcommand_matches("sanity") {
            // NOTE ::: You don't have to manually call sanity check
            // Because read operation always check file sanity after reading a file
            // and return erros if sanity was not assured.
            let rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
        } 
        Ok(())
    }
}
