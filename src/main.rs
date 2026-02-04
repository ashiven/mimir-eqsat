use egg::*;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    define_language! {
        enum Mim {
            "%core.nat.add" = CoreAdd([Id; 2]),
            "%core.icmp.ul" = CoreUL(Id),

            "app" = App([Id; 2]),
            "lam" = Lam([Id; 2]),
            // con (name, argtuple, body)
            "con" = Con([Id; 3]),

            // var (name, type)
            "var" = Var([Id; 2]),
            // lit (value, type)
            "lit" = Lit([Id; 2]),

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

    let _rules: &[Rewrite<SymbolLang, ()>] = &[
        //rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        //rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        //rw!("add-0"; "(+ ?x 0)" => "?x"),
        //rw!("mul-0"; "(* ?x 0)" => "0"),
        //rw!("mul-1"; "(* ?x 1)" => "?x"),
    ];

    let example = fs::read_to_string("./examples/example.sexpr")?;
    let mut egraph: EGraph<Mim, ()> = Default::default();
    egraph.add_expr(&example.parse().unwrap());
    egraph.dot().to_png("./examples/example.png").unwrap();

    // let runner = Runner::default().with_expr(&example).run(rules);
    // let extractor = Extractor::new(&runner.egraph, AstSize);
    // let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    Ok(())
}
