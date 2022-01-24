#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedTermination,
    NoToken,
    InvalidType,
    UnexpectedToken,
    InvalidIdentifier,
}
