use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use serde::{Serialize,Deserialize};

use crate::models::FileStatus;
use crate::RifError;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Hook { 
    pub trigger: bool,
    pub command: Option<String>,
    pub arg_type: HookArgument,
}

impl Hook {
    pub fn execute(&self, arguments :Vec<(FileStatus ,PathBuf)>) -> Result<(), RifError> {
        // Don't trigger
        if !self.trigger {
            return Ok(());
        }

        let filtered_args : Vec<String>;

        match self.arg_type {
            HookArgument::Fresh => {
                filtered_args = arguments
                    .into_iter()
                    .filter(|file| if let FileStatus::Fresh = file.0 {true} else {false} )
                    .map(|file| 
                        file.1
                        .as_path()
                        .display()
                        .to_string()
                    )
                    .collect();
            },
            HookArgument::Stale => {
                filtered_args = arguments
                    .into_iter()
                    .filter(|file| if let FileStatus::Stale = file.0 {true} else {false} )
                    .map(|file| 
                        file.1
                        .as_path()
                        .display()
                        .to_string()
                    )
                    .collect();
            },
            HookArgument::All => {
                filtered_args = arguments
                    .into_iter()
                    .map(|file| 
                        file.1
                        .as_path()
                        .display()
                        .to_string()
                    )
                    .collect();
            }
            HookArgument::None => filtered_args = Vec::new(),
        }

        if let Some(cmd) = &self.command {
            let output = Command::new(cmd).args(&filtered_args[..]).output()?;
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();

            Ok(())
        } else {
            Err(RifError::ConfigError(String::from("Hook trigger is true but it's command is null")))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HookArgument {
    Stale,
    Fresh,
    All,
    None
}
