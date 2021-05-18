mod checker;
mod cli;
mod consts;
mod fileio;
mod models;
mod utils;

use cli::Cli;
use models::rif_error::RifError;

fn main() -> Result<(), RifError> {

    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
