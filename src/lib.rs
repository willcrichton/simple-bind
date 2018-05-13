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
    Pat::Wild(_) => {
      (quote! { () }, quote! { _ })
    }
    Pat::Ident(PatIdent { ident, .. }) => {
      // HACK: syn stopped parsing _ as Wild, instead as Ident?
      if format!("{}", ident) == "_" {
        (quote! { () }, quote! { _ })
      } else {
        (quote! { #ident }, quote! { #pat })
      }
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
      let (lhs, rhs): (Punctuated<Tokens, Comma>, Punctuated<Tokens, Comma>) =
        fields.into_iter().map(|pat| {
          let (lhs, rhs) = trans_pat(*pat.pat.clone());
          let member = pat.member;
          (lhs, quote!{ #member : #rhs })
        }).unzip();
      (quote! { (#lhs) }, quote! { #path { #rhs, .. } })
    }
    _ => {
      panic!("Pattern must be an enum (e.g. A(...)) or struct (e.g. A{...})")
    }
  }
}


/// Non-exhaustively binds a single variant of an enum.
///
/// # Examples
///
/// ```ignore
/// use simple_bind::bind;
/// enum A { Foo(i32), Bar };
///
/// let x = A::Foo(10);
/// bind!{let A::Foo(y) = x;}
///
/// assert_eq!(y, 10);
/// ```
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
  let input: Stmt =
    syn::parse(input).expect("Failed to parse into let binding");

  let binding = if let Stmt::Local(Local { pats, init, .. }) = input {
    if let Some((_, init)) = init {
      let (lhs, rhs) = trans_pat(pats[0].clone());
      quote!{
        let #lhs = #[allow(non_shorthand_field_patterns)] match #init { #rhs => { #lhs }, _ => unreachable!() };
      }
    } else {
      panic!("Let binding must have a right-hand side")
    }
  } else {
    panic!("Must be a let-binding")
  };

  binding.into()
}
