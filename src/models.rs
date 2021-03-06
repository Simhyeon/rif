use serde::{Serialize, Deserialize};
use crate::utils;

/// File status of any file.
///
/// Fresh means a file is up to date
/// while stale means referencing file has been updated and the file has not.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub(crate) enum FileStatus {
    Fresh,
    Neutral, // Reserved for future usages. Might be deleted after all.
    Stale,
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let FileStatus::Stale = self {
            write!(f, "{}", utils::red("(s)"))
        } else { // Print nothing
            write!(f, "")
        }
    }
}

/// Sanity type to branch sanity check.
///
/// Direct only check self referencing while indirect also checks infinite loop
pub(crate) enum SanityType {
    Direct,
    Indirect
}

/// Medium variable to check reference status.
///
/// This enumerator is used for sanity checking and determines whether rif
/// sanity is assured or not.
pub(crate) enum RefStatus {
    Invalid,
    Valid
}

/// Loop diversion enumerator
///
/// Used with walk_directory_recursive method, so that given function can decide when to stop recursion.
pub(crate) enum LoopBranch {
    Exit,
    Continue,
}

// This is exposed to user
#[derive(Debug)]
pub enum ListType {
    All,
    Stale,
    None,
}

impl ListType {
    pub fn from(raw : &str) -> Self {
        match raw.to_lowercase().as_str() {
            "all" => {
                Self::All
            }
            "stale" => {
                Self::Stale
            }
            _ => Self::None,
        }
    }
}
