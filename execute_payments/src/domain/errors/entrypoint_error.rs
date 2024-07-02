use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum EntrypointError {
    Error(String),
}

impl Error for EntrypointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "An error has ocurred in entrypoint"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for EntrypointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntrypointError::Error(error) => write!(f, "{}", error),
        }
    }
}
