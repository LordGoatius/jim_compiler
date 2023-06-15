use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Token {
    Identifier(String),
    Number(Number),
    Keyword(Keyword),
    Operator(Operator),
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Keyword {
    Equals,
    Assign,
    Semicolon,
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Number {
    Float(f64),
    Int(i32),
}

pub fn lex(contents: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    
    for slice in contents.split_whitespace() {
        tokens.push(get_token(slice));
    }

    return tokens;
}

fn get_token(slice: &str) -> Token {
    if let Result::Ok(value) = slice.parse::<i32>() {
        return Token::Number(Number::Int(value));
    }

    if let Result::Ok(value) = slice.parse::<f64>() {
        return Token::Number(Number::Float(value));
    }

    if slice.len() == 1 {
        match slice {
            "+" => return Token::Operator(Operator::Add),
            "-" => return Token::Operator(Operator::Subtract),
            "*" => return Token::Operator(Operator::Multiply),
            "/" => return Token::Operator(Operator::Divide),
            "=" => return Token::Keyword(Keyword::Equals),
            ":" => return Token::Keyword(Keyword::Assign),
            ";" => return  Token::Keyword(Keyword::Semicolon),
            _ => println!("Non Operator"),
        }
    }

    if slice.chars().all(char::is_alphanumeric) {
        return Token::Identifier(slice.to_string());
    }

    return Token::Identifier("12".to_string());
}
