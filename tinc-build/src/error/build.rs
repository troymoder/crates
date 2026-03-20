use std::path::PathBuf;

use crate::error::CodegenError;
#[cfg(feature = "prost")]
use crate::error::ProstExploreError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum InternalBuildError {
    #[error("failed to create tinc directory")]
    CreateTincDir(#[source] std::io::Error),

    #[error("failed to write tinc_annotations.rs")]
    WriteTincAnnotations(#[source] std::io::Error),

    #[cfg(feature = "prost")]
    #[error("failed to generate tonic fds")]
    GenerateTonicFds(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to read tonic fds")]
    ReadTonicFds(#[source] std::io::Error),

    #[cfg(feature = "prost")]
    #[error("failed to add tonic fds")]
    AddTonicFds(#[source] prost_reflect::DescriptorError),

    #[error("failed to remove file {path}")]
    RemoveFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write fds")]
    WriteFds(#[source] std::io::Error),

    #[cfg(feature = "prost")]
    #[error("prost compile failed")]
    ProstCompile(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to write root module")]
    WriteRootModule(#[source] std::io::Error),

    #[error("failed to parse module {path}")]
    ParseModule {
        path: PathBuf,
        #[source]
        source: syn::Error,
    },

    #[error("failed to read module {path}")]
    ReadModule {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write module {path}")]
    WriteModuleFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[cfg(feature = "prost")]
    #[error(transparent)]
    ProstExplore(#[from] ProstExploreError),

    #[error(transparent)]
    Codegen(#[from] CodegenError),
}
