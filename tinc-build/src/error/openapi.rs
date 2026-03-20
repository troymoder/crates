use crate::types::ProtoPath;

#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenApiError {
    #[error("map keys must be a string")]
    MapKeyNotString,

    #[error("f64 is not a valid float")]
    InvalidF64,

    #[error("i64 is not a valid int")]
    InvalidI64,

    #[error("u64 is not a valid uint")]
    InvalidU64,

    #[error("could not find enum {0}")]
    EnumNotFound(ProtoPath),

    #[error("{enum_path} has no value for {value}")]
    EnumValueNotFound { enum_path: ProtoPath, value: i32 },

    #[error("cel parse failed: {0}")]
    CelParse(#[source] cel_parser::ParseError),

    #[error("cel resolve failed: {0}")]
    CelResolve(#[from] crate::codegen::cel::compiler::CompileError),

    #[error("expression needs runtime evaluation")]
    ExpressionNeedsRuntimeEvaluation,

    #[error("bad openapi schema: {0}")]
    BadSchema(#[source] serde_json::Error),

    #[error("cannot extract field on non-message type: {0}")]
    ExtractFieldNonMessage(String),

    #[error("message does not have field: {0}")]
    MessageMissingField(String),

    #[error("type cannot be mapped: {0}")]
    TypeCannotBeMapped(String),

    #[error("well-known type can only have one field")]
    WellKnownTypeMultipleFields,

    #[error("well-known type can only have field 'value'")]
    WellKnownTypeWrongField,

    #[error("duplicate path: {0}")]
    DuplicatePath(String),

    #[error("{0} is already used by another operation")]
    FieldAlreadyUsed(String),

    #[error("query string cannot be used on nested types")]
    QueryStringNestedTypes,

    #[error("query string can only be used on message types")]
    QueryStringNonMessage,

    #[error("content-type must be a string type")]
    ContentTypeNotString,

    #[error("content-type cannot be nested")]
    ContentTypeNested,

    #[error("binary bodies must be on bytes fields")]
    BinaryBodyNonBytes,

    #[error("binary bodies cannot be nested")]
    BinaryBodyNested,

    #[error("text bodies can only be used on string")]
    TextBodyNonString,

    #[error("text bodies cannot be nested")]
    TextBodyNested,

    #[error("missing enum: {0}")]
    MissingEnum(ProtoPath),

    #[error("missing message: {0}")]
    MissingMessage(ProtoPath),

    #[error("openapi json serialization failed: {0}")]
    Json(#[source] serde_json::Error),
}
