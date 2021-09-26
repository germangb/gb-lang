use crate::Span;
pub use error::Error;
use std::{borrow::Cow, iter::Peekable, str::Chars};
pub use token::*;

mod error;
mod token;
#[macro_use]
mod utils;

#[derive(Debug)]
pub struct Tokenizer<'input> {
    ended: bool,
    input: &'input str,
    chars: Peekable<Chars<'input>>,
    begin: [usize; 2],
    fi: [usize; 2],
}

impl<'input> Tokenizer<'input> {
    fn skip_whitespace(&mut self) -> Result<(), Error> {
        loop {
            match self.chars.peek() {
                Some(c) if c.is_whitespace() => {
                    let _ = self.chars.next().unwrap();
                }
                _ => return Ok(()),
            }
        }
    }

    fn next_token_eof(&mut self) -> Option<Result<Token<'input>, Error>> {
        if self.chars.peek().is_none() {
            self.ended = true;
            Some(Ok(Token::EOF(EOF {
                inner: Cow::Borrowed(self.input),
                span: Default::default(),
            })))
        } else {
            None
        }
    }

    fn next_token_non_alphanum(&mut self) -> Option<Result<Token<'input>, Error>> {
        handle_non_alphanum! {
            self,
            '&' => { And, '=' => AndEquals },
            '@' => { At },
            ':' => { Colon, ':' => ColonColon },
            '{' => { CurlyLeft },
            '}' => { CurlyRight },
            '=' => { Equals, '=' => EqualsEquals },
            '/' => { ForwardSlash, '=' => ForwardSlashEquals },
            '>' => { GreaterThan, '=' => GreaterEqualsThan },
            '<' => { LessThan, '=' => LessEqualsThan },
            '-' => { Minus, '=' => MinusEquals },
            '~' => { Not, '=' => NotEquals },
            '|' => { Or, '=' => OrEquals },
            '(' => { ParLeft },
            ')' => { ParRight },
            '+' => { Plus, '=' => PlusEquals },
            ';' => { SemiColon },
            '[' => { SquareLeft },
            ']' => { SquareRight },
            '*' => { Star, '=' => StarEquals },
            '^' => { Xor, '=' => XorEquals },
        }
    }

    // TODO(german): remove alloc
    #[rustfmt::skip]
    fn next_token_alpha(&mut self) -> Result<Token<'input>, Error> {
        let mut aux = String::new();
        loop {
            match self.chars.peek().copied() {
                None => break,
                Some(c) if c.is_alphanumeric() || c == '_' => aux.push(self.chars.next().unwrap()),
                _ => break,
            }
        }
        match aux.as_str() {
            "addr" => Ok(Token::Addr(Addr { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "break" => Ok(Token::Break(Break { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "const" => Ok(Token::Const(Const { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "continue" => Ok(Token::Continue(Continue { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "deref" => Ok(Token::Deref(Deref { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "else" => Ok(Token::Else(Else { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "if" => Ok(Token::If(If { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "let" => Ok(Token::Let(Let { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "loop" => Ok(Token::Loop(Loop { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "ptr" => Ok(Token::Ptr(Ptr { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "static" => Ok(Token::Static(Static { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "struct" => Ok(Token::Struct(Struct { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "union" => Ok(Token::Union(Union { inner: Cow::Borrowed(self.input), span: Span::default()})),
            "while" => Ok(Token::While(While { inner: Cow::Borrowed(self.input), span: Span::default()})),
            _ => todo!(),
        }
    }

    fn next_token_alphanum(&mut self) -> Result<Token<'input>, Error> {
        let next = self.chars.peek().expect("Expected character");
        if next.is_numeric() {
            todo!()
        } else {
            self.next_token_alpha()
        }
    }

    fn next_token(&mut self) -> Result<Token<'input>, Error> {
        assert!(!self.ended);
        self.skip_whitespace()?;
        self.next_token_eof()
            .or_else(|| self.next_token_non_alphanum())
            .unwrap_or_else(|| self.next_token_alphanum())
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else {
            Some(self.next_token())
        }
    }
}

/// Tokenizes the input string and returns an iterator of tokens.
pub fn tokenize(input: &str) -> Tokenizer<'_> {
    Tokenizer {
        // an EOF token is always returned at the very end, so even if the input string is empty,
        // the iterator hasn't ended yet.
        ended: false,
        input,
        chars: input.chars().peekable(),
        begin: [0; 2],
        fi: [0; 2],
    }
}
