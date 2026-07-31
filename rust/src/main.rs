use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let _string: String = fs::read_to_string(filename)
        .expect(&format!("Unable to open file {filename}"));
}
