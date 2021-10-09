use crate::{
    ast::{Context, Error, Grammar},
    lex::{tokens, tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait TypeGrammar<'input>: Grammar<'input> {}

impl<'input, T: TypeGrammar<'input>> TypeGrammar<'input> for Box<T> {}

pub enum Type<'input> {
    U8(U8<'input>),
    Array(Array<'input, Box<Type<'input>>>),
    Ptr(Ptr<'input, Box<Type<'input>>>),
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
            Some(Ok(Token::Ptr(_))) => Ok(Type::Ptr(Grammar::parse(tokens, context)?)),
            Some(Ok(_)) => {
                tokens.next().unwrap().unwrap();
                Err(Error::UnexpectedToken)
            }
            Some(Err(_)) => {
                tokens.next().expect("Expected some token")?;
                unreachable!();
            }
            None => Err(Error::TokenizerEmpty),
        }
    }
}

#[derive(parse_derive::TypeGrammar)]
pub struct U8<'input>(pub tokens::U8<'input>);

#[derive(parse_derive::TypeGrammar)]
pub struct Array<'input, T>
where
    T: TypeGrammar<'input>,
{
    pub square_left: tokens::SquareLeft<'input>,
    pub type_: T,
    pub number: tokens::Number<'input>,
    pub square_right: tokens::SquareRight<'input>,
}

#[derive(parse_derive::TypeGrammar)]
pub struct Ptr<'input, T>
where
    T: TypeGrammar<'input>,
{
    pub ptr: tokens::Ptr<'input>,
    pub less_than: tokens::LessThan<'input>,
    pub type_: T,
    pub greater_than: tokens::GreaterThan<'input>,
}
