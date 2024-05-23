// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut sum = n;
    let mut counter = 1;

    while counter < n {
        sum *= counter;
        counter += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
