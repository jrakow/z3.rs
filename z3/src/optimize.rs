use std::ffi::CStr;
use std::fmt;
use z3_sys::*;
use Ast;
use Context;
use Model;
use Optimize;

impl<'ctx> Optimize<'ctx> {
    /// Create a new optimize context.
    pub fn new(ctx: &'ctx Context) -> Optimize<'ctx> {
        Optimize {
            ctx,
            z3_opt: unsafe {
                let opt = Z3_mk_optimize(ctx.z3_ctx);
                Z3_optimize_inc_ref(ctx.z3_ctx, opt);
                opt
            },
        }
    }

    /// Assert hard constraint to the optimization context.
    ///
    /// # See also:
    ///
    /// - [`Optimize::maximize()`](#method.maximize)
    /// - [`Optimize::minimize()`](#method.minimize)
    pub fn assert(&self, ast: &Ast<'ctx>) {
        unsafe { Z3_optimize_assert(self.ctx.z3_ctx, self.z3_opt, ast.z3_ast) };
    }

    /// Add a maximization constraint.
    ///
    /// # See also:
    ///
    /// - [`Optimize::assert()`](#method.assert)
    /// - [`Optimize::minimize()`](#method.minimize)
    pub fn maximize(&self, ast: &Ast<'ctx>) {
        unsafe { Z3_optimize_maximize(self.ctx.z3_ctx, self.z3_opt, ast.z3_ast) };
    }

    /// Add a minimization constraint.
    ///
    /// # See also:
    ///
    /// - [`Optimize::assert()`](#method.assert)
    /// - [`Optimize::maximize()`](#method.maximize)
    pub fn minimize(&self, ast: &Ast<'ctx>) {
        unsafe { Z3_optimize_minimize(self.ctx.z3_ctx, self.z3_opt, ast.z3_ast) };
    }

    /// Create a backtracking point.
    ///
    /// The optimize solver contains a set of rules, added facts and assertions.
    /// The set of rules, facts and assertions are restored upon calling
    /// [`Optimize::pop()`](#method.pop).
    ///
    /// # See also:
    ///
    /// - [`Optimize::pop()`](#method.pop)
    pub fn push(&self) {
        unsafe { Z3_optimize_push(self.ctx.z3_ctx, self.z3_opt) };
    }

    /// Backtrack one level.
    ///
    /// # Preconditions:
    ///
    /// - The number of calls to [`Optimize::pop`] cannot exceed the number of calls to
    ///   [`Optimize::push()`](#method.push).
    ///
    /// # See also:
    ///
    /// - [`Optimize::push()`](#method.push)
    pub fn pop(&self) {
        unsafe { Z3_optimize_pop(self.ctx.z3_ctx, self.z3_opt) };
    }

    /// Check consistency and produce optimal values.
    ///
    /// # See also:
    ///
    /// - [`Optimize::get_model()`](#method.get_model)
    pub fn check(&self) -> bool {
        unsafe { Z3_optimize_check(self.ctx.z3_ctx, self.z3_opt) == Z3_L_TRUE }
    }

    /// Retrieve the model for the last [`Optimize::check()`](#method.check)
    ///
    /// The error handler is invoked if a model is not available because
    /// the commands above were not invoked for the given optimization
    /// solver, or if the result was `Z3_L_FALSE`.
    pub fn get_model(&self) -> Model<'ctx> {
        Model::of_optimize(self)
    }

    pub fn get_help(&self) -> String {
        unsafe {
            let s = Z3_optimize_get_help(self.ctx.z3_ctx, self.z3_opt);
            CStr::from_ptr(s).to_str().unwrap().to_string()
        }
    }
}

impl<'ctx> fmt::Display for Optimize<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let p = unsafe {
            CStr::from_ptr(Z3_optimize_to_string(self.ctx.z3_ctx, self.z3_opt) as *mut i8)
        };
        if p.as_ptr().is_null() {
            return Result::Err(fmt::Error);
        }
        match p.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Result::Err(fmt::Error),
        }
    }
}

impl<'ctx> Drop for Optimize<'ctx> {
    fn drop(&mut self) {
        unsafe { Z3_optimize_dec_ref(self.ctx.z3_ctx, self.z3_opt) };
    }
}
