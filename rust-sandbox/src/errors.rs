#[derive(Debug)]
struct DivisionByZeroError;

pub fn main() {
    // panic!("The Disco");

    //Option<T>

    let langs = vec!["TypeScript", "JavaScript", "C#", "Rust"];

    let first = langs.get(0);
    println!("See that first is a Some(T): {:?}", first);

    let third = langs.get(2);
    println!("See that third is a Some(T): {:?}", third);

    let none = langs.get(99);
    println!("See that none is a None: {:?}", none);

    //Match expression

    for &index in [0, 3, 99].iter() {
        match langs.get(index) {
            Some(&"Rust") => {
                println!("I'm loving Rust!")
            }
            Some(lang_name) => {
                println!("The index {} lang name is: {}", index, lang_name)
            }
            None => {
                println!("The {} lang doesn't exist", index)
            }
        }
    }

    //If we want to match just one expression then we use the if let expression
    let favorite_number: Option<u32> = Some(3);

    if let Some(3) = favorite_number {
        println!("3 is my favorite_number");
    }

    //Result Type
    let safe_division = |dividend: f64, divisor: f64| -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 {
            Err(DivisionByZeroError)
        } else {
            Ok(dividend / divisor)
        }
    };

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
