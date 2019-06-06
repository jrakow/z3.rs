use std::convert::TryInto;
use z3_sys::*;
use {Ast, Context, Pattern};

impl<'ctx> Pattern<'ctx> {
    pub fn new(ctx: &'ctx Context, terms: &[&Ast<'ctx>]) -> Self {
        assert!(terms.iter().all(|a| a.ctx.z3_ctx == ctx.z3_ctx));

        let terms: Vec<_> = terms.iter().map(|a| a.z3_ast).collect();
        unsafe {
            let a = Z3_mk_pattern(ctx.z3_ctx, terms.len().try_into().unwrap(), terms.as_ptr());
            Z3_inc_ref(ctx.z3_ctx, a as Z3_ast);
            Self { ctx, z3_pattern: a }
        }
    }
}

impl<'ctx> Drop for Pattern<'ctx> {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.ctx.z3_ctx, self.z3_pattern as Z3_ast) }
    }
}
