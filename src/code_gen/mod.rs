use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{FunctionValue, PointerValue},
};
// #[no_mangle]
// pub unsafe extern "C" fn print_str(data: *mut u8, len: usize) {
//     let s = String::from_raw_parts(data, len, len);
//     print!("{}", s);
// }

mod compile;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
}
