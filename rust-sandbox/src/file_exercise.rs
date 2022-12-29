use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    let mut file = File::open(path)?;

    file.read_to_string(&mut string)?;

    Ok(string)
}

pub fn main() {
    println!(
        "{}",
        read_file_contents(PathBuf::from("src/example.txt")).unwrap()
    );

    println!(
        "{}",
        read_file_contents(PathBuf::from("non-existent-file.txt"))
            .unwrap_or("Lmao this doesn't exist".to_owned())
    );

    if read_file_contents(PathBuf::from("src/example.txt")).is_ok() {
        println!("The program found the example file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}
