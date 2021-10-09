use crate::{
    ast::{Context, Error, Grammar},
    lex::{tokens, tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait ExpressionGrammar<'input>: Grammar<'input> {}

impl<'input, E: ExpressionGrammar<'input>> ExpressionGrammar<'input> for Box<E> {}

pub enum Expression<'input> {
    Parenthesis(Parenthesis<'input, Box<Expression<'input>>>),
    Number(Number<'input>),
    Str(Str<'input>),
    Identifier(Identifier<'input>),
}

impl<'input> ExpressionGrammar<'input> for Expression<'input> {}

impl<'input> Grammar<'input> for Expression<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Ok(Token::Number(_))) => Ok(Expression::Number(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Str(_))) => Ok(Expression::Str(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Identifier(_))) => {
                Ok(Expression::Identifier(Grammar::parse(tokens, context)?))
            }
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

#[derive(parse_derive::ExpressionGrammar)]
pub struct Parenthesis<'input, E>
where
    E: ExpressionGrammar<'input>,
{
    pub par_left: tokens::ParLeft<'input>,
    pub inner: E,
    pub par_right: tokens::ParLeft<'input>,
}

#[derive(parse_derive::ExpressionGrammar)]
pub struct Number<'input>(pub tokens::Number<'input>);

#[derive(parse_derive::ExpressionGrammar)]
pub struct Str<'input>(pub tokens::Str<'input>);

#[derive(parse_derive::ExpressionGrammar)]
pub struct Identifier<'input>(pub tokens::Identifier<'input>);

#[derive(parse_derive::ExpressionGrammar)]
pub struct Add<'input, L, R>
where
    L: ExpressionGrammar<'input>,
    R: ExpressionGrammar<'input>,
{
    pub left: L,
    pub plus: tokens::Plus<'input>,
    pub right: R,
}

#[derive(parse_derive::ExpressionGrammar)]
pub struct Index<'input, E>
where
    E: ExpressionGrammar<'input>,
{
    pub ident: tokens::Identifier<'input>,
    pub square_left: tokens::SquareLeft<'input>,
    pub index: E,
    pub square_right: tokens::SquareRight<'input>,
}
