use crate::lexer::*;
use crate::ast::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn generate(ast: Program) {
    let mut variable_map: HashMap<String, String> = HashMap::new();
    let mut ids: Vec<String> = Vec::new();

    let mut intermediate_rep: Vec<String> = Vec::new();

    for statement in ast.statements {
        match statement {
            Statement::Assignment(_) => {
                intermediate_rep.append(&mut parse_assignment(statement, &mut variable_map, &mut ids));
            },
            Statement::Print(_) => {
                intermediate_rep.append(&mut parse_print(statement, &mut variable_map, &mut ids));
            },
        }
        intermediate_rep.push(" ".to_string());
    }

    let mut str: String = String::new();

    for line in intermediate_rep {
        str = format!("{}{}\n",str,line);
    }

    let mut file = File::create("main.ir").unwrap();
    file.write_all(str.as_bytes()).unwrap();

}

fn next_id(ids: &mut Vec<String>) -> String {
    match ids.last() {
        Some(last) => {
            let num = &mut last[2..].parse::<i32>().unwrap();
            *num = *num + 1;
            ids.push(format!("id{}",num));
            format!("id{}",num)
        },
        None => {
            ids.push("id1".to_string());
            "id1".to_string()
      },
    }
}

fn curr_id(ids: &mut Vec<String>) -> String {
    return ids.last().unwrap().to_string();
}

fn parse_assignment(statement: Statement, vars: &mut HashMap<String, String>, ids: &mut Vec<String>) -> Vec<String> {
    let mut code: Vec<String> = Vec::new(); 
    next_id(ids);
    if let Statement::Assignment(assignment) = statement {
        if let Token::Identifier(id) = assignment.identifier {
            vars.insert(id, ids.last().unwrap().to_string());
        } else { panic!("how"); }
        let clone = ids.clone();
        let id = clone.last().unwrap();
        code.append(&mut parse_value(assignment.value, vars, ids));
        code.append(&mut vec![format!("{} = {}",id,ids.last().unwrap())]);
        code
    } else { panic!("how"); }
}

fn parse_print(statement: Statement, vars: &mut HashMap<String, String>, ids: &mut Vec<String>) -> Vec<String> {
    let mut code: Vec<String> = Vec::new(); 
    if let Statement::Print(print) = statement {
        let id_value = next_id(ids);
        code.append(&mut parse_value(print.expression, vars, ids));
        code.append(& mut vec![format!("print = {}",id_value)]);
        code
    } else { panic!("how"); }
}
/*
parse print: next id = value calculated, current id = next id
    if expression
    value calculated = current id
        value_1 = stored in next
        value_2 = stored in next
        ret current = val1 op val2

 */
fn parse_value(statement: Value, vars: &mut HashMap<String, String>, ids: &mut Vec<String>) -> Vec<String> {
    match statement {
        Value::Expression(expr) => {
            let clone = ids.clone();
            let id = clone.last().unwrap();

            let id_val_1 = next_id(ids);
            let mut val_1 = parse_value(expr.value_1, vars, ids);
            // let clone_1 = ids.clone();
            // let id_val_1 = clone_1.last().unwrap();

            let id_val_2 = next_id(ids);
            let mut val_2 = parse_value(expr.value_2, vars, ids);
            // let id_val_2 = ids.last().unwrap();
            
            let mut to_return: Vec<String> = Vec::new();
            to_return.append(&mut val_1);
            to_return.append(&mut val_2);

            to_return.append(& mut vec![format!("{} = {} {} {}",id,id_val_1,expr.operator.to_string(),id_val_2)]);
            to_return
        },
        Value::Identifier(ident) => {
            if let Token::Identifier(ident) = ident {
                let next = curr_id(ids);
                return vec![format!("{} = {}",next,vars[&ident])]
            } else { panic!("What the hell happened here"); }
        },
        Value::Number(num) => {
            let next = curr_id(ids);
            return vec![format!("{} = {}",next,num.to_string())]
        },
    }
}