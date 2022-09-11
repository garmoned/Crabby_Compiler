use std::{
    collections::{btree_map::Values, HashMap},
    vec,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::{self, Builder},
    context::Context,
    module::Module,
    passes::PassManager,
    types::BasicMetadataTypeEnum,
    values::{
        BasicMetadataValueEnum, BasicValue, CallSiteValue, FunctionValue, InstructionValue,
        PointerValue,
    },
    IntPredicate,
};

use crate::parser::{
    decls::{Decl, Decls},
    expr::{Expr, ExprData, Operation},
    program::Program,
    stmts::{self, AssignStmt, ControlStmt, ControlType, PrintStmt, StmtType, Stmts},
    var::Var,
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
        let fn_type = context.void_type().fn_type(vec![].as_slice(), false);
        let fn_val = module.add_function("main", fn_type, None);
        let mut compiler: Compiler<'a, 'ctx> = Compiler {
            context: context,
            builder: builder,
            module: module,
            fpm: pass_manager,
            fn_val: fn_val,
            variables: HashMap::new(),
        };
        compiler.compile_program(program)
    }

    fn compile_program(&mut self, program: Program) -> FunctionValue<'ctx> {
        let entry = self.context.append_basic_block(self.fn_val, "entry");
        self.builder.position_at_end(entry);

        match program.decls {
            Some(decls) => self.compile_decls(*decls),
            None => {}
        }

        match program.stmts {
            Some(stmts) => self.compile_stmts(*stmts),
            None => {}
        }

        self.builder.build_return(None);

        if self.fn_val.verify(true) {
            self.fpm.run_on(&self.fn_val);
        } else {
            println!("main is borked")
        }

        return self.fn_val;
    }

    fn compile_stmts(&mut self, stmts: Stmts) {
        match stmts.stmt {
            StmtType::Control(control) => self.compile_control(*control),
            StmtType::Print(print) => self.compile_print(*print),
            StmtType::Assign(assign) => self.compile_assign(*assign),
        }
        match stmts.stmts {
            Some(next) => self.compile_stmts(*next),
            None => {}
        }
    }

    fn compile_assign(&mut self, stmt: AssignStmt) {
        let ptr = self.variables.get(&stmt.name).unwrap();
        match self.compile_expr(*stmt.expr) {
            BasicMetadataValueEnum::IntValue(int) => self.builder.build_store(*ptr, int),
            _ => todo!(),
        };
    }

    fn compile_control(&mut self, stmt: ControlStmt) {
        match stmt.control_type {
            ControlType::If => {
                let cond = self.compile_expr(*stmt.bool).into_int_value();
                let zero_const = self.context.i16_type().const_int(0, false);
                let cond = self
                    .builder
                    .build_int_compare(IntPredicate::NE, cond, zero_const, "if");
                let then_bb = self.context.append_basic_block(self.fn_val, "then");
                let cont_bb = self.context.append_basic_block(self.fn_val, "cont");
                let else_bb = self.context.append_basic_block(self.fn_val, "else");
                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb);
                self.builder.position_at_end(then_bb);
                if let Some(decls) = stmt.decls {
                    self.compile_decls(*decls)
                }
                if let Some(stmts) = stmt.stmts {
                    self.compile_stmts(*stmts)
                }
                self.builder.build_unconditional_branch(cont_bb);

                /* TODO: implement else statements */
                self.builder.position_at_end(else_bb);
                self.builder.build_unconditional_branch(cont_bb);
                self.builder.position_at_end(cont_bb)
            }
            ControlType::While => {
                let zero_const = self.context.i16_type().const_int(0, false);
                let loop_bb = self.context.append_basic_block(self.fn_val, "loop body");
                self.builder.build_unconditional_branch(loop_bb);
                self.builder.position_at_end(loop_bb);
                if let Some(decls) = stmt.decls {
                    self.compile_decls(*decls)
                }
                if let Some(stmts) = stmt.stmts {
                    self.compile_stmts(*stmts)
                }
                let after_bb = self.context.append_basic_block(self.fn_val, "afterloop");
                let cond = self.compile_expr(*stmt.bool).into_int_value();
                let cond = self
                    .builder
                    .build_int_compare(IntPredicate::EQ, cond, zero_const, "if");
                self.builder
                    .build_conditional_branch(cond, after_bb, loop_bb);
                self.builder.position_at_end(after_bb);
            }
        }
    }

    fn compile_print(&self, print: PrintStmt) {
        let e = self.compile_expr(*print.expr);
        let func_name = match e {
            BasicMetadataValueEnum::ArrayValue(_) => todo!(),
            BasicMetadataValueEnum::IntValue(_) => "print_int",
            BasicMetadataValueEnum::FloatValue(_) => todo!(),
            BasicMetadataValueEnum::PointerValue(_) => todo!(),
            BasicMetadataValueEnum::StructValue(_) => todo!(),
            BasicMetadataValueEnum::VectorValue(_) => todo!(),
            BasicMetadataValueEnum::MetadataValue(_) => todo!(),
        };
        let func = self.module.get_function(func_name).unwrap();
        let args = vec![e];
        self.builder.build_call(func, &args, "tmp");
    }

    fn compile_decls(&mut self, decls: Decls) {
        self.compile_decl(*decls.decl);
        match decls.decls {
            Some(decls) => self.compile_decls(*decls),
            None => {}
        }
    }

    fn compile_decl(&mut self, decl: Decl) {
        let ty = match decl.ty {
            Var::Str => todo!(),
            Var::Int => self.context.i16_type(),
            Var::Bool => todo!(),
        };

        let alloc = self.builder.build_alloca(ty, decl.name.as_str());

        match self.compile_expr(*decl.expr) {
            BasicMetadataValueEnum::ArrayValue(_) => todo!(),
            BasicMetadataValueEnum::IntValue(int_val) => {
                self.builder.build_store(alloc, int_val);
            }
            BasicMetadataValueEnum::FloatValue(_) => todo!(),
            BasicMetadataValueEnum::PointerValue(_) => todo!(),
            BasicMetadataValueEnum::StructValue(_) => todo!(),
            BasicMetadataValueEnum::VectorValue(_) => todo!(),
            BasicMetadataValueEnum::MetadataValue(_) => todo!(),
        }
        self.variables.insert(decl.name.to_string(), alloc);
    }

    fn compile_expr(&self, expr: Expr) -> BasicMetadataValueEnum {
        match expr {
            Expr::Unary(data) => match data {
                ExprData::StrLit(_) => todo!(),
                ExprData::IntLit(int) => self
                    .context
                    .i16_type()
                    .const_int(int.try_into().unwrap(), false)
                    .into(),
                ExprData::Name(name) => {
                    let ptr = self.variables.get(name.as_str()).unwrap();
                    self.builder
                        .build_load(*ptr, name.as_str())
                        .into_int_value()
                        .into()
                }
            },
            Expr::Binary(left, right, op) => self.compile_binary_expr(*left, *right, op),
        }
    }

    fn compile_binary_expr(
        &self,
        left: Expr,
        right: Expr,
        op: Operation,
    ) -> BasicMetadataValueEnum {
        let left = self.compile_expr(left);
        let right = self.compile_expr(right);

        match op {
            Operation::Equals => {
                let comp = self.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    left.into_int_value(),
                    right.into_int_value(),
                    "tmp",
                );
                self.builder
                    .build_int_cast(comp, self.context.i16_type(), "cast")
                    .into()
            }
            Operation::GT => {
                let comp = self.builder.build_int_compare(
                    inkwell::IntPredicate::SGT,
                    left.into_int_value(),
                    right.into_int_value(),
                    "tmp",
                );
                self.builder
                    .build_int_cast(comp, self.context.i16_type(), "cast")
                    .into()
            }
            Operation::LT => {
                let comp = self.builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    left.into_int_value(),
                    right.into_int_value(),
                    "tmp",
                );
                self.builder
                    .build_int_cast(comp, self.context.i16_type(), "cast")
                    .into()
            }
            Operation::Times => self
                .builder
                .build_int_mul(left.into_int_value(), right.into_int_value(), "tmp")
                .into(),
            Operation::Plus => self
                .builder
                .build_int_add(left.into_int_value(), right.into_int_value(), "tmp")
                .into(),
        }
    }
}
