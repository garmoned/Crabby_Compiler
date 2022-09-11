use crate::parser::decls::Decls;
use crate::parser::expr::Expr;
use crate::{Lexeme, Token};

pub struct Stmts {
    pub(crate) stmt: StmtType,
    pub(crate) stmts: Option<Box<Stmts>>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct ControlStmt {
    pub bool: Box<Expr>,
    pub control_type: ControlType,
    pub(crate) decls: Option<Box<Decls>>,
    pub stmts: Option<Box<Stmts>>,
}

pub struct AssignStmt {
    pub name: String,
    pub expr: Box<Expr>,
}

pub enum StmtType {
    Control(Box<ControlStmt>),
    Print(Box<PrintStmt>),
    Assign(Box<AssignStmt>),
}

impl ToString for StmtType {
    fn to_string(&self) -> String {
        match self {
            StmtType::Control(control) => control.to_string(),
            StmtType::Print(print) => print.to_string(),
            StmtType::Assign(assign) => assign.to_string(),
        }
    }
}

pub enum ControlType {
    If,
    While,
}

impl ControlStmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<ControlStmt>>
    where
        Self: Sized,
    {
        let mut control_type = ControlType::While;
        match &lex[*x] {
            Token::If => control_type = ControlType::If,
            Token::While => control_type = ControlType::While,
            _ => return Option::None,
        }
        *x += 1;
        match &lex[*x] {
            Token::OpenParen => {}
            _ => return Option::None,
        }
        *x += 1;
        let e = Expr::new(lex, x);
        match &lex[*x] {
            Token::CloseParen => {}
            _ => return Option::None,
        }
        *x += 1;
        match &lex[*x] {
            Token::OpenBrace => {}
            _ => return Option::None,
        }
        *x += 1;
        let decls = Decls::new(lex, x);
        let stmts = Stmts::new(lex, x);
        match &lex[*x] {
            Token::CloseBrace => {}
            _ => return Option::None,
        }
        *x += 1;
        Some(Box::new(ControlStmt {
            bool: e.unwrap(),
            control_type,
            decls,
            stmts,
        }))
    }

    fn to_string(&self) -> String {
        let decl_string = match &self.decls {
            None => "".to_string(),
            Some(decls) => decls.to_string(),
        };
        let stmt_string = match &self.stmts {
            None => "".to_string(),
            Some(stmts) => stmts.to_string(),
        };
        format!(
            "{}({}){{{}{}}}",
            match self.control_type {
                ControlType::If => {
                    "if"
                }
                ControlType::While => {
                    "while"
                }
            },
            self.bool.to_string(),
            decl_string,
            stmt_string
        )
    }
}

impl PrintStmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<PrintStmt>> {
        match &lex[*x] {
            Token::Print => {}
            _ => return Option::None,
        };
        *x += 1;
        match &lex[*x] {
            Token::OpenParen => {}
            _ => return Option::None,
        };
        *x += 1;
        let e = Expr::new(lex, x);
        match &lex[*x] {
            Token::CloseParen => {}
            _ => return Option::None,
        };
        *x += 1;
        Some(Box::new(PrintStmt { expr: e.unwrap() }))
    }

    fn to_string(&self) -> String {
        format!("print({})", self.expr.to_string())
    }
}

impl AssignStmt {
    fn parse(lex: &Lexeme, x: &mut usize) -> Option<Box<AssignStmt>> {
        let mut name = "";
        match &lex[*x] {
            Token::Name(real_name) => name = real_name,
            _ => return Option::None,
        };
        *x += 1;
        match &lex[*x] {
            Token::Assign => {}
            _ => return Option::None,
        };
        *x += 1;
        if let Some(e) = Expr::new(lex, x) {
            return Some(Box::new(AssignStmt {
                expr: e,
                name: name.to_string(),
            }));
        }
        None
    }
    fn to_string(&self) -> String {
        format!("assign {} = {}", self.name, self.expr.to_string())
    }
}

fn parse_stmt(lex: &Lexeme, x: &mut usize) -> Option<StmtType> {
    let save = x.clone();
    match PrintStmt::parse(lex, x) {
        None => {
            *x = save.clone();
            match ControlStmt::parse(lex, x) {
                None => {
                    *x = save.clone();
                    match AssignStmt::parse(lex, x) {
                        Some(assign) => Some(StmtType::Assign(assign)),
                        None => {
                            *x = save.clone();
                            None
                        }
                    }
                }
                Some(if_stmt) => Some(StmtType::Control(if_stmt)),
            }
        }
        Some(print) => Some(StmtType::Print(print)),
    }
}

impl Stmts {
    pub fn new(lex: &Lexeme, x: &mut usize) -> Option<Box<Self>> {
        let save = x.clone();
        let st = parse_stmt(lex, x);
        match st {
            None => {
                *x = save;
                None
            }
            Some(stmt) => Some(Box::new(Self {
                stmt,
                stmts: Stmts::new(lex, x),
            })),
        }
    }
}

impl ToString for Stmts {
    fn to_string(&self) -> String {
        match &self.stmts {
            Some(ds) => {
                self.stmt.to_string() + &"\n".to_string() + &ds.to_string() + &"\n".to_string()
            }
            None => self.stmt.to_string() + &"\n".to_string(),
        }
    }
}
