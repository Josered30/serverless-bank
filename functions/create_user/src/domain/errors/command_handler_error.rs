use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CommandHandlerError {
    Error(String),
}

impl Error for CommandHandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "An error has ocurred in command handler"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for CommandHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandHandlerError::Error(error) => write!(f, "{}", error),
        }
    }
}
