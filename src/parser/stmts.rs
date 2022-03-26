use std::borrow::Borrow;

use crate::{Lexeme, Token};
use crate::parser::decls::Decls;
use crate::parser::expr::Expr;

pub(crate) struct Stmts {
    stmt: Box<dyn Stmt>,
    stmts: Option<Box<Stmts>>,
}


struct PrintStmt {
    expr: Box<Expr>,
}

struct IfStmt {
    bool: Box<Expr>,
    decls: Option<Box<Decls>>,
    stmts: Option<Box<Stmts>>
}

impl Stmt for IfStmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<dyn Stmt>> where Self: Sized {
        match &lex[*x] {
            Token::If => {}
            _ => return Option::None
        }
        *x += 1;
        match &lex[*x] {
            Token::OpenParen => {}
            _ => return Option::None
        }
        let e = Expr::new(lex, x);
        match &lex[*x] {
            Token::CloseParen => {}
            _ => return Option::None
        }
        *x += 1;
        match &lex[*x] {
            Token::OpenBrace => {}
            _ => return Option::None
        }
        *x += 1;
        let decls = Decls::new(lex, x);
        let stmts = Stmts::new(lex, x);
        match &lex[*x] {
            Token::CloseBrace => {}
            _ => return Option::None
        }
        *x += 1;
        Some(Box::new(IfStmt {
            bool: e.unwrap(),
            decls,
            stmts,
        }))
    }

    fn to_string(&self) -> String {
        let decl_string = &self.decls.as_ref().unwrap().to_string();
        let stmt_string = &self.stmts.as_ref().unwrap().to_string();
        format!("if({}){{{}{}}}",
                self.bool.to_string(), decl_string, stmt_string)
    }
}

impl Stmt for PrintStmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<dyn Stmt>> {
        match &lex[*x] {
            Token::Print => {}
            _ => return Option::None
        };
        *x += 1;
        match &lex[*x] {
            Token::OpenParen => {}
            _ => return Option::None
        };
        *x += 1;
        let e = Expr::new(lex, x);
        match &lex[*x] {
            Token::CloseParen => {}
            _ => return Option::None
        };
        *x += 1;
        Some(Box::new(PrintStmt {
            expr: e.unwrap()
        }))
    }

    fn to_string(&self) -> String {
        format!("print({})", self.expr.to_string())
    }
}

trait Stmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<dyn Stmt>> where Self: Sized;
    fn to_string(&self) -> String;
}

fn parse_stmt(lex: &Lexeme, mut x: &mut usize) -> Option<Box<dyn Stmt>> {
    let save = x.clone();
    match PrintStmt::parse(lex, x) {
        None => {
            *x = save.clone();
            match IfStmt::parse(lex, x) {
                None => {
                    *x = save.clone();
                    None
                }
                Some(if_stmt) => {
                    Some(if_stmt)
                }
            }
        }
        Some(print) => { Some(print) }
    }
}


impl Stmts {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        let save = x.clone();
        println!("new statements");
        let st = parse_stmt(lex, x);
        match st {
            None => {
                *x = save;
                None
            }
            Some(stmt) => {
                Some(Box::new(Self {
                    stmt,
                    stmts: Stmts::new(lex, x),
                }))
            }
        }
    }
}


impl ToString for Stmts {
    fn to_string(&self) -> String {
        match &self.stmts {
            Some(ds) => {
                self.stmt.to_string() + &"\n".to_string() + &ds.to_string() + &"\n".to_string()
            }
            None => {
                self.stmt.to_string() + &"\n".to_string()
            }
        }
    }
}

enum Operation {
    Equals,
}
