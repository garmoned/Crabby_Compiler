use std::borrow::Borrow;

use regex::RegexSet;

use crate::lexer::Token::{
    Assign, CloseBrace, CloseParen, Equals, If, Ignore, Int, OpenBrace, OpenParen, Print, Semi,
    Str, WhiteSpace,
};

#[derive(Debug, Clone)]
pub enum Token {
    Int,
    Str,
    While,
    StringLit(String),
    IntLit(i16),
    OpenParen,
    CloseParen,
    Assign,
    Equals,
    Print,
    OpenBrace,
    CloseBrace,
    WhiteSpace,
    Semi,
    Name(String),
    If,
    Ignore,
    Times,
    Plus,
    Bool,
    EOF,
}

pub type Lexeme = Vec<Token>;

pub(crate) struct Lexer {
    match_set: RegexSet,
}

impl Lexer {
    pub fn new() -> Self {
        let letter = r"[A-Za-z]";
        let digit = r"[0-9]";
        let name = format!("^{}({}|{})*$", letter, letter, digit);
        let int_lit = format!(r"^{}+$", digit);
        let string_lit = r#"^"([^"\\]|\\.)*"$"#;
        let white_space = r"^[ \n\t\r]$";
        Self {
            match_set: RegexSet::new(&[
                r"^int$", //0
                r"^str$", //1
                r"^bool$",
                r"^while$",
                string_lit,       //2
                int_lit.borrow(), //3
                r"^\($",          //4
                r"^\)$",          //5
                r"^=$",           //6
                r"^==$",          //7
                r"^print$",       //8
                r"^\{$",          //9
                r"^}$",           //10
                white_space,      //11
                r"^;$",           //12
                r"^if$",          // 13
                r"^\*$",          // 14
                r"^\+$",          //15
                name.borrow(),    //16
            ])
            .unwrap(),
        }
    }

    pub fn tokenize(&self, code: String) -> Lexeme {
        let mut lexeme = vec![];
        let mut prev = "".to_string();
        for char in code.chars() {
            let new = format!("{}{}", prev, char);
            println!("{}, {}", prev, new);
            match self.match_token(new.as_str()) {
                None => match self.match_token(prev.as_str()) {
                    None => {
                        prev.push(char);
                    }
                    Some(tok) => {
                        match tok {
                            WhiteSpace => (),
                            _ => lexeme.push(tok),
                        }
                        prev = char.to_string();
                    }
                },
                Some(_) => {
                    prev.push(char);
                }
            }
        }
        match self.match_token(&prev) {
            None => {}
            Some(tok) => lexeme.push(tok),
        }
        lexeme.push(Token::EOF);
        lexeme
    }

    fn match_token(&self, txt: &str) -> Option<Token> {
        let matches: Vec<usize> = self.match_set.matches(txt).into_iter().collect();
        let m = matches.get(0).unwrap_or(&(1000 as usize));
        match m {
            0 => Some(Int),
            1 => Some(Str),
            2 => Some(Token::Bool),
            3 => Some(Token::While),
            4 => Some(Token::StringLit(txt.replace("'", ""))),
            5 => Some(Token::IntLit(txt.parse().unwrap())),
            6 => Some(OpenParen),
            7 => Some(CloseParen),
            8 => Some(Assign),
            9 => Some(Equals),
            10 => Some(Print),
            11 => Some(OpenBrace),
            12 => Some(CloseBrace),
            13 => Some(WhiteSpace),
            14 => Some(Semi),
            15 => Some(If),
            16 => Some(Token::Times),
            17 => Some(Token::Plus),
            18 => Some(Token::Name(txt.parse().unwrap())),
            _ => None,
        }
    }
}
