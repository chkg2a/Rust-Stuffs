use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];
    let filename = &args[2];
    println!("Searching for : {}",query);
    println!("In file : {}",filename);

    let contents = fs::read_to_string(filename)
        .expect("Error reading file.");

    println!("Text: \n{}",contents)
}
