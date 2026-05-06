use crate::mim_slotted::{MimSlotted, analysis::MimSlottedAnalysis};
use slotted_egraphs::{AbstractVecSet, Rewrite, Slot};

pub fn rules() -> Vec<Rewrite<MimSlotted, MimSlottedAnalysis>> {
    let rules = vec![beta(), eta()];

    rules
}

fn beta() -> Rewrite<MimSlotted, MimSlottedAnalysis> {
    let pat = "(app (con ?dom-type $var ?body) ?e)";
    let outpat = "(let var$ (scope ?e ?body))";
    Rewrite::new("beta", pat, outpat)
}

fn eta() -> Rewrite<MimSlotted, MimSlottedAnalysis> {
    let pat = "(con ?dom-type $var (scope ?filter (app ?fn (var $var))))";
    let outpat = "?fn";

    // On condition that $var is not bound in ?fn because this makes
    // the definiton of the function ?fn dependent on the var of the con
    // we are trying to eta-reduce away.
    Rewrite::new_if("eta", pat, outpat, |subst, _| {
        !subst["fn"].slots().contains(&Slot::named("var"))
    })
}
