#[cfg(test)]
pub mod tests {

    fn is_even(num: i32) -> bool {
        num % 2 == 0
    }

    #[test]
    fn is_true_when_even() {
        assert_eq!(is_even(2), true);
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(7), false);
    }
}
