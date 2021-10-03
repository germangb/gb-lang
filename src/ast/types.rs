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
                tokens
                    .next()
                    .expect("Expected some token")
                    .expect("Expected Ok token");
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

pub struct U8<'input>(pub tokens::U8<'input>);

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
    pub square_left: tokens::SquareLeft<'input>,
    pub type_: T,
    pub number: tokens::Number<'input>,
    pub square_right: tokens::SquareRight<'input>,
}

impl<'input, T> TypeGrammar<'input> for Array<'input, T> where T: TypeGrammar<'input> {}

impl<'input, T> Grammar<'input> for Array<'input, T>
where
    T: TypeGrammar<'input>,
{
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

pub struct Ptr<'input, T: TypeGrammar<'input>> {
    pub ptr: tokens::Ptr<'input>,
    pub less_than: tokens::LessThan<'input>,
    pub type_: T,
    pub greater_than: tokens::GreaterThan<'input>,
}

impl<'input, T: TypeGrammar<'input>> TypeGrammar<'input> for Ptr<'input, T> {}

impl<'input, T> Grammar<'input> for Ptr<'input, T>
where
    T: TypeGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            ptr: Grammar::parse(tokens, context)?,
            less_than: Grammar::parse(tokens, context)?,
            type_: Grammar::parse(tokens, context)?,
            greater_than: Grammar::parse(tokens, context)?,
        })
    }
}
