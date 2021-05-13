use clap::clap_app;

struct Cli{}

impl Cli {
    pub fn parse() {
        // TODO ::: Currently testing and learning clap
        let matches = clap_app!(myapp =>
            (version: "0.0.1")
            (author: "Simon Creek <simoncreek@tutanota.com>")
            (about: "Manipulate rif file")
            (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
            (@subcommand add =>
                (about: "Add file to rif")
                (@arg debug: -d ... "Sets the level of debugging information")
            )
        ).get_matches();
    }
}

