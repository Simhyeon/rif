mod models;
mod process;
mod test;
mod utils;
mod checker;

use std::path::{PathBuf};
use models::{RifList, RifError, FileStatus};
use checker::Checker;
use process::Reader;

fn main() -> Result<(), RifError> {

    let mut rif_list = Reader::read(
        PathBuf::from("test/test.json")
    )?;

    //let path = PathBuf::from("test/new.md");
    //rif_list.add_file(&path)?;
    //rif_list.add_reference(&path, &PathBuf::from("test/test.json"))?;
    
    //println!("{:#?}", rif_list);
    let mut checker = Checker::new();
    checker.add_rif_list(&rif_list)?;
    checker.check(&mut rif_list)?;
    println!("LOG ::: Checking ");
    println!("{:#?}", rif_list);

    Ok(())
}
