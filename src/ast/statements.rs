use crate::{
    ast::{
        expressions::{Expression, ExpressionGrammar},
        types::{Type, TypeGrammar},
        Context, Error, Grammar,
    },
    lex::{tokens, tokens::Token, Tokenizer},
};
use std::iter::Peekable;

pub trait StatementGrammar<'input>: Grammar<'input> {}

impl<'input, S> StatementGrammar<'input> for Box<S> where S: StatementGrammar<'input> {}
impl<'input> StatementGrammar<'input> for () {}

pub enum Statement<'input> {
    Let(Let<'input, Type<'input>, Expression<'input>>),
    Const(Const<'input, Type<'input>>),
    Static(Static<'input, Type<'input>, Expression<'input>>),
    Scope(Scope<'input, Box<Statement<'input>>>),
    If(If<'input, Expression<'input>, Box<Statement<'input>>>),
    Loop(Loop<'input, Box<Statement<'input>>>),
    While(While<'input, Expression<'input>, Box<Statement<'input>>>),
}

impl<'input> StatementGrammar<'input> for Statement<'input> {}

impl<'input> Grammar<'input> for Statement<'input> {
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Ok(Token::Let(_))) => Ok(Statement::Let(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Const(_))) => Ok(Statement::Const(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Static(_))) => Ok(Statement::Static(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::CurlyLeft(_))) => Ok(Statement::Scope(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::If(_))) => Ok(Statement::If(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::Loop(_))) => Ok(Statement::Loop(Grammar::parse(tokens, context)?)),
            Some(Ok(Token::While(_))) => Ok(Statement::While(Grammar::parse(tokens, context)?)),
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

#[derive(parse_derive::StatementGrammar)]
pub struct Let<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
    pub let_: tokens::Let<'input>,
    pub identifier: tokens::Identifier<'input>,
    pub colon_colon: tokens::ColonColon<'input>,
    pub type_: T,
    pub equals: tokens::Equals<'input>,
    pub expression: E,
    pub semi_colon: tokens::SemiColon<'input>,
}

#[derive(parse_derive::StatementGrammar)]
pub struct Const<'input, T>
where
    T: TypeGrammar<'input>,
{
    pub const_: tokens::Const<'input>,
    pub identifier: tokens::Identifier<'input>,
    pub colon_colon: tokens::ColonColon<'input>,
    pub type_: T,
    pub semi_colon: tokens::SemiColon<'input>,
}

#[derive(parse_derive::StatementGrammar)]
pub struct Static<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
    pub static_: tokens::Static<'input>,
    pub identifier: tokens::Identifier<'input>,
    pub colon_colon: tokens::ColonColon<'input>,
    pub type_: T,
    pub equals: tokens::Equals<'input>,
    pub expression: E,
    pub semi_colon: tokens::SemiColon<'input>,
}

#[derive(parse_derive::StatementGrammar)]
pub struct Scope<'input, I>
where
    I: StatementGrammar<'input>,
{
    pub curly_left: tokens::CurlyLeft<'input>,
    pub inner: I,
    pub curly_right: tokens::CurlyRight<'input>,
}

#[derive(parse_derive::StatementGrammar)]
pub struct If<'input, E, I>
where
    E: ExpressionGrammar<'input>,
    I: StatementGrammar<'input>,
{
    pub if_: tokens::If<'input>,
    pub expression: E,
    pub inner: I,
}

#[derive(parse_derive::StatementGrammar)]
pub struct Loop<'input, I>
where
    I: StatementGrammar<'input>,
{
    pub loop_: tokens::Loop<'input>,
    pub inner: I,
}

#[derive(parse_derive::StatementGrammar)]
pub struct While<'input, E, I>
where
    E: ExpressionGrammar<'input>,
    I: StatementGrammar<'input>,
{
    pub while_: tokens::While<'input>,
    pub expression: E,
    pub inner: I,
}
