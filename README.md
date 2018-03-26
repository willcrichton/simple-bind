# simple-bind: one-line non-exhaustive binds in Rust

```rust
#![feature(proc_macro, pattern_parentheses)]
extern crate simple_bind;
use simple_bind::bind;

fn main() {
  enum A { Foo(i32), Bar };

  // Let's say you have a variant of an enum.
  let x = A::Foo(10);

  // Previously, if you _knew_ `x` was `Foo` and just wanted to access the inside,
  // you had to do either:
  let y = match x { A::Foo(y) => y, _ => unreachable!() };
  // or...
  let y = if let A::Foo(y) = x { y } else { unreachable!() };

  // With simple-bind, you can instead do:
  bind!{let A::Foo(y) = x;}

  // No more nested match/if statements!
}
```
