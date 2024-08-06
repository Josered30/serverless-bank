use crate::domain::errors::command_handler_error::CommandHandlerError;
use crate::domain::errors::entrypoint_error::EntrypointError;
use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug)]
pub struct BaseError {
   pub message: String,
}

impl BaseError {
    fn new(message: String) -> Self {
        BaseError {
            message
        }
    }
}

impl From<RepositoryError> for BaseError {
    fn from(value: RepositoryError) -> Self {
        match value { RepositoryError::Error(message) => BaseError::new(message) }
    }
}

impl From<EntrypointError> for BaseError {
    fn from(value: EntrypointError) -> Self {
        match value { EntrypointError::Error(message) => BaseError::new(message) }
    }
}

impl From<CommandHandlerError> for BaseError {
    fn from(value: CommandHandlerError) -> Self {
        match value { CommandHandlerError::Error(message) => BaseError::new(message) }
    }
}