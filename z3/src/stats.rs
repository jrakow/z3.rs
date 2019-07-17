use crate::*;

impl<'ctx> Stats<'ctx> {
    pub fn with_solver(solver: &Solver<'ctx>) -> Self {
        let ctx = solver.ctx;
        unsafe {
            let z3_stats = Z3_solver_get_statistics(ctx.z3_ctx, solver.z3_slv);
            Z3_stats_inc_ref(ctx.z3_ctx, z3_stats);

            Self { ctx, z3_stats }
        }
    }
}

impl<'ctx> Drop for Stats<'ctx> {
    fn drop(&mut self) {
        unsafe {
            Z3_stats_dec_ref(self.ctx.z3_ctx, self.z3_stats);
        }
    }
}

impl<'ctx> std::fmt::Display for Stats<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = unsafe { Z3_stats_to_string(self.ctx.z3_ctx, self.z3_stats) };
        if p.is_null() {
            Err(std::fmt::Error)
        } else {
            let s = unsafe { std::ffi::CStr::from_ptr(p) };

            write!(f, "{}", s.to_str().map_err(|_| std::fmt::Error)?)
        }
    }
}
