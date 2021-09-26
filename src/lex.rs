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

macro_rules! handle_non_alphanum {
    ($s:ident ,) => { return None; };
    ($s:ident , $c0:expr => { $t0:ident }, $($tt:tt)*) => {
        if $s.chars.peek() == Some(&$c0) {
            $s.chars.next().unwrap();
            return Some(Ok(token::Token::$t0($crate::lex::token::$t0 {
                inner: std::borrow::Cow::Borrowed($s.input),
                span: $crate::Span::default(),
            })));
        }
        handle_non_alphanum! { $s , $($tt)* }
    };
    ($s:ident , $c0:expr => { $t0:ident, $c1:expr => $t1:ident }, $($tt:tt)*) => {
        if $s.chars.peek() == Some(&$c0) {
            $s.chars.next().unwrap();
            return if $s.chars.peek() == Some(&$c1) {
                $s.chars.next().unwrap();
                Some(Ok(token::Token::$t1($crate::lex::token::$t1 {
                    inner: std::borrow::Cow::Borrowed($s.input),
                    span: Default::default(),
                })))
            } else {
                Some(Ok(token::Token::$t0($crate::lex::token::$t0 {
                    inner: std::borrow::Cow::Borrowed($s.input),
                    span: $crate::Span::default(),
                })))
            };
        }
        handle_non_alphanum! { $s , $($tt)* }
    };
}

macro_rules! handle_alpha {
    ($s:ident , $($e:expr => { $t0:ident } ,)*) => {
        // TODO(german): remove alloc
        let mut aux = String::new();
        loop {
            match $s.chars.peek().copied() {
                None => break,
                Some(c) if c.is_alphanumeric() || c == '_' => {
                    $s.chars.next().unwrap();
                    aux.push(c)
                },
                _ => break,
            }
        }
        match aux.as_str() {
            $($e => Ok(Token::$t0($t0 {
                inner: Cow::Borrowed($s.input),
                span: Span::default(),
            })),)*
            _ => todo!(),
        }
    };
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

    fn next_token_alpha(&mut self) -> Result<Token<'input>, Error> {
        handle_alpha! {
            self,
            "addr" => { Addr },
            "break" => { Break },
            "const" => { Const },
            "continue" => { Continue },
            "deref" => { Deref },
            "else" => { Else },
            "if" => { If },
            "let" => { Let },
            "loop" => { Loop },
            "ptr" => { Ptr },
            "static" => { Static },
            "struct" => { Struct },
            "union" => { Union },
            "while" => { While },
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

    fn next_token_string(&mut self) -> Option<Result<Token<'input>, Error>> {
        match self.chars.peek() {
            Some('"') => {
                self.chars.next().unwrap();
                loop {
                    match self.chars.next() {
                        Some('"') => {
                            return Some(Ok(Token::Str(token::Str {
                                inner: Cow::Borrowed(self.input),
                                span: Span::default(),
                            })))
                        }
                        Some(_) => {}
                        None => return Some(Err(Error::OpenEndedStringToken)),
                    }
                }
            }
            _ => return None,
        }
    }

    fn next_token(&mut self) -> Result<Token<'input>, Error> {
        assert!(!self.ended);
        self.skip_whitespace()?;
        self.next_token_eof()
            .or_else(|| self.next_token_non_alphanum())
            .or_else(|| self.next_token_string())
            .unwrap_or_else(|| self.next_token_alphanum())
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else {
            let next = self.next_token();
            // we cannot do `self.ended = next.is_err()` because next might be the EOF
            // token, which internally sets `ended = true`.
            if next.is_err() {
                self.ended = true;
            }
            Some(next)
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
