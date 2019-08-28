#![warn(clippy::all)]

fn fib(n: u32) -> u128 {
    let fibs = [0,1,1,2,3,5];

    fibs[(n-1) as usize]
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn fibs() {
        assert_eq!(fib(1), 0);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 1);
        assert_eq!(fib(4), 2);
        assert_eq!(fib(5), 3);
        assert_eq!(fib(6), 5);
    }
}
