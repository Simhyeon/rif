mod models;
mod fileio;
mod test;
mod utils;
mod checker;

use std::path::{PathBuf};
use models::{RifList, RifError, FileStatus};
use checker::Checker;
use fileio::FileIO;

fn main() -> Result<(), RifError> {

    let mut rif_list = FileIO::read(
        PathBuf::from("test/test.json")
    )?;
    
    //let mut checker = Checker::new();
    //checker.add_rif_list(&rif_list)?;
    //checker.check(&mut rif_list)?;

    //println!("{:#?}", rif_list);
    
    //FileIO::save(PathBuf::from("test/test_new.json"), rif_list)?;

    Ok(())
}
