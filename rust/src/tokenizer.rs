use core::fmt;

// File to take in text and turn it into tokens
pub enum TokenType {
    AddOp,
    SubtractOp,
    MultiplyOp,
    DivideOp,
    Number,
    IllformedNumber,
    Invalid,
}

pub struct Token {
    token_type: TokenType,
}

impl Token {
    
    pub fn new(token_type: TokenType) -> Self {
        return Self { token_type };
    }
    
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self.token_type {
            TokenType::AddOp => String::from("AddOp"),
            TokenType::SubtractOp => String::from("SubtractOp"),
            TokenType::MultiplyOp => String::from("MultiplyOp"),
            TokenType::DivideOp => String::from("DivideOp"),
            TokenType::Number => String::from("Number"),
            TokenType::IllformedNumber => String::from("Illformed Number"),
            TokenType::Invalid => String::from("Invalid"),
        };
        write!(f, "{string}") 
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
            '/' => tokens.push( Token::new(TokenType::DivideOp) ),
            ' ' => (), // do nothing
            '.' | '0'..='9' => tokens.push( Token::new(TokenType::Number) ),
            _ => tokens.push( Token::new(TokenType::Invalid) )
        }
        index += 1;
    }

    return tokens;
}

// fn consume_number(chars: &Vec<char>, index: &mut usize) -> Token {
//     match {}
// }