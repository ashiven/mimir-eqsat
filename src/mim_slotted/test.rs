use regex::Regex;
use std::fs;

use crate::ffi::FFI;
use crate::ffi::bridge::{CostFn, RuleSet};
use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use crate::mim_slotted::convert_rules;
use crate::mim_slotted::get_rules;
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
    let a = "(let $foo (scope (lit 1) (var $foo)))";
    let b = "(lit 1)";
    assert_reaches::<MimSlotted, MimSlottedAnalysis>(a, b, &get_rules(vec![RuleSet::Standard]), 1);
}

#[test]
fn bind_con_var_add0() {
    let a = "(root extern foo (lam $arg (scope (lit ff) (app %core.nat.add (tuple (cons (var $arg) (cons (lit 0) nil)))))))";
    let b = "(root extern foo (lam $arg (scope (lit ff) (var $arg))))";
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
fn parse_pow_slotted() {
    let pow_slotted =
        fs::read_to_string("examples/pow.slotted").expect("Failed to read pow.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&pow_slotted);
}

#[test]
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
            (lit ff))
            (@ (bot (type (lit 0 Univ)))
            (app
                (@ (cn I8)
                (var $return_22296))
                (@ I8
                (lit 6))))))))";

    let parsed: RecExpr<MimSlotted> = RecExpr::parse(annotated).unwrap();
    println!("{}", parsed.to_ffi().pretty(80));
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
