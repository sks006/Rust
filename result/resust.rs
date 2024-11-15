use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; 
    // Using ? to propagate the error if the file can't be opened

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; 
    // Using ? to propagate the error if reading fails

    Ok(contents) 
    // Return the file contents wrapped in Ok
}

fn main() {
    match read_file("file.txt") {
        Ok(contents) => println!("File Contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
