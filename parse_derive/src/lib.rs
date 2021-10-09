extern crate proc_macro;

use syn::__private::TokenStream2;

#[proc_macro_derive(StatementGrammar)]
pub fn statement_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::statements::StatementGrammar);
    derive(&subtrait_path, input)
}

#[proc_macro_derive(TypeGrammar)]
pub fn type_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::types::TypeGrammar);
    derive(&subtrait_path, input)
}

#[proc_macro_derive(ExpressionGrammar)]
pub fn expression_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::expressions::ExpressionGrammar);
    derive(&subtrait_path, input)
}

#[rustfmt::skip]
fn derive(subtrait_path: &proc_macro2::TokenStream,
              input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match &derive_input.data {
        syn::Data::Struct(struct_) => derive_struct(&subtrait_path, &derive_input, &struct_),
        _ => unimplemented!(),
    }.into()
}

#[rustfmt::skip]
fn derive_struct(subtrait_path: &proc_macro2::TokenStream,
                 derive_input: &syn::DeriveInput,
                 data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(named) => derive_struct_named(subtrait_path, derive_input, named),
        syn::Fields::Unnamed(unnamed) => derive_struct_unnamed(subtrait_path, derive_input, unnamed),
        Unit => todo!(),
    }
}

#[rustfmt::skip]
fn derive_struct_named(subtrait_path: &proc_macro2::TokenStream,
                       derive_input: &syn::DeriveInput,
                       fields_named: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let (impl_, ty, where_) = derive_input.generics.split_for_impl();
    let ident = &derive_input.ident;
    let fields = fields_named.named.iter()
        .filter_map(|field| field.ident.as_ref())
        .map(|ident| quote::quote!(#ident : Grammar::parse(tokens, context)?,));
    quote::quote! {
        impl #impl_ #subtrait_path <'input> for #ident #ty #where_ {}
        impl #impl_ crate::ast::Grammar <'input> for #ident #ty #where_ {
            fn parse(tokens: &mut std::iter::Peekable<crate::lex::Tokenizer<'input>>,
                     context: &mut crate::ast::Context) -> Result<Self, crate::ast::Error> {
                Ok(Self { #(#fields)* })
            }
        }
    }
}

#[rustfmt::skip]
fn derive_struct_unnamed(subtrait_path: &proc_macro2::TokenStream,
                       derive_input: &syn::DeriveInput,
                       fields_unnamed: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let (impl_, ty, where_) = derive_input.generics.split_for_impl();
    let ident = &derive_input.ident;
    let fields = fields_unnamed.unnamed.iter()
        .map(|_| quote::quote!(Grammar::parse(tokens, context)?,));
    quote::quote! {
        impl #impl_ #subtrait_path <'input> for #ident #ty #where_ {}
        impl #impl_ crate::ast::Grammar <'input> for #ident #ty #where_ {
            fn parse(tokens: &mut std::iter::Peekable<crate::lex::Tokenizer<'input>>,
                     context: &mut crate::ast::Context) -> Result<Self, crate::ast::Error> {
                Ok(Self(#(#fields)*))
            }
        }
    }
}
