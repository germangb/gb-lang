use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// Tokenization errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Triggered when the input source ends with an open-ended string token.
    OpenEndedStringToken,

    /// Invalid number format.
    InvalidNumberToken,
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
