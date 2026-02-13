use crate::rules::*;

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let rules = vec![
        add0(),
        add_same(),
        commute_add(),
        sub0(),
        sub_same(),
        mul0(),
        mul1(),
        commute_mul(),
        icmp_equal(),
        icmp_not_equal(),
        icmp_true(),
        icmp_false(),
        ncmp_equal(),
        ncmp_not_equal(),
        ncmp_true(),
        ncmp_false(),
        shr_arith_amount0(),
        shr_arith_val0(),
        shr_logical_amount0(),
        shr_logical_val0(),
    ];

    rules
}

/* core.nat */

fn add0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("add0", pat, outpat).unwrap()
}

fn add_same() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 2) ?a))".parse().unwrap();

    Rewrite::new("add_same", pat, outpat).unwrap()
}

fn commute_add() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.add (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("commute_add", pat, outpat).unwrap()
}

fn sub0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.sub (tuple ?e (lit 0)))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("sub0", pat, outpat).unwrap()
}

fn sub_same() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.sub (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit 0)".parse().unwrap();

    Rewrite::new("sub_same", pat, outpat).unwrap()
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

fn commute_mul() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.mul (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("commute_mul", pat, outpat).unwrap()
}

/* core.icmp */

fn icmp_equal() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.icmp.e (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit tt)".parse().unwrap();

    Rewrite::new("icmp_equal", pat, outpat).unwrap()
}

fn icmp_not_equal() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.icmp.ne (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit ff)".parse().unwrap();

    Rewrite::new("icmp_not_equal", pat, outpat).unwrap()
}

fn icmp_true() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.icmp.t (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit tt)".parse().unwrap();

    Rewrite::new("icmp_true", pat, outpat).unwrap()
}

fn icmp_false() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.icmp.f (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit ff)".parse().unwrap();

    Rewrite::new("icmp_false", pat, outpat).unwrap()
}

/* core.ncmp */

fn ncmp_equal() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.ncmp.e (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit tt)".parse().unwrap();

    Rewrite::new("ncmp_equal", pat, outpat).unwrap()
}

fn ncmp_not_equal() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.ncmp.ne (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit ff)".parse().unwrap();

    Rewrite::new("ncmp_not_equal", pat, outpat).unwrap()
}

fn ncmp_true() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.ncmp.t (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit tt)".parse().unwrap();

    Rewrite::new("ncmp_true", pat, outpat).unwrap()
}

fn ncmp_false() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.ncmp.f (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit ff)".parse().unwrap();

    Rewrite::new("ncmp_false", pat, outpat).unwrap()
}

// TODO:
/* core.bit1 */
/* core.bit2 */

/* core.shr */

fn shr_arith_val0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.shr.a (tuple (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();

    Rewrite::new("shr_arith_val0", pat, outpat).unwrap()
}

fn shr_arith_amount0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.shr.a (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("shr_arith_amount0", pat, outpat).unwrap()
}

fn shr_logical_val0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.shr.l (tuple (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();

    Rewrite::new("shr_logical_val0", pat, outpat).unwrap()
}

fn shr_logical_amount0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.shr.l (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("shr_logical_amount0", pat, outpat).unwrap()
}

pub fn fold_core(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Mim> {
    if let Some(folded) = fold_nat(egraph, enode) {
        return Some(folded);
    } else if let Some(folded) = fold_icmp(egraph, enode) {
        return Some(folded);
    }

    None
}

fn fold_nat(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Mim> {
    let c = |id: &Id| egraph[*id].data.constant.clone();
    // TODO: we are not entering the below branch because c(arg) and c(t1), c(t2) will return
    // nothing since no data has been associated with eclasses containing tuple enodes
    // and literal enodes
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

/*
* cases 3 and 4 (xyGle and xygLe) implement less than and greater than
* when integers are not represented as two's complement binary where u > v
* could actually be true if u represents a negative number and v a positive one (xFF and x01)
*
* i suppose if integers are represented as two's complement then the cases 1 and 2 should be used
*
/    bool res = false;
/    auto plusminus  = !(u >> UT(w - 1)) &&  (v >> UT(w - 1));   // u pos and v neg
/    auto minusplus  =  (u >> UT(w - 1)) && !(v >> UT(w - 1));   // u neg and v pos
/    res |= ((id & icmp::Xygle) != icmp::f) && plusminus;  // is u pos and v neg ?
/    res |= ((id & icmp::xYgle) != icmp::f) && minusplus;  // is u neg and v pos ?
/    res |= ((id & icmp::xyGle) != icmp::f) && u > v && !minusplus;  // is u greater than v
/    res |= ((id & icmp::xygLe) != icmp::f) && u < v && !plusminus;  // is u less than v
/    res |= ((id & icmp::xyglE) != icmp::f) && u == v; // is u equal to v
/    return res;
*/
// TODO: implement
fn fold_icmp(_egraph: &mut EGraph<Mim, MimAnalysis>, _enode: &Mim) -> Option<Mim> {
    None
}
