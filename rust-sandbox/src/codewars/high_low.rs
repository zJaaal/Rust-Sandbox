pub fn high_and_low(numbers: &str) -> String {
    use std::cmp;
    let new_numbers = numbers
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .fold((std::i32::MIN, std::i32::MAX), |acc, number| {
            (cmp::max(number, acc.0), cmp::min(number, acc.1))
        });

    format!("{} {}", new_numbers.0, new_numbers.1)
}
