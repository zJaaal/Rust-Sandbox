use std::collections::HashMap;

pub fn freq_seq(s: &str, sep: &str) -> String {
    //This was my first solution

    // let mut char_map: HashMap<char, u32> = HashMap::new();

    // for char in s.chars() {
    //     char_map
    //         .entry(char)
    //         .and_modify(|value| *value += 1)
    //         .or_insert(1);
    // }

    // s.chars()
    //     .map(|char| char_map[&char].to_string())
    //     .collect::<Vec<String>>()
    //     .join(sep);

    //or

    s.chars()
        .map(|c| s.matches(c).count().to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

pub fn find_outlier(values: &[i32]) -> i32 {
    let mut hash_map: HashMap<String, Vec<i32>> = HashMap::new();

    hash_map.insert("even".to_owned(), Vec::new());
    hash_map.insert("odd".to_owned(), Vec::new());

    for &value in values {
        if value % 2 == 0 {
            hash_map.get_mut("even").unwrap().push(value);
        } else {
            hash_map.get_mut("odd").unwrap().push(value);
        }

        if hash_map.get("even").unwrap().len() > 1 && hash_map.get("odd").unwrap().len() == 1 {
            return hash_map.get_mut("odd").unwrap().pop().unwrap();
        } else if hash_map.get("odd").unwrap().len() > 1 && hash_map.get("even").unwrap().len() == 1
        {
            return hash_map.get_mut("even").unwrap().pop().unwrap();
        }
    }

    return 0;

    //Check 3 to see if the weird is odd or even

    //     let sum: i32 = values.iter()
    //     .take(3)
    //     .map(|n| n.abs() % 2)
    //     .sum();

    // let m = if sum == 1 || sum == 0 { 1 } else { 0 };

    //Then we look for the weird number
    // values
    //     .iter()
    //     .find(|n| n.abs() % 2 == m)
    //     .map(|n| *n)
    //     .unwrap_or(0)
}

fn basic_test() {
    let t1 = [2, 6, 8, -10, 3];
    let t2 = [
        206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
    ];
    let t3 = [std::i32::MAX, 0, 1];

    let t4 = [1, 0, 0];

    println!("{:?}", find_outlier(&t1));
    println!("{:?}", find_outlier(&t2));
    println!("{:?}", find_outlier(&t3));
    println!("{:?}", find_outlier(&t4));
}
pub fn main() {
    basic_test();
}

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

pub fn add_length(s: &str) -> Vec<String> {
    s.split(" ").map(|x| format!("{} {}", x, x.len())).collect()
}

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
