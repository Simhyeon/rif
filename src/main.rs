mod models;
mod fileio;
mod utils;
mod checker;
mod cli;
mod consts;

use models::rif_error::RifError;
use cli::Cli;

fn main() -> Result<(), RifError> {

    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
