use crate::rules::*;
use egg::*;
use std::error::Error;
use std::fs;

mod rules;

/*
* TODO: I want this to turn into a function that I can import from a mimir plugin
* via an extern import which then takes an sexpr and simply returns the rewritten sexpr
*   so something like:
*       fn equality_saturate(sexpr) -> sexpr
* for the sake of keeping the project well structured, rewrite rules will
* be divided into the same directory structure that exists in the mimir compiler
* plugin infrastructure, so we get something like:
* > > eqsat.rs
* > > rules/core.rs
* > > rules/math.rs
* > > rules/compile.rs
*/
fn main() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
