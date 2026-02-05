use egg::*;
use std::error::Error;
use std::fs;

// TODO: I want this to turn into a function that I can import from a mimir plugin
// via an extern import which then takes an sexpr and simply returns the rewritten sexpr
//
// so something like:
//
//  fn equality_saturate(sexpr) -> sexpr
//
//
// for the sake of keeping the project well structured, rewrite rules will
// be divided into the same directory structure that exists in the mimir compiler
// plugin infrastructure, so we get something like:
// > > eqsat.rs
// > > rules/core.rs
// > > rules/math.rs
// > > rules/compile.rs
fn main() -> Result<(), Box<dyn Error>> {
    define_language! {
        enum Mim {
            "app" = App([Id; 2]),
            "lam" = Lam([Id; 2]),
            // con (name, argtuple, body)
            "con" = Con([Id; 3]),

            // var (name, type)
            "var" = Var([Id; 2]),
            // lit (value, <type>)
            "lit" = Lit(Box<[Id]>),

            "tuple" = Tuple(Box<[Id]>),
            "extract" = Extract([Id; 2]),
            "ins" = Ins([Id; 3]),

            // TYPES
            "sigma" = Sigma(Box<[Id]>),
            "arr" = Arr([Id; 2]),
            "cn" = Cn(Id),
            "idx" = Idx(Id),

            Nat(i32), Ident(String),
        }
    }

    let rules: &[Rewrite<Mim, ()>] =
        &[rewrite!("add-0"; "(app %core.nat.add (tuple (lit 0) ?x))" => "?x")];

    let example = fs::read_to_string("./examples/core/nat.sexpr")?;
    let runner = Runner::default()
        .with_expr(&example.parse().unwrap())
        .run(rules);

    // Egraph before equality saturation
    // runner
    //     .egraph
    //     .dot()
    //     .to_png("./examples/core/natpre.png")
    //     .unwrap();

    // runner.run(rules);

    // Egraph after equality saturation
    runner
        .egraph
        .dot()
        .to_png("./examples/core/natpost.png")
        .unwrap();

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    println!("The best cost is: {}", best_cost);
    println!("Post rewrite: {}", best_expr);

    Ok(())
}
