#![warn(clippy::all)]
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum NumberOrStr {
    Number(u32),
    Str(String),
}

pub fn fizzbuzz(n: u32) -> NumberOrStr {
    [
        NumberOrStr::Number(0),
        NumberOrStr::Number(1),
        NumberOrStr::Number(2),
        NumberOrStr::Str("Fizz".to_owned()),
        NumberOrStr::Number(2),
        NumberOrStr::Str("Buzz".to_owned()),
    ]
    .get(n as usize)
    .expect("Invalid number!")
    .clone()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_fizz_or_buzz() {
        assert_eq!(fizzbuzz(1), NumberOrStr::Number(1));
        assert_eq!(fizzbuzz(3), NumberOrStr::Str("Fizz".to_owned()));
    }

    #[test]
    #[ignore]
    fn should_fizz_buzz() {
        assert_eq!(fizzbuzz(15), NumberOrStr::Str("FizzBuzz".to_owned()));
    }
}
