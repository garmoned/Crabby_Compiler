extern crate core;

use std::borrow::Borrow;
use std::collections::hash_map;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::Map;
use std::mem::zeroed;
use std::ptr::addr_of_mut;
use std::str::Chars;
use std::sync::Mutex;

use code_gen::Compiler;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Linkage;
use inkwell::passes::PassManager;
use inkwell::types::{FunctionType, VoidType};
use inkwell::OptimizationLevel;
use libc::c_void;
use llvm_sys::support::LLVMAddSymbol;
use regex::RegexSet;

use parser::program::Program;

use crate::lexer::{Lexeme, Lexer, Token};
use crate::Token::IntLit;

mod code_gen;
mod lexer;
mod parser;

#[no_mangle]
pub extern "C" fn print_int(int: i16) {
    println!("{}", int);
    return;
}

// // Adding the functions above to a global array,
// // so Rust compiler won't remove them.
// #[used]
// static EXTERNAL_FNS: [extern "C" fn(i16); 1] = [print_int];

// #[no_mangle]
// pub extern "C" fn printd(x: f64) -> f64 {
//     println!("{}", x);
//     x
// }

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

    let int_type = context.f16_type();
    let fn_type = context
        .void_type()
        .fn_type(vec![int_type.into()].as_slice(), false);

    module.add_function("print_int", fn_type, None);

    Compiler::compile(&context, &builder, &module, &fpm, p);
    unsafe {
        let c_str = CString::new(b"print_int" as &[u8]).unwrap();
        LLVMAddSymbol(c_str.as_ptr(), print_int as *mut c_void)
    }

    let ee = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    // let compiled_func = unsafe { ee.get_function::<unsafe extern "C" fn(f64) -> f64>("printd") };

    // unsafe {
    //     compiled_func.unwrap().call(10.0);
    // }
    let compiled_func = unsafe { ee.get_function::<unsafe extern "C" fn()>("main") };
    match compiled_func {
        Ok(func) => unsafe { func.call() },
        Err(err) => {
            println!("error during compilation {:?}", &err);
        }
    };
}
