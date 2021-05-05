mod models;
mod process;
mod test;
mod utils;
mod checker;

use std::path::{PathBuf};
use models::{RifList, RifError, FileStatus};
use process::Reader;

fn main() -> Result<(), RifError> {

    let mut rif_list = Reader::read(
        PathBuf::from("test/test.json")
    )?;

    let path = PathBuf::from("test/new.md");

    rif_list.add_file(&path)?;
    rif_list.add_reference(&path, &PathBuf::from("test/test.json"))?;
    println!("{:#?}", rif_list);
    //rif_list.set_file_status(&path, FileStatus::Stale)?;
    //rif_list.remove_file(&path)?;

    Ok(())
}
