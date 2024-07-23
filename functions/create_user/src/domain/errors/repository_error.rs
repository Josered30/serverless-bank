use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum RepositoryError {
    Error(String),
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "An error has ocurred in repository"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Error(error) => write!(f, "{}", error),
        }
    }
}
