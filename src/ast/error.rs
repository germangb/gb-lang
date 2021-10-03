use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Tokenization error.
    Lex(#[from] crate::lex::Error),

    /// Unexpected token error.
    UnexpectedToken,

    /// Tokenizer ran out of tokens.
    TokenizerEmpty,
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
