mod checker;
mod cli;
mod consts;
mod fileio;
mod models;
mod utils;
mod history;

use cli::Cli;
use models::rif_error::RifError;
use colored::*;

fn main() -> Result<(), RifError> {

    // Enable color on pager such as "less"
    colored::control::set_override(true);
    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
