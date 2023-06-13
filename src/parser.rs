use crate::lexer::*;
use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut program: Program = Program {statements: vec![]};

    for statement in tokens
    .split(|token| *token == Token::Keyword(Keyword::Semicolon))
    .filter(|statement| !statement.is_empty()) {

        let current_statement: Statement;

        match statement.first().unwrap() {
            Token::Operator(_) => {
                if *statement.last().unwrap() == Token::Keyword(Keyword::Equals) {
                    current_statement = Statement::Print(parse_print(statement));
                } else {
                    panic!("Incorrect grammar");
                }
            },
            Token::Identifier(_) => current_statement = parse_assignment(statement),
            _ => panic!("Incorrect grammar"),
        }
        
        program.statements.push(current_statement);
    }

    return program;
}

#[allow(dead_code)]
fn parse_value(tokens: &[Token]) -> Value {
    let first = tokens.first().unwrap();

    match first {
        Token::Number(number) => {
            return Value::Number(*number);
        },
        Token::Identifier(identifier) => {
            return Value::Identifier(Token::Identifier(identifier.clone()));
        },
        Token::Operator(operator) => {
            let expression = Expression {
                operator: *operator,
                value_1: parse_value(&tokens[1..]),
                value_2: parse_value(&tokens[1..]),
            };
            return Value::Expression(Box::new(expression));
        },
        _ => panic!("Incorrect token")
    }

    // c := first item of prefix
    // if c is a number then
    //     return a tree node containing the number
    // else
    //     create a tree node t with c (an operator in this case)
    //     t.left := buildTree(rest of prefix)
    //     t.right := buildTree(rest of prefix)
    //     return t
}

fn parse_value_2(tokens: &[Token]) -> Value {
    let mut stack: Vec<Value> = vec![];

    for token in tokens.iter().rev() {
        match token {
            Token::Operator(operator) => {
                let val = Value::Expression(Box::new(Expression {
                    operator: *operator,
                    value_1: stack.pop().unwrap(),
                    value_2: stack.pop().unwrap(),
                }));
                stack.push(val);
            },
            Token::Number(number) => {
                let num = Value::Number(*number);
                stack.push(num);
            },
            Token::Identifier(identifier) => {
                let num = Value::Identifier(Token::Identifier(identifier.clone()));
                stack.push(num);
            }
            _ => { panic!("Incorrect value expression"); },
        }
    }

    let root = stack.last().unwrap();
    return root.clone();

    // Stack<Node> stack = new Stack<Node>();
 
    //     for (int i = 0; i < postfix.length(); i++) {
    //         char c = postfix.charAt(i);
 
    //         if (isOperator(c)) {
    //             Node right = stack.pop();
    //             Node left = stack.pop();
    //             Node node = new Node(c);
    //             node.left = left;
    //             node.right = right;
    //             stack.push(node);
    //         }
    //         else {
    //             Node node = new Node(c);
    //             stack.push(node);
    //         }
    //     }
 
    //     Node root = stack.peek();
    //     stack.pop();
 
    //     return root;
}

fn parse_assignment(tokens: &[Token]) -> Statement {
    if tokens.len() < 3 {
        panic!("Incorrect assignment");
    }
    let identifier: Token;
    let assign: Keyword;
    let value: Value;

    if let Token::Identifier(_) = tokens[0] {
        identifier = tokens[0].clone();
    } else {
        panic!("Incorrect indentifier");
    }

    if let Token::Keyword(keyword) = tokens[1] {
        if keyword == Keyword::Assign {
            assign = Keyword::Assign;
        } else {
            panic!("Incorrect assignment");
        }
    } else {
        panic!("Incorrect assignment keyword");
    }

    let value = parse_value_2(&tokens[2..]);

    return Statement::Assignment(
        Assignment { 
            identifier: identifier, 
            assign: assign, 
            value: value,
        }
    );
    
}

fn parse_print(tokens: &[Token]) -> Print {
    if let Token::Keyword(Keyword::Equals) = tokens.last().unwrap() {
        return Print {
            expression: parse_value_2(&tokens[0..tokens.len()-1]),
            equals: Keyword::Equals,
        }
    } else {
        panic!("Incorrect print statement");
    }
}

