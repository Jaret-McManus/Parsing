use std::env;
use std::fs;

mod tokenizer;
use tokenizer::Token as Token;

fn main() {
    let args: Vec<String> = get_args();

    let filename = &args[1];

    let string: String = fs::read_to_string(filename)
        .expect(&format!("Unable to open file {filename}"));

    let tokens: Vec<Token> = tokenizer::tokenize(string);
    tokens.iter()
        .for_each(|t| println!("{t}"));
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}