#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;

extern crate z3_sys;

use std::ffi::CString;
use z3_sys::*;

mod ast;
mod config;
mod context;
mod func_decl;
mod model;
mod optimize;
mod param_descrs;
mod params;
mod pattern;
mod solver;
mod sort;
mod symbol;

/// Configuration used to initialize logical contexts.
pub struct Config {
    kvs: Vec<(CString, CString)>,
    z3_cfg: Z3_config,
}

/// Manager of all other Z3 objects, global configuration options, etc.
pub struct Context {
    z3_ctx: Z3_context,
}

/// Symbols are used to name several term and type constructors.
///
/// # Creation:
///
/// Symbols can be created with either [`Symbol::from_int()`] or
/// [`Symbol::from_string()`].
///
/// [`Symbol::from_int()`]: struct.Symbol.html#method.from_int
/// [`Symbol::from_string()`]: struct.Symbol.html#method.from_string
pub struct Symbol<'ctx> {
    ctx: &'ctx Context,
    cst: Option<CString>,
    z3_sym: Z3_symbol,
}

/// Kind of [`Ast`](struct.Ast.html) used to represent types.
pub struct Sort<'ctx> {
    ctx: &'ctx Context,
    z3_sort: Z3_sort,
}

/// Abstract syntax tree node. That is, the data structure used in Z3
/// to represent terms, formulas, and types.
pub struct Ast<'ctx> {
    ctx: &'ctx Context,
    z3_ast: Z3_ast,
}

/// (Incremental) solver, possibly specialized by a particular tactic or logic.
pub struct Solver<'ctx> {
    ctx: &'ctx Context,
    z3_slv: Z3_solver,
}

/// Model for the constraints inserted into the logical context.
pub struct Model<'ctx> {
    ctx: &'ctx Context,
    z3_mdl: Z3_model,
}

/// Context for solving optimization queries.
pub struct Optimize<'ctx> {
    ctx: &'ctx Context,
    z3_opt: Z3_optimize,
}

pub struct FuncDecl<'ctx> {
    ctx: &'ctx Context,
    z3_func_decl: Z3_func_decl,
}

pub struct Pattern<'ctx> {
    ctx: &'ctx Context,
    z3_pattern: Z3_pattern,
}

pub struct Params<'ctx> {
    ctx: &'ctx Context,
    z3_params: Z3_params,
}

pub struct ParamDescrs<'ctx> {
    ctx: &'ctx Context,
    z3_param_descrs: Z3_param_descrs,
}
