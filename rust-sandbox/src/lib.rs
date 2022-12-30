/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = rust_sandbox::div(4,2);
/// assert_eq!(result, 2);
/// ```
///
/// # Example #2: 6 / 3 = 2
///
/// ```
/// let result = rust_sandbox::div(12, 6);
/// assert_eq!(result, 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// rust_sandbox::div(3,0);
/// ```
///
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// let result = rust_sandbox::sub(9,2);
/// assert_eq!(result, 7)
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// let result = rust_sandbox::sub(6,9);
/// assert_eq!(result, -3)
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
