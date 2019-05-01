#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const Z3_TRUE: bool = true;
pub const Z3_FALSE: bool = false;

pub use Z3_ast_kind::*;
pub use Z3_ast_print_mode::*;
pub use Z3_decl_kind::*;
pub use Z3_error_code::*;
pub use Z3_goal_prec::*;
pub use Z3_lbool::*;
pub use Z3_param_kind::*;
pub use Z3_parameter_kind::*;
pub use Z3_sort_kind::*;
