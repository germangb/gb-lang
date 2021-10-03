use crate::lex::Tokenizer;
pub use error::Error;
use std::iter::Peekable;

mod error;
pub mod expressions;
pub mod statements;
pub mod types;

pub fn parse<'input, G: Grammar<'input>>(input: &'input str) -> Result<G, Error> {
    let mut tokens = crate::lex::tokenize(input).peekable();
    let mut context = Context::default();
    G::parse(&mut tokens, &mut context)
}

pub trait Grammar<'input>: Sized {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error>;
}

impl<'input, G: Grammar<'input>> Grammar<'input> for Box<G> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Box::new(Grammar::parse(tokens, context)?))
    }
}

#[derive(Default)]
pub struct Context {}
