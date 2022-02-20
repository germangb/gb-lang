use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, thiserror::Error)]
pub enum Error<'input> {
    /// Tokenization error.
    Lex(#[from] crate::lex::Error),

    /// Unexpected token error.
    UnexpectedToken(crate::lex::Token<'input>),

    /// Tokenizer ran out of tokens.
    TokenizerEmpty,
}

impl Display for Error<'_> {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
