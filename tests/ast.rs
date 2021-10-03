use gb_lang::ast::{
    expressions::{Number, Str},
    statements::Let,
    types::{Array, U8},
};

#[test]
fn let_statement() {
    gb_lang::parse::<Let<U8, Number>>("let foo::u8 = 42;").unwrap();
    gb_lang::parse::<Let<Array<U8>, Str>>("let foo::[u8 6] = \"german\";").unwrap();
}
