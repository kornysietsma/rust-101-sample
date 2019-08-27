use rust_101;
use pretty_assertions::assert_eq;

#[test]
fn foo_is_foo() {
    assert_eq!("Foo!", rust_101::foo());
}