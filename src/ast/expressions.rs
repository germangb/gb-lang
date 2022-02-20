use crate::{
    ast::{Context, Error, Grammar},
    lex::{tokens, tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait ExpressionGrammar<'input>: Grammar<'input> {}

impl<'input, E: ExpressionGrammar<'input>> ExpressionGrammar<'input> for Box<E> {}

#[derive(Debug)]
pub enum Expression<'input> {
    Parenthesis(Parenthesis<'input, Box<Expression<'input>>>),
    Index(Index<'input, Box<Expression<'input>>, Box<Expression<'input>>>),
    Call(Call<'input, Box<Expression<'input>>>),
    Number(Number<'input>),
    Str(Str<'input>),
    Identifier(Identifier<'input>),
    Add(Add<'input, Box<Expression<'input>>, Box<Expression<'input>>>),
    Subtract(Subtract<'input, Box<Expression<'input>>, Box<Expression<'input>>>),
    Multiply(Multiply<'input, Box<Expression<'input>>, Box<Expression<'input>>>),
    Divide(Divide<'input, Box<Expression<'input>>, Box<Expression<'input>>>),
}

impl<'input> ExpressionGrammar<'input> for Expression<'input> {}

impl<'input> Grammar<'input> for Expression<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error<'input>> {
        use Expression::*;
        match tokens.peek() {
            None => Err(Error::TokenizerEmpty),
            Some(Ok(Token::Number(_))) => Ok(Number(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Str(_))) => Ok(Str(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Identifier(_))) => Ok(Identifier(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::ParLeft(_))) => Ok(Parenthesis(Grammar::parse(tokens, context)?)),
            Some(Ok(_)) => Err(Error::UnexpectedToken(tokens.next().unwrap()?)),
            Some(Err(_)) => {
                tokens.next().expect("Expected some token")?;
                unreachable!();
            }
        }
    }
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Parenthesis<'input, E>
where
    E: ExpressionGrammar<'input>,
{
    pub par_left: tokens::ParLeft<'input>,
    pub inner: E,
    pub par_right: tokens::ParLeft<'input>,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Number<'input>(pub tokens::Number<'input>);

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Str<'input>(pub tokens::Str<'input>);

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Identifier<'input>(pub tokens::Identifier<'input>);

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Add<'input, L, R>
where
    L: ExpressionGrammar<'input>,
    R: ExpressionGrammar<'input>,
{
    pub left: L,
    pub plus: tokens::Plus<'input>,
    pub right: R,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Subtract<'input, L, R>
where
    L: ExpressionGrammar<'input>,
    R: ExpressionGrammar<'input>,
{
    pub left: L,
    pub minus: tokens::Minus<'input>,
    pub right: R,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Multiply<'input, L, R>
where
    L: ExpressionGrammar<'input>,
    R: ExpressionGrammar<'input>,
{
    pub left: L,
    pub star: tokens::Star<'input>,
    pub right: R,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Divide<'input, L, R>
where
    L: ExpressionGrammar<'input>,
    R: ExpressionGrammar<'input>,
{
    pub left: L,
    pub forward_slash: tokens::ForwardSlash<'input>,
    pub right: R,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Index<'input, In, I>
where
    In: ExpressionGrammar<'input>,
    I: ExpressionGrammar<'input>,
{
    pub indexable: In,
    pub square_left: tokens::SquareLeft<'input>,
    pub index: I,
    pub square_right: tokens::SquareRight<'input>,
}

#[derive(Debug, parse_derive::ExpressionGrammar)]
pub struct Call<'input, C>
where
    C: ExpressionGrammar<'input>,
{
    pub callable: C,
    pub par_left: tokens::ParLeft<'input>,
    pub par_right: tokens::ParRight<'input>,
}
