use regex::Regex;
use std::fs;

use crate::ffi::bridge::{CostFn, RuleSet};
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use crate::mim_slotted::convert_rules;
use crate::mim_slotted::get_rules;
use crate::mim_slotted::types::{add_expr_typed, extract_type_annotations};
use crate::mim_slotted::{MimSlotted, split_sexprs};
use crate::{eqsat_slotted, pretty_ffi};
use slotted_egraphs::*;

fn parse_sexprs(sexpr: &str) -> Vec<RecExpr<MimSlotted>> {
    let sexprs = split_sexprs(sexpr);

    let mut res = vec![];
    for sexpr in sexprs {
        res.push(RecExpr::parse(&sexpr).expect("Failed to parse RecExpr"));
    }
    res
}

fn eqsat_equals(file: &str, file_rw: &str) {
    let slotted = fs::read_to_string(file).expect("Failed to read file.slotted");
    let nodes = eqsat_slotted(&slotted, vec![], CostFn::AstSize);

    let slotted = pretty_ffi(nodes, LINE_LEN);
    let slotted_rw = fs::read_to_string(file_rw)
        .expect("Failed to read file_rw.slotted")
        .replace("\r\n", "\n");

    let slot_re = Regex::new(r"\$[_A-Za-z0-9]+").unwrap();
    let slotted = slot_re.replace_all(&slotted, "slot");
    let slotted_rw = slot_re.replace_all(&slotted_rw, "slot");

    assert_eq!(slotted, slotted_rw);
}

const LINE_LEN: usize = 80;

#[test]
fn get_ruleset_standard() {
    let standard = get_rules(vec![RuleSet::Standard]);
    assert_ne!(standard.len(), 0);
}

#[test]
fn let_var_same() {
    let a = "(let $foo (scope (lit 1 Nat) (var $foo)))";
    let b = "(lit 1 Nat)";
    assert_reaches::<MimSlotted, MimSlottedAnalysis>(a, b, &get_rules(vec![RuleSet::Standard]), 1);
}

#[test]
fn lam_var_add0() {
    let a = "(root extern foo (lam $x (scope (lit ff Bool) (app %core.nat.add (tuple (cons (var $x) (cons (lit 0 Nat) nil)))))))";
    let b = "(root extern foo (lam $x (scope (lit ff Bool) (var $x))))";
    assert_reaches::<MimSlotted, MimSlottedAnalysis>(a, b, &get_rules(vec![RuleSet::Standard]), 1);
}

