use std::convert::TryInto;
use z3_sys::*;
use ParamDescrs;
use {Context, Params, Symbol};

impl<'ctx> Params<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        unsafe {
            let z3_params = Z3_mk_params(ctx.z3_ctx);
            Z3_params_inc_ref(ctx.z3_ctx, z3_params);
            Self { ctx, z3_params }
        }
    }

    pub fn set_bool(&self, k: &Symbol<'ctx>, v: bool) {
        assert_eq!(self.ctx.z3_ctx, k.ctx.z3_ctx);

        unsafe { Z3_params_set_bool(self.ctx.z3_ctx, self.z3_params, k.z3_sym, v) }
    }

    pub fn set_uint(&self, k: &Symbol<'ctx>, v: usize) {
        assert_eq!(self.ctx.z3_ctx, k.ctx.z3_ctx);

        unsafe {
            Z3_params_set_uint(
                self.ctx.z3_ctx,
                self.z3_params,
                k.z3_sym,
                v.try_into().unwrap(),
            )
        }
    }

    pub fn set_f64(&self, k: &Symbol<'ctx>, v: f64) {
        assert_eq!(self.ctx.z3_ctx, k.ctx.z3_ctx);

        unsafe { Z3_params_set_double(self.ctx.z3_ctx, self.z3_params, k.z3_sym, v) }
    }

    pub fn set_symbol(&self, k: &Symbol<'ctx>, v: &Symbol<'ctx>) {
        assert_eq!(self.ctx.z3_ctx, k.ctx.z3_ctx);
        assert_eq!(self.ctx.z3_ctx, v.ctx.z3_ctx);

        unsafe { Z3_params_set_symbol(self.ctx.z3_ctx, self.z3_params, k.z3_sym, v.z3_sym) }
    }

    pub fn validate(&self, d: &ParamDescrs<'ctx>) {
        assert_eq!(self.ctx.z3_ctx, d.ctx.z3_ctx);

        unsafe {
            Z3_params_validate(self.ctx.z3_ctx, self.z3_params, d.z3_param_descrs);
        }
    }
}

impl<'ctx> std::fmt::Display for Params<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = unsafe { Z3_params_to_string(self.ctx.z3_ctx, self.z3_params) };
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

impl<'ctx> Drop for Params<'ctx> {
    fn drop(&mut self) {
        unsafe { Z3_params_dec_ref(self.ctx.z3_ctx, self.z3_params) }
    }
}
