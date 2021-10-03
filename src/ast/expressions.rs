use crate::{
    ast::{Context, Error, Grammar},
    lex,
    lex::{tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait ExpressionGrammar<'input>: Grammar<'input> {}

impl<'input, E: ExpressionGrammar<'input>> ExpressionGrammar<'input> for Box<E> {}

pub enum Expression<'input> {
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

pub struct Number<'input>(lex::tokens::Number<'input>);
pub struct Str<'input>(lex::tokens::Str<'input>);
pub struct Identifier<'input>(lex::tokens::Identifier<'input>);

impl<'input> ExpressionGrammar<'input> for Number<'input> {}
impl<'input> ExpressionGrammar<'input> for Str<'input> {}
impl<'input> ExpressionGrammar<'input> for Identifier<'input> {}

impl<'input> Grammar<'input> for Number<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self(Grammar::parse(tokens, context)?))
    }
}

impl<'input> Grammar<'input> for Str<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self(Grammar::parse(tokens, context)?))
    }
}

impl<'input> Grammar<'input> for Identifier<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self(Grammar::parse(tokens, context)?))
    }
}
