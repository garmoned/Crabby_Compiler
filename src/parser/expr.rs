use crate::{Lexeme, Token};
use std::{fmt::format, fs::OpenOptions};

pub enum Operation {
    Equals,
    Times,
    Plus,
    GT,
    LT,
}

#[derive(Debug)]
pub enum ExprData {
    StrLit(String),
    IntLit(i16),
    Name(String),
}

pub enum Expr {
    Unary(ExprData),
    Binary(Box<Expr>, Box<Expr>, Operation),
}

impl Expr {
    pub(crate) fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        let save = x.clone();
        let mut expr_stack: Vec<Expr> = vec![];
        let mut op_stack: Vec<Operation> = vec![];
        let mut get_op = false;
        loop {
            match &lex[*x] {
                Token::StringLit(string) => {
                    if get_op {
                        break;
                    }
                    expr_stack.push(Expr::Unary(ExprData::StrLit(string.to_string())))
                }
                Token::IntLit(int) => {
                    if get_op {
                        break;
                    }
                    expr_stack.push(Expr::Unary(ExprData::IntLit(*int)))
                }

                Token::Name(name) => {
                    if get_op {
                        break;
                    }
                    expr_stack.push(Expr::Unary(ExprData::Name(name.to_string())))
                }
                Token::Times => {
                    if !get_op {
                        break;
                    }
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::Equals => {
                    if !get_op {
                        break;
                    }
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::Plus => {
                    if !get_op {
                        break;
                    }
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::GT => {
                    if !get_op {
                        break;
                    }
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::LT => {
                    if !get_op {
                        break;
                    }
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                _ => {
                    break;
                }
            }
            get_op = !get_op;
            *x += 1
        }

        while op_stack.len() > 0 {
            combine_expression(&mut expr_stack, &mut op_stack)
        }

        return Some(Box::new(expr_stack.pop().unwrap()));
    }
}

impl From<Token> for Operation {
    fn from(tok: Token) -> Self {
        match tok {
            Token::Times => Operation::Times,
            Token::Plus => Operation::Plus,
            Token::Equals => Operation::Equals,
            Token::GT => Operation::GT,
            Token::LT => Operation::LT,
            _ => panic!("invalid operator"),
        }
    }
}

fn get_priority(op: &Operation) -> u8 {
    match op {
        Operation::Times => 3,
        Operation::Plus => 2,
        Operation::Equals => 1,
        Operation::GT => 1,
        Operation::LT => 1,
        _ => panic!("invalid operator"),
    }
}

fn perform_operation(expr_stack: &mut Vec<Expr>, op_stack: &mut Vec<Operation>, tok: Operation) {
    let oper: Operation = tok.into();
    while op_stack.len() >= 1 && get_priority(&oper) <= get_priority(&*op_stack.last().unwrap()) {
        combine_expression(expr_stack, op_stack)
    }
    op_stack.push(oper)
}

fn combine_expression(expr_stack: &mut Vec<Expr>, op_stack: &mut Vec<Operation>) {
    let op = op_stack.pop().unwrap();
    let operand2 = expr_stack.pop().unwrap();
    let operand1 = expr_stack.pop().unwrap();
    let expr = Expr::Binary(Box::new(operand1), Box::new(operand2), op);
    expr_stack.push(expr)
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Unary(data) => format!("{:?}", data),
            Expr::Binary(oper1, oper2, op) => {
                format!(
                    "({} {} {})",
                    oper1.to_string(),
                    op.to_string(),
                    oper2.to_string()
                )
            }
        }
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Equals => "==".to_owned(),
            Operation::Times => "*".to_owned(),
            Operation::Plus => "+".to_owned(),
            Operation::GT => ">".to_owned(),
            Operation::LT => "<".to_owned(),
        }
    }
}
