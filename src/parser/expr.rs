use crate::{Lexeme, Token};

pub enum Operation {
    Equals,
    Times,
    Plus
}

pub enum ExprData {
    StrLit(String),
    IntLit(i16),
    Name(String),
}

pub enum Factor {
    Data(ExprData),
    Expr(Box<Expr>)
}

pub enum Term {
    TermFactor(Option<Box<TermPrime>>, Box<Factor>)
}

pub enum TermPrime {
    More(Box<Factor>, Option<Box<TermPrime>>),
}

pub enum Expr {
    Term(Term),
    TermExpression(Term, Box<Expr>)
}

impl Expr {
    pub(crate) fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        // println!("new expr");
        let save = x.clone();
        match Term::new(lex,x){
            None => {
                *x = save;
                None}
            Some(term) => {
                match lex[*x] {
                    Token::Plus => {
                        *x+=1;
                        // println!("matched plus");
                        let save = x.clone();
                        match Expr::new(lex,x) {
                            None => {*x = save; None}
                            Some(expr) => {
                                Some(Box::from(Expr::TermExpression(term, expr)))
                            }
                        }
                    }
                    _ => {
                        Some(Box::from(Expr::Term(term)))
                    }
                }
            }
        }
    }
}

impl Term {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Self> {
        // println!("new term {:?}", lex[*x]);
        let save = x.clone();
        match Factor::new(lex,x) {
            None => {
                *x = save;
                None
            }
            Some(fact) => {
               Some(Term::TermFactor(
                    TermPrime::new(lex,x),
                    Box::from(fact)
                ))
            }
        }

    }
}

impl TermPrime {
    fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Box<Self>> {
        // println!("new termprime");
        let save = x.clone();
        // println!("{:?}",lex[*x]);
        match lex[*x] {
            Token::Times => {}
            _ => {return None}
        }
        *x+=1;
        let factor = Factor::new(lex,x);
        match factor {
            None => {
                *x = save;
                None}
            Some(fact) => {
                Some(Box::from(TermPrime::More(Box::from(fact), TermPrime::new(lex, x))))
            }
        }
    }
}

impl Factor {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Self>{
        // println!("new factor {:?}", lex[*x]);
        let save = x.clone();
        match Factor::try_paren(lex,x){
            None => {
                *x = save;
                match Factor::try_id(lex,x){
                    None => {None}
                    Some(id) => {Some(id)}
                }
            }
            Some(expr) => {
                Some(expr)
            }
        }
    }

    pub fn try_paren(lex: &Lexeme, mut x: &mut usize) -> Option<Self>{
        let save = x.clone();
        match lex[*x] {
            Token::OpenParen => {},
            _ => {return None}
        }
        *x+=1;
        let data = Expr::new(lex,x);
        match lex[*x] {
            Token::CloseParen => {},
            _ => {return None}
        }
        *x+=1;
        match data {
            None => {*x = save; None}
            Some(expr) => {
                Some(Factor::Expr(expr))
            }
        }
    }

    pub fn try_id(lex: &Lexeme, mut x: &mut usize) -> Option<Self> {
        match ExprData::new(lex,x) {
            None => { None}
            Some(data) => { Some(Factor::Data(data))}
        }
    }
}

impl ExprData {
    pub fn new(lex: &Lexeme, mut x: &mut usize) -> Option<Self> {
        match &lex[*x] {
            Token::Name(string) => {
                *x+= 1;Some(ExprData::Name(string.clone()))}
            Token::IntLit(int) => {
                *x+= 1;Some(ExprData::IntLit(int.clone()))}
            Token::StringLit(string) => {
                *x+= 1;Some(ExprData::StrLit(string.clone()))}
            _ => { None }
        }

    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Term(term) => {format!("({})",term.to_string())}
            Expr::TermExpression(term, expr)
            => {format!("({}) + {}",term.to_string(), expr.to_string())}
        }
    }
}

impl ToString for TermPrime {
    fn to_string(&self) -> String {
        match self { TermPrime::More(factor, more) => {
         match more {
             None => {format!("*{}",factor.to_string())}
             Some(more) => {format!("*{}{}",factor.to_string(),more.to_string())}
         }
        }}
    }
}

impl ToString for Term {
    fn to_string(&self) -> String {
        match self { Term::TermFactor(more, factor) => {
            match more {
                None => { format!("{}",factor.to_string())}
                Some(more) => { format!("{}{}",factor.to_string(),more.to_string())}
            }
        } }
    }
}

impl ToString for ExprData {
    fn to_string(&self) -> String {
         match self {
             ExprData::StrLit(str) => {str.to_string()}
             ExprData::IntLit(int) => {int.to_string()}
             ExprData::Name(str) => {str.to_string()}
         }
    }
}

impl ToString for Factor {
    fn to_string(&self) -> String {
        match self {
            Factor::Data(data) => {
                data.to_string()
            }
            Factor::Expr(expr) => {
                expr.to_string()
            }
        }
    }
}