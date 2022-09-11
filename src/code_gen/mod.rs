use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{FunctionValue, PointerValue},
};

mod compile;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub fn_val: FunctionValue<'ctx>,
}
