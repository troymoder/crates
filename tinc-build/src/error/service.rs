use crate::error::OpenApiError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ServiceError {
    #[error("currently streaming is not supported by tinc methods")]
    StreamingNotSupported,

    #[error("openapi: {0}")]
    OpenApiSchema(#[from] OpenApiError),
}
