// File to take in text and turn it into tokens
pub enum TokenType {
    AddOp,
    SubtractOp,
    MultiplyOp,
    Invalid,
    Number,
}

pub struct Token {
    token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        return Self { token_type };
    }
}

pub fn tokenize(input_stream: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index: usize = 0;
    let chars: Vec<char> = input_stream.chars().collect();
    while index < chars.len() {
        let ch: char = chars[index];
        match ch {
            '+' => tokens.push( Token::new(TokenType::AddOp) ),
            '-' => tokens.push( Token::new(TokenType::SubtractOp) ),
            '*' => tokens.push( Token::new(TokenType::MultiplyOp) ),
            ' ' => (), // do nothing
            '.' | '0'..='9' => tokens.push( Token::new(TokenType::Number) ),
            _ => tokens.push( Token::new(TokenType::Invalid) )
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
        TokenType::Number => string = String::from("Number"),
        TokenType::Invalid => string = String::from("Invalid"),
    }
    println!("{string}");
}