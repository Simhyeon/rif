use rif::cli::Cli;
use rif::models::rif_error::RifError;

fn main() -> Result<(), RifError> {

    // Enable color on pager such as "less"
    colored::control::set_override(true);
    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
