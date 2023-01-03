use std::collections::HashMap;

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
