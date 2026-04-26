use crate::mim_egg::Mim;
use crate::mim_slotted::MimSlotted;
use crate::{equality_saturate, equality_saturate_slotted, mim_node_str, pretty, pretty_slotted};
use bridge::{MimKind, MimNode, RecExprFFI};
use egg::{Id, RecExpr};
use slotted_egraphs::Id as IdSlotted;
use slotted_egraphs::RecExpr as RecExprSlotted;
use std::fmt;

#[cxx::bridge]
pub mod bridge {
    #[derive(Debug)]
    enum RuleSet {
        // Egg
        Core,
        Math,
        // Slotted
        Default,
    }

    #[derive(Debug)]
    enum CostFn {
        AstSize,
        AstDepth,
    }

    #[derive(Debug)]
    enum MimKind {
        Let,
        Lam,
        Con,
        App,
        Var,
        Lit,
        Pack,
        Tuple,
        Extract,
        Insert,
        Rule,
        Inj,
        Merge,
        Axm,
        Match,
        Proxy,
        Join,
        Meet,
        Bot,
        Top,
        Arr,
        Sigma,
        Cn,
        Pi,
        Idx,
        Hole,
        Type,
        Reform,
        Scope,
        Cons,
        Nil,
        Num,
        Symbol,
    }

    // TODO:
    // - implement display for mimnode to print a sexpr representation of the node
    #[derive(Debug, PartialEq)]
    struct MimNode {
        kind: MimKind,
        children: Vec<u32>,
        num: u64,
        symbol: String,
        slot: String,
    }

    #[derive(Debug)]
    struct RecExprFFI {
        nodes: Vec<MimNode>,
    }

    extern "Rust" {
        fn equality_saturate(
            sexpr: &str,
            rulesets: Vec<RuleSet>,
            cost_fn: CostFn,
        ) -> Vec<RecExprFFI>;
        fn pretty(sexpr: &str, line_len: usize) -> String;

        fn equality_saturate_slotted(
            sexpr: &str,
            rulesets: Vec<RuleSet>,
            cost_fn: CostFn,
        ) -> Vec<RecExprFFI>;
        fn pretty_slotted(sexpr: &str, line_len: usize) -> String;

        fn mim_node_str(node: MimNode) -> String;
    }
}

impl fmt::Display for MimNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            MimKind::Let => f.write_str("let"),
            _ => f.write_str("todo"),
        }
    }
}

/* ------------------------------------------------------------ */
/* ---- Pretty-printing implementation from the egg library --- */
/* ------------------------------------------------------------ */

// Source: https://github.com/egraphs-good/egg/blob/main/src/sexp.rs
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Sexpr {
    String(String),
    List(Vec<Sexpr>),
    Empty,
}

impl fmt::Display for Sexpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sexpr::String(s) => {
                if s.contains(' ') || s.contains('(') || s.contains(')') || s.is_empty() {
                    write!(f, "\"{}\"", s)
                } else {
                    write!(f, "{}", s)
                }
            }
            Sexpr::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Sexpr::Empty => write!(f, "()"),
        }
    }
}

// Source: https://github.com/egraphs-good/egg/blob/main/src/util.rs
fn pretty_print(buf: &mut String, sexpr: &Sexpr, width: usize, level: usize) -> std::fmt::Result {
    use std::fmt::Write;
    if let Sexpr::List(list) = sexpr {
        let indent = sexpr.to_string().len() > width;
        write!(buf, "(")?;

        for (i, val) in list.iter().enumerate() {
            if indent && i > 0 {
                writeln!(buf)?;
                for _ in 0..level {
                    write!(buf, "  ")?;
                }
            }
            pretty_print(buf, val, width, level + 1)?;
            if !indent && i < list.len() - 1 {
                write!(buf, " ")?;
            }
        }

        write!(buf, ")")?;
        Ok(())
    } else {
        write!(buf, "{}", sexpr.to_string().trim_matches('"'))
    }
}

