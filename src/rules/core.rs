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

pub fn fold_core_add(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Mim> {
    let c = |id: &Id| egraph[*id].data.clone();
    if let App([callee, arg]) = enode
        && let Some(Symbol(s)) = c(callee)
        && s == "%core.nat.add"
        && let Some(Tuple(t)) = c(arg)
        && let [t1, t2] = &*t
        && let Some(Lit(l1)) = c(t1)
        && let Some(Lit(l2)) = c(t2)
        && let Some(Num(n1)) = c(&l1[0])
        && let Some(Num(n2)) = c(&l2[0])
    {
        return Some(Num(n1 + n2));
    }

    None
}
