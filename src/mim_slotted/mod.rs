use crate::ffi::FFI;
use crate::ffi::bridge::{CostFn, MimKind, RecExprFFI, RuleSet};
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use crate::mim_slotted::rulesets::get_rules;
use slotted_egraphs::*;

pub mod analysis;
pub mod rulesets;

#[cfg(test)]
mod test;

define_language! {
    pub enum MimSlotted {
        // TERMS

        // NOTE: Bind<AppliedId> is apparently a wrapper for a pattern like "(bind $1 (expr $1))" whose
        // first child defines a slot while its second child defines some pattern using the slot.
        // This lead to a whole lot of confusion because it means that a pattern like "(let $1 (var $1) ?e)"
        // contains this bind node implicitly as if it was defined as "(let (bind $1 (var $1)) ?e)"
        // and therefore we would have to define Let(Bind<AppliedId>, AppliedId) instead of
        // Let(Bind<AppliedId>, AppliedId, AppliedId) as I initially assumed

        // This now reads as: "let definition equal name in scope containing definition and expression".
        // Instead of (in egg): "let name equal definition in expression".
        // (let <name> <name-scope>)
        Let(Bind<AppliedId>) = "let",
        // (lam <extern> <name> <domain-type> <codomain-type> <var-name> <var-scope>)
        Lam(AppliedId, AppliedId, AppliedId, AppliedId, Bind<AppliedId>) = "lam",
        // (con <extern> <name> <domain-type> <var-name> <var-scope>)
        Con(AppliedId, AppliedId, AppliedId, Bind<AppliedId>) = "con",
        // (app <callee> <arg>)
        App(AppliedId, AppliedId) = "app",
        // (var <name>)
        Var(Slot) = "var",
        // (lit <value> <type>)
        Lit(AppliedId, AppliedId) = "lit",
        // (pack <arity> <body>)
        Pack(AppliedId, AppliedId) = "pack",
        // (tuple <elem-cons>)
        Tuple(AppliedId) = "tuple",
        // (extract <tuple> <index>)
        Extract(AppliedId, AppliedId) = "extract",
        // (insert <tuple> <index> <value>)
        Insert(AppliedId, AppliedId, AppliedId) = "insert",
        // (rule <name> <meta_var> <lhs> <rhs> <guard>)
        Rule(AppliedId, AppliedId, AppliedId, AppliedId, AppliedId) = "rule",
        // (inj <type> <value>)
        Inj(AppliedId, AppliedId) = "inj",
        // (merge <type> <type-cons>)
        Merge(AppliedId, AppliedId) = "merge",
        // (axm <name> <type>)
        Axm(AppliedId, AppliedId) = "axm",
        // (match <op-cons>)
        Match(AppliedId) = "match",
        // (proxy <type> <pass> <tag> <op-cons>)
        Proxy(AppliedId, AppliedId, AppliedId, AppliedId) = "proxy",


        // TYPES

        // (join <type-cons>)
        Join(AppliedId) = "join",
        // (meet <type-cons>)
        Meet(AppliedId) = "meet",
        // (bot <type>)
        Bot(AppliedId) = "bot",
        // (top <type>)
        Top(AppliedId) = "top",
        // (arr <arity> <body>)
        Arr(AppliedId, AppliedId) = "arr",
        // (sigma <type-cons>)
        Sigma(AppliedId) = "sigma",
        // (cn <domain>)
        Cn(AppliedId) = "cn",
        // (pi <domain> <codomain>)
        Pi(AppliedId, AppliedId) = "pi",
        // (idx <size>)
        Idx(AppliedId) = "idx",
        // (hole <type>) - does it even make sense to have this?
        Hole(AppliedId) = "hole",
        // (type <level>)
        Type(AppliedId) = "type",
        // (reform <meta_type>)
        Reform(AppliedId) = "reform",


        // STRUCTURAL

        // (root <extern> <name> <definition>)
        // Root(AppliedId, AppliedId, AppliedId)

        // This is needed so we can bind a lambda variable to both its filter and body
        // and also bind a let variable to both its definition and its expression:
        // (scope <filter> <body>) or (scope <definition> <expression>)  i.e.: (let $foo (scope <def> <expr>))
        Scope(AppliedId, AppliedId) = "scope",

        // Enables variadic language constructs such as Tuple, Sigma, Match, ...
        // (cons <elem> <next>)
        Cons(AppliedId, AppliedId) = "cons",
        Nil() = "nil",

        // Leaf nodes
        Num(u64),
        Symbol(Symbol),
    }
}

