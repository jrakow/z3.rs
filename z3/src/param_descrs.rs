use z3_sys::*;
use {Context, ParamDescrs};

impl<'ctx> ParamDescrs<'ctx> {
    pub(crate) unsafe fn new(ctx: &'ctx Context, p: Z3_param_descrs) -> Self {
        Z3_param_descrs_inc_ref(ctx.z3_ctx, p);
        Self {
            ctx,
            z3_param_descrs: p,
        }
    }
}

impl<'ctx> std::fmt::Display for ParamDescrs<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = unsafe { Z3_param_descrs_to_string(self.ctx.z3_ctx, self.z3_param_descrs) };
        if p.is_null() {
            Err(std::fmt::Error)
        } else {
            write!(
                f,
                "{}",
                unsafe { std::ffi::CStr::from_ptr(p) }.to_str().unwrap()
            )
        }
    }
}

impl<'ctx> Drop for ParamDescrs<'ctx> {
    fn drop(&mut self) {
        unsafe { Z3_param_descrs_dec_ref(self.ctx.z3_ctx, self.z3_param_descrs) }
    }
}
