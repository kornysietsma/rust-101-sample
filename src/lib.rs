#![warn(clippy::all)]

extern crate failure;

use failure::Error;

pub fn foo() -> String {
    "Foo!".to_owned()
}

pub fn bar() -> Result<String, Error> {
    Ok("Bar!".to_owned())
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn true_is_true() {
        assert_eq!(1,1)
    }

    #[test]
    fn foo_is_foo() {
        assert_eq!(foo(), "Foo!")
    }
}