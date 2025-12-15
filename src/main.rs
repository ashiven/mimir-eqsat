use egg::*;

fn main() {
    define_language! {
        enum Mim {
            "%core.nat.add" = CoreAdd([Id; 2]),
            "%core.icmp.ul" = CoreUL(Id),

            "app" = App([Id; 2]),
            "lam" = Lam([Id; 2]),

            "tuple" = Tuple(Box<[Id]>),
            "#" = Extract([Id; 2]),

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

    // args = (tuple (tuple a b) return)
    let nat_mim = "(lam args (app (# 1 args) (# 1 (# 0 args))))";
    let mut egraph: EGraph<Mim, ()> = Default::default();
    egraph.add_expr(&nat_mim.parse().unwrap());
    egraph.dot().to_png("./examples/core/nat_eg.png").unwrap();

    // loop_lam need reference to previously defined loop lambda
    // let_inc = 1 + i
    // let_acc = i + acc
    let _loop_mim = "(lam args (app \
    (lam loop_args (app (# (app %core.icmp.ul (tuple (# 1 loop_args) (# 1 (# 0 args)))) \
    (tuple (lam exit_args (app (# 1 args) (tuple exit_args (# 2 loop_args)))) \
    (lam body_args (app loop_lam? (tuple body_args let_inc? let_acc?))) \
    (tuple (# 0 (# 0 args)) 0 0))";

    // let start = "(+ 0 (* 1 a))".parse().unwrap();
    // let runner = Runner::default().with_expr(&start).run(rules);
    // let extractor = Extractor::new(&runner.egraph, AstSize);
    // let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    // assert_eq!(best_expr, "a".parse().unwrap());
    // assert_eq!(best_cost, 1);
}
