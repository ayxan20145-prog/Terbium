use crate::{
    ast::{Expression, Program, Statement},
    token::{Token, Type, Value},
};

const STD_MATH: &str = include_str!("../std/math.tbc");
const STD_FS: &str = include_str!("../std/fs.tbc");
const STD_SH: &str = include_str!("../std/sh.tbc");
const STD_STR: &str = include_str!("../std/str.tbc");

pub fn compile(program: &Program) -> String {
    let mut bytecode = String::new();
    let mut label_id = 0;

    compile_statements(&program.statements, &mut bytecode, &mut label_id);

    bytecode
}
fn compile_statements(statements: &[Statement], bytecode: &mut String, label_id: &mut usize) {
    for statement in statements {
        compile_statement(statement, bytecode, label_id);
    }
}
fn compile_statement(statement: &Statement, bytecode: &mut String, label_id: &mut usize) {
    match statement {
        Statement::Decleration { name, value } => {
            bytecode.push_str(&compile_expression(value));
            bytecode.push_str(&format!("store {}\n", name));
        }

        Statement::Output { values } => {
            for value in values {
                bytecode.push_str(&compile_expression(value));
                bytecode.push_str("print\n");
            }
        }

        Statement::Input { name, typee } => {
            bytecode.push_str("read\n");

            match typee {
                Type::Int => bytecode.push_str("stoi\n"),
                Type::Float => bytecode.push_str("stof\n"),
                Type::String => {}
                Type::Bool => bytecode.push_str("stob\n"),
            }

            bytecode.push_str(&format!("store {}\n", name));
        }

        Statement::Import { name } => match name.as_str() {
            "math" => bytecode.push_str(STD_MATH),
            "fs" => bytecode.push_str(STD_FS),
            "sh" => bytecode.push_str(STD_SH),
            "str" => bytecode.push_str(STD_STR),
            _ => panic!("unknown import: {}", name),
        },

        Statement::If {
            condition,
            body,
            else_body,
        } => {
            let if_label = format!("__if_{}", *label_id);
            let else_label = format!("__else_{}", *label_id);
            let end_label = format!("__end_{}", *label_id);

            *label_id += 1;

            bytecode.push_str(&compile_expression(condition));

            bytecode.push_str(&format!("jumpif true {}\n", if_label));
            bytecode.push_str(&format!("jumpif false {}\n", else_label));

            bytecode.push_str(&format!("label {}\n", if_label));

            bytecode.push_str("pop\n");
            compile_statements(body, bytecode, label_id);

            bytecode.push_str(&format!("jump {}\n", end_label));

            bytecode.push_str(&format!("label {}\n", else_label));

            bytecode.push_str("pop\n");
            if let Some(else_body) = else_body {
                compile_statements(else_body, bytecode, label_id);

                bytecode.push_str(&format!("jump {}\n", end_label));
            }

            bytecode.push_str(&format!("label {}\n", end_label));
        }
        Statement::Push { value } => {
            bytecode.push_str(&compile_expression(value));
        }
        Statement::Pop { name } => {
            bytecode.push_str(&format!("store {}\n", name));
        }
        Statement::Label { name, body } => {
            let end_label = format!("__end_{}", *label_id);
            *label_id += 1;

            bytecode.push_str(&format!("jump {}\n", end_label));

            bytecode.push_str(&format!("label {}\n", name));

            compile_statements(body, bytecode, label_id);

            bytecode.push_str("ret\n");

            bytecode.push_str(&format!("label {}\n", end_label));
        }
        Statement::Call { name } => {
            bytecode.push_str(&format!("call {}\n", name));
        }
        Statement::While { condition, body } => {
            let while_label = format!("__while_{}", *label_id);
            let end_label = format!("__end_{}", *label_id);
            *label_id += 1;

            bytecode.push_str(&format!("label {}\n", while_label));

            bytecode.push_str(&compile_expression(condition));

            bytecode.push_str(&format!("jumpif false {}\n", end_label));

            bytecode.push_str("pop\n");
            compile_statements(body, bytecode, label_id);

            bytecode.push_str(&format!("jump {}\n", while_label));

            bytecode.push_str(&format!("label {}\n", end_label));
            bytecode.push_str("pop\n");
        }
    }
}
fn compile_value(value: &Value) -> String {
    match value {
        Value::String(value) => format!("pushstr {}\n", value),
        Value::Bool(value) => format!("pushbool {}\n", value),
        _ => format!("push {}\n", value),
    }
}
fn compile_expression(expression: &Expression) -> String {
    match expression {
        Expression::Value(value) => compile_value(value),
        Expression::Variable(name) => format!("load {}\n", name),
        Expression::Operation(left, op, right) => {
            let mut bytecode = String::new();

            bytecode.push_str(&compile_expression(left));
            bytecode.push_str(&compile_expression(right));

            match op {
                Token::Plus => bytecode.push_str("add\n"),
                Token::Minus => bytecode.push_str("sub\n"),
                Token::Star => bytecode.push_str("mul\n"),
                Token::Slash => bytecode.push_str("div\n"),

                Token::Less => bytecode.push_str("lt\n"),
                Token::LessEqual => bytecode.push_str("le\n"),
                Token::Greater => bytecode.push_str("gt\n"),
                Token::GreaterEqual => bytecode.push_str("ge\n"),
                Token::EqualEqual => bytecode.push_str("eq\n"),
                Token::NotEqual => bytecode.push_str("ne\n"),

                _ => panic!("invalid operator"),
            }

            bytecode
        }
    }
}
