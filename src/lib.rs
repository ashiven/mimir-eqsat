use ffi::bridge::{CostFn, NodeFFI, RecExprFFI, RuleSet};

pub mod ffi;
mod mim_egg;
mod mim_slotted;

pub fn equality_saturate(sexpr: &str, rulesets: Vec<RuleSet>, cost_fn: CostFn) -> Vec<RecExprFFI> {
    mim_egg::equality_saturate_ffi(sexpr, rulesets, cost_fn)
}

pub fn pretty(sexpr: &str, line_len: usize) -> String {
    mim_egg::pretty(sexpr, line_len)
}

pub fn equality_saturate_slotted(
    sexpr: &str,
    rulesets: Vec<RuleSet>,
    cost_fn: CostFn,
) -> Vec<RecExprFFI> {
    mim_slotted::equality_saturate_ffi(sexpr, rulesets, cost_fn)
}

pub fn pretty_slotted(sexpr: &str, line_len: usize) -> String {
    mim_slotted::pretty(sexpr, line_len)
}

pub fn node_ffi_str(node: NodeFFI) -> String {
    format!("{:?}", node)
}

pub fn pretty_ffi(sexprs: Vec<RecExprFFI>, line_len: usize) -> String {
    ffi::pretty_ffi(sexprs, line_len)
}
