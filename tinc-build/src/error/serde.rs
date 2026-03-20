use crate::{codegen::cel::compiler::CompileError, error::CelEvalError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum SerdeError {
    #[error("oneof fields cannot be flattened")]
    OneofFlattened,

    #[error("flattened fields must be messages or oneofs")]
    FlattenedFieldNotMessageOrOneof,

    #[error("expression parse failed: {0}")]
    ExpressionParse(#[source] cel_parser::ParseError),

    #[error("cel expression failed: {0}")]
    CelExpression(#[from] CompileError),

    #[error("message format failed: {0}")]
    MessageFormat(#[from] CelEvalError),

    #[error("message-level cel expression parse failed: {0}")]
    MessageLevelExpressionParse(#[source] cel_parser::ParseError),

    #[error("message-level cel expression failed: {0}")]
    MessageLevelCelExpression(#[source] CompileError),

    #[error("message-level message format failed: {0}")]
    MessageLevelMessageFormat(#[source] CelEvalError),
}
