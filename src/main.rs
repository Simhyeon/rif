mod models;
mod fileio;
mod test;
mod utils;
mod checker;
mod cli;
mod consts;

use models::RifError;
use cli::Cli;

fn main() -> Result<(), RifError> {

    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
