use serde::{Serialize, Deserialize};
use colored::*;

/// File status of any file.
///
/// Fresh means a file is up to date
/// while stale means referencing file has been updated and the file has not.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum FileStatus {
    Fresh,
    Neutral, // Reserved for future usages. Might be deleted after all.
    Stale,
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

/// Sanity type to branch sanity check.
///
/// Direct only check self referencing while indirect also checks infinite loop
pub enum SanityType {
    Direct,
    Indirect
}

/// Medium variable to check reference status.
///
/// This enumerator is used for sanity checking and determines whether rif
/// sanity is assured or not.
pub enum RefStatus {
    Invalid,
    Valid
}
