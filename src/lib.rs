#![feature(proc_macro, box_patterns)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use quote::Tokens;
use proc_macro::TokenStream;
use syn::*;
use syn::token::Comma;
use syn::punctuated::Punctuated;

fn trans_pat(pat: syn::Pat) -> (Tokens, Tokens) {
  match pat {
    Pat::Ident(PatIdent { ident, .. }) => {
      (quote! { #ident }, quote! { #pat })
    }
    Pat::Ref(PatRef { box pat, mutability, .. }) => {
      let (lhs, rhs) = trans_pat(pat);
      (lhs, quote! { & #mutability #rhs })
    }
    Pat::TupleStruct(PatTupleStruct { path, pat }) => {
      let (lhs, rhs): (Punctuated<Tokens, Comma>, Punctuated<Tokens, Comma>) =
        pat.front.into_iter().map(|pat| trans_pat(pat)).unzip();
      (quote! { (#lhs) }, quote! { #path (#rhs) })
    }
    Pat::Struct(PatStruct { path, fields, .. }) => {
      (quote! { (#fields) }, quote! { #path { #fields, .. } })
    }
    _ => {
      panic!("Pattern must be an enum (e.g. A(...)) or struct (e.g. A{...})")
    }
  }
}

#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
  let input: Stmt =
    syn::parse(input).expect("Failed to parse into let binding");

  let binding = if let Stmt::Local(Local { box pat, init, .. }) = input {
    if let Some((_, init)) = init {
      let (lhs, rhs) = trans_pat(pat);
      quote!{
        let #lhs = match #init { #rhs => { #lhs }, _ => unreachable!() };
      }
    } else {
      panic!("Let binding must have a right-hand side")
    }
  } else {
    panic!("Must be a let-binding")
  };

  binding.into()
}
