pub mod cli;
pub mod utils;
pub(crate) mod rif;
pub mod models;
pub(crate) mod checker;
pub(crate) mod consts;
mod error;

pub use crate::error::RifError;
pub use crate::rif::*;
