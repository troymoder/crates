#[derive(Debug, thiserror::Error)]
pub(crate) enum ProstExploreError {
    #[error("empty proto package for file: {0}")]
    EmptyPackage(String),

    #[error("message {0}: {1}")]
    Message(String, #[source] ProstExploreMessageError),

    #[error("enum {0}: {1}")]
    Enum(String, #[source] ProstExploreEnumError),

    #[error("service {0}: {1}")]
    Service(String, #[source] ProstExploreServiceError),

    #[error("gathering cel expressions: {0}")]
    GatherCelExpressions(#[source] ProstExtensionDecodeError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProstExploreMessageError {
    #[error("message decode failed: {0}")]
    MessageDecode(#[source] ProstExtensionDecodeError),

    #[error("field decode failed: {0}")]
    FieldDecode(#[source] ProstExtensionDecodeError),

    #[error("gathering cel expressions failed: {0}")]
    GatherCel(#[source] ProstExtensionDecodeError),

    #[error("oneof decode failed: {0}")]
    OneofDecode(#[source] ProstExtensionDecodeError),

    #[error("child enum failed: {0}")]
    ChildEnum(#[from] ProstExploreEnumError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProstExploreEnumError {
    #[error("enum decode failed: {0}")]
    EnumDecode(#[source] ProstExtensionDecodeError),

    #[error("variant decode failed: {0}")]
    VariantDecode(#[source] ProstExtensionDecodeError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProstExploreServiceError {
    #[error("service decode failed: {0}")]
    ServiceDecode(#[source] ProstExtensionDecodeError),

    #[error("method decode failed: {0}")]
    MethodDecode(#[source] ProstExtensionDecodeError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProstExtensionDecodeError {
    #[error("expected message")]
    ExpectedMessage,

    #[error("expected message or list of messages")]
    ExpectedMessageOrList,

    #[error("transcoding failed: {0}")]
    TranscodeFailed(#[source] prost::DecodeError),
}
