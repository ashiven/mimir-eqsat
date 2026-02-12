use crate::rules::*;

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let rules = vec![add0(), sub0(), mul0(), mul1(), commute_add(), commute_mul()];

    rules
}

fn add0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("add0", pat, outpat).unwrap()
}

fn sub0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.sub (tuple ?e (lit 0)))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("sub0", pat, outpat).unwrap()
}

fn mul0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit 0)".parse().unwrap();

    Rewrite::new("mul0", pat, outpat).unwrap()
}

fn mul1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 1) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("mul1", pat, outpat).unwrap()
}

fn commute_add() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.add (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("commute_add", pat, outpat).unwrap()
}

fn commute_mul() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.mul (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("commute_mul", pat, outpat).unwrap()
}

// TODO: fold_core should be further split up into:
// fold_nat
//  -> fold_nat_lit
//  -> fold_nat_zero
// fold_icmp
//  -> fold_
//  -> fold_
//
// etc.
pub fn fold_core(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Mim> {
    if let Some(folded) = fold_nat(egraph, enode) {
        return Some(folded);
    }

    None
}

fn fold_nat(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Mim> {
    let c = |id: &Id| egraph[*id].data.clone();
    if let App([callee, arg]) = enode
        && let Some(Symbol(s)) = c(callee)
        && let Some(Tuple(t)) = c(arg)
        && let [t1, t2] = &*t
        && let Some(Lit(l1)) = c(t1)
        && let Some(Lit(l2)) = c(t2)
        && let Some(Num(n1)) = c(&l1[0])
        && let Some(Num(n2)) = c(&l2[0])
    {
        match s.as_str() {
            "%core.nat.add" => return Some(Num(n1 + n2)),
            "%core.nat.sub" => return Some(Num(n1 - n2)),
            "%core.nat.mul" => return Some(Num(n1 * n2)),
            _ => return None,
        }
    }

    None
}
