use crate::mim_slotted::{MimSlotted, analysis::MimSlottedAnalysis};
use slotted_egraphs::{AbstractVecSet, Rewrite, Slot};

type RW = Rewrite<MimSlotted, MimSlottedAnalysis>;

pub fn rules() -> Vec<RW> {
    let rules = vec![
        // EVAL
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
        // RISE
        map_fusion(),
        map_fission(),
        double_transpose(),
    ];

    rules
}

fn beta() -> RW {
    let pat = "(app (con ?dom-type $dom (scope ?filter ?body)) ?e)";
    let outpat = "(let var$ (scope ?e ?body))";
    Rewrite::new("beta", pat, outpat)
}

fn eta() -> RW {
    let pat = "(con ?dom-type $dom (scope ?filter (app ?fn (var $dom))))";
    let outpat = "?fn";

    // On condition that $dom is not bound in ?fn because this makes
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
    let outpat = "(con DOM-TYPE $dom (scope (lit ff Bool) (app ?fn (var $dom))))";
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
    let pat = "(let $name (scope ?def (con ?dom-type $dom (scope ?filter ?body))))";
    let outpat = "(con ?dom-type $dom (scope ?filter (let $name (scope ?def ?body))))";
    Rewrite::new_if("let-lam-diff", pat, outpat, |subst, _| {
        subst["body"].slots().contains(&Slot::named("name"))
    })
}

fn let_lam_diff_unopt() -> RW {
    let pat = "(let $name (scope ?def (con ?dom-type $dom (scope ?filter ?body))))";
    let outpat = "(con ?dom-type $dom (scope ?filter (let $name (scope ?def ?body))))";
    Rewrite::new("let-lam-diff", pat, outpat)
}

// TODO:
// fun let_const() -> RW {...}

// ((map f) ((map g) arg)) => ((map λx.(f (g x))) arg)
fn map_fusion() -> RW {
    let pat = "(app (app %rise.map ?f) (app (app %rise.map ?g) ?arg))";
    let outpat =
        "(app (app %rise.map (con ?x-type $x (scope ?filter (app ?f (app ?g (var $x)))))) ?arg)";
    Rewrite::new("map-fusion", pat, outpat)
}

// (map λx.(f (g x))) => λy.((map f) ((map λx.(g x)) y))
fn map_fission() -> RW {
    let pat = "(app %rise.map (con ?x-type $x (scope ?filter (app ?f ?gx))))";
    // TODO: What do we write for y-type?
    let outpat = "
(con y-type $y 
    (scope
        (lit ff Bool)
        (app
            (app %rise.map ?f)
            (app
                (app %rise.map (con ?x-type $x (scope ?filter ?gx)))
                (var $y)))))";
    Rewrite::new_if("map-fission", pat, outpat, |subst, _| {
        !subst["f"].slots().contains(&Slot::named("x"))
    })
}

fn double_transpose() -> RW {
    let pat = "(app %rise.transpose (app %rise.transpose ?arg))";
    let outpat = "?arg";
    Rewrite::new("double-transpose", pat, outpat)
}
