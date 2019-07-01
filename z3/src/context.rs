use z3_sys::*;
use Config;
use Context;
use FuncDecl;
use Sort;
use Symbol;
use {Ast, Pattern};

impl Context {
    pub fn new(cfg: &Config) -> Context {
        Context {
            z3_ctx: unsafe {
                let p = Z3_mk_context_rc(cfg.z3_cfg);
                debug!("new context {:p}", p);
                p
            },
        }
    }

    // Helpers for common constructions

    pub fn bool_sort(&self) -> Sort {
        Sort::bool(self)
    }

    pub fn int_sort(&self) -> Sort {
        Sort::int(self)
    }

    pub fn real_sort(&self) -> Sort {
        Sort::real(self)
    }

    pub fn bitvector_sort(&self, sz: u32) -> Sort {
        Sort::bitvector(self, sz)
    }

    pub fn array_sort<'ctx>(&'ctx self, domain: &Sort<'ctx>, range: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::array(self, domain, range)
    }

    pub fn set_sort<'ctx>(&'ctx self, elt: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::set(self, elt)
    }

    /// Create an enumeration sort.
    ///
    /// Creates a Z3 enumeration sort with the given `name`.
    /// The enum variants will have the names in `enum_names`.
    /// Three things are returned:
    /// - the created `Sort`,
    /// - constants to create the variants,
    /// - and testers to check if a value is equal to a variant.
    ///
    /// # Examples
    /// ```
    /// # use z3::{Config, Context, Solver};
    /// # let cfg = Config::new();
    /// # let ctx = Context::new(&cfg);
    /// # let solver = Solver::new(&ctx);
    /// let (colors, color_consts, color_testers) = ctx.enumeration_sort(
    ///     &ctx.str_sym("Color"),
    ///     &[
    ///         &ctx.str_sym("Red"),
    ///         &ctx.str_sym("Green"),
    ///         &ctx.str_sym("Blue"),
    ///     ],
    /// );
    ///
    /// let red_const = color_consts[0].apply(&[]);
    /// let red_tester = &color_testers[0];
    /// let eq = red_tester.apply(&[&red_const]);
    ///
    /// assert!(solver.check());
    /// let model = solver.get_model();
    ///
    /// assert!(model.eval(&eq).unwrap().as_bool().unwrap());
    /// ```
    pub fn enumeration_sort<'ctx>(
        &'ctx self,
        name: &Symbol<'ctx>,
        enum_names: &[&Symbol<'ctx>],
    ) -> (Sort<'ctx>, Vec<FuncDecl<'ctx>>, Vec<FuncDecl<'ctx>>) {
        Sort::enumeration(self, name, enum_names)
    }

    pub fn int_sym(&self, i: u32) -> Symbol {
        Symbol::from_int(self, i)
    }

    pub fn str_sym(&self, s: &str) -> Symbol {
        Symbol::from_string(self, s)
    }

    pub fn named_const<'ctx>(&'ctx self, s: &str, sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::new_const(&self.str_sym(s), sort)
    }

    pub fn numbered_const<'ctx>(&'ctx self, i: u32, sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::new_const(&self.int_sym(i), sort)
    }

    pub fn fresh_const<'ctx>(&'ctx self, prefix: &str, sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::fresh_const(self, prefix, sort)
    }

    pub fn named_bool_const(&self, s: &str) -> Ast {
        Ast::new_const(&self.str_sym(s), &self.bool_sort())
    }

    pub fn numbered_bool_const(&self, i: u32) -> Ast {
        Ast::new_const(&self.int_sym(i), &self.bool_sort())
    }

    pub fn fresh_bool_const<'ctx>(&'ctx self, prefix: &str) -> Ast<'ctx> {
        Ast::fresh_const(self, prefix, &self.bool_sort())
    }

    pub fn named_int_const(&self, s: &str) -> Ast {
        Ast::new_const(&self.str_sym(s), &self.int_sort())
    }

    pub fn numbered_int_const(&self, i: u32) -> Ast {
        Ast::new_const(&self.int_sym(i), &self.int_sort())
    }

    pub fn fresh_int_const<'ctx>(&'ctx self, prefix: &str) -> Ast<'ctx> {
        Ast::fresh_const(self, prefix, &self.int_sort())
    }

    pub fn named_real_const(&self, s: &str) -> Ast {
        Ast::new_const(&self.str_sym(s), &self.real_sort())
    }

    pub fn numbered_real_const(&self, i: u32) -> Ast {
        Ast::new_const(&self.int_sym(i), &self.real_sort())
    }

    pub fn fresh_real_const<'ctx>(&'ctx self, prefix: &str) -> Ast<'ctx> {
        Ast::fresh_const(self, prefix, &self.real_sort())
    }

    pub fn named_bitvector_const(&self, s: &str, sz: u32) -> Ast {
        Ast::new_const(&self.str_sym(s), &self.bitvector_sort(sz))
    }

    pub fn numbered_bitvector_const(&self, i: u32, sz: u32) -> Ast {
        Ast::new_const(&self.int_sym(i), &self.bitvector_sort(sz))
    }

    pub fn fresh_bitvector_const<'ctx>(&'ctx self, prefix: &str, sz: u32) -> Ast<'ctx> {
        Ast::fresh_const(self, prefix, &self.bitvector_sort(sz))
    }

    pub fn from_bool(&self, b: bool) -> Ast {
        Ast::from_bool(self, b)
    }

    pub fn from_u64(&self, u: u64) -> Ast {
        Ast::from_u64(self, u)
    }

    pub fn from_i64(&self, i: i64) -> Ast {
        Ast::from_i64(self, i)
    }

    pub fn from_real(&self, num: i32, den: i32) -> Ast {
        Ast::from_real(self, num, den)
    }

    pub fn func_decl<'ctx>(
        &'ctx self,
        name: Symbol<'ctx>,
        domain: &[&Sort<'ctx>],
        range: &Sort<'ctx>,
    ) -> FuncDecl<'ctx> {
        FuncDecl::new(self, name, domain, range)
    }

    /// Create a forall quantifier.
    ///
    /// # Examples
    /// ```
    /// # use z3::{Config, Context, Solver};
    /// # let cfg = Config::new();
    /// # let ctx = Context::new(&cfg);
    /// # let solver = Solver::new(&ctx);
    /// let f = ctx.func_decl(ctx.str_sym("f"), &[&ctx.int_sort()], &ctx.int_sort());
    ///
    /// let x = ctx.named_int_const("x");
    /// let f_x = f.apply(&[&x]);
    /// solver.assert(&ctx.forall_const(&[&x], &x._eq(&f_x)));
    ///
    /// assert!(solver.check());
    /// let model = solver.get_model();
    ///
    /// let f_f_3 = f.apply(&[&f.apply(&[&ctx.from_u64(3)])]);
    /// assert_eq!(3, model.eval(&f_f_3).unwrap().as_u64().unwrap());
    /// ```
    pub fn forall_const<'ctx>(&'ctx self, bounds: &[&Ast<'ctx>], body: &Ast<'ctx>) -> Ast<'ctx> {
        Ast::forall_const(self, bounds, body)
    }

    pub fn forall_const_weight_patterns<'ctx>(
        &'ctx self,
        weight: usize,
        bounds: &[&Ast<'ctx>],
        patterns: &[&Pattern<'ctx>],
        body: &Ast<'ctx>,
    ) -> Ast<'ctx> {
        Ast::forall_const_weight_patterns(self, weight, bounds, patterns, body)
    }

    pub fn pattern<'ctx>(&'ctx self, terms: &[&Ast<'ctx>]) -> Pattern<'ctx> {
        Pattern::new(self, terms)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.z3_ctx) };
    }
}