// Source: https://github.com/egraphs-good/egg/blob/main/src/language.rs
impl RecExprFFI {
    fn to_sexpr(&self) -> Sexpr {
        let last = self.nodes.len() - 1;
        self.to_sexpr_rec(last, &mut |_| None)
    }

    fn to_sexpr_rec(&self, i: usize, f: &mut impl FnMut(u32) -> Option<String>) -> Sexpr {
        let node = &self.nodes[i];
        let op = Sexpr::String(node.to_string());
        if node.children.is_empty() {
            op
        } else {
            let mut vec = vec![op];
            for child in node.children.iter() {
                vec.push(if let Some(s) = f(*child) {
                    return Sexpr::String(s);
                } else if (*child as usize) < i {
                    self.to_sexpr_rec(*child as usize, f)
                } else {
                    Sexpr::String(format!("<<<< CYCLE to {} = {:?} >>>>", i, node))
                })
            }
            Sexpr::List(vec)
        }
    }

    pub fn pretty(&self, width: usize) -> String {
        let sexp = self.to_sexpr();

        let mut buf = String::new();
        pretty_print(&mut buf, &sexp, width, 1).unwrap();
        buf
    }
}

/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */
/* ------------------------------------------------------------ */

fn new_mim(kind: MimKind, children: &[Id], num: Option<u64>, symbol: Option<String>) -> MimNode {
    let converted_ids = children.iter().map(|id| usize::from(*id) as u32).collect();

    MimNode {
        kind,
        children: converted_ids,
        num: num.unwrap_or_default(),
        symbol: symbol.unwrap_or_default(),
        slot: String::new(),
    }
}

pub fn rec_expr_to_res(rec_expr: &RecExpr<Mim>) -> RecExprFFI {
    let mut nodes = Vec::new();

    for node in rec_expr {
        match node {
            Mim::Let(children) => nodes.push(new_mim(MimKind::Let, children, None, None)),
            Mim::Lam(children) => nodes.push(new_mim(MimKind::Lam, children, None, None)),
            Mim::Con(children) => nodes.push(new_mim(MimKind::Con, children, None, None)),
            Mim::App(children) => nodes.push(new_mim(MimKind::App, children, None, None)),
            Mim::Var(children) => nodes.push(new_mim(MimKind::Var, children, None, None)),
            Mim::Lit(children) => nodes.push(new_mim(MimKind::Lit, children, None, None)),
            Mim::Pack(children) => nodes.push(new_mim(MimKind::Pack, children, None, None)),
            Mim::Tuple(children) => nodes.push(new_mim(MimKind::Tuple, children, None, None)),
            Mim::Extract(children) => nodes.push(new_mim(MimKind::Extract, children, None, None)),
            Mim::Insert(children) => nodes.push(new_mim(MimKind::Insert, children, None, None)),
            Mim::Rule(children) => nodes.push(new_mim(MimKind::Rule, children, None, None)),
            Mim::Inj(children) => nodes.push(new_mim(MimKind::Inj, children, None, None)),
            Mim::Merge(children) => nodes.push(new_mim(MimKind::Merge, children, None, None)),
            Mim::Axm(children) => nodes.push(new_mim(MimKind::Axm, children, None, None)),
            Mim::Match(children) => nodes.push(new_mim(MimKind::Match, children, None, None)),
            Mim::Proxy(children) => nodes.push(new_mim(MimKind::Proxy, children, None, None)),

            Mim::Join(children) => nodes.push(new_mim(MimKind::Join, children, None, None)),
            Mim::Meet(children) => nodes.push(new_mim(MimKind::Meet, children, None, None)),
            Mim::Bot(child) => nodes.push(new_mim(MimKind::Bot, &[*child], None, None)),
            Mim::Top(child) => nodes.push(new_mim(MimKind::Top, &[*child], None, None)),
            Mim::Arr(children) => nodes.push(new_mim(MimKind::Arr, children, None, None)),
            Mim::Sigma(children) => nodes.push(new_mim(MimKind::Sigma, children, None, None)),
            Mim::Cn(child) => nodes.push(new_mim(MimKind::Cn, &[*child], None, None)),
            Mim::Pi(children) => nodes.push(new_mim(MimKind::Pi, children, None, None)),
            Mim::Idx(child) => nodes.push(new_mim(MimKind::Idx, &[*child], None, None)),
            Mim::Hole(child) => nodes.push(new_mim(MimKind::Hole, &[*child], None, None)),
            Mim::Type(child) => nodes.push(new_mim(MimKind::Type, &[*child], None, None)),
            Mim::Reform(child) => nodes.push(new_mim(MimKind::Type, &[*child], None, None)),

            Mim::Num(n) => nodes.push(new_mim(MimKind::Num, &[], Some(*n), None)),
            Mim::Symbol(s) => nodes.push(new_mim(MimKind::Symbol, &[], None, Some(s.clone()))),
        }
    }

    RecExprFFI { nodes }
}

