use crate::rules::Mim;
use egg::*;

pub fn rules() -> Vec<Rewrite<Mim, ()>> {
    let mut rules = Vec::new();

    // rules.push(add0());

    rules
}

// fn add0() -> Rewrite<Mim, ()> {
//     let pat: Pattern<Mim> = "(app %core.nat.add (tuple (lit 0) ?e))".parse().unwrap();
//     let outpat: Pattern<Mim> = "?e".parse().unwrap();
//
//     Rewrite::new("add0", pat, outpat).unwrap()
// }
