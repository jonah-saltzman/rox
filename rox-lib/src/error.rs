use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanErr {
    #[error("Scan error: {0}")]
    UnexpectedToken(String),
    #[error("Scan error: {0}")]
    ParseNumErr(String),
    #[error("Scan error: {0}")]
    UnexpectedEOF(String)
}

impl From<Vec<ScanErr>> for ParseError {
    fn from(value: Vec<ScanErr>) -> Self {
        Self::ScanError(DisplayableVec(value))
    }
}

#[derive(Debug)]
pub struct DisplayableVec(pub Vec<ScanErr>);

impl<'a> std::fmt::Display for DisplayableVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errs_str = self.0
            .iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", errs_str)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{0}")]
    ScanError(DisplayableVec),
}

impl From<ParseError> for RoxError {
    fn from(error: ParseError) -> Self {
        RoxError::ParseError(error)
    }
}

#[derive(Debug, Error)]
pub enum RoxError {
    #[error("{0}")]
    ParseError(ParseError),
}