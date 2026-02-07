use crate::rules::*;

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let mut rules = Vec::new();

    rules.push(add0());
    rules.push(commute_add());

    rules
}

fn add0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("add0", pat, outpat).unwrap()
}

fn commute_add() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.add (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("commute_add", pat, outpat).unwrap()
}
