use crate::Mim::*;
use crate::rules::*;
use egg::*;
use ffi::MimNode;
use std::fs;

mod rules;

// TODO: take sexpr as string argument
pub fn equality_saturate() -> Vec<MimNode> {
    let rules: &[Rewrite<Mim, MimAnalysis>] = &rules();

    // TODO: if we have a series of sexpr's like multiple lambdas in a row,
    // only the first lambda is parsed into an egraph here.
    // gotta find a way that all of them are added to the egraph.
    let example =
        fs::read_to_string("./examples/core/normalize_add.sexpr").expect("Failed to read file.");

    let runner = Runner::<Mim, MimAnalysis, ()>::default()
        .with_expr(&example.parse().unwrap())
        .run(rules);

    runner
        .egraph
        .dot()
        .to_png("./examples/core/normalize_add.png")
        .expect("Failed to create dot graph.");

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    println!("The best cost is: {}", best_cost);
    println!("Post rewrite: {}", best_expr);

    rexpr_to_vec(best_expr)
}

#[cxx::bridge]
pub mod ffi {
    struct MimNode {
        variant: u32,
        children: Vec<u32>,
        num: i32,
        symbol: String,
    }

    extern "Rust" {
        fn equality_saturate() -> Vec<MimNode>;
    }
}

fn new_mim(variant: u32, children: &[Id], num: i32, symbol: String) -> MimNode {
    let mut converted_ids = Vec::new();
    for id in children {
        converted_ids.push(usize::from(*id) as u32);
    }

    MimNode {
        variant,
        children: converted_ids,
        num,
        symbol,
    }
}

fn rexpr_to_vec(rexpr: RecExpr<Mim>) -> Vec<MimNode> {
    let mut nodes = Vec::new();

    for node in rexpr.as_ref() {
        match node {
            Lam(children) => nodes.push(new_mim(0, children, 0, String::new())),
            Con(children) => nodes.push(new_mim(1, children, 0, String::new())),
            App(children) => nodes.push(new_mim(2, children, 0, String::new())),

            Mim::Var(children) => nodes.push(new_mim(3, children, 0, String::new())),
            Lit(children) => nodes.push(new_mim(4, children, 0, String::new())),

            Tuple(children) => nodes.push(new_mim(5, children, 0, String::new())),
            Extract(children) => nodes.push(new_mim(6, children, 0, String::new())),
            Ins(children) => nodes.push(new_mim(7, children, 0, String::new())),

            Sigma(children) => nodes.push(new_mim(8, children, 0, String::new())),
            Arr(children) => nodes.push(new_mim(9, children, 0, String::new())),
            Cn(child) => nodes.push(new_mim(10, &[*child], 0, String::new())),
            Idx(child) => nodes.push(new_mim(11, &[*child], 0, String::new())),

            Num(n) => nodes.push(new_mim(12, &[], *n, String::new())),
            Symbol(s) => nodes.push(new_mim(13, &[], 0, s.clone())),
        }
    }

    nodes
}
