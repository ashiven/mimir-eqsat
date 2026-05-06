use crate::mim_slotted::{MimSlotted, analysis::MimSlottedAnalysis};
use slotted_egraphs::{AbstractVecSet, Rewrite, Slot};

type RW = Rewrite<MimSlotted, MimSlottedAnalysis>;

pub fn rules() -> Vec<RW> {
    let rules = vec![
        beta(),
        eta(),
        eta_expansion(),
        let_unused(),
        let_var_same(),
        let_var_diff(),
        let_app(),
        let_app_unopt(),
        let_lam_diff(),
        let_lam_diff_unopt(),
    ];

    rules
}

fn beta() -> RW {
    let pat = "(app (con ?dom-type $var ?body) ?e)";
    let outpat = "(let var$ (scope ?e ?body))";
    Rewrite::new("beta", pat, outpat)
}

fn eta() -> RW {
    let pat = "(con ?dom-type $var (scope ?filter (app ?fn (var $var))))";
    let outpat = "?fn";

    // On condition that $var is not bound in ?fn because this makes
    // the definiton of the function ?fn dependent on the var of the con
    // we are trying to eta-reduce away.
    Rewrite::new_if("eta", pat, outpat, |subst, _| {
        !subst["fn"].slots().contains(&Slot::named("var"))
    })
}

fn eta_expansion() -> RW {
    let pat = "?fn";
    // TODO: How do we infer the dom-type of the con?
    // - an analysis that stores types on eclasses and finds the type of ?fn ?
    let outpat = "(con DOM-TYPE $var (scope (lit ff Bool) (app ?fn (var $var))))";
    Rewrite::new("eta-expansion", pat, outpat)
}

fn let_unused() -> RW {
    let pat = "(let $name (scope ?def ?expr))";
    let outpat = "?expr";
    Rewrite::new_if("let-unused", pat, outpat, |subst, _| {
        !subst["expr"].slots().contains(&Slot::named("name"))
    })
}

fn let_var_same() -> RW {
    let pat = "(let $name (scope ?def (var $name))";
    let outpat = "?def";
    Rewrite::new("let-var-same", pat, outpat)
}

fn let_var_diff() -> RW {
    let pat = "(let $name (scope ?def (var $other)))";
    let outpat = "(var $other)";
    Rewrite::new("let-var-diff", pat, outpat)
}

fn let_app() -> RW {
    let pat = "(let $name (scope ?def (app ?a ?b))";
    let outpat = "(app (let $name (scope ?def ?a)) (let $name (scope ?def ?b)))";
    Rewrite::new_if("let-app", pat, outpat, |subst, _| {
        subst["a"].slots().contains(&Slot::named("name"))
            || subst["b"].slots().contains(&Slot::named("name"))
    })
}

fn let_app_unopt() -> RW {
    let pat = "(let $name (scope ?def (app ?a ?b))";
    let outpat = "(app (let $name (scope ?def ?a)) (let $name (scope ?def ?b)))";
    Rewrite::new("let-app", pat, outpat)
}

fn let_lam_diff() -> RW {
    let pat = "(let $name (scope ?def (con ?dom-type $var (scope ?filter ?body))))";
    let outpat = "(con ?dom-type $var (scope ?filter (let $name (scope ?def ?body))))";
    Rewrite::new_if("let-lam-diff", pat, outpat, |subst, _| {
        subst["body"].slots().contains(&Slot::named("name"))
    })
}

fn let_lam_diff_unopt() -> RW {
    let pat = "(let $name (scope ?def (con ?dom-type $var (scope ?filter ?body))))";
    let outpat = "(con ?dom-type $var (scope ?filter (let $name (scope ?def ?body))))";
    Rewrite::new("let-lam-diff", pat, outpat)
}
