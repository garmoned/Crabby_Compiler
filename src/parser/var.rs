pub enum Var {
    Str,
    Int,
}

impl ToString for Var {
    fn to_string(&self) -> String {
        match self {
            Var::Str => { "str" }
            Var::Int => { "int" }
        }.parse().unwrap()
    }
}