#[test]
fn parse_loop_slotted() {
    let loop_slotted =
        fs::read_to_string("examples/loop.slotted").expect("Failed to read loop.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&loop_slotted);
}

#[test]
fn eqsat_loop_slotted() {
    eqsat_equals("examples/loop.slotted", "examples/loop_rw.slotted");
}

#[test]
fn parse_import_slotted() {
    let import_slotted =
        fs::read_to_string("examples/import.slotted").expect("Failed to read import.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&import_slotted);
}

#[test]
fn eqsat_import_slotted() {
    eqsat_equals("examples/import.slotted", "examples/import_rw.slotted");
}

#[test]
fn parse_fun_slotted() {
    let fun_slotted =
        fs::read_to_string("examples/fun.slotted").expect("Failed to read fun.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&fun_slotted);
}

#[test]
fn eqsat_fun_slotted() {
    eqsat_equals("examples/fun.slotted", "examples/fun_rw.slotted");
}

#[test]
#[ignore]
fn parse_pow_slotted() {
    let pow_slotted =
        fs::read_to_string("examples/pow.slotted").expect("Failed to read pow.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&pow_slotted);
}

#[test]
#[ignore]
fn eqsat_pow_slotted() {
    eqsat_equals("examples/pow.slotted", "examples/pow_rw.slotted");
}

#[test]
fn convert_custom_rule() {
    let rule = "
    (rule 
        foo
        (cons
            (metavar
                pat_a_22735
                Nat)
        (cons
            (metavar
                slot_b_22734
                Nat)
        nil))
        (app
            %core.nat.add
            (tuple
                (cons
                    (app
                        %core.nat.sub
                        (tuple
                            (cons
                                slot_b_22734
                            (cons
                                pat_a_22735
                            nil))))
                (cons
                    pat_a_22735
                nil))))
        slot_b_22734
        (lit tt Bool))";

    let mut sexprs = vec![rule.to_string()];
    let mut rules = Vec::new();
    convert_rules(&mut sexprs, &mut rules);

    assert_eq!(rules.len(), 1);
}

#[test]
fn extract_type_info() {
    let annotated = "
    (root extern add_lit
        (@ (cn (cn I8))
        (lam
            $return_22296
            (scope
                (@ Bool
                (lit ff Bool))
                (@ (bot (type (lit 0 Univ)))
                (app
                    (@ (cn I8)
                    (var $return_22296))
                    (@ I8
                    (lit 6 I8))))))))";

    let annotated: RecExpr<MimSlotted> = RecExpr::parse(annotated).unwrap();
    let typed = extract_type_annotations(&annotated);

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();
    let typed_id = add_expr_typed(&mut eg, typed);

    let enodes = eg.enodes_applied(&typed_id);
    let typed = enodes
        .first()
        .expect("Failed to find typed rec expr in egraph");

    let lam_type = eg
        .analysis_data(typed.applied_id_occurrences()[2].id)
        .type_
        .clone();

    assert_eq!(lam_type, RecExpr::parse("(cn (cn I8))").unwrap());
}

#[test]
fn make_eta_expansion_hole() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let fun_annotated = "(@ (pi Nat Bool) fun)";
    let fun_annotated: RecExpr<MimSlotted> = RecExpr::parse(fun_annotated).unwrap();
    let fun_typed = extract_type_annotations(&fun_annotated);
    let fun_typed_id = add_expr_typed(&mut eg, fun_typed);

    let eta_exp = "(lam $x (scope (lit ff Bool) (app fun (var $x))))";
    let eta_exp: RecExpr<MimSlotted> = RecExpr::parse(eta_exp).unwrap();
    let eta_exp_id = eg.add_expr(eta_exp);

    assert_eq!(type_of(&eg, fun_typed_id), type_("(pi Nat Bool)"));
    assert_eq!(
        type_of(&eg, eta_exp_id),
        type_("(pi (hole (type (lit 0 Univ))) Bool)")
    );
}

#[test]
fn make_types_var_lit() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let lit = "(lit 10 (idx 3))";
    let lit: RecExpr<MimSlotted> = RecExpr::parse(lit).unwrap();
    let lit_id = eg.add_expr(lit);

    assert_eq!(type_of(&eg, lit_id), type_("(idx 3)"));

    let binding = "(let $x (scope (lit tt Bool) (app (lam $y (scope (lit ff Bool) (lit 10 (idx 3)))) (var $x))))";
    let binding: RecExpr<MimSlotted> = RecExpr::parse(binding).unwrap();
    let binding_id = eg.add_expr(binding);

    assert_eq!(type_of(&eg, binding_id), type_("(idx 3)"));

    let var = "(var $foo)";
    let var: RecExpr<MimSlotted> = RecExpr::parse(var).unwrap();
    let var_id = eg.add_expr(var);

    assert_eq!(type_of(&eg, var_id), type_("(hole (type (lit 0 Univ)))"));

    let app = "(app (var $foo) (var $bar))";
    let app: RecExpr<MimSlotted> = RecExpr::parse(app).unwrap();
    let app_id = eg.add_expr(app);

    assert_eq!(type_of(&eg, app_id), type_("(hole (type (lit 0 Univ)))"));
}

#[test]
fn make_types_tuple_pack() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let tuple = "(tuple (cons (lit 1 Nat) (cons (lit 2 Nat) (cons (lit 3 Nat) nil))))";
    let tuple: RecExpr<MimSlotted> = RecExpr::parse(tuple).unwrap();
    let tuple_id = eg.add_expr(tuple);

    assert_eq!(
        type_of(&eg, tuple_id),
        type_("(sigma (cons Nat (cons Nat (cons Nat nil))))")
    );

    let tuple_empty = "(tuple nil)";
    let tuple_empty: RecExpr<MimSlotted> = RecExpr::parse(tuple_empty).unwrap();
    let tuple_empty_id = eg.add_expr(tuple_empty);

    assert_eq!(type_of(&eg, tuple_empty_id), type_("(sigma nil)"));

    let pack = "(pack (top Nat) (lit 3 Nat))";
    let pack: RecExpr<MimSlotted> = RecExpr::parse(pack).unwrap();
    let pack_id = eg.add_expr(pack);

    assert_eq!(type_of(&eg, pack_id), type_("(arr (top Nat) Nat)"));
}

#[test]
fn make_types_extract_insert() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let insert_tuple =
        "(insert (tuple (cons (lit 1 Nat) (cons (lit 2 Nat) nil))) (lit tt Bool) (lit ff Bool))";
    let insert_tuple: RecExpr<MimSlotted> = RecExpr::parse(insert_tuple).unwrap();
    let insert_tuple_id = eg.add_expr(insert_tuple);

    assert_eq!(
        type_of(&eg, insert_tuple_id),
        type_("(sigma (cons Nat (cons Bool nil)))")
    );

    let insert_pack = "(insert (pack (top Nat) (lit ff Bool)) (lit tt Bool) (lit ff Bool))";
    let insert_pack: RecExpr<MimSlotted> = RecExpr::parse(insert_pack).unwrap();
    let insert_pack_id = eg.add_expr(insert_pack);

    assert_eq!(
        type_of(&eg, insert_pack_id),
        type_("(arr (top Nat) (Bool))")
    );

    let extract_tuple =
        "(extract (tuple (cons (lit 1 Nat) (cons (lit 3 (idx i32)) nil))) (lit tt Bool))";
    let extract_tuple: RecExpr<MimSlotted> = RecExpr::parse(extract_tuple).unwrap();
    let extract_tuple_id = eg.add_expr(extract_tuple);

    assert_eq!(type_of(&eg, extract_tuple_id), type_("(idx i32)"));

    let extract_pack = "(extract (pack (top Nat) (lit ff Bool)) (lit 0 (idx 1)))";
    let extract_pack: RecExpr<MimSlotted> = RecExpr::parse(extract_pack).unwrap();
    let extract_pack_id = eg.add_expr(extract_pack);

    assert_eq!(type_of(&eg, extract_pack_id), type_("Bool"));

    let extract_var = "(extract (var $foo) (lit 0 (idx 1)))";
    let extract_var: RecExpr<MimSlotted> = RecExpr::parse(extract_var).unwrap();
    let extract_var_id = eg.add_expr(extract_var);

    assert_eq!(
        type_of(&eg, extract_var_id),
        type_("(hole (type (lit 0 Univ)))")
    );
}