pub(crate) fn equality_saturate_ffi(
    sexpr: &str,
    rulesets: Vec<RuleSet>,
    cost_fn: CostFn,
) -> Vec<RecExprFFI> {
    equality_saturate_internal(sexpr, rulesets, cost_fn)
        .iter()
        .map(|rec_expr: &RecExpr<MimSlotted>| rec_expr.to_ffi())
        .collect()
}

pub(crate) fn pretty(sexpr: &str, _line_len: usize) -> String {
    let normalized = sexpr.replace("\r\n", "\n");
    let mut sexprs: Vec<&str> = normalized.split("\n\n").collect();
    sexprs.retain(|s| !s.trim().is_empty());
    let mut res = String::new();

    for (i, sexpr) in sexprs.iter().enumerate() {
        let parsed: RecExpr<MimSlotted> = RecExpr::parse(sexpr).unwrap();
        res.push_str(&parsed.to_string());
        if i < sexprs.len() - 1 {
            res.push_str("\n\n");
        } else {
            res.push('\n');
        }
    }

    res
}

fn equality_saturate_internal(
    sexpr: &str,
    rulesets: Vec<RuleSet>,
    cost_fn: CostFn,
) -> Vec<RecExpr<MimSlotted>> {
    let normalized = sexpr.replace("\r\n", "\n");
    let mut sexprs: Vec<&str> = normalized.split("\n\n").collect();
    sexprs.retain(|s| !s.trim().is_empty());

    let mut rules = get_rules(rulesets);

    convert_rules(&mut sexprs, &mut rules);

    match cost_fn {
        CostFn::AstSize => rewrite_sexprs(sexprs, rules, || AstSize),
        _ => panic!("Unknown cost function provided."),
    }
}

fn rewrite_sexprs<C, F>(
    sexprs: Vec<&str>,
    rules: Vec<Rewrite<MimSlotted, MimSlottedAnalysis>>,
    cost_fn: F,
) -> Vec<RecExpr<MimSlotted>>
where
    C: CostFunction<MimSlotted>,
    F: Fn() -> C,
{
    let mut rewritten_sexprs: Vec<RecExpr<MimSlotted>> = Vec::new();

    let mut runner = Runner::<MimSlotted, MimSlottedAnalysis>::default();
    for sexpr in &sexprs {
        let rec_expr = RecExpr::parse(sexpr).unwrap();
        runner = runner.with_expr(&rec_expr);
    }

    let _report = runner.run(&rules);

    let extractor = Extractor::new(&runner.egraph, cost_fn());
    for i in 0..sexprs.len() {
        let best_expr = extractor.extract(&runner.roots[i], &runner.egraph);
        rewritten_sexprs.push(best_expr);
    }

    rewritten_sexprs
}

fn convert_rules(sexprs: &mut Vec<&str>, rules: &mut Vec<Rewrite<MimSlotted, MimSlottedAnalysis>>) {
    sexprs.retain(|sexpr| {
        let parsed: RecExpr<MimSlotted> = RecExpr::parse(sexpr).unwrap();

        // (rule <name> <meta_var> <lhs> <rhs> <guard>)
        if let MimSlotted::Rule(..) = parsed.node {
            let flattened = parsed.to_ffi();

            let mut rule_name = "";
            if let MimSlotted::Symbol(s) = parsed.children[0].node {
                rule_name = s.into();
            }

            let mut meta_vars: Vec<String> = Vec::new();
            for node in flattened.nodes {
                if node.kind == MimKind::Var {
                    meta_vars.push(node.symbol.clone());
                }
            }

            let mut lhs_rexpr = parsed.children[2].clone();
            inject_meta_vars(&meta_vars, &mut lhs_rexpr);

            let mut rhs_rexpr = parsed.children[3].clone();
            inject_meta_vars(&meta_vars, &mut rhs_rexpr);

            let pat = format!("{}", re_to_pattern(&lhs_rexpr));
            let outpat = format!("{}", re_to_pattern(&rhs_rexpr));
            let rule: Rewrite<MimSlotted, MimSlottedAnalysis> =
                Rewrite::new(rule_name, &pat, &outpat);
            rules.push(rule);
            false
        } else {
            true
        }
    });
}

fn inject_meta_vars(meta_vars: &Vec<String>, rec_expr: &mut RecExpr<MimSlotted>) {
    // for (_id, node) in rec_rexpr.items_mut() {
    //     if let MimSlotted::Symbol(s) = node
    //         && meta_vars.contains(s)
    //     {
    //         s.insert(0, '?')
    //     }
    // }
}
