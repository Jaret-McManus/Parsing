use std::{collections::VecDeque};

use crate::tokenizer::{self};
use tokenizer::Token as Token;

struct ParseTree {
    token: Token,
    children: Vec<ParseTree>
}

impl ParseTree {
    pub fn new(root_token: Token) -> ParseTree {
        ParseTree { token: root_token, children: Vec::new() }
    }
}

enum ParseError {
    EmptyTokenStream,
    NotImplemented
}

fn create_tree(mut token_stream: VecDeque<Token>) -> Result<ParseTree, ParseError> {
    let token = match token_stream.pop_front() {
        None => return Err(ParseError::EmptyTokenStream),
        Some(token) => token
    };

    let operand_stack: Vec<Token> = Vec::new();
    let operator_stack: Vec<Token> = Vec::new();

    return Err(ParseError::NotImplemented);

}

// fn consume_number_token(token_stream: VecDeque<Token>) ->


fn is_operator(token: &Token) -> bool {
    use Token::*;
    match token {
        AddOp | SubtractOp | MultiplyOp | DivideOp => true,
        Number{value: _} | IllformedNumber | Invalid => false 
    }
}

fn get_operator_precedence(token: &Token) -> i32 {
    assert!(is_operator(token));
    match token {
        Token::AddOp | Token::SubtractOp => 1,
        Token::MultiplyOp | Token::DivideOp => 2,
        other => panic!("Cannot get operator precedence of {other}")
    }
}