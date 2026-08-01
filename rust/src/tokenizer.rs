// File to take in text and turn it into tokens
pub enum TokenType {
    AddOp,
    SubtractOp,
    MultiplyOp,
    Invalid,
}

pub struct Token {
    token_type: TokenType,
}

pub fn tokenize(input_stream: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index: usize = 0;
    let chars: Vec<char> = input_stream.chars().collect();
    while index < chars.len() {
        let ch: char = chars[index];
        match ch {
            '+' => tokens.push( Token { token_type: TokenType::AddOp } ),
            '-' => tokens.push( Token { token_type: TokenType::SubtractOp } ),
            '*' => tokens.push( Token { token_type: TokenType::MultiplyOp } ),
            ' ' => (), // do nothing
            _ => tokens.push( Token { token_type: TokenType::Invalid } )
        }
        index += 1;
    }

    return tokens;
}

pub fn print_token(token: Token) -> () {
    let string: String;
    match token.token_type {
        TokenType::AddOp => string = String::from("AddOp"),
        TokenType::MultiplyOp => string = String::from("MultiplyOp"),
        TokenType::SubtractOp => string = String::from("SubtractOp"),
        TokenType::Invalid => string = String::from("Invalid")
    }
    println!("{string}");
}