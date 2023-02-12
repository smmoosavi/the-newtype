use the_newtype::Newtype;
struct Foo;

#[derive(Newtype)]
struct FooTuple(Foo);

#[derive(Newtype)]
#[allow(dead_code)]
struct FooStruct {
    foo: Foo,
}

#[derive(Newtype)]
struct Wrapper<T>(T);

#[derive(Newtype)]
struct WrapperList<T>(Vec<T>);

#[derive(Newtype)]
struct WrapperRef<'a, T>(&'a T);

#[test]
fn test() {
    assert_eq!(
        std::any::TypeId::of::<<FooTuple as Newtype>::Inner>(),
        std::any::TypeId::of::<Foo>()
    );

    assert_eq!(
        std::any::TypeId::of::<<FooStruct as Newtype>::Inner>(),
        std::any::TypeId::of::<Foo>()
    );

    assert_eq!(
        std::any::TypeId::of::<<Wrapper<Foo> as Newtype>::Inner>(),
        std::any::TypeId::of::<Foo>()
    );

    assert_eq!(
        std::any::TypeId::of::<<WrapperList<Foo> as Newtype>::Inner>(),
        std::any::TypeId::of::<Vec<Foo>>()
    );

    assert_eq!(
        std::any::TypeId::of::<<WrapperRef<'_, Foo> as Newtype>::Inner>(),
        std::any::TypeId::of::<&Foo>()
    );
}
