#[macro_export]
macro_rules! assert_token_matches {
    (
        $tokens:expr,
        [ $($token_enum_pat:pat),* $(,)* ]
        $(,)*
    ) => {{
        let mut tokens = $crate::lex::tokenize($tokens);
        $(
            assert!(
                matches!(tokens.next(), Some(Ok($token_enum_pat)))
            );
        )*
    }};
}

macro_rules! handle_non_alphanum {
    ($s:ident ,) => { return None; };
    ($s:ident , $c0:expr => { $t0:ident }, $($tt:tt)*) => {
        if $s.chars.peek() == Some(&$c0) {
            $s.chars.next().unwrap();
            return Some(Ok($crate::lex::token::Token::$t0($crate::lex::token::$t0 {
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
                Some(Ok($crate::lex::token::Token::$t1($crate::lex::token::$t1 {
                    inner: std::borrow::Cow::Borrowed($s.input),
                    span: Default::default(),
                })))
            } else {
                Some(Ok($crate::lex::token::Token::$t0($crate::lex::token::$t0 {
                    inner: std::borrow::Cow::Borrowed($s.input),
                    span: $crate::Span::default(),
                })))
            };
        }
        handle_non_alphanum! { $s , $($tt)* }
    };
}
