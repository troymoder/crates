mod build;
mod cel;
mod codegen;
mod openapi;
#[cfg(feature = "prost")]
mod prost_explore;
mod serde;
mod service;

pub(crate) use build::InternalBuildError;
pub(crate) use cel::CelEvalError;
pub(crate) use codegen::CodegenError;
pub(crate) use openapi::OpenApiError;
#[cfg(feature = "prost")]
pub(crate) use prost_explore::{
    ProstExploreEnumError, ProstExploreError, ProstExploreMessageError, ProstExploreServiceError,
    ProstExtensionDecodeError,
};
pub(crate) use serde::SerdeError;
pub(crate) use service::ServiceError;

/// Error returned when compiling protos or loading file descriptor sets.
#[derive(Debug)]
pub struct BuildError(InternalBuildError);

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<InternalBuildError> for BuildError {
    fn from(err: InternalBuildError) -> Self {
        BuildError(err)
    }
}
