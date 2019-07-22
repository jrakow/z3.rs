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
mod datatype_builder;
mod func_decl;
mod model;
mod optimize;
mod param_descrs;
mod params;
mod pattern;
mod solver;
mod sort;
mod stats;
mod symbol;

/// Configuration used to initialize logical contexts.
pub struct Config {
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

pub struct Stats<'ctx> {
    ctx: &'ctx Context,
    z3_stats: Z3_stats,
}

/// Build a datatype sort.
///
/// Example:
/// ```
/// # use z3::{Config, Context, DatatypeBuilder, Solver};
/// # let cfg = Config::new();
/// # let ctx = Context::new(&cfg);
/// # let solver = Solver::new(&ctx);
///
/// // Like Rust's Option<int> type
/// let option_int = DatatypeBuilder::new(&ctx)
///         .variant("None", &[])
///         .variant("Some", &[("value", &ctx.int_sort())])
///         .finish("OptionInt");
///
/// // Assert x.is_none()
/// let x = ctx.named_const("x", &option_int.sort);
/// solver.assert(&option_int.variants[0].tester.apply(&[&x]));
///
/// // Assert y == Some(r)
/// let y = ctx.named_const("y", &option_int.sort);
/// let value = option_int.variants[1].constructor.apply(&[&ctx.from_i64(3)]);
/// solver.assert(&y._eq(&value));
///
/// assert!(solver.check());
/// let model = solver.get_model();
///
/// // Get the value out of Some(3)
/// let ast = option_int.variants[1].accessors[0].apply(&[&y]);
/// assert_eq!(3, model.eval(&ast).unwrap().as_i64().unwrap());
/// ```
pub struct DatatypeBuilder<'ctx> {
    ctx: &'ctx Context,
    // num_fields and constructor
    variants: Vec<(usize, Z3_constructor)>,
}

pub struct DatatypeVariant<'ctx> {
    pub constructor: FuncDecl<'ctx>,
    pub tester: FuncDecl<'ctx>,
    pub accessors: Vec<FuncDecl<'ctx>>,
}

pub struct Datatype<'ctx> {
    ctx: &'ctx Context,
    pub sort: Sort<'ctx>,
    pub variants: Vec<DatatypeVariant<'ctx>>,
}
