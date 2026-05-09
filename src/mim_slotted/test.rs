use regex::Regex;
use std::fs;

use crate::ffi::FFI;
use crate::ffi::bridge::{CostFn, RuleSet};
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use crate::mim_slotted::convert_rules;
use crate::mim_slotted::get_rules;
use crate::mim_slotted::{MimSlotted, add_expr_typed, extract_type_annotations};
use crate::{eqsat_slotted, pretty_ffi};
use slotted_egraphs::*;

fn parse_sexprs(sexpr: &str) -> Vec<RecExpr<MimSlotted>> {
    let normalized = sexpr.replace("\r\n", "\n");
    let mut sexprs: Vec<&str> = normalized.split("\n\n").collect();
    sexprs.retain(|s| !s.trim().is_empty());

    let mut res = vec![];
    for sexpr in sexprs {
        res.push(RecExpr::parse(sexpr).expect("Failed to parse RecExpr"));
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
#[ignore]
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
#[ignore]
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
    (lit tt Bool))
";

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

    assert_eq!(lam_type, Some(RecExpr::parse("(cn (cn I8))").unwrap()));
}

#[test]
fn eta_expansion_hole() {
    let mut eg = EGraph::<MimSlotted, MimSlottedAnalysis>::default();

    let fun_annotated = "(@ (pi Nat Bool) fun)";
    let fun_annotated: RecExpr<MimSlotted> = RecExpr::parse(fun_annotated).unwrap();
    let fun = extract_type_annotations(&fun_annotated);
    let fun_id = add_expr_typed(&mut eg, fun);

    let eta_exp = "(lam $x (scope (lit ff Bool) (app fun (var $x))))";
    let eta_exp: RecExpr<MimSlotted> = RecExpr::parse(eta_exp).unwrap();
    let eta_exp_id = eg.add_expr(eta_exp);

    let fun_type = eg.analysis_data(fun_id.id).type_.clone();
    let lam_type = eg.analysis_data(eta_exp_id.id).type_.clone();

    assert_eq!(fun_type, Some(RecExpr::parse("(pi Nat Bool)").unwrap()));
    assert_eq!(
        lam_type,
        Some(RecExpr::parse("(pi (hole (lit 0 Univ)) Bool)").unwrap())
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
