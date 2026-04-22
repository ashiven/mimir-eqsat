use crate::mim_slotted::{MimSlotted, analysis::MimSlottedAnalysis};
use slotted_egraphs::Rewrite;

pub fn rules() -> Vec<Rewrite<MimSlotted, MimSlottedAnalysis>> {
    let rules = vec![let_var_same(), core_nat_add0()];

    rules
}

fn let_var_same() -> Rewrite<MimSlotted, MimSlottedAnalysis> {
    let pat = "(let ?def $1 (var $1))";
    let outpat = "?def";
    Rewrite::new("let_var_same", pat, outpat)
}

fn core_nat_add0() -> Rewrite<MimSlotted, MimSlottedAnalysis> {
    let pat = "(app %core.nat.add (tuple (cons (var $1) (cons (lit 0 Nat) nil))))";
    let outpat = "(var $1)";
    Rewrite::new("core_nat_add0", pat, outpat)
}
