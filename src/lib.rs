#![feature(proc_macro, box_patterns)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
  let input: Stmt = syn::parse(input).expect("Failed to parse into let binding");

  let binding = if let Stmt::Local(Local { box pat, init, .. }) = input {
    if let Some((_, init)) = init {
      match pat {
        Pat::TupleStruct(PatTupleStruct { path, pat }) => {
          quote! {
            let #pat = if let #path #pat = #init {
              #pat
            } else {
              unreachable!()
            };
          }
        },
        Pat::Struct(PatStruct {path, fields, .. }) => {
          quote! {
            let (#fields) = if let #path { #fields, .. } = #init {
              (#fields)
            } else {
              unreachable!()
            };
          }
        },
        _ => panic!("Pattern must be an enum (e.g. A(...)) or struct (e.g. A{...})")
      }
    } else {
      panic!("Let binding must have a right-hand side")
    }
  } else {
    panic!("Must be a let-binding")
  };

  binding.into()
}
