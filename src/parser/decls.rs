use crate::parser::expr::TopExpr;
use crate::parser::var::Var;
use crate::{Lexeme, Token};

use super::expr::Expr;

pub(crate) struct Decl {
    pub(crate) ty: Var,
    pub(crate) name: String,
    pub(crate) expr: Box<Expr>,
}

pub(crate) struct Decls {
    decl: Box<Decl>,
    decls: Option<Box<Decls>>,
}

impl Decl {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Self {
        let ty = match lex[*x] {
            Token::Int => Var::Int,
            Token::Str => Var::Str,
            Token::Bool => Var::Bool,
            _ => {
                panic!("syntax error")
            }
        };
        *x += 1;
        let name = match &lex[*x] {
            Token::Name(str) => str,
            _ => {
                panic!("syntax error")
            }
        };
        *x += 1;
        match &lex[*x] {
            Token::Assign => {}
            _ => {
                panic!("syntax error")
            }
        }
        *x += 1;
        let e = Expr::new(lex, x);
        match e {
            None => {
                println!("{:?}", lex[*x]);
                panic!("syntax error when parsing declaration expression")
            }
            Some(e) => Self {
                ty,
                name: name.to_owned(),
                expr: e,
            },
        }
    }
}

impl Decls {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        match lex.get(*x) {
            None => None,
            Some(tok) => match tok {
                Token::Str | Token::Int | Token::Bool => {
                    let d = Decl::new(lex, x);
                    let ds = Decls::new(lex, x);
                    Some(Box::from(Self {
                        decl: Box::from(d),
                        decls: ds,
                    }))
                }
                _ => None,
            },
        }
    }
}

impl ToString for Decl {
    fn to_string(&self) -> String {
        self.ty.to_string() + " " + &self.name + " = " + &self.expr.to_string()
    }
}

impl ToString for Decls {
    fn to_string(&self) -> String {
        match &self.decls {
            Some(ds) => {
                self.decl.to_string() + &"\n".to_string() + &ds.to_string() + &"\n".to_string()
            }
            None => self.decl.to_string() + &"\n".to_string(),
        }
    }
}
