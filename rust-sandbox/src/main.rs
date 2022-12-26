fn main() {
    print!("{}[2J", 27 as char);
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
        "Hi my name is: {}\nMy grade last year was: {}",
        student.name, my_grades.0
    )
}
