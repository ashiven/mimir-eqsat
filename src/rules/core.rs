use crate::rules::*;

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let rules = vec![
        nat_add0(),
        nat_add_same(),
        nat_commute_add(),
        nat_sub0(),
        nat_sub_same(),
        nat_mul0(),
        nat_mul1(),
        nat_commute_mul(),
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
        wrap_add0(),
        wrap_commute_add(),
        wrap_sub0(),
        wrap_mul0(),
        wrap_mul1(),
        wrap_commute_mul(),
        wrap_shl_val0(),
        wrap_shl_amount0(),
        div_sdiv0(),
        div_sdiv1(),
        div_udiv0(),
        div_udiv1(),
        div_srem0(),
        div_srem1(),
        div_urem0(),
        div_urem1(),
    ];

    rules
}

/* core.nat */

fn nat_add0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("nat_add0", pat, outpat).unwrap()
}

fn nat_add_same() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 2) ?a))".parse().unwrap();

    Rewrite::new("nat_add_same", pat, outpat).unwrap()
}

fn nat_commute_add() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.add (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.add (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("nat_commute_add", pat, outpat).unwrap()
}

fn nat_sub0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.sub (tuple ?e (lit 0)))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("nat_sub0", pat, outpat).unwrap()
}

fn nat_sub_same() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.sub (tuple ?a ?a))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit 0)".parse().unwrap();

    Rewrite::new("nat_sub_same", pat, outpat).unwrap()
}

fn nat_mul0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 0) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "(lit 0)".parse().unwrap();

    Rewrite::new("nat_mul0", pat, outpat).unwrap()
}

fn nat_mul1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple (lit 1) ?e))".parse().unwrap();
    let outpat: Pattern<Mim> = "?e".parse().unwrap();

    Rewrite::new("nat_mul1", pat, outpat).unwrap()
}

fn nat_commute_mul() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.nat.mul (tuple ?a ?b))".parse().unwrap();
    let outpat: Pattern<Mim> = "(app %core.nat.mul (tuple ?b ?a))".parse().unwrap();

    Rewrite::new("nat_commute_mul", pat, outpat).unwrap()
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

// TODO: finish today and then look into constant folding some more (math.rs and prop.rs)

/* core.wrap */

fn wrap_add0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.add ?mode) (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("wrap_add0", pat, outpat).unwrap()
}

// TODO: how to get the type for (lit 2 ?type)
// fn wrap_add_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app (app %core.wrap.add ?mode) (tuple ?a ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(app (app %core.wrap.mul ?mode) (tuple ?a (lit 2 ?type))".parse().unwrap();
//
//     Rewrite::new("wrap_add_equal", pat, outpat).unwrap()
// }

fn wrap_commute_add() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.add ?mode) (tuple ?a ?b))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(app (app %core.wrap.add ?mode) (tuple ?b ?a))"
        .parse()
        .unwrap();

    Rewrite::new("wrap_commute_add", pat, outpat).unwrap()
}

fn wrap_sub0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.sub ?mode) (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("wrap_sub0", pat, outpat).unwrap()
}

// TODO: how to get the type for (lit 0 ?type)
// fn wrap_sub_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app (app %core.wrap.sub ?mode) (tuple ?a ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();
//
//     Rewrite::new("wrap_sub_equal", pat, outpat).unwrap()
// }

fn wrap_mul0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.mul ?mode) (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();

    Rewrite::new("wrap_mul0", pat, outpat).unwrap()
}

fn wrap_mul1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.mul ?mode) (tuple ?a (lit 1 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("wrap_mul1", pat, outpat).unwrap()
}

fn wrap_commute_mul() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.mul ?mode) (tuple ?a ?b))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(app (app %core.wrap.mul ?mode) (tuple ?b ?a))"
        .parse()
        .unwrap();

    Rewrite::new("wrap_commute_mul", pat, outpat).unwrap()
}

