use std::collections::HashMap;

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let hash_map: HashMap<&i32, usize> = numbers.iter().enumerate().fold(
        HashMap::new(),
        |mut acc, (i, number)| -> HashMap<&i32, usize> {
            acc.insert(number, i);
            acc
        },
    );

    let mut result: (usize, usize) = (0, 0);

    numbers.iter().enumerate().find(|(i, &num)| {
        let possible_result = hash_map.contains_key(&(target - num));
        if possible_result {
            result.0 = *hash_map.get(&(target - num)).unwrap();
            result.1 = *i;
        }

        possible_result
    });

    result
}
