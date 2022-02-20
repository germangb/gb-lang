extern crate proc_macro;

#[proc_macro_derive(Grammar)]
pub fn grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::statements::StatementGrammar);
    derive(Some(&subtrait_path), input)
}

#[proc_macro_derive(StatementGrammar)]
pub fn statement_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::statements::StatementGrammar);
    derive(Some(&subtrait_path), input)
}

#[proc_macro_derive(TypeGrammar)]
pub fn type_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::types::TypeGrammar);
    derive(Some(&subtrait_path), input)
}

#[proc_macro_derive(ExpressionGrammar)]
pub fn expression_grammar_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let subtrait_path = quote::quote!(crate::ast::expressions::ExpressionGrammar);
    derive(Some(&subtrait_path), input)
}

fn derive(
    subtrait_path: Option<&proc_macro2::TokenStream>,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match &derive_input.data {
        syn::Data::Struct(struct_) => derive_struct(subtrait_path, &derive_input, &struct_),
        _ => unimplemented!(),
    }
    .into()
}

fn derive_struct(
    subtrait_path: Option<&proc_macro2::TokenStream>,
    derive_input: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let grammar = match &data.fields {
        syn::Fields::Named(named) => derive_struct_named(derive_input, named),
        syn::Fields::Unnamed(unnamed) => derive_struct_unnamed(derive_input, unnamed),
        syn::Fields::Unit => derive_struct_unit(derive_input),
    };
    let subgrammar = if let Some(subtrait_path) = subtrait_path {
        let ident = &derive_input.ident;
        let (impl_, ty, where_) = derive_input.generics.split_for_impl();
        quote::quote! { impl #impl_ #subtrait_path <'input> for #ident #ty #where_ {} }
    } else {
        proc_macro2::TokenStream::new()
    };
    quote::quote! {#grammar #subgrammar}
}

fn derive_struct_named(
    derive_input: &syn::DeriveInput,
    fields_named: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let (impl_, ty, where_) = derive_input.generics.split_for_impl();
    let ident = &derive_input.ident;
    let fields = fields_named
        .named
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .map(|ident| quote::quote!(#ident : Grammar::parse(tokens, context)?,));
    quote::quote! {
        impl #impl_ crate::ast::Grammar <'input> for #ident #ty #where_ {
            fn parse(tokens: &mut std::iter::Peekable<crate::lex::Tokenizer<'input>>,
                     context: &mut crate::ast::Context) -> Result<Self, crate::ast::Error<'input>> {
                Ok(Self { #(#fields)* })
            }
        }
    }
}

fn derive_struct_unnamed(
    derive_input: &syn::DeriveInput,
    fields_unnamed: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let (impl_, ty, where_) = derive_input.generics.split_for_impl();
    let ident = &derive_input.ident;
    let fields = fields_unnamed
        .unnamed
        .iter()
        .map(|_| quote::quote!(Grammar::parse(tokens, context)?,));
    quote::quote! {
        impl #impl_ crate::ast::Grammar <'input> for #ident #ty #where_ {
            fn parse(tokens: &mut std::iter::Peekable<crate::lex::Tokenizer<'input>>,
                     context: &mut crate::ast::Context) -> Result<Self, crate::ast::Error<'input>> {
                Ok(Self(#(#fields)*))
            }
        }
    }
}

#[rustfmt::skip]
fn derive_struct_unit(
                      derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let (impl_, ty, where_) = derive_input.generics.split_for_impl();
    let ident = &derive_input.ident;
    quote::quote! {
        impl #impl_ crate::ast::Grammar <'input> for #ident #ty #where_ {
            fn parse(tokens: &mut std::iter::Peekable<crate::lex::Tokenizer<'input>>,
                     context: &mut crate::ast::Context) -> Result<Self, crate::ast::Error<'input>> {
                Ok(Self)
            }
        }
    }
}
