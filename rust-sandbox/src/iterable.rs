use std::vec;

//Arrays and vectors
pub fn main() {
    //Array are fixed length structures
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    //Value = 0, Length = 5
    let zeroes = [0; 5];

    println!("Index 3 of array: {}", array[3]);
    println!("5 zeroes {:?}", zeroes);

    //Vector is an array but the length is not fixed

    let mut vector = vec![1, 2, 3, 4, 5, 6, 7];
    let mut zeroes_vector = vec![0; 8];

    //That means we can push and pop values
    println!("This is the original vector: {:?}", vector);
    let popped = vector.pop();
    println!(
        "We popped: {}\nNow the vector is: {:?}",
        popped.unwrap(),
        vector
    );
    println!("8 Zeroes: {:?}", zeroes_vector);
    zeroes_vector.push(3);
    println!("8 Zeroes and a 3: {:?}", zeroes_vector);
    println!("Index 4 of vector: {}", vector[4]);
}
