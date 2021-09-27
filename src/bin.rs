#[cfg(feature = "clap")]
use rif::Cli;
use rif::RifError;

fn main() -> Result<(), RifError> {

    // Enable color on pager such as "less"
    #[cfg(feature = "color")]
    colored::control::set_override(true);
    #[cfg(feature = "clap")]
    if let Err(error_content) =  Cli::parse() {
        println!("{}", error_content);
    }

    Ok(())
}
