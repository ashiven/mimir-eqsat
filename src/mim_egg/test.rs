use crate::mim_egg::equality_saturate_internal;
use crate::mim_egg::{CostFn, Mim, RuleSet};
use egg::*;

const LINE_LEN: usize = 80;

fn first(rewrites: Vec<RecExpr<Mim>>) -> String {
    rewrites
        .first()
        .expect("rewrites was empty")
        .clone()
        .pretty(LINE_LEN)
}

#[test]
fn fold_core_nat_simple() {
    let sexpr = "(app %core.nat.add (tuple (lit 1 Nat) (lit 1 Nat)))";
    let rewrites = equality_saturate_internal(sexpr, vec![RuleSet::Core], CostFn::AstSize);
    let res = first(rewrites);
    assert_eq!(res, "(lit 2 Nat)");
}

#[test]
fn fold_core_nat_complex() {
    let sexpr = "(app %core.nat.mul (tuple (app %core.nat.add (tuple (lit 1 Nat) (lit 1 Nat))) (app %core.nat.sub (tuple (lit 5 Nat) (lit 4 Nat)))))";
    let rewrites = equality_saturate_internal(sexpr, vec![RuleSet::Core], CostFn::AstSize);
    let res = first(rewrites);
    assert_eq!(res, "(lit 2 Nat)");
}
