mod models;
mod fileio;
mod test;
mod utils;
mod checker;
mod cli;
mod tracker;
mod consts;

use std::path::{PathBuf};
use models::{RifList, RifError, FileStatus};
use checker::Checker;
use fileio::FileIO;
use cli::Cli;

fn main() -> Result<(), RifError> {

    println!("{:#?}", std::env::current_dir()?);
    Cli::parse()?;

    //let mut rif_list = FileIO::read(
        //PathBuf::from("test/test.json")
    //)?;
    
    //let mut checker = Checker::new();
    //checker.add_rif_list(&rif_list)?;
    //checker.check(&mut rif_list)?;

    //println!("{:#?}", rif_list);
    
    //FileIO::save(PathBuf::from("test/test_new.json"), rif_list)?;

    Ok(())
}
