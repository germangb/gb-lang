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
