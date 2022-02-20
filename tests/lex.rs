use gb_lang::{
    assert_token_matches,
    lex::{tokens::Token, Error},
};

#[test]
fn tokenize_empty() {
    assert_token_matches!("", [Token::EOF(_)]);
    assert_token_matches!(" ", [Token::EOF(_)]);
    assert_token_matches!("\n", [Token::EOF(_)]);
    assert_token_matches!("\r\n", [Token::EOF(_)]);
    assert_token_matches!("\t", [Token::EOF(_)]);
}

#[test]
fn return_none_after_eof() {
    let mut tokens = gb_lang::tokenize("");
    assert!(matches!(
        [tokens.next(), tokens.next()],
        [Some(Ok(Token::EOF(_))), None]
    ));
}

#[test]
fn tokenize_two_chars() {
    assert_token_matches!(
        "&=::==/=>=<=-=~=|=+=*=^=",
        [
            Token::AndEquals(_),
            Token::ColonColon(_),
            Token::EqualsEquals(_),
            Token::ForwardSlashEquals(_),
            Token::GreaterEqualsThan(_),
            Token::LessEqualsThan(_),
            Token::MinusEquals(_),
            Token::NotEquals(_),
            Token::OrEquals(_),
            Token::PlusEquals(_),
            Token::StarEquals(_),
            Token::XorEquals(_),
            Token::EOF(_),
        ],
    );
}

#[test]
fn tokenize_one_char() {
    assert_token_matches!(
        "&@:,{}=/><-~|()+;[]*^",
        [
            Token::And(_),
            Token::At(_),
            Token::Colon(_),
            Token::Comma(_),
            Token::CurlyLeft(_),
            Token::CurlyRight(_),
            Token::Equals(_),
            Token::ForwardSlash(_),
            Token::GreaterThan(_),
            Token::LessThan(_),
            Token::Minus(_),
            Token::Not(_),
            Token::Or(_),
            Token::ParLeft(_),
            Token::ParRight(_),
            Token::Plus(_),
            Token::SemiColon(_),
            Token::SquareLeft(_),
            Token::SquareRight(_),
            Token::Star(_),
            Token::Xor(_),
            Token::EOF(_),
        ],
    );
}

#[test]
fn tokenize_keywords() {
    assert_token_matches!(
        "addr array asm break const continue deref else if let loop ptr static struct union u8 while",
        [
            Token::Addr(_),
            Token::Array(_),
            Token::Asm(_),
            Token::Break(_),
            Token::Const(_),
            Token::Continue(_),
            Token::Deref(_),
            Token::Else(_),
            Token::If(_),
            Token::Let(_),
            Token::Loop(_),
            Token::Ptr(_),
            Token::Static(_),
            Token::Struct(_),
            Token::Union(_),
            Token::U8(_),
            Token::While(_),
            Token::EOF(_),
        ],
    );
}

#[test]
fn tokenize_string() {
    assert_token_matches!("\"Hello, world\"", [Token::Str(_)]);
}

#[test]
fn tokenize_string_error() {
    let mut tokens = gb_lang::lex::tokenize("\"hello");
    assert!(matches!(
        tokens.next(),
        Some(Err(Error::OpenEndedStringToken))
    ));
}

#[test]
fn tokenize_num() {
    assert_token_matches!(
        "0 123456789",
        [Token::Number(_), Token::Number(_), Token::EOF(_),],
    );
}

#[test]
#[ignore]
fn tokenize_num_multiple_raxix() {
    assert_token_matches!(
        "0 123456789 0x123456789abcdefABCDEF 01234567 0b1010",
        [
            Token::Number(_),
            Token::Number(_),
            Token::Number(_),
            Token::Number(_),
            Token::Number(_),
            Token::EOF(_),
        ],
    );
}

#[test]
#[ignore]
fn tokenize_invalid_number_error() {
    todo!()
}

#[test]
fn tokenize_identifier() {
    assert_token_matches!(
        "hello world",
        [Token::Identifier(_), Token::Identifier(_), Token::EOF(_),],
    );
}
