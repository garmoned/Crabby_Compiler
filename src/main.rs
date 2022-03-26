extern crate core;
use std::borrow::Borrow;
use std::collections::hash_map;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::Map;
use std::mem::zeroed;
use std::ptr::addr_of_mut;
use std::str::Chars;
use std::sync::Mutex;

use regex::RegexSet;

use parser::program::Program;

use crate::lexer::{Lexeme, Lexer, Token};
use crate::Token::IntLit;

mod lexer;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new();
    let contents = fs::read_to_string("test_01.txt").unwrap();
    println!("{}", contents);

    let lexeme = lexer.tokenize(contents);
    for tok in &lexeme {
        println!("{:?}", tok)
    }
    let mut lex_p = 0;

    let p = Program::new(&lexeme, &mut lex_p);

    println!("{}", p.to_string())
}


struct RecurList<T> {
    base: Box<T>,
    link: Option<Box<RecurList<T>>>,
}