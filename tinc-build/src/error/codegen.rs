use crate::error::{SerdeError, ServiceError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum CodegenError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Service(#[from] ServiceError),
}
