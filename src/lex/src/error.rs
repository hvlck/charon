#[derive(Debug)]
pub enum LexError {
    UnterminatedString,
    InvalidNumber,
    UnterminatedComment
}
