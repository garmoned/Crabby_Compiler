#![feature(macro_metavar_expr)]
extern crate core;
use std::ffi::CString;
use std::fs;
use std::process::Command;

use code_gen::Compiler;
use inkwell::passes::PassManager;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target};
use inkwell::OptimizationLevel;
use inkwell::{context::Context, targets::TargetMachine};
use libc::c_void;
use llvm_sys::support::LLVMAddSymbol;

use llvm_sys::target_machine;
use parser::program::Program;

use crate::lexer::{Lexeme, Token};

mod code_gen;
mod io;
mod lexer;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new();
    let contents = fs::read_to_string("fib.txt").unwrap();
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

    Compiler::compile(&context, &builder, &module, &fpm, p);

    unsafe {
        let c_str = CString::new(b"print_int" as &[u8]).unwrap();
        LLVMAddSymbol(c_str.as_ptr(), io::print_int as *mut c_void)
    }

    Target::initialize_all(&InitializationConfig::default());

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();
    let output_filename = "object_file";
    target_machine
        .write_to_file(
            &module,
            inkwell::targets::FileType::Object,
            output_filename.as_ref(),
        )
        .unwrap();

    Command::new("clang")
        .args(&["object_file", "libio.a"])
        .status()
        .unwrap();
}
