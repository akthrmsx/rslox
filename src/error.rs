use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ScanError {
    #[error("[line {}] error: unexpected character", line)]
    UnexpectedCharacter { line: usize },
    #[error("[line {}] error: unterminated string", line)]
    UnterminatedString { line: usize },
}
