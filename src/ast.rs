// <program>    ::= <statement>*
// <statement>  ::= <assignment> ";" |
//                       <print> ";" |
// <value>      ::= <expression> |
//                      "Number" |
//                  "Identifier" |
// <assignment> ::= "Identifier" ":" <value>
// <expression> ::= "Operator" <value> <value> |
//                                     <value>
// <print>      ::= <expression> "="

use crate::lexer::{Token,Operator,Keyword,Number};

// <program>    ::= <statement>*
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// <statement>  ::= <assignment> |
//                       <print> |
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Print(Print),
}

// <assignment> ::= "Identifier" ":" <value>
#[derive(Debug, Clone)]
pub struct Assignment {
    pub identifier: Token, // Token identifier
    pub assign: Keyword, //Keyword Assign
    pub value: Value,
}
// <value>      ::= <expression> |
//                      "Number" |
//                  "Identifier" |
#[derive(Debug, Clone)]
pub enum Value {
    Expression(Box<Expression>),
    Number(Number), 
    Identifier(Token) // Token identifier
}

// <expression> ::= "Operator" <value> <value> 
#[derive(Debug, Clone)]
pub struct Expression {
    pub operator: Operator, // Operator
    pub value_1: Value,
    pub value_2: Value,
}
// <print>      ::= <expression> "="
#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Value,
    pub equals: Keyword, // Keyword equals
}