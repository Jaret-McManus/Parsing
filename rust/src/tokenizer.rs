use core::fmt;

// File to take in text and turn it into tokens
pub enum Token {
    // regular tokens
    AddOp,
    SubtractOp,
    MultiplyOp,
    DivideOp,
    Number{value: usize},

    // meta tokens
    IllformedNumber,
    Invalid,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self {
            Token::AddOp => String::from("AddOp"),
            Token::SubtractOp => String::from("SubtractOp"),
            Token::MultiplyOp => String::from("MultiplyOp"),
            Token::DivideOp => String::from("DivideOp"),
            Token::Number{value} => String::from(format!("Number{{{value}}}")),
            Token::IllformedNumber => String::from("Illformed Number"),
            Token::Invalid => String::from("Invalid"),
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
            '+' => tokens.push( Token::AddOp ),
            '-' => tokens.push( Token::SubtractOp ),
            '*' => tokens.push( Token::MultiplyOp ),
            '/' => tokens.push( Token::DivideOp ),
            ' ' => (), // do nothing
            '.' | '0'..='9' => tokens.push( consume_number(&chars, &mut index) ),
            _ => tokens.push( Token::Invalid )
        }
        index += 1;
    }

    return tokens;
}

fn consume_number(chars: &Vec<char>, index: &mut usize) -> Token {
    let mut value: usize = 0;
    let mut is_reading: bool = true;
    
    while is_reading && *index < chars.len(){
        let ch: char = chars[*index];
        match ch {
            '0'..='9' => value = value * 10 + get_digit(&ch),
            ' ' => is_reading = false,
            '.' | _ => return Token::IllformedNumber
        }
        *index += 1;
    }

    // decrement index in prep 
    *index -= 1;

    return Token::Number{ value };
}

fn get_digit(ch: &char) -> usize {
    assert!(*ch >= '0' && *ch <= '9');
    return ch.to_digit(10).unwrap() as usize;
}