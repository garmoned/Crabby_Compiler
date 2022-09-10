#![feature(macro_metavar_expr)]
extern crate core;
use std::ffi::CString;
use std::fs;

use code_gen::Compiler;
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::OptimizationLevel;
use libc::c_void;
use llvm_sys::support::LLVMAddSymbol;

use parser::program::Program;

use crate::lexer::{Lexeme, Token};

mod code_gen;
mod lexer;
mod parser;

#[no_mangle]
pub extern "C" fn print_int(int: i16) {
    println!("{}", int);
    return;
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(i16); 1] = [print_int];

fn main() {
    let lexer = lexer::Lexer::new();
    let contents = fs::read_to_string("test_01.txt").unwrap();
    let lexeme = lexer.tokenize(contents);
    let mut lex_p = 0;
    let p = Program::new(&lexeme, &mut lex_p);

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main");

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let int_type = context.i16_type();
    let fn_type = context
        .void_type()
        .fn_type(vec![int_type.into()].as_slice(), false);

    module.add_function("print_int", fn_type, None);

    println!("{}", p.to_string());

    // Compiler::compile(&context, &builder, &module, &fpm, p);

    // unsafe {
    //     let c_str = CString::new(b"print_int" as &[u8]).unwrap();
    //     LLVMAddSymbol(c_str.as_ptr(), print_int as *mut c_void)
    // }

    // let ee = module
    //     .create_jit_execution_engine(OptimizationLevel::None)
    //     .unwrap();

    // let compiled_func = unsafe { ee.get_function::<unsafe extern "C" fn()>("main") };
    // match compiled_func {
    //     Ok(func) => unsafe { func.call() },
    //     Err(err) => {
    //         println!("error during compilation {:?}", &err);
    //     }
    // };
}
