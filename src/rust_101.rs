#![warn(clippy::all)]
#[allow(unused_imports)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use failure::Error;

fn main() -> Result<(), Error> {
    info!("hello there {}", foo());
    Ok(())
}

fn foo() -> String {
    "Foo!".to_owned()
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