use crate::{
    ast::{Context, Error, Grammar},
    lex,
    lex::{tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait TypeGrammar<'input>: Grammar<'input> {}

impl<'input, T: TypeGrammar<'input>> TypeGrammar<'input> for Box<T> {}

pub enum Type<'input> {
    U8(U8<'input>),
    Array(Array<'input, Box<Type<'input>>>),
}

impl<'input> TypeGrammar<'input> for Type<'input> {}

impl<'input> Grammar<'input> for Type<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Ok(Token::U8(_))) => Ok(Type::U8(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::SquareLeft(_))) => Ok(Type::Array(Grammar::parse(tokens, context)?)),
            Some(Ok(_)) => {
                tokens
                    .next()
                    .expect("Expected some token")
                    .expect("Expected Ok token");
                Err(Error::UnexpectedToken)
            }
            Some(Err(_)) => Err(Error::Lex(
                tokens
                    .next()
                    .expect("Expected some token")
                    .expect_err("Expected token error."),
            )),
            None => {
                let _ = tokens.next();
                Err(Error::TokenizerEmpty)
            }
        }
    }
}

pub struct U8<'input>(lex::tokens::U8<'input>);

impl<'input> TypeGrammar<'input> for U8<'input> {}

impl<'input> Grammar<'input> for U8<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self(Grammar::parse(tokens, context)?))
    }
}

pub struct Array<'input, T: TypeGrammar<'input>> {
    pub square_left: lex::tokens::SquareLeft<'input>,
    pub type_: T,
    pub number: lex::tokens::Number<'input>,
    pub square_right: lex::tokens::SquareRight<'input>,
}

impl<'input, T: TypeGrammar<'input>> TypeGrammar<'input> for Array<'input, T> {}

impl<'input, T: TypeGrammar<'input>> Grammar<'input> for Array<'input, T> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            square_left: Grammar::parse(tokens, context)?,
            type_: Grammar::parse(tokens, context)?,
            number: Grammar::parse(tokens, context)?,
            square_right: Grammar::parse(tokens, context)?,
        })
    }
}
