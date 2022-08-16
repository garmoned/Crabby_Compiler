pub enum Var {
    Str,
    Int,
    Bool,
}

impl ToString for Var {
    fn to_string(&self) -> String {
        match self {
            Var::Bool => "bool",
            Var::Str => "str",
            Var::Int => "int",
        }
        .parse()
        .unwrap()
    }
}
