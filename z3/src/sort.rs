use std::convert::TryInto;
use std::ffi::CStr;
use std::fmt;
use z3_sys::*;
use Ast;
use Context;
use FuncDecl;
use Sort;
use Symbol;

impl<'ctx> Sort<'ctx> {
    pub fn uninterpreted(ctx: &'ctx Context, sym: &Symbol<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx,
            z3_sort: unsafe { Z3_mk_uninterpreted_sort(ctx.z3_ctx, sym.z3_sym) },
        }
    }

    pub fn bool(ctx: &Context) -> Sort {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_bool_sort(ctx.z3_ctx);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn int(ctx: &Context) -> Sort {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_int_sort(ctx.z3_ctx);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn real(ctx: &Context) -> Sort {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_real_sort(ctx.z3_ctx);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn bitvector(ctx: &Context, sz: u32) -> Sort {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_bv_sort(ctx.z3_ctx, sz as ::std::os::raw::c_uint);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn array(ctx: &'ctx Context, domain: &Sort<'ctx>, range: &Sort<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_array_sort(ctx.z3_ctx, domain.z3_sort, range.z3_sort);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn set(ctx: &'ctx Context, elt: &Sort<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_set_sort(ctx.z3_ctx, elt.z3_sort);
                Z3_inc_ref(ctx.z3_ctx, Z3_sort_to_ast(ctx.z3_ctx, s));
                s
            },
        }
    }

    pub fn enumeration(
        ctx: &'ctx Context,
        name: &Symbol<'ctx>,
        enum_names: &[&Symbol<'ctx>],
    ) -> (Sort<'ctx>, Vec<FuncDecl<'ctx>>, Vec<FuncDecl<'ctx>>) {
        assert_eq!(ctx.z3_ctx, name.ctx.z3_ctx);
        assert!(enum_names.iter().all(|s| s.ctx.z3_ctx == ctx.z3_ctx));

        let enum_names: Vec<_> = enum_names.iter().map(|s| s.z3_sym).collect();
        let mut enum_consts = vec![std::ptr::null_mut(); enum_names.len()];
        let mut enum_testers = vec![std::ptr::null_mut(); enum_names.len()];

        let sort = Sort {
            ctx,
            z3_sort: unsafe {
                let s = Z3_mk_enumeration_sort(
                    ctx.z3_ctx,
                    name.z3_sym,
                    enum_names.len().try_into().unwrap(),
                    enum_names.as_ptr(),
                    enum_consts.as_mut_ptr(),
                    enum_testers.as_mut_ptr(),
                );
                Z3_inc_ref(ctx.z3_ctx, s as Z3_ast);
                s
            },
        };

        // increase ref counts
        for i in &enum_consts {
            unsafe {
                Z3_inc_ref(ctx.z3_ctx, *i as Z3_ast);
            }
        }
        for i in &enum_testers {
            unsafe {
                Z3_inc_ref(ctx.z3_ctx, *i as Z3_ast);
            }
        }

        // convert to Rust types
        let enum_consts: Vec<_> = enum_consts
            .iter()
            .map(|z3_func_decl| FuncDecl {
                ctx,
                z3_func_decl: *z3_func_decl,
            })
            .collect();
        let enum_testers: Vec<_> = enum_testers
            .iter()
            .map(|z3_func_decl| FuncDecl {
                ctx,
                z3_func_decl: *z3_func_decl,
            })
            .collect();

        (sort, enum_consts, enum_testers)
    }

    /// Converts an unsigned integer to an `Ast` of the given `Sort`
    ///
    /// `self` must be an integer, bit-vector, or finite-domain sort.
    ///
    /// # Example
    ///
    /// This example checks that bitvector arithmetic is wrapping
    /// by solving for an 8-bit value `x` such that `x + 1 == 0`.
    ///
    /// ```
    /// use z3::*;
    ///
    /// let cfg = Config::new();
    /// let ctx = Context::new(&cfg);
    ///
    /// let x = ctx.named_bitvector_const("b", 8);
    /// let bv_sort = ctx.bitvector_sort(8);
    ///
    /// let solver = Solver::new(&ctx);
    /// solver.assert(&x.bvadd(&bv_sort.from_u64(1))
    ///     ._eq(&bv_sort.from_u64(0)));
    /// assert!(solver.check());
    ///
    /// let model = solver.get_model();
    /// assert!(model.eval(&x).and_then(|i| i.as_u64()).unwrap_or(0) == 0xFF);
    /// ```
    pub fn from_u64(&self, u: u64) -> Ast<'ctx> {
        Ast::new(self.ctx, unsafe {
            Z3_mk_unsigned_int64(self.ctx.z3_ctx, u, self.z3_sort)
        })
    }

    /// Converts a signed integer to an `Ast` of the given `Sort`
    ///
    /// `self` must be an integer, bit-vector, or finite-domain sort.
    ///
    /// # Example
    ///
    /// Checking that multiplication of signed 8-bit values overflows mod 256
    ///
    /// ```
    /// use z3::*;
    ///
    /// let cfg = Config::new();
    /// let ctx = Context::new(&cfg);
    ///
    /// let bv_sort = ctx.bitvector_sort(8);
    ///
    /// let solver = Solver::new(&ctx);
    /// solver.assert(&bv_sort.from_i64(-5)
    ///     .bvmul(&bv_sort.from_i64(-100))
    ///     ._eq(&bv_sort.from_i64((-5 * -100) % 256)));
    /// assert!(solver.check());
    /// ```
    pub fn from_i64(&self, i: i64) -> Ast<'ctx> {
        Ast::new(self.ctx, unsafe {
            Z3_mk_int64(self.ctx.z3_ctx, i, self.z3_sort)
        })
    }
}

impl<'ctx> fmt::Display for Sort<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let p =
            unsafe { CStr::from_ptr(Z3_sort_to_string(self.ctx.z3_ctx, self.z3_sort) as *mut i8) };
        if p.as_ptr().is_null() {
            return Result::Err(fmt::Error);
        }
        match p.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Result::Err(fmt::Error),
        }
    }
}

impl<'ctx> PartialEq<Sort<'ctx>> for Sort<'ctx> {
    fn eq(&self, other: &Sort<'ctx>) -> bool {
        unsafe { Z3_is_eq_sort(self.ctx.z3_ctx, self.z3_sort, other.z3_sort) }
    }
}

impl<'ctx> Eq for Sort<'ctx> {}

impl<'ctx> Drop for Sort<'ctx> {
    fn drop(&mut self) {
        unsafe {
            Z3_dec_ref(
                self.ctx.z3_ctx,
                Z3_sort_to_ast(self.ctx.z3_ctx, self.z3_sort),
            );
        }
    }
}
