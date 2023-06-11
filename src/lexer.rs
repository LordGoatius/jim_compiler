#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Int(i32),
    Keyword(Keyword),
    Operator(Operator),
}
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
pub enum Keyword {
    Equals,
    Assign,
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
        return Token::Int(value);
    }

    if slice.len() == 1 {
        match slice {
            "+" => return Token::Operator(Operator::Add),
            "-" => return Token::Operator(Operator::Subtract),
            "*" => return Token::Operator(Operator::Multiply),
            "/" => return Token::Operator(Operator::Divide),
            "=" => return Token::Keyword(Keyword::Equals),
            ":" => return Token::Keyword(Keyword::Assign),
            _ => println!("Non Operator"),
        }
    } 

    if slice.chars().all(char::is_alphanumeric) {
        return Token::Identifier(slice.to_string());
    }

    return Token::Identifier("12".to_string());
}
