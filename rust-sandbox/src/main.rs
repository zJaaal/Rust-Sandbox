mod car_exercise;
mod codewars {
    pub mod add_length;
    pub mod find;
    pub mod freq_seq;
    pub mod high_low;
    pub mod two_sum;
}

use codewars::add_length::add_length;
use codewars::find::main as find;
use codewars::freq_seq::freq_seq;
use codewars::high_low::high_and_low;
use codewars::two_sum::two_sum;

mod errors;
mod file_exercise;
mod groups;
mod iterable;
mod lifetime;
mod loop_exercise;
mod modules;
mod person;
mod test;
mod traits;
// mod two_sum;

// mod find;
// mod freq_seq;

fn divide_by_5(value: i32) -> i32 {
    if value == 0 {
        return value; // Early return
    }

    value / 5 //Implicit return
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

fn car_factory(color: String, transmission: Transmission, converitble: bool) -> Car {
    let car = Car {
        color,
        transmission,
        convertible: converitble,
        mileage: 0,
    };
    return car;
}

fn main() {
    println!("Hello, world!");
    //This way you can print variables in a template
    println!(
        "Hi I'm {} and I'm learning {} on {}/{}/{}",
        "Jal", "Rust", 26, 12, 2022
    );

    //This is inmutable variables
    let name = "Jalinson";
    //Thi is mutable variable
    let mut age = 21;

    println!("My name is {} and I'm {} years old", name, age);

    //That means this is wrong:
    // name = "Jose"

    //But we can mutate this onw
    age += 1;

    println!("Now I'm {} years old", age);

    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);

    //Boolean
    let is_bigger: bool = 1 > 4;

    println!("is 1 bigger that 4? > {}", is_bigger);

    //There are multiple integers type signed and unsigned,
    //the difference are their max-min values and how they behave on overflow

    let my_char: char = 'W';
    let my_str: &str = "This is a string";

    println!("I have a char: {}\nI have a string: {}", my_char, my_str);

    //Tuple type
    let my_languages = ("JavaScript", "TypeScript", "C#", "now learning Rust");

    println!(
        "My languages are: {}, {}, {} and {}",
        my_languages.0, my_languages.1, my_languages.2, my_languages.3
    );

    //structs

    struct Student {
        name: String,
        age: i32,
        remote: bool,
    }

    struct Grades(i32, i32, i32);

    let student = Student {
        name: "Jalinson".to_owned(), // &str and String are not the same
        age: 21,
        remote: false,
    };

    let my_grades = Grades(19, 20, 17);

    println!(
        "Hi my name is: {}\nMy grade last year was: {}\nMy age is: {}\n Am I a remote student?: {}",
        student.name, my_grades.0, student.age, student.remote
    );

    //Enums
    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEKeys(KeyPress),
        WEClick(MouseClick),
    }
    // enum WebEvent {
    //     WELoad,
    //     WEKeys(String, char),
    //     WEMouse { x: i64, y: i64 },
    // }

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );

    println!("10 divided by 5 is: {}", divide_by_5(10));

    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    //Here I'll start to execute other scripts
    println!("---------------------Array-------------------------");
    iterable::main();
    println!("---------------------Cars--------------------------");
    car_exercise::main();
    println!("---------------------Loop--------------------------");
    loop_exercise::main();
    println!("---------------------Errors------------------------");
    errors::main();
    println!("---------------------Person------------------------");
    person::main();
    println!("---------------------Files-------------------------");
    file_exercise::main();
    println!("---------------------Lifetime----------------------");
    lifetime::main();
    println!("---------------------Traits------------------------");
    traits::main();
    println!("---------------------Groups------------------------");
    groups::main();
    println!("---------------------Modules-----------------------");
    modules::main();
    println!("---------------------CodeWars----------------------");
    println!("{}", freq_seq("Hello world", "-"));
    println!("{:?}", two_sum(&[1234, 5678, 9012], 14690));
    println!("{:?}", add_length("you will win"));
    println!("{:?}", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    find();
    println!("---------------------End---------------------------");
}
