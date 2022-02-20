use crate::{
    ast::{Context, Error, Grammar},
    lex::{tokens, tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait TypeGrammar<'input>: Grammar<'input> {}

impl<'input, T: TypeGrammar<'input>> TypeGrammar<'input> for Box<T> {}

#[derive(Debug)]
pub enum Type<'input> {
    U8(U8<'input>),
    Array(Array<'input, Box<Type<'input>>>),
    Ptr(Ptr<'input, Box<Type<'input>>>),
    Struct(Struct<'input>),
}

impl<'input> TypeGrammar<'input> for Type<'input> {}

impl<'input> Grammar<'input> for Type<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error<'input>> {
        match tokens.peek() {
            Some(Ok(Token::U8(_))) => Ok(Type::U8(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Array(_))) => Ok(Type::Array(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Ptr(_))) => Ok(Type::Ptr(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Struct(_))) => Ok(Type::Struct(Grammar::parse(tokens, context)?)),
            Some(Ok(_)) => Err(Error::UnexpectedToken(tokens.next().unwrap()?)),
            Some(Err(_)) => {
                tokens.next().expect("Expected some token")?;
                unreachable!();
            }
            None => Err(Error::TokenizerEmpty),
        }
    }
}

#[derive(Debug, parse_derive::TypeGrammar)]
pub struct U8<'input>(pub tokens::U8<'input>);

#[derive(Debug, parse_derive::TypeGrammar)]
pub struct Array<'input, T>
where
    T: TypeGrammar<'input>,
{
    pub array: tokens::Array<'input>,
    pub less_than: tokens::LessThan<'input>,
    pub type_: T,
    pub comma: tokens::Comma<'input>,
    pub number: tokens::Number<'input>,
    pub greater_than: tokens::GreaterThan<'input>,
}

#[derive(Debug, parse_derive::TypeGrammar)]
pub struct Ptr<'input, T>
where
    T: TypeGrammar<'input>,
{
    pub ptr: tokens::Ptr<'input>,
    pub less_than: tokens::LessThan<'input>,
    pub type_: T,
    pub greater_than: tokens::GreaterThan<'input>,
}

#[derive(Debug, parse_derive::TypeGrammar)]
pub struct Struct<'input> {
    pub struct_: tokens::Struct<'input>,
    pub curly_left: tokens::CurlyLeft<'input>,
    pub curly_right: tokens::CurlyRight<'input>,
}
