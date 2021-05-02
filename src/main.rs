mod models;
mod process;
mod test;

use std::path::{PathBuf};
use models::{RifList, RifError};
use process::Reader;

fn main() -> Result<(), RifError> {
    let mut rif_list = Reader::read(
        PathBuf::from("test/test.json")
    )?;
    println!("{:#?}", rif_list);

    rif_list.add_file(PathBuf::from("test/new.md"))?;
    println!("{:#?}", rif_list);

    Ok(())
}
