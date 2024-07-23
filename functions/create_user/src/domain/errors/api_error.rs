use super::{
    command_handler_error::CommandHandlerError, entrypoint_error::EntrypointError,
    repository_error::RepositoryError,
};

pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(code: u16, message: String) -> Self {
        ApiError { code, message }
    }
}

impl From<CommandHandlerError> for ApiError {
    fn from(value: CommandHandlerError) -> Self {
        match value {
            CommandHandlerError::Error(error) => ApiError::new(500, error),
        }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Error(error) => ApiError::new(500, error),
        }
    }
}

impl From<EntrypointError> for ApiError {
    fn from(value: EntrypointError) -> Self {
        match value {
            EntrypointError::Error(error) => ApiError::new(500, error),
        }
    }
}
