use crate::mim_egg::Mim;
use crate::mim_slotted::MimSlotted;
use crate::{equality_saturate, equality_saturate_slotted, mim_node_str, pretty, pretty_slotted};
use bridge::{MimKind, MimNode, RecExprFFI};
use egg::{Id, RecExpr};
use slotted_egraphs::RecExpr as RecExprSlotted;
use slotted_egraphs::{Id as IdSlotted, Language};
use std::collections::BTreeMap;
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
            MimKind::Lam => f.write_str("lam"),
            MimKind::Con => f.write_str("con"),
            MimKind::App => f.write_str("app"),
            MimKind::Var => f.write_str("var"),
            MimKind::Lit => f.write_str("lit"),
            MimKind::Pack => f.write_str("pack"),
            MimKind::Tuple => f.write_str("tuple"),
            MimKind::Extract => f.write_str("extract"),
            MimKind::Insert => f.write_str("insert"),
            MimKind::Rule => f.write_str("rule"),
            MimKind::Inj => f.write_str("inj"),
            MimKind::Merge => f.write_str("merge"),
            MimKind::Axm => f.write_str("axm"),
            MimKind::Match => f.write_str("match"),
            MimKind::Proxy => f.write_str("proxy"),
            MimKind::Join => f.write_str("join"),
            MimKind::Meet => f.write_str("meet"),
            MimKind::Bot => f.write_str("bot"),
            MimKind::Top => f.write_str("top"),
            MimKind::Arr => f.write_str("arr"),
            MimKind::Sigma => f.write_str("sigma"),
            MimKind::Cn => f.write_str("cn"),
            MimKind::Pi => f.write_str("pi"),
            MimKind::Idx => f.write_str("idx"),
            MimKind::Hole => f.write_str("hole"),
            MimKind::Type => f.write_str("type"),
            MimKind::Reform => f.write_str("reform"),
            MimKind::Scope => f.write_str("scope"),
            MimKind::Cons => f.write_str("cons"),
            MimKind::Nil => f.write_str("nil"),
            MimKind::Num => f.write_str(&self.num.to_string()),
            MimKind::Symbol => f.write_str(&self.symbol),
            _ => todo!(),
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
        if node.children.is_empty() && node.slot.is_empty() {
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
            // Some nodes introduce or use slots which don't
            // have their own nodes so we insert them manually.
            match node.kind {
                MimKind::Let => vec.insert(1, Sexpr::String(node.slot.clone())),
                MimKind::Lam => vec.insert(vec.len() - 1, Sexpr::String(node.slot.clone())),
                MimKind::Con => vec.insert(vec.len() - 1, Sexpr::String(node.slot.clone())),
                MimKind::Var => vec.insert(1, Sexpr::String(node.slot.clone())),
                _ => (),
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

pub fn to_ffi(rec_expr: &RecExpr<Mim>) -> RecExprFFI {
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

pub fn to_ffi_slotted(rec_expr: &RecExprSlotted<MimSlotted>) -> RecExprFFI {
    let mut idxmap = BTreeMap::<IdSlotted, MimNode>::new();
    to_ffi_slotted_internal(rec_expr, &mut idxmap);

    let mut nodes = Vec::new();
    for (_id, mimnode) in idxmap {
        nodes.push(mimnode);
    }
    nodes.push(to_mim_slotted(rec_expr));

    RecExprFFI { nodes }
}

fn to_ffi_slotted_internal(
    rec_expr: &RecExprSlotted<MimSlotted>,
    idxmap: &mut BTreeMap<IdSlotted, MimNode>,
) {
    for (child_id, child) in rec_expr
        .node
        .applied_id_occurrences()
        .iter()
        .zip(&rec_expr.children)
    {
        let child_node = to_mim_slotted(child);
        idxmap.insert(child_id.id, child_node);
        to_ffi_slotted_internal(child, idxmap);
    }
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

fn to_mim_slotted(rec_expr: &RecExprSlotted<MimSlotted>) -> MimNode {
    match &rec_expr.node {
        MimSlotted::Let(bind) => new_mim_slotted(
            MimKind::Let,
            &[bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Lam(ext, name, dom, codom, bind) => new_mim_slotted(
            MimKind::Lam,
            &[ext.id, name.id, dom.id, codom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Con(ext, name, dom, bind) => new_mim_slotted(
            MimKind::Con,
            &[ext.id, name.id, dom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Scope(filter, body) => {
            new_mim_slotted(MimKind::Scope, &[filter.id, body.id], None, None, None)
        }
        MimSlotted::App(callee, arg) => {
            new_mim_slotted(MimKind::App, &[callee.id, arg.id], None, None, None)
        }
        MimSlotted::Var(slot) => {
            new_mim_slotted(MimKind::Var, &[], None, None, Some(format!("{}", slot)))
        }
        MimSlotted::Lit(val, type_) => {
            new_mim_slotted(MimKind::Lit, &[val.id, type_.id], None, None, None)
        }
        MimSlotted::Pack(arity, body) => {
            new_mim_slotted(MimKind::Pack, &[arity.id, body.id], None, None, None)
        }
        MimSlotted::Tuple(elem_cons) => {
            new_mim_slotted(MimKind::Tuple, &[elem_cons.id], None, None, None)
        }
        MimSlotted::Extract(tuple, index) => {
            new_mim_slotted(MimKind::Extract, &[tuple.id, index.id], None, None, None)
        }
        MimSlotted::Insert(tuple, index, value) => new_mim_slotted(
            MimKind::Insert,
            &[tuple.id, index.id, value.id],
            None,
            None,
            None,
        ),
        MimSlotted::Rule(name, meta_var, lhs, rhs, guard) => new_mim_slotted(
            MimKind::Rule,
            &[name.id, meta_var.id, lhs.id, rhs.id, guard.id],
            None,
            None,
            None,
        ),
        MimSlotted::Inj(type_, val) => {
            new_mim_slotted(MimKind::Inj, &[type_.id, val.id], None, None, None)
        }
        MimSlotted::Merge(type_, type_cons) => {
            new_mim_slotted(MimKind::Merge, &[type_.id, type_cons.id], None, None, None)
        }
        MimSlotted::Axm(name, type_) => {
            new_mim_slotted(MimKind::Axm, &[name.id, type_.id], None, None, None)
        }
        MimSlotted::Match(op_cons) => {
            new_mim_slotted(MimKind::Match, &[op_cons.id], None, None, None)
        }
        MimSlotted::Proxy(type_, pass, tag, op_cons) => new_mim_slotted(
            MimKind::Proxy,
            &[type_.id, pass.id, tag.id, op_cons.id],
            None,
            None,
            None,
        ),
        MimSlotted::Join(type_cons) => {
            new_mim_slotted(MimKind::Join, &[type_cons.id], None, None, None)
        }
        MimSlotted::Meet(type_cons) => {
            new_mim_slotted(MimKind::Meet, &[type_cons.id], None, None, None)
        }
        MimSlotted::Bot(type_) => new_mim_slotted(MimKind::Bot, &[type_.id], None, None, None),
        MimSlotted::Top(type_) => new_mim_slotted(MimKind::Top, &[type_.id], None, None, None),
        MimSlotted::Arr(arity, body) => {
            new_mim_slotted(MimKind::Arr, &[arity.id, body.id], None, None, None)
        }
        MimSlotted::Sigma(type_cons) => {
            new_mim_slotted(MimKind::Sigma, &[type_cons.id], None, None, None)
        }
        MimSlotted::Cn(domain) => new_mim_slotted(MimKind::Cn, &[domain.id], None, None, None),
        MimSlotted::Pi(domain, codomain) => {
            new_mim_slotted(MimKind::Pi, &[domain.id, codomain.id], None, None, None)
        }
        MimSlotted::Idx(size) => new_mim_slotted(MimKind::Idx, &[size.id], None, None, None),
        MimSlotted::Hole(type_) => new_mim_slotted(MimKind::Hole, &[type_.id], None, None, None),
        MimSlotted::Type(level) => new_mim_slotted(MimKind::Type, &[level.id], None, None, None),
        MimSlotted::Reform(meta_type) => {
            new_mim_slotted(MimKind::Type, &[meta_type.id], None, None, None)
        }

        MimSlotted::Cons(elem, next) => {
            new_mim_slotted(MimKind::Cons, &[elem.id, next.id], None, None, None)
        }
        MimSlotted::Nil() => new_mim_slotted(MimKind::Nil, &[], None, None, None),

        MimSlotted::Num(n) => new_mim_slotted(MimKind::Num, &[], Some(*n), None, None),
        MimSlotted::Symbol(s) => {
            new_mim_slotted(MimKind::Symbol, &[], None, Some(s.to_string()), None)
        }
    }
}