fn new_mim_slotted(
    kind: MimKind,
    children: &[IdSlotted],
    num: Option<u64>,
    symbol: Option<String>,
    slot: Option<String>,
) -> MimNode {
    let converted_ids = children.iter().map(|id| id.0 as u32).collect();

    MimNode {
        kind,
        children: converted_ids,
        num: num.unwrap_or_default(),
        symbol: symbol.unwrap_or_default(),
        slot: slot.unwrap_or_default(),
    }
}

pub fn rec_expr_to_res_slotted(rec_expr: &RecExprSlotted<MimSlotted>) -> RecExprFFI {
    RecExprFFI {
        nodes: rec_expr_to_res_slotted_internal(rec_expr),
    }
}

pub fn rec_expr_to_res_slotted_internal(rec_expr: &RecExprSlotted<MimSlotted>) -> Vec<MimNode> {
    let mut nodes = Vec::new();

    for child in &rec_expr.children {
        let child_nodes = rec_expr_to_res_slotted_internal(child);
        for child_node in child_nodes {
            if !nodes.contains(&child_node) {
                nodes.push(child_node);
            }
        }
    }

    match &rec_expr.node {
        MimSlotted::Let(bind) => nodes.push(new_mim_slotted(
            MimKind::Let,
            &[bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        )),
        MimSlotted::Lam(ext, name, dom, codom, bind) => nodes.push(new_mim_slotted(
            MimKind::Lam,
            &[ext.id, name.id, dom.id, codom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        )),
        MimSlotted::Con(ext, name, dom, bind) => nodes.push(new_mim_slotted(
            MimKind::Con,
            &[ext.id, name.id, dom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        )),
        MimSlotted::Scope(filter, body) => nodes.push(new_mim_slotted(
            MimKind::Scope,
            &[filter.id, body.id],
            None,
            None,
            None,
        )),
        MimSlotted::App(callee, arg) => nodes.push(new_mim_slotted(
            MimKind::App,
            &[callee.id, arg.id],
            None,
            None,
            None,
        )),
        MimSlotted::Var(slot) => nodes.push(new_mim_slotted(
            MimKind::Var,
            &[],
            None,
            None,
            Some(format!("{}", slot)),
        )),
        MimSlotted::Lit(val, type_) => nodes.push(new_mim_slotted(
            MimKind::Lit,
            &[val.id, type_.id],
            None,
            None,
            None,
        )),
        MimSlotted::Pack(arity, body) => nodes.push(new_mim_slotted(
            MimKind::Pack,
            &[arity.id, body.id],
            None,
            None,
            None,
        )),
        MimSlotted::Tuple(elem_cons) => nodes.push(new_mim_slotted(
            MimKind::Tuple,
            &[elem_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Extract(tuple, index) => nodes.push(new_mim_slotted(
            MimKind::Extract,
            &[tuple.id, index.id],
            None,
            None,
            None,
        )),
        MimSlotted::Insert(tuple, index, value) => nodes.push(new_mim_slotted(
            MimKind::Insert,
            &[tuple.id, index.id, value.id],
            None,
            None,
            None,
        )),
        MimSlotted::Rule(name, meta_var, lhs, rhs, guard) => nodes.push(new_mim_slotted(
            MimKind::Rule,
            &[name.id, meta_var.id, lhs.id, rhs.id, guard.id],
            None,
            None,
            None,
        )),
        MimSlotted::Inj(type_, val) => nodes.push(new_mim_slotted(
            MimKind::Inj,
            &[type_.id, val.id],
            None,
            None,
            None,
        )),
        MimSlotted::Merge(type_, type_cons) => nodes.push(new_mim_slotted(
            MimKind::Merge,
            &[type_.id, type_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Axm(name, type_) => nodes.push(new_mim_slotted(
            MimKind::Axm,
            &[name.id, type_.id],
            None,
            None,
            None,
        )),
        MimSlotted::Match(op_cons) => nodes.push(new_mim_slotted(
            MimKind::Match,
            &[op_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Proxy(type_, pass, tag, op_cons) => nodes.push(new_mim_slotted(
            MimKind::Proxy,
            &[type_.id, pass.id, tag.id, op_cons.id],
            None,
            None,
            None,
        )),

        MimSlotted::Join(type_cons) => nodes.push(new_mim_slotted(
            MimKind::Join,
            &[type_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Meet(type_cons) => nodes.push(new_mim_slotted(
            MimKind::Meet,
            &[type_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Bot(type_) => {
            nodes.push(new_mim_slotted(MimKind::Bot, &[type_.id], None, None, None))
        }
        MimSlotted::Top(type_) => {
            nodes.push(new_mim_slotted(MimKind::Top, &[type_.id], None, None, None))
        }
        MimSlotted::Arr(arity, body) => nodes.push(new_mim_slotted(
            MimKind::Arr,
            &[arity.id, body.id],
            None,
            None,
            None,
        )),
        MimSlotted::Sigma(type_cons) => nodes.push(new_mim_slotted(
            MimKind::Sigma,
            &[type_cons.id],
            None,
            None,
            None,
        )),
        MimSlotted::Cn(domain) => {
            nodes.push(new_mim_slotted(MimKind::Cn, &[domain.id], None, None, None))
        }
        MimSlotted::Pi(domain, codomain) => nodes.push(new_mim_slotted(
            MimKind::Pi,
            &[domain.id, codomain.id],
            None,
            None,
            None,
        )),
        MimSlotted::Idx(size) => {
            nodes.push(new_mim_slotted(MimKind::Idx, &[size.id], None, None, None))
        }
        MimSlotted::Hole(type_) => nodes.push(new_mim_slotted(
            MimKind::Hole,
            &[type_.id],
            None,
            None,
            None,
        )),
        MimSlotted::Type(level) => nodes.push(new_mim_slotted(
            MimKind::Type,
            &[level.id],
            None,
            None,
            None,
        )),
        MimSlotted::Reform(meta_type) => nodes.push(new_mim_slotted(
            MimKind::Type,
            &[meta_type.id],
            None,
            None,
            None,
        )),

        MimSlotted::Cons(elem, next) => nodes.push(new_mim_slotted(
            MimKind::Cons,
            &[elem.id, next.id],
            None,
            None,
            None,
        )),
        MimSlotted::Nil() => nodes.push(new_mim_slotted(MimKind::Nil, &[], None, None, None)),

        MimSlotted::Num(n) => nodes.push(new_mim_slotted(MimKind::Num, &[], Some(*n), None, None)),
        MimSlotted::Symbol(s) => nodes.push(new_mim_slotted(
            MimKind::Symbol,
            &[],
            None,
            Some(s.to_string()),
            None,
        )),
    }

    nodes
}
