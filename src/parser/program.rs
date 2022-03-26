use crate::Lexeme;
use crate::parser::decls::Decls;
use crate::parser::stmts::Stmts;

pub struct Program {
    decls: Option<Box<Decls>>,
    stmts: Option<Box<Stmts>>,
}

impl Program {
    pub fn new(lex: &Lexeme, x: &mut usize) -> Self {
        Self {
            decls: Decls::new(lex, x),
            stmts: Stmts::new(lex, x),
        }
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        format!("Program {{\n{}{}\n}}",
            &match &self.decls {
                None => {"".to_string()}
                Some(decls) => {decls.to_string()}
            },
            &match &self.stmts {
                None => {"".to_string()}
                Some(stmts) => {stmts.to_string()}
            }
        )
    }
}
