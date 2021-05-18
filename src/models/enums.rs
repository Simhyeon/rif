use colored::*;
use serde::{ Serialize, Deserialize };

pub enum SanityType {
    Direct,
    Indirect
}

pub enum RefStatus {
    Invalid,
    Valid
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FileStatus {
    Stale,
    Fresh,
    Neutral // Reserved for future usages. Might be deleted after all.
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatus::Stale => write!(f, "{}", "[Stale]".red()),
            FileStatus::Fresh => write!(f, "{}", "[Fresh]".blue()),
            FileStatus::Neutral => write!(f, "{}", "[Neutral]".green()),
        }
    }
}
