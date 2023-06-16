use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Token {
    Identifier(String),
    Number(Number),
    Keyword(Keyword),
    Operator(Operator),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(str) => str.clone(),
            Token::Number(num) => num.to_string(),
            Token::Keyword(keyword) => keyword.to_string(),
            Token::Operator(operator) => operator.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Add => "+".to_string(),
            Operator::Subtract => "-".to_string(),
            Operator::Multiply => "*".to_string(),
            Operator::Divide => "/".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Keyword {
    Equals,
    Assign,
    Semicolon,
}
impl Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Assign => ":".to_string(),
            Keyword::Equals => "=".to_string(),
            Keyword::Semicolon => ";".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Number {
    Float(f64),
    Int(i32),
}

impl Number {
    pub fn to_float(&self) -> Number {
        match self {
            Number::Float(_) => *self,
            Number::Int(num) => Number::Float(*num as f64)
        }
    }

    pub fn to_int(&self) -> Number {
        match self {
            Number::Float(num) => Number::Int(*num as i32),
            Number::Int(_) => *self,
        }
    }

    pub fn ret_float(self) -> f64 {
        match self {
            Number::Float(num) => num,
            Number::Int(num) => num as f64,
        }
    }

    pub fn ret_int(self) -> i32 {
        match self {
            Number::Float(num) => num as i32,
            Number::Int(num) => num,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Number::Float(num) => num.to_string(),
            Number::Int(num) => num.to_string(),
        }
    }
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
