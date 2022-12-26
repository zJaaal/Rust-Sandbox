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
}
