use std::fmt;

pub enum Error {
    LexError(LexError),
    ParseError(ParseError),
    TypeError(TypeError),
    ICE(Ice),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexError(err) => write!(f, "{}", err.display()),
            Error::ParseError(_) => {}
            Error::TypeError(_) => {}
            Error::ICE(_) => {}
        }
    }
}

pub struct ErrorMessage<'a> {
    file: &'a str,
    line: usize,
    column: usize,
    // length from column to end of error
    len: usize,
    source_text: &'a str,
    message: Error,
}

impl<'a> ErrorMessage<'a> {
    pub fn new(
        line: usize,
        column: usize,
        len: usize,
        source: &'a str,
        message: Error,
    ) -> ErrorMessage {
        ErrorMessage {
            line,
            column,
            len,
            source_text: source,
            message,
        }
    }
}

pub fn error_to_stdout(error: ErrorMessage) {
    println!(
        "line {}:{}\n{}\n{}",
        error.line, error.column, error.source_text, error.message
    )
}
