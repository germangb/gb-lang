use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// Tokenization errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error variant for internal-use only.
    Internal,

    /// Triggered when the input source ends with an open-ended string token.
    OpenEndedStringToken,
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
