use std::fs;

use crate::equality_saturate_slotted;
use crate::ffi::bridge::{CostFn, RuleSet};
use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use crate::mim_slotted::get_rules;
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

#[test]
fn get_ruleset_default() {
    let default = get_rules(vec![RuleSet::Default]);
    assert_ne!(default.len(), 0);
}

#[test]
fn let_var_same() {
    let a = "(let (lit 1 Nat) $foo (var $foo))";
    let b = "(lit 1 Nat)";
    assert_reaches::<MimSlotted, MimSlottedAnalysis>(a, b, &get_rules(vec![RuleSet::Default]), 1);
}

#[test]
fn bind_con_var_add0() {
    let a = "(con extern foo Nat $arg (lamdef (lit ff Bool) (app %core.nat.add (tuple (cons (var $arg) (cons (lit 0 Nat) nil))))))";
    let b = "(con extern foo Nat $arg (lamdef (lit ff Bool) (var $arg)))";
    assert_reaches::<MimSlotted, MimSlottedAnalysis>(a, b, &get_rules(vec![RuleSet::Default]), 1);
}

#[test]
fn parse_loop_slotted() {
    let loop_slotted =
        fs::read_to_string("examples/loop.slotted").expect("Failed to read loop.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&loop_slotted);
}

// TODO: The below test case would fail because the loop continuation in loop.slotted
// has a recursive definition where it calls on the body continuation which once again
// wants to call on the loop continuation. The problem with this is that we end up with a
// var use of the loop continuation before it has even been bound by the the let-binding
// surrounding it. I.e. in the body continuation we have "(var $loop_22536)" but this is
// in a scope where "$loop_22536" has not been bound yet and so running equality saturation fails.
// #[test]
// fn eqsat_loop_slotted() {
//     let loop_slotted =
//         fs::read_to_string("examples/loop.slotted").expect("Failed to read loop.slotted");
//     let _nodes = equality_saturate_slotted(&loop_slotted, vec![RuleSet::Default], CostFn::AstSize);
// }

#[test]
fn parse_import_slotted() {
    let import_slotted =
        fs::read_to_string("examples/import.slotted").expect("Failed to read import.slotted");
    let _parsed: Vec<RecExpr<MimSlotted>> = parse_sexprs(&import_slotted);
}

#[test]
fn eqsat_import_slotted() {
    let import_slotted =
        fs::read_to_string("examples/import.slotted").expect("Failed to read import.slotted");
    let _nodes =
        equality_saturate_slotted(&import_slotted, vec![RuleSet::Default], CostFn::AstSize);
    // TODO: comparison against expected ffi nodes
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
