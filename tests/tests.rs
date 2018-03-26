#![feature(proc_macro, pattern_parentheses)]

extern crate simple_bind;

use simple_bind::bind;

enum B { Field(i32) }
enum A { Single(i32), Nested(B), Multi(i32, i32), Struct{x: i32} }

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
