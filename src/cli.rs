use std::path::Path;
use clap::clap_app;
use crate::RifError;
use crate::Rif;
use crate::models::ListType;
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
        Cli::subcommand_revert(args)?;
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
            (version: "0.2.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Rif is a program to track impact of file changes")
            (@setting ArgRequiredElseHelp)
            (@subcommand add =>
                (about: "Add file to rif")
                (@arg FILE: ... +required "File to add")
                (@arg force: -f --force "Force add")
            )
            (@subcommand revert =>
                (about: "Revert addition")
                (@arg FILE: ... "File to revert")
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
                (@arg message: -m --message +takes_value "Message to add in update")
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
                (@arg type: -t --type +takes_value "List Type, default is all (all|stale)")
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
            } else {
                eprintln!("No argument for add");
            }
        }
        Ok(())
    }

    /// Check if `revert` subcommand was given and parse subcommand options
    fn subcommand_revert(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("revert") {
            let files = sub_match
                .values_of("FILE")
                .map(|s| s.into_iter().map(|s| Path::new(s)).collect());

            let rif_path = utils::get_rif_directory()?;
            let mut rif = Rif::new(Some(&rif_path))?;
            rif.revert(files.as_ref())?;
        } 
        Ok(())
    }

    /// Check if `commit` subcommand was given and parse subcommand options
    fn subcommand_commit(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("commit") {
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
            } else {
                eprintln!("No argument for rm");
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
                } else {
                    eprintln!("Mv needs second argument as a new file name");
                }
            } else {
                eprintln!("No argument for mv");
            }

        }
        Ok(())
    }

    /// Check if `set` subcommand was given and parse subcommand options
    fn subcommand_set(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("set") {
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let file = Path::new(file);
                    let refs = refs.map(|p| Path::new(p)).collect();

                    let rif_path = utils::get_rif_directory()?;
                    let mut rif = Rif::new(Some(&rif_path))?;
                    rif.set(file, &refs)?;
                } else {
                    eprintln!("Set requires second argument as references");
                }
            } else {
                eprintln!("No argument for set");
            }
        } 
        Ok(())
    }

    /// Check if `unset` subcommand was given and parse subcommand options
    fn subcommand_unset(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("unset") {
            if let Some(file) = sub_match.value_of("FILE") {
                if let Some(refs) = sub_match.values_of("REFS") {
                    let file = Path::new(file);
                    let refs = refs.map(|p| Path::new(p)).collect();

                    let rif_path = utils::get_rif_directory()?;
                    let mut rif = Rif::new(Some(&rif_path))?;
                    rif.unset(file, &refs)?;
                } else {
                    eprintln!("Unset requires second argments as references");
                }

            } else {
                eprintln!("No argument for unset");
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
            } else {
                eprintln!("No argument for discard");
            }
        } 
        Ok(())
    }

    /// Check if `list` subcommand was given and parse subcommand options
    fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), RifError>{
        if let Some(sub_match) = matches.subcommand_matches("ls") {
            let file = sub_match.value_of("FILE");
            let list_type = ListType::from(sub_match.value_of("type").unwrap_or("all"));
            let depth = sub_match
                .value_of("depth")
                .map(|num| {
                    num.parse::<usize>().expect("Depth value should be an unsigned integer")
                });

            let rif_path = utils::get_rif_directory()?;
            let rif = Rif::new(Some(&rif_path))?;
            rif.list(file,list_type,depth)?;
        } 
        Ok(())
    }

    /// Check if `check` subcommand was given and parse subcommand options
    fn subcommand_check(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(_) = matches.subcommand_matches("check") {
            let rif_path = utils::get_rif_directory()?;
            let mut rif = Rif::new(Some(&rif_path))?;
            rif.check()?;
        } 
        Ok(())
    }

    /// Check if `sanity` subcommand was given and parse subcommand options
    fn subcommand_sanity(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("sanity") {
            let fix = sub_match.is_present("fix");

            let rif_path = utils::get_rif_directory()?;
            let mut rif = Rif::new(Some(&rif_path))?;
            rif.sanity(fix)?;
        } 
        Ok(())
    }

    /// Check if `status` subcommand was given and parse subcommand options
    fn subcommand_status(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("status") {
            let ignore = sub_match.is_present("ignore");
            let verbose = sub_match.is_present("verbose");

            let rif_path = utils::get_rif_directory()?;
            let mut rif = Rif::new(Some(&rif_path))?;
            rif.status(ignore, verbose)?;
        } 
        Ok(())
    }
    
    fn subcommand_depend(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("depend") {
            if let Some(file) = sub_match.value_of("FILE") {
                let rif_path = utils::get_rif_directory()?;
                let rif = Rif::new(Some(&rif_path))?;
                rif.depend(Path::new(file))?;
            } else {
                eprintln!("No argument for depend");
            }

        }

        Ok(())
    }

    /// Check if `data` subcommand is given
    fn subcommand_data(matches: &clap::ArgMatches) -> Result<(), RifError> {
        if let Some(sub_match) = matches.subcommand_matches("data") {
            let data_type = sub_match.value_of("TYPE");
            let compact = sub_match.is_present("compact");

            let rif_path = utils::get_rif_directory()?;
            let rif = Rif::new(Some(&rif_path))?;
            rif.data(data_type, compact)?;
        }

        Ok(())
    }
}
