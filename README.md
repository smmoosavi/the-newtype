# The `Newtype` trait

[![Build Status](https://github.com/smmoosavi/the-newtype/workflows/CI/badge.svg)](https://github.com/smmoosavi/the-newtype/actions)
[![Latest Version](https://img.shields.io/crates/v/the-newtype.svg)](https://crates.io/crates/the-newtype)
[![Rust Documentation](https://docs.rs/the-newtype/badge.svg)](https://docs.rs/the-newtype)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)

## The Problem

Sometimes you want to wrap a type in a newtype, but you want the newtype implements the same traits as the wrapped type. The `Newtype` trait helps you to implement the traits of the wrapped type for the newtype automatically.

## The Solution

When you define a trait, you can support the `Newtype` trait and any type that implements the `Newtype` trait will automatically implement your trait.

```rust
use the_newtype::Newtype;
use derive_more::AsRef;

pub trait MyTrait {
    fn my_method(&self) -> String;
}

impl<T> MyTrait for T
where
    T: Newtype + AsRef<T::Inner>,
    T::Inner: MyTrait,
{
    fn my_method(&self) -> String {
        self.as_ref().my_method()
    }
}

// Now we can use the `MyTrait` trait for the newtype.

struct Foo;

impl MyTrait for Foo {
    fn my_method(&self) -> String {
        "foo".to_string()
    }
}

#[derive(AsRef)]
struct Bar(Foo);

impl Newtype for Bar {
    type Inner = Foo;
}

fn main() {
    let bar = Bar(Foo);
    assert_eq!(bar.my_method(), "foo");
}
```

## When to use

You can use the `Newtype` trait when you want to wrap a type in a newtype and you want the newtype implements **ALL** the newtype-supported traits of the wrapped type. If you need some traits, you should implement them manually and avoid using the `Newtype` trait.

## Drawbacks

The `Newtype` trait is not a good solution for the following cases:

- If you want to implement a trait for any type that implements other trait (e.g. every type that implements the `Fancy` trait will implement the `Awesome` trait). only one general implementation for each trait is possible. You can't use the `Newtype` trait in this case.

```rust
use the_newtype::Newtype;

trait Fancy {
    fn fancy_method(&self) -> String;
}

// it's ok to implement the `Fancy` trait for the `Newtype` trait

impl<T> Fancy for T
where
    T: Newtype + AsRef<T::Inner>,
    T::Inner: Fancy,
{
    fn fancy_method(&self) -> String {
        self.as_ref().fancy_method()
    }
}

trait Awesome {
    fn awesome_method(&self) -> String;
}

// every type that implements the `Fancy` trait will implement the `Awesome` trait
// it's not possible to implement the `Awesome` trait for the `Newtype` trait

impl<T> Awesome for T
where
    T: Fancy,
{
    fn awesome_method(&self) -> String {
        let fancy = self.fancy_method();
        format!("{} is awesome!", fancy)
    }
}

```

## Tips

### Use `derive_more` and `Newtype` macros

You can use the [`derive_more`][derive_more] crate to implement the `AsRef`, `AsMut`, and `Into` traits for the newtype. And you can use the `Newtype` macro to implement the `Newtype` trait for the newtype.

```rust
use the_newtype::Newtype;
use derive_more::AsRef;

#[derive(AsRef, Newtype)]
struct Bar(String);
```

### How to implement a trait for `&self`

If you want to implement a trait for `&self`, you can use the `AsRef` trait.

```rust
use the_newtype::Newtype;

trait MyTrait {
    fn my_method(&self) -> String;
}

impl<T> MyTrait for T
where
    T: Newtype + AsRef<T::Inner>,
    T::Inner: MyTrait,
{
    fn my_method(&self) -> String {
        self.as_ref().my_method()
    }
}
```

### How to implement a trait for `&mut self`

If you want to implement a trait for `&mut self`, you can use the `AsMut` trait.

```rust
use the_newtype::Newtype;

trait MyTrait {
    fn my_method(&mut self) -> String;
}

impl<T> MyTrait for T
where
    T: Newtype + AsMut<T::Inner>,
    T::Inner: MyTrait,
{
    fn my_method(&mut self) -> String {
        self.as_mut().my_method()
    }
}
```

### How to implement a trait for `self`

If you want to implement a trait for `self`, you can use the `Into` trait.

```rust
use the_newtype::Newtype;

trait MyTrait {
    fn my_method(self) -> String;
}

impl<T> MyTrait for T
where
    T: Newtype + Into<T::Inner>,
    T::Inner: MyTrait,
{
    fn my_method(self) -> String {
        self.into().my_method()
    }
}
```

### How to implement a trait without `self`

If you want to implement a trait without `self`, no extra trait is needed.

```rust
use the_newtype::Newtype;


trait MyTrait {
    fn my_method() -> String;
}

impl<T> MyTrait for T
where
    T: Newtype,
    T::Inner: MyTrait,
{
    fn my_method() -> String {
        T::Inner::my_method()
    }
}
```

### How to combine `self`, `&self` or `&mut self`

If you want to implement a trait for `self`, `&self` or `&mut self`, you can use the `Into`, `AsRef` or `AsMut` traits together.

```rust
use the_newtype::Newtype;

trait MyTrait {
    fn my_method(&self) -> String;
    fn my_method_mut(&mut self) -> String;
}

impl<T> MyTrait for T
where
    T: Newtype + AsRef<T::Inner> + AsMut<T::Inner>,
    T::Inner: MyTrait,
{
    fn my_method(&self) -> String {
        self.as_ref().my_method()
    }

    fn my_method_mut(&mut self) -> String {
        self.as_mut().my_method_mut()
    }
}
```

## Installation

```toml
[dependencies]
the-newtype = "0.1"
```

[derive_more]: https://crates.io/crates/derive_more
