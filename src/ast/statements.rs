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

impl<'input, T, E> StatementGrammar<'input> for Let<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
}

impl<'input, T, E> Grammar<'input> for Let<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            let_: Grammar::parse(tokens, context)?,
            identifier: Grammar::parse(tokens, context)?,
            colon_colon: Grammar::parse(tokens, context)?,
            type_: Grammar::parse(tokens, context)?,
            equals: Grammar::parse(tokens, context)?,
            expression: Grammar::parse(tokens, context)?,
            semi_colon: Grammar::parse(tokens, context)?,
        })
    }
}

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

impl<'input, T> StatementGrammar<'input> for Const<'input, T> where T: TypeGrammar<'input> {}

impl<'input, T> Grammar<'input> for Const<'input, T>
where
    T: TypeGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            const_: Grammar::parse(tokens, context)?,
            identifier: Grammar::parse(tokens, context)?,
            colon_colon: Grammar::parse(tokens, context)?,
            type_: Grammar::parse(tokens, context)?,
            semi_colon: Grammar::parse(tokens, context)?,
        })
    }
}

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

impl<'input, T, E> StatementGrammar<'input> for Static<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
}

impl<'input, T, E> Grammar<'input> for Static<'input, T, E>
where
    T: TypeGrammar<'input>,
    E: ExpressionGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            static_: Grammar::parse(tokens, context)?,
            identifier: Grammar::parse(tokens, context)?,
            colon_colon: Grammar::parse(tokens, context)?,
            type_: Grammar::parse(tokens, context)?,
            equals: Grammar::parse(tokens, context)?,
            expression: Grammar::parse(tokens, context)?,
            semi_colon: Grammar::parse(tokens, context)?,
        })
    }
}

pub struct Scope<'input, I>
where
    I: StatementGrammar<'input>,
{
    pub curly_left: tokens::CurlyLeft<'input>,
    pub inner: I,
    pub curly_right: tokens::CurlyRight<'input>,
}

impl<'input, I> StatementGrammar<'input> for Scope<'input, I> where I: StatementGrammar<'input> {}

impl<'input, I> Grammar<'input> for Scope<'input, I>
where
    I: StatementGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            curly_left: Grammar::parse(tokens, context)?,
            inner: Grammar::parse(tokens, context)?,
            curly_right: Grammar::parse(tokens, context)?,
        })
    }
}

pub struct If<'input, E, I>
where
    E: ExpressionGrammar<'input>,
    I: StatementGrammar<'input>,
{
    pub if_: tokens::If<'input>,
    pub expression: E,
    pub inner: I,
}

impl<'input, E, I> StatementGrammar<'input> for If<'input, E, I>
where
    E: ExpressionGrammar<'input>,
    I: StatementGrammar<'input>,
{
}

impl<'input, E, I> Grammar<'input> for If<'input, E, I>
where
    E: ExpressionGrammar<'input>,
    I: StatementGrammar<'input>,
{
    fn parse(
        tokens: &mut Peekable<Tokenizer<'input>>,
        context: &mut Context,
    ) -> Result<Self, Error> {
        Ok(Self {
            if_: Grammar::parse(tokens, context)?,
            expression: Grammar::parse(tokens, context)?,
            inner: Grammar::parse(tokens, context)?,
        })
    }
}
