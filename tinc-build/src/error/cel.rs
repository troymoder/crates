use crate::codegen::cel::compiler::CompileError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum CelEvalError {
    #[error("failed to parse message format: {0}")]
    ParseMessageFormat(String),

    #[error("failed to parse cel expression: {0}")]
    ParseCelExpression(#[source] cel_parser::ParseError),

    #[error("cel resolve failed: {0}")]
    Resolve(#[from] CompileError),
}
