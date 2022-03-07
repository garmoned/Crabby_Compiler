extern crate core;

mod lexer;

use std::borrow::Borrow;
use std::collections::hash_map;
use std::fmt::{Display, Formatter};
use regex::RegexSet;
use std::fs;
use std::iter::Map;
use std::mem::zeroed;
use std::str::Chars;
use std::sync::Mutex;
use crate::ExprType::StrLit;
use crate::lexer::{Lexeme, Lexer, Token};
use crate::Token::IntLit;

fn main() {

    let lexer = lexer::Lexer::new();
    let contents = fs::read_to_string("test_01.txt").unwrap();
    println!("{}",contents);

    let lexeme = lexer.tokenize(contents);

    let mut lex_p = 0;

    let p = Program::new(&lexeme, &mut lex_p);


    println!("{}",p.to_string())

    
}

impl Program {

    pub fn new(lex: &Lexeme, x: & mut usize) -> Self {
        Self {
            decls:Decls::new(lex,x)
        }
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
       "Program{\n".to_string()  + &self.decls.as_ref().unwrap().to_string() + &*"}".to_string()
    }
}

impl ToString for Decls {
    fn to_string(&self) -> String {
        match &self.decls {
            Some(ds) => {
                self.decl.to_string() + &"\n".to_string() + &ds.to_string() + &"\n".to_string()
            },
            None => {
                self.decl.to_string() + &"\n".to_string()
            }
        }
    }
}


impl ToString for Decl {
    fn to_string(&self) -> String {
        self.ty.to_string() + " " + &self.name + " = " + &self.expr.to_string()
    }
}

impl Decls {
    pub fn new(lex : &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        match lex.get(*x) {
            None => None,
            Some(tok) => {
                match tok {
                Token::Str | Token::Int   => {
                    let d = Decl::new(lex,x);
                    let ds = Decls::new(lex, x);
                    Some(Box::from(Self {
                        decl: d,
                        decls: ds
                    }))
                }
                _ => None
            }}
        }

    }
}


impl Decl {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Self
    {
        let ty = match lex[*x] {
            Token::Int => { Var::Int }
            Token::Str => { Var::Str }
            _ => {panic!("syntax error")}
        };
        *x += 1;
        let name = match &lex[*x] {
            Token::Name(str) => { str }
            _ => {panic!("syntax error")}
        };
        *x += 1;
        let e = Expr::new(lex, x);
        Self {
            ty,
            name: name.to_owned(),
            expr: e
        }
    }
}


impl Expr {
    pub fn new(lex : &Lexeme, mut x: &mut usize) -> Self {
        *x += 1;
        let e = match &lex[*x] {
            Token::StringLit(str) => {StrLit(str.to_owned())}
            Token::IntLit(i) => {ExprType::IntLit(i.to_owned())}
            _ => {panic!("syntax error")}
        };
        *x += 1;
        Self {
            expr_type: e
        }
    }
}

struct Program {
    decls : Option<Box<Decls>>,
    // stmts : Stmts
}

struct Decls {
    decl : Decl,
    decls :Option<Box<Decls>>
}

struct Stmts {
    stmt : Box<Stmt>,
    stmts : Option<Box<Stmts>>
}

struct Expr {
   expr_type: ExprType
}

enum ExprType {
    StrLit(String),
    IntLit(i16)
    //Name
    //Action(Expression,Operation,Expression)
}

enum Operation {
    Equals,
}

struct Action {
    exp1 : Expr,
    exp2 : Expr,
    op : Operation
}

enum Stmt {
    Print(Expr),
    If (Expr,Decls,Stmts)
}

struct Decl {
    ty : Var,
    name : String,
    expr : Expr
}


enum Var {
    Str,
    Int
}


impl ToString for Var{
    fn to_string(&self) -> String {
        match self {
            Var::Str => { "str" }
            Var::Int => { "int" }
        }.parse().unwrap()
    }
}


impl ToString for Expr {
    fn to_string(&self) -> String {
        match &self.expr_type {
            StrLit( str ) => { str.to_string()}
            ExprType::IntLit(i) => {i.clone().to_string()}
        }
    }
}