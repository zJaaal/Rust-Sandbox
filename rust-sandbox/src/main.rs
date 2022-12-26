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
}
