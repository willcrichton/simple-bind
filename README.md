# simple-bind: one-line non-exhaustive binds in Rust
[![Build Status](https://travis-ci.org/willcrichton/simple-bind.svg?branch=master)](https://travis-ci.org/willcrichton/simple-bind)
[![](https://img.shields.io/crates/v/simple-bind.svg)](https://crates.io/crates/simple-bind)
[![](https://docs.rs/simple-bind/badge.svg)](https://docs.rs/crate/simple-bind/)

**Nightly-only crate.**

```rust
// Here's a brief example that demonstrates how simple-bind works.

#![feature(proc_macro, pattern_parentheses)]
extern crate simple_bind;
use simple_bind::bind;

fn main() {
  enum A { Foo(i32), Bar };

  // Let's say you have a variant of an enum.
  let x = A::Foo(10);

  // Previously, if you knew `x` was `Foo` and just wanted to access the inside,
  // you had to do either:
  let y = match x { A::Foo(y) => y, _ => unreachable!() };
  // or...
  let y = if let A::Foo(y) = x { y } else { unreachable!() };

  // With simple-bind, you can instead do:
  bind!{let A::Foo(y) = x;}

  // No more nested match/if statements!
  assert_eq!(y, 10);
}
```

## Setup

Use of this crate uses the unstable `proc_macro` API, so it requires nightly and a few feature gates.

Enable nightly on your repository:
```
rustup override set nightly
```

Add this line to your `cargo.toml`:
```toml
[dependencies]
simple-bind = "0.1.0"
```

To your main module file (`lib.rs` or `main.rs`), add:
```rust
#![feature(proc_macro, pattern_parentheses)]
extern crate simple_bind;
```

Then wherever you want to use the macro, use `use` (not `#[macro_use]`):
```rust
use simple_bind::bind;
```

## Examples

```rust
fn main() {
  enum B { Quux(i32) };
  enum A { Foo(i32), Bar{y: i32}, Baz(B) };

  // Simple binds
  bind!{let A::Foo(x) = A::Foo(10);}

  // Struct binds
  let s = A::Bar{y: 1};
  bind!{let A::Bar{y} = s;}

  // Nested binds
  bind!{let A::Baz(B::Quux(x)) = A::Baz(B::Quux(10));}

  // Reference binds
  let mut s = A::Foo(10);
  bind!{let &mut A::Foo(ref mut y) = &mut s;}
  *y = 3;
}
```
