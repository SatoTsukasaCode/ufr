use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].parse::<String>().unwrap();
    let arg2 = args[2].parse::<String>().unwrap();
    let file_inside = fs::read_to_string(file_path).expect("No File Detected In Specified Path");

    if arg2 == "-r" {
        println!("File Exists")
    } else {
        println!("{file_inside}");
    }
}