fn wrap_shl_val0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.shl ?mode) (tuple (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();

    Rewrite::new("wrap_shl_val0", pat, outpat).unwrap()
}

fn wrap_shl_amount0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app (app %core.wrap.shl ?mode) (tuple ?a (lit 0 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "?a".parse().unwrap();

    Rewrite::new("wrap_shl_amount0", pat, outpat).unwrap()
}

/* core.div */

fn div_sdiv0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.sdiv (tuple ?mem (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_sdiv0", pat, outpat).unwrap()
}

fn div_udiv0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.udiv (tuple ?mem (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_udiv0", pat, outpat).unwrap()
}

fn div_srem0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.srem (tuple ?mem (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_srem0", pat, outpat).unwrap()
}

fn div_urem0() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.urem (tuple ?mem (lit 0 ?type) ?a))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_urem0", pat, outpat).unwrap()
}

fn div_sdiv1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.sdiv (tuple ?mem ?a (lit 1 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem ?a)".parse().unwrap();

    Rewrite::new("div_sdiv1", pat, outpat).unwrap()
}

fn div_udiv1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.udiv (tuple ?mem ?a (lit 1 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 1 ?type))".parse().unwrap();

    Rewrite::new("div_udiv1", pat, outpat).unwrap()
}

fn div_srem1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.srem (tuple ?mem ?a (lit 1 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_srem1", pat, outpat).unwrap()
}

fn div_urem1() -> Rewrite<Mim, MimAnalysis> {
    let pat: Pattern<Mim> = "(app %core.div.urem (tuple ?mem ?a (lit 1 ?type)))"
        .parse()
        .unwrap();
    let outpat: Pattern<Mim> = "(tuple ?mem (lit 0 ?type))".parse().unwrap();

    Rewrite::new("div_urem1", pat, outpat).unwrap()
}

// TODO: figure out how to get ?type for the output literal
// fn div_sdiv_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app %core.div.sdiv (tuple ?mem (lit 0 ?type) ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();
//
//     Rewrite::new("div_sdiv0", pat, outpat).unwrap()
// }
//
// fn div_udiv_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app %core.div.udiv (tuple ?mem (lit 0 ?type) ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();
//
//     Rewrite::new("div_udiv0", pat, outpat).unwrap()
// }
//
// fn div_srem_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app %core.div.srem (tuple ?mem (lit 0 ?type) ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();
//
//     Rewrite::new("div_srem0", pat, outpat).unwrap()
// }
//
// fn div_urem_equal() -> Rewrite<Mim, MimAnalysis> {
//     let pat: Pattern<Mim> = "(app %core.div.urem (tuple ?mem (lit 0 ?type) ?a))"
//         .parse()
//         .unwrap();
//     let outpat: Pattern<Mim> = "(lit 0 ?type)".parse().unwrap();
//
//     Rewrite::new("div_urem0", pat, outpat).unwrap()
// }

/* constant folding */

pub fn fold_core(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Const> {
    if let Lit(l) = enode
        && let Some(n) = find_node!(egraph, &l[0], Num(n) => n)
    {
        // We have a typed literal like (lit 4 I8)
        if l.len() == 2
            && let Some(t) = egraph[l[1]].nodes.first()
        {
            return Some(Const {
                val: Some(Num(*n)),
                type_: Some(t.clone()),
            });
        }

        // We have an untyped literal like (lit 5)
        return Some(Const {
            val: Some(Num(*n)),
            type_: None,
        });
    }

    if let Some(folded) = fold_nat(egraph, enode) {
        return Some(folded);
    } else if let Some(folded) = fold_icmp(egraph, enode) {
        return Some(folded);
    }

    None
}

fn fold_nat(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Const> {
    let c = |id: &Id| egraph[*id].data.constant.clone();

    if let App([callee, arg]) = enode
        && let Some(s) = find_node!(egraph, callee, Symbol(s) => s)
        && let Some(t) = find_node!(egraph, arg, Tuple(t) => t)
        && let [t1, t2] = &**t
        && let Some(Num(n1)) = c(t1)?.val
        && let Some(Num(n2)) = c(t2)?.val
    {
        match s.as_str() {
            "%core.nat.add" => {
                return Some(Const {
                    val: Some(Num(n1 + n2)),
                    type_: None,
                });
            }
            // TODO: this can lead to negative numbers (cap at zero?)
            "%core.nat.sub" => {
                return Some(Const {
                    val: Some(Num(n1 - n2)),
                    type_: None,
                });
            }
            "%core.nat.mul" => {
                return Some(Const {
                    val: Some(Num(n1 * n2)),
                    type_: None,
                });
            }
            _ => (),
        }
    }

    None
}

// TODO: implement:
//  - fold_icmp
//  - fold_ncmp
//  - fold_shr
//  - fold_wrap
//  - fold_div

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
fn fold_icmp(egraph: &mut EGraph<Mim, MimAnalysis>, enode: &Mim) -> Option<Const> {
    let c = |id: &Id| egraph[*id].data.constant.clone();

    if let App([callee, arg]) = enode
        && let Some(s) = find_node!(egraph, callee, Symbol(s) => s)
        && let Some(t) = find_node!(egraph, callee, Tuple(t) => t)
        && let [t1, t2] = &**t
        && let Some(Num(n1)) = c(t1)?.val
        && let Some(Num(n2)) = c(t2)?.val
    {
        match s.as_str() {
            "%core.icmp.Xygle" => (), // x positive, y negative
            "%core.icmp.xYgle" => (), // x negative, y positive
            "%core.icmp.xyGle" => (), // greater, same sign
            "%core.icmp.xygLe" => (), // less, same sign
            "%core.icmp.xyglE" => (), // equal (alias %core.icmp.e)
            _ => (),
        }
    }

    None
}