#[test]
fn make_var_type_hole() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let var_annotated = "(@ Bool (var $foo))";
    let var_annotated: RecExpr<MimSlotted> = RecExpr::parse(var_annotated).unwrap();
    let var_typed = extract_type_annotations(&var_annotated);
    let var_typed_id = add_expr_typed(&mut eg, var_typed);

    // The annotated type for var should be overwritten with hole at this point.
    // Since all vars are represented with the same singleton var eclass, we
    // can't maintain the variables' types with an analysis and should hope that
    // the mim compiler can type-infer these var holes.
    assert_eq!(
        type_of(&eg, var_typed_id),
        type_("(hole (type (lit 0 Univ)))")
    );

    let var = "(var $bar)";
    let var: RecExpr<MimSlotted> = RecExpr::parse(var).unwrap();
    let var_id = eg.add_expr(var);

    assert_eq!(type_of(&eg, var_id), type_("(hole (type (lit 0 Univ)))"));
}

#[test]
fn infer_let_type() {
    let type_of = |eg: &EGraph<MimSlotted, MimSlottedAnalysis>, id: AppliedId| {
        eg.analysis_data(id.id).type_.clone()
    };
    let type_ = |s: &str| RecExpr::<MimSlotted>::parse(s).unwrap();

    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let let_annotated = "(let $foo (scope (@ Bool (lit ff Bool)) (@ Nat (lit 1 Nat))))";
    let let_annotated: RecExpr<MimSlotted> = RecExpr::parse(let_annotated).unwrap();
    let let_typed = extract_type_annotations(&let_annotated);
    let let_typed_id = add_expr_typed(&mut eg, let_typed);

    assert_eq!(type_of(&eg, let_typed_id), type_("Nat"));

    let let_var_annotated = "(let $foo (scope (@ Bool (lit ff Bool)) (@ Nat (var $bar))))";
    let let_var_annotated: RecExpr<MimSlotted> = RecExpr::parse(let_var_annotated).unwrap();
    let let_var_typed = extract_type_annotations(&let_var_annotated);
    let let_var_typed_id = add_expr_typed(&mut eg, let_var_typed);

    assert_eq!(
        type_of(&eg, let_var_typed_id),
        type_("(hole (type (lit 0 Univ)))")
    );
}

