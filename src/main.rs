use std::env;
use std::fs;

fn main() {
    let file_path: String = env::args().nth(1).expect("You need one argument!");
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{contents}");
}
