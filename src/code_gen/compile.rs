use std::{collections::HashMap, vec};

use inkwell::{
    basic_block::BasicBlock,
    builder::{self, Builder},
    context::Context,
    module::Module,
    passes::PassManager,
    values::{
        BasicMetadataValueEnum, CallSiteValue, FunctionValue, InstructionValue, PointerValue,
    },
};

use crate::parser::{
    decls::{Decl, Decls},
    expr::{AddExpr, Expr, ExprData, Factor, Term, TopExpr},
    program::{self, Program},
    stmts::{ControlStmt, PrintStmt, Stmt, StmtType, Stmts},
};

use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        program: Program,
    ) -> FunctionValue<'ctx> {
        let compiler: Compiler<'a, 'ctx> = Compiler {
            context: context,
            builder: builder,
            module: module,
            fpm: pass_manager,
            variables: HashMap::new(),
        };
        compiler.compile_program(program)
    }

    fn compile_program(&self, program: Program) -> FunctionValue<'ctx> {
        let fn_type = self.context.void_type().fn_type(vec![].as_slice(), false);
        let fn_val = self.module.add_function("main", fn_type, None);
        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);
        self.compile_stmts(*program.stmts.unwrap());
        self.builder.build_return(None);

        if fn_val.verify(true) {
            println!("verified");
            self.fpm.run_on(&fn_val);
        } else {
            println!("main is borked")
        }

        return fn_val;
    }

    fn compile_stmts(&self, stmts: Stmts) {
        match stmts.stmt {
            StmtType::Control(_) => todo!(),
            StmtType::Print(print) => self.compile_print(*print),
        }
    }

    fn compile_print(&self, print: PrintStmt) {
        let func = self.module.get_function("print_int").unwrap();
        let args = vec![self.compile_expr(*print.expr)];
        self.builder.build_call(func, &args, "tmp");
    }

    fn compile_decls(&self, decls: Decls) {}

    fn compile_decl(&self, decl: Decl) {}

    fn compile_expr(&self, expr: Expr) -> BasicMetadataValueEnum {
        match expr {
            Expr::Unary(data) => match data {
                ExprData::StrLit(_) => todo!(),
                ExprData::IntLit(_) => self.context.i16_type().const_int(10, false).into(),
                ExprData::Name(_) => todo!(),
            },
            Expr::Binary(_, _, _) => todo!(),
        }
    }
}
