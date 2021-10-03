use gb_lang::ast::{
    expressions::{Expression, Number, Str},
    statements::{Const, If, Let, Scope, Statement, Static},
    types::{Array, Type, U8},
};

#[test]
fn let_statement() {
    gb_lang::parse::<Let<U8, Number>>("let foo::u8 = 42;").unwrap();
    gb_lang::parse::<Let<Array<U8>, Str>>("let bar::[u8 6] = \"german\";").unwrap();
    gb_lang::parse::<Statement>("let baz::ptr<[ptr<u8> 4]> = 42;").unwrap();
}

#[test]
fn const_statement() {
    gb_lang::parse::<Const<U8>>("const FOO::u8;").unwrap();
    gb_lang::parse::<Const<Type>>("const FOO::[u8 123];").unwrap();
}

#[test]
fn static_statement() {
    gb_lang::parse::<Static<U8, Number>>("static FOO::u8 = 42;").unwrap();
    gb_lang::parse::<Static<Type, Expression>>("static FOO::[u8 1024] = \"...\";").unwrap();
}

#[test]
fn if_statement() {
    gb_lang::parse::<If<Number, Statement>>("if 0\nlet foo::u8 = 4;").unwrap();
    gb_lang::parse::<If<Number, Scope<Statement>>>("if 0{let foo::u8=4;}").unwrap();
}
