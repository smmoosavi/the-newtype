## Usage

```rust
use the_newtype::Newtype;

#[derive(Newtype)]
struct Foo(u32);
```

It will generate:

```rust
use the_newtype::Newtype;
struct Foo(u32);

impl Newtype for Foo {
    type Inner = u32;
}
```

It also supports more complex types:

```rust
use the_newtype::Newtype;

#[derive(Newtype)]
struct Foo(Box<u32>);

#[derive(Newtype)]
struct Bar(Vec<Option<u32>>);
```

And supports generics and lifetimes:

```rust
use the_newtype::Newtype;

#[derive(Newtype)]
struct Foo<T>(T);

#[derive(Newtype)]
struct Bar<'a>(&'a str);

#[derive(Newtype)]
struct Baz<'a, T>(&'a T);

#[derive(Newtype)]
struct Qux<T>(Vec<T>);
```