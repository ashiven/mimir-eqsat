use egg::{rewrite as rw, *};

fn main() {
    define_language! {
        enum Mim {
            "%core.nat.add" = CoreAdd([Id; 2]),

            //"fun" = Fun()
            "app" = App([Id; 2]),
            "lam" = Lam([Id; 2]),
            "var" = Var(Id),

            "tuple" = Tuple(Box<[Id]>),

            // given a tuple and an index, return element in tuple
            "#" = Extract([Id; 2]),

            Nat(i32), Ident(String),
        }
    }

    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("mul-0"; "(* ?x 0)" => "0"),
        rw!("mul-1"; "(* ?x 1)" => "?x"),
    ];

    // args = (tuple (tuple a b) return)
    let _nat_mim = "(lam args (app (# 1 args) (# 1 (# 0 args)";

    let start = "(+ 0 (* 1 a))".parse().unwrap();

    let runner = Runner::default().with_expr(&start).run(rules);

    let extractor = Extractor::new(&runner.egraph, AstSize);

    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    assert_eq!(best_expr, "a".parse().unwrap());
    assert_eq!(best_cost, 1);
}
