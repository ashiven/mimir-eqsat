use egg::{*, rewrite as rw};

fn main() {
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),

        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("mul-0"; "(* ?x 0)" => "0"),
        rw!("mul-1"; "(* ?x 1)" => "?x")
    ];

    let start = "(+ 0 (* 1 a))".parse().unwrap();

    let runner = Runner::default().with_expr(&start).run(rules);

    let extractor = Extractor::new(&runner.egraph, AstSize);

    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);

    assert_eq!(best_expr, "a".parse().unwrap());
    assert_eq!(best_cost, 1);
}
