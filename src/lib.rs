use crate::rules::*;
use egg::*;
use std::error::Error;
use std::fs;

mod rules;

#[unsafe(no_mangle)]
pub extern "C" fn equality_saturate() -> Result<RecExpr<Mim>, Box<dyn Error>> {
    let rules: &[Rewrite<Mim, MimAnalysis>] = &rules();

    // TODO: if we have a series of sexpr's like multiple lambdas in a row,
    // only the first lambda is parsed into an egraph here.
    // gotta find a way that all of them are added to the egraph.
    let example = fs::read_to_string("./examples/core/normalize_add.sexpr")?;
    let runner = Runner::<Mim, MimAnalysis, ()>::default()
        .with_expr(&example.parse().unwrap())
        .run(rules);

    runner
        .egraph
        .dot()
        .to_png("./examples/core/normalize_add.png")
        .unwrap();

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    println!("The best cost is: {}", best_cost);
    println!("Post rewrite: {}", best_expr);

    Ok(best_expr)
}
