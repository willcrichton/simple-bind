#![feature(proc_macro, proc_macro_non_items, stmt_expr_attributes, pattern_parentheses)]

extern crate simple_bind;

use simple_bind::bind;

enum B { Field(i32) }
enum A { Single(i32), Nested(B), Multi(i32, i32), Struct{x: i32}, Ref(String) }

#[test]
fn basic() {
  bind!{let A::Single(y) = A::Single(3);}
  assert_eq!(y, 3);
}

#[test]
fn capture() {
  let x = A::Single(3);
  bind!{let A::Single(y) = x;}
  assert_eq!(y, 3);
}

#[test]
fn multiple() {
  bind!{let A::Multi(x, y) = A::Multi(1, 2);}
  assert_eq!(x, 1);
  assert_eq!(y, 2);
}

#[test]
fn nested() {
  bind!{let A::Nested(B::Field(x)) = A::Nested(B::Field(1));}
  assert_eq!(x, 1);
}

#[test]
fn struct_() {
  let y = A::Struct{x: 1};
  bind!{let A::Struct{x} = y;}
  assert_eq!(x, 1);
}

#[test]
fn ref_() {
  let x = A::Ref(String::from("Hello"));
  bind!{let &A::Ref(ref y) = &x;}
  assert_eq!(y, "Hello");
}

#[test]
fn mut_ref() {
  let mut x = A::Ref(String::from("Hello"));
  bind!{let &mut A::Ref(ref mut y) = &mut x;}
  *y = String::from("Hi");
}

#[test]
fn mut_ref2() {
  let mut x = A::Single(1);
  bind!{let &mut A::Single(ref mut y) = &mut x;}
  *y = 2;
}

#[test]
fn wildcard() {
  bind!{let A::Multi(_, x) = A::Multi(1, 2);}
  assert_eq!(x, 2);
}

#[test]
fn struct_ref() {
  let x = A::Struct{x: 1};
  bind!{let A::Struct{x: ref y} = x;}
  assert_eq!(*y, 1);
}
