use crate::{Lexeme, Token};
use std::{fmt::format, fs::OpenOptions};

pub enum Operation {
    Equals,
    Times,
    Plus,
}

#[derive(Debug)]
pub enum ExprData {
    StrLit(String),
    IntLit(i16),
    Name(String),
}

pub enum Factor {
    Data(ExprData),
    Expr(Box<TopExpr>),
}

pub enum Term {
    TermFactor(Option<Box<TermPrime>>, Box<Factor>),
}

pub enum TermPrime {
    More(Box<Factor>, Option<Box<TermPrime>>),
}

pub enum AddExpr {
    Term(Term),
    TermExpression(Term, Box<AddExpr>),
}

pub enum TopExpr {
    Term(AddExpr),
    TermExpression(AddExpr, Box<TopExpr>),
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
        loop {
            match &lex[*x] {
                Token::StringLit(string) => {
                    expr_stack.push(Expr::Unary(ExprData::StrLit(string.to_string())))
                }
                Token::IntLit(int) => expr_stack.push(Expr::Unary(ExprData::IntLit(*int))),
                Token::Name(name) => expr_stack.push(Expr::Unary(ExprData::Name(name.to_string()))),
                Token::Times => {
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::Equals => {
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                Token::Plus => {
                    perform_operation(&mut expr_stack, &mut op_stack, lex[*x].clone().into())
                }
                _ => {
                    break;
                }
            }
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
            _ => panic!("invalid operator"),
        }
    }
}

fn get_priority(op: &Operation) -> u8 {
    match op {
        Operation::Times => 3,
        Operation::Plus => 2,
        Operation::Equals => 1,
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
    let operand1 = expr_stack.pop().unwrap();
    let operand2 = expr_stack.pop().unwrap();
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
        }
    }
}
