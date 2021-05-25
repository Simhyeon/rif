use std::process::Command;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{ 
    // History Capacity
    // Check hook status
    // Check hook script 
    // Argument options(Which argument to pass) -> Only get fresh vector, Only get stale vector, Get json struct
    // Set check after update, or --check flag as default
}
