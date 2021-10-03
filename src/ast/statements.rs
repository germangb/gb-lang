use crate::{
    ast::{expressions::ExpressionGrammar, types::TypeGrammar, Context, Error, Grammar},
    lex::{tokens, Tokenizer},
};
use std::iter::Peekable;

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