// Source: https://github.com/memoryleak47/slotted-egraphs/blob/main/tests/entry.rs
// Had to copy-paste the code below since it didn't seem to be exposed as part of the library.

#[derive(Clone, Debug)]
enum ReachError {
    Reached,
    Failed,
}

#[allow(clippy::type_complexity)]
fn reach_hook<'a, L, N, IterData>(
    start: &'a RecExpr<L>,
    goal: &'a RecExpr<L>,
    steps: usize,
) -> Box<dyn FnMut(&mut Runner<L, N, IterData, ReachError>) -> Result<(), ReachError>>
where
    L: Language + 'static,
    N: Analysis<L>,
    IterData: IterationData<L, N>,
{
    let start = start.clone();
    let goal = goal.clone();
    Box::new(move |runner: &mut Runner<L, N, IterData, ReachError>| {
        if let Some(i2) = lookup_rec_expr(&goal, &runner.egraph) {
            let i1 = lookup_rec_expr(&start, &runner.egraph).unwrap();

            if runner.egraph.eq(&i1, &i2) {
                println!(
                    "{}",
                    &(runner.egraph)
                        .explain_equivalence(start.clone(), goal.clone())
                        .to_string(&runner.egraph)
                );
                return Err(ReachError::Reached);
            }
        }
        if runner.iterations.len() >= steps - 1 {
            return Err(ReachError::Failed);
        }
        Ok(())
    })
}

fn assert_reaches<L, N>(start: &str, goal: &str, rewrites: &[Rewrite<L, N>], steps: usize)
where
    L: Language + 'static,
    N: Analysis<L> + Default + 'static,
{
    let start: RecExpr<L> = RecExpr::parse(start).unwrap();
    let goal: RecExpr<L> = RecExpr::parse(goal).unwrap();

    let mut runner: Runner<L, N, (), ReachError> = Runner::default()
        .with_expr(&start)
        .with_iter_limit(60)
        .with_iter_limit(steps)
        .with_hook(reach_hook(&start, &goal, steps));
    let report = runner.run(rewrites);

    dbg!(&report.stop_reason);
    if let StopReason::Other(ReachError::Reached) = report.stop_reason {
        runner.egraph.explain_equivalence(start, goal);
        return;
    }

    runner.egraph.dump();
    panic!("Couldn't reach goal in provided number of steps.");
}
