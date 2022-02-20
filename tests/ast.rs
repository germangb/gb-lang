use gb_lang::ast::{
    expressions::{Expression, Number, Str},
    statements::{Break, Const, Continue, If, Let, Loop, Scope, Statement, Static, While},
    types::{Array, Type, U8},
};

#[test]
fn statement_let() {
    gb_lang::parse::<Let<U8, Number>>("let foo::u8 = 42;").unwrap();
    gb_lang::parse::<Let<Array<U8>, Str>>("let bar::array<u8, 6> = \"german\";").unwrap();
    gb_lang::parse::<Statement>("let baz::ptr<array<ptr<u8>, 4>> = 42;").unwrap();
}

#[test]
fn statement_const() {
    gb_lang::parse::<Const<U8>>("const FOO::u8;").unwrap();
    gb_lang::parse::<Const<Type>>("const FOO::array<u8, 123>;").unwrap();
    gb_lang::parse::<Const<Type>>("const FOO::array<u8, 123>;").unwrap();
}

#[test]
fn statement_static() {
    gb_lang::parse::<Static<U8, Number>>("static FOO::u8 = 42;").unwrap();
    gb_lang::parse::<Static<Type, Expression>>("static FOO::array<u8, 1024> = \"...\";").unwrap();
    gb_lang::parse::<Static<U8, Number>>("static FOO::u8 = 42;").unwrap();
}

#[test]
fn statement_if() {
    gb_lang::parse::<If<Number, Statement>>("if 0\nlet foo::u8 = 4;").unwrap();
    gb_lang::parse::<If<Number, Scope<Statement>>>("if 0{let foo::u8=4;}").unwrap();
    gb_lang::parse::<If<Number, Scope<Statement>>>("if 1 {continue;}").unwrap();
}

#[test]
fn statement_loop() {
    gb_lang::parse::<Loop<Statement>>("loop let foo::u8 = 4;").unwrap();
    gb_lang::parse::<Loop<Scope<Statement>>>("loop {let foo::u8 = 4;}").unwrap();
}

#[test]
fn statement_while() {
    gb_lang::parse::<While<Number, Statement>>("while 1 let foo::u8 = 4;").unwrap();
    gb_lang::parse::<While<Number, Scope<Statement>>>("while 0 {let foo::u8 = 4;}").unwrap();
}

#[test]
fn statement_loop_break() {
    gb_lang::parse::<Loop<Break>>("loop break;").unwrap();
    gb_lang::parse::<While<Number, Break>>("while 1 break;").unwrap();
}

#[test]
fn statement_loop_continue() {
    gb_lang::parse::<Loop<Continue>>("loop continue;").unwrap();
    gb_lang::parse::<While<Number, Continue>>("while 1 continue;").unwrap();
}

#[test]
fn statement_scope() {
    gb_lang::parse::<Scope<()>>("{}").unwrap();
    gb_lang::parse::<Scope<Vec<Statement>>>("{}").unwrap();
}
