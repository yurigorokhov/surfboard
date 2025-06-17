use chrono::ParseError;

#[derive(Debug)]
pub enum SurfboardLibError {
    DateParsingError(ParseError),
}

impl From<ParseError> for SurfboardLibError {
    fn from(value: ParseError) -> Self {
        SurfboardLibError::DateParsingError(value)
    }
}
