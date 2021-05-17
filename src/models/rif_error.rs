#[derive(Debug)]
pub enum RifError {
    AddFail(String),
    Ext(String),
    RifIoError(String),
    CliError(String),
    GetFail(String),
    InvalidFormat(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    CheckerError(String),
}

impl std::fmt::Display for RifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RifError::AddFail(content) => write!(f, "{}", content),
            RifError::Ext(content) => write!(f, "{}", content),
            RifError::GetFail(content) => write!(f, "{}", content),
            RifError::InvalidFormat(content) => write!(f, "{}", content),
            RifError::IoError(content) => write!(f, "{}", content),
            RifError::CliError(content) => write!(f, "{}", content),
            RifError::RifIoError(content) => write!(f, "{}", content),
            RifError::SerdeError(content) => write!(f, "{}", content),
            RifError::CheckerError(content) => write!(f, "{}", content),
        }
    }
}

impl From<std::io::Error> for RifError {
    fn from(err : std::io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<serde_json::Error> for RifError {
    fn from(err : serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}
