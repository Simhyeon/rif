use std::path::PathBuf;
use clap::clap_app;
use crate::fileio::FileIO;
use crate::models::{RifError, RifList};
use crate::checker::Checker;
use crate::consts::*;

pub struct Cli{}

impl Cli {
    pub fn parse() -> Result<(), RifError>{
        // TODO ::: Currently testing and learning clap
        let mut action = CliAction::None;

        let cli_args = clap_app!(myapp =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Manipulate rif file")
            (@subcommand add =>
                (about: "Add file to rif")
                (@arg FILE: +required "File to add")
            )
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
                (@arg RIF: +required "Rif file to check references")
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
        ).get_matches();


        // TODO 
        // This need changes because all of this commands are all subcommands and should
        // utilize subcommand_matches
        // Add, Check, Discard, List, New, Remove, SanityCheck, Update, Set
        if let Some(file) = cli_args.value_of("add") {
            println!("Add");
            // action = CliAction::Add(PathBuf::from(file));
        } else if cli_args.is_present("CHECK") {
            println!("Check");
            //action = CliAction::Check;
        } else if let Some(file) = cli_args.value_of("DISCARD") {
            action = CliAction::Discard(PathBuf::from(file));
        } else if cli_args.is_present("LIST") {
            action = CliAction::List;
        } else if cli_args.is_present("NEW") {
            action = CliAction::New;
        } else if let Some(file) = cli_args.value_of("REMOVE") {
            action = CliAction::Remove(PathBuf::from(file));
        } else if cli_args.is_present("SANITY") {
            action = CliAction::SanityCheck;
        } else if let Some(file) = cli_args.value_of("UPDATE") {
            action = CliAction::Update(PathBuf::from(file));
        } 
        // Not yet
        //else if let Some(file) = cli_args.value_of("SET") {
            //action = CliAction::Set(PathBuf::from(file));
        //}

        match action {
            CliAction::Add(file) => {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.add_file(&file)?;
                FileIO::save(&file, rif_list)?;
            },
            CliAction::Check => {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                let mut checker = Checker::new();
                checker.add_rif_list(&rif_list)?;
                checker.check(&mut rif_list)?;
                FileIO::save_with_str(RIF_LIST_NAME, rif_list)?;
            },
            CliAction::Discard(file) => {
                // TODO 
                // This should be done
            },
            CliAction::List => {
                // TODO
                // This should be also done
                // Create new display method
            },
            CliAction::Remove(file) => {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.remove_file(&file)?;
                FileIO::save(&file, rif_list)?;
            },
            CliAction::SanityCheck => {
                let rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.sanity_check()?;
            },
            CliAction::New => {
                let new_rif_list = RifList::new();
                println!("Current working diretory joined with file name is \n{:#?}", 
                        std::env::current_dir()?.join(PathBuf::from(RIF_LIST_NAME)));
                FileIO::save(
                    &std::env::current_dir()?.join(PathBuf::from(RIF_LIST_NAME)), 
                    new_rif_list
                    )?;
                crate::tracker::Tracker::save_new_file()?;
            }
            CliAction::Update(file) => {
                let mut rif_list = FileIO::read_with_str(RIF_LIST_NAME)?;
                rif_list.update_filestamp(&file)?;
            }
            // TODO set references references action enum might be helpful
            // Because reference can be added or deleted rather than reallocated 
            CliAction::Set(file, refs) => {
                ();
            }
            CliAction::None => () // Do nothing if no subcommand was given.
        }

        Ok(())
    //end Parse
    }
}

pub enum CliAction {
    None,
    Add(PathBuf),
    Check,
    Discard(PathBuf),
    List,
    New,
    Remove(PathBuf),
    SanityCheck,
    Update(PathBuf),
    Set(PathBuf, Vec<PathBuf>),
}
