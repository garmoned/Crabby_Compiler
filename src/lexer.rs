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
    OpenParen,
    CloseParen,
    Assign,
    Equals,
    Print,
    OpenBrace,
    CloseBrace,
    WhiteSpace,
    Semi,
    If,
    Ignore,
    Times,
    Plus,
    Bool,
    EOF,
    GT,
    LT,
    Name(String),
    StringLit(String),
    IntLit(i16),
}

#[derive(Debug, Clone)]

enum RawToken {
    Int,
    Str,
    While,
    OpenParen,
    CloseParen,
    Assign,
    Equals,
    Print,
    OpenBrace,
    CloseBrace,
    WhiteSpace,
    Semi,
    If,
    Ignore,
    Times,
    Plus,
    Bool,
    EOF,
    Name,
    StringLit,
    IntLit,
    GT,
    LT,
}

impl RawToken {
    fn into_token(self, data: &str) -> Token {
        match self {
            RawToken::Int => Token::Int,
            RawToken::Str => Token::Str,
            RawToken::While => Token::While,
            RawToken::OpenParen => Token::OpenParen,
            RawToken::CloseParen => Token::CloseParen,
            RawToken::Assign => Token::Assign,
            RawToken::Equals => Token::Equals,
            RawToken::Print => Token::Print,
            RawToken::OpenBrace => Token::OpenBrace,
            RawToken::CloseBrace => Token::CloseBrace,
            RawToken::WhiteSpace => Token::WhiteSpace,
            RawToken::Semi => Token::Semi,
            RawToken::If => Token::If,
            RawToken::Ignore => Token::Ignore,
            RawToken::Times => Token::Times,
            RawToken::Plus => Token::Plus,
            RawToken::Bool => Token::Bool,
            RawToken::EOF => Token::EOF,
            RawToken::GT => Token::GT,
            RawToken::LT => Token::LT,
            RawToken::Name => Token::Name(data.to_string()),
            RawToken::StringLit => Token::StringLit(data.replace("'", "")),
            RawToken::IntLit => Token::IntLit(data.parse().unwrap()),
        }
    }
}

pub enum DataToken {}

pub type Lexeme = Vec<Token>;

pub(crate) struct Lexer {
    match_set: RegexSet,
}
const LETTER: &str = r"[A-Za-z]";
const DIGIT: &str = r"[0-9]";
const STRING_LIT: &str = r#"^"([^"\\]|\\.)*"$"#;
const WHITE_SPACE: &str = r"^[ \n\t\r]$";

macro_rules! implement_lexer {
        ($($pat:expr => $tok:expr), *) => {
                pub fn new() -> Self {
                    Self {
                        match_set: RegexSet::new(&[
                            $(
                                $pat,
                            )*
                        ])
                        .unwrap(),
                    }
                }

                fn match_token(&self, txt: &str) -> Option<RawToken> {
                    let matches: Vec<usize> = self.match_set.matches(txt).into_iter().collect();
                    let m = matches.get(0).unwrap_or(&(1000 as usize));
                    match m {
                        $(
                            ${index()} => Some($tok),
                        )*
                        _ => None
                    }
                }
        };

    }

impl Lexer {
    implement_lexer!(
        r"^int$" => RawToken::Int,
        r"^str$" => RawToken::Str,
        r"^bool$" => RawToken::Bool,
        r"^while$" => RawToken::While,
        STRING_LIT => RawToken::StringLit,
        &format!(r"^{}+$", DIGIT) => RawToken::IntLit,
        r"^\($" => RawToken::OpenParen,
        r"^\)$" => RawToken::CloseParen,
        r"^=$" => RawToken::Assign,
        r"^==$" => RawToken::Equals,
        r"^>$" => RawToken::GT,
        r"^<$" => RawToken::LT,
        r"^print$" => RawToken::Print,
        r"^\{$"=> RawToken::OpenBrace,
        r"^}$" => RawToken::CloseBrace,
        WHITE_SPACE => RawToken::WhiteSpace,
        r"^;$" => RawToken::Semi,
        r"^if$" => RawToken::If,
        r"^\*$" => RawToken::Times,
        r"^\+$" => RawToken::Plus,
        &format!("^{}({}|{})*$", LETTER, LETTER, DIGIT) => RawToken::Name

    );
    pub fn tokenize(&self, code: String) -> Lexeme {
        let mut lexeme: Vec<Token> = vec![];
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
                            RawToken::WhiteSpace => (),
                            _ => lexeme.push(tok.into_token(&prev)),
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
            Some(tok) => lexeme.push(tok.into_token(&prev)),
        }
        lexeme.push(Token::EOF);
        lexeme
    }
}
