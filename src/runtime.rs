use crate::lexer::*;
use crate::ast::*;
use std::collections::HashMap;

pub fn interpret(program: Program) {
    let mut variable_map: HashMap<String, Number> = HashMap::new();

    for statement in program.statements {
        if let Option::Some(string) = execute(statement, &mut variable_map) {
            println!("{}",string);
        }
    }
}

pub fn execute(statement: Statement, variables: &mut HashMap<String, Number>) -> Option<String> {
    match statement {
        Statement::Assignment(assignment) => {
            interpret_assign(assignment, variables);
            Option::None
        },
        Statement::Print(print) => {
            Option::Some(interpret_print(print, variables))
        }
    }
}

fn interpret_print(print: Print, variables: &HashMap<String, Number>) -> String {
    evaluate_value(print.expression, variables).to_string()
}


fn interpret_assign(assign: Assignment, variables: &mut HashMap<String, Number>) {
    variables.insert(assign.identifier.to_string(), evaluate_value(assign.value, variables));
}

fn evaluate_expression(expression: Expression, variables: &HashMap<String, Number>) -> Number {
    let num_1: Number = evaluate_value(expression.value_1, variables);
    let num_2: Number = evaluate_value(expression.value_2, variables);
    let operator = expression.operator;
    calculate(operator, num_1, num_2)
}

fn evaluate_value(value: Value, variables: &HashMap<String, Number>) -> Number {
    match value {
        Value::Expression(expression) => {
            evaluate_expression(*expression, variables)
        },
        Value::Number(number) => {
            number
        },
        Value::Identifier(identifier) => {
            let name = identifier.to_string();
            variables[&name]
        },
    }
}

fn calculate(operator: Operator, num_1: Number, num_2: Number) -> Number {
    match operator {
        Operator::Add => {
            Number::Float(num_1.to_float().ret_float() + num_2.to_float().ret_float())
        },
        Operator::Subtract => {
            Number::Float(num_1.to_float().ret_float() - num_2.to_float().ret_float())
        },
        Operator::Multiply => {
            Number::Float(num_1.to_float().ret_float() * num_2.to_float().ret_float())
        },
        Operator::Divide => {
            Number::Float(num_1.to_float().ret_float() / num_2.to_float().ret_float())
        },
    }
}