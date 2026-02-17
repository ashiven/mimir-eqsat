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

fn rexpr_to_vec(rexpr: RecExpr<Mim>) -> Vec<MimNode> {
    let mut nodes = Vec::new();

    for node in rexpr.as_ref() {
        match node {
            Num(n) => nodes.push(MimNode {
                variant: 0,
                children: vec![],
                num: *n,
                symbol: String::new(),
            }),
            Symbol(s) => nodes.push(MimNode {
                variant: 1,
                children: vec![],
                num: 0,
                symbol: s.clone(),
            }),
            App([callee, arg]) => nodes.push(MimNode {
                variant: 2,
                children: vec![usize::from(*callee) as u32, usize::from(*arg) as u32],
                num: 0,
                symbol: String::new(),
            }),
            // TODO: other variants + correct variant order
            _ => (),
        }
    }

    nodes
}
