use std::fmt;

#[derive(Debug)]
pub struct DateParseError;
impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value for date is invalid")
    }
}
impl From<chrono::format::ParseError> for DateParseError {
    fn from(_: chrono::format::ParseError) -> Self {
        Self
    }
}
