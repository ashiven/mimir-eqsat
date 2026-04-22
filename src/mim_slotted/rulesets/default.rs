use crate::mim_slotted::{MimSlotted, analysis::MimSlottedAnalysis};
use slotted_egraphs::Rewrite;

pub fn rules() -> Vec<Rewrite<MimSlotted, MimSlottedAnalysis>> {
    let rules = vec![let_var_same()];

    rules
}

fn let_var_same() -> Rewrite<MimSlotted, MimSlottedAnalysis> {
    let pat = "(let ?def $1 (var $1))";
    let outpat = "?def";
    Rewrite::new("let_var_same", pat, outpat)
}
