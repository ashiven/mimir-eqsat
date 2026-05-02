use crate::mim_egg::Mim;
use crate::mim_slotted::MimSlotted;
use crate::{eqsat_egg, eqsat_slotted, node_ffi_str, pretty_egg, pretty_slotted};
use bridge::{MimKind, NodeFFI, RecExprFFI};
use egg::{Id, RecExpr};
use slotted_egraphs::RecExpr as RecExprSlotted;
use std::collections::HashMap;
use std::fmt;

#[cxx::bridge]
pub mod bridge {
    #[derive(Debug)]
    enum RuleSet {
        // Egg
        Core,
        Math,
        // Slotted
        Standard,
    }

    #[derive(Debug)]
    enum CostFn {
        AstSize,
        AstDepth,
    }

    #[derive(Debug, Hash, Default)]
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
        MetaVar,
        Root,
        Scope,
        Cons,
        #[default]
        Nil,
        Num,
        Symbol,
    }

    #[derive(Debug, PartialEq, Hash, Eq, Clone, Default)]
    struct NodeFFI {
        kind: MimKind,
        children: Vec<u32>,
        num: u64,
        symbol: String,
        slot: String,
    }

    #[derive(Debug)]
    struct RecExprFFI {
        nodes: Vec<NodeFFI>,
    }

    extern "Rust" {
        fn eqsat_egg(sexpr: &str, rulesets: Vec<RuleSet>, cost_fn: CostFn) -> Vec<RecExprFFI>;
        fn pretty_egg(sexpr: &str, line_len: usize) -> String;

        fn eqsat_slotted(sexpr: &str, rulesets: Vec<RuleSet>, cost_fn: CostFn) -> Vec<RecExprFFI>;
        fn pretty_slotted(sexpr: &str, line_len: usize) -> String;

        fn pretty_ffi(sexpr: Vec<RecExprFFI>, line_len: usize) -> String;
        fn node_ffi_str(node: NodeFFI) -> String;
    }
}

pub(crate) fn pretty_ffi(sexprs: Vec<RecExprFFI>, line_len: usize) -> String {
    let mut res = String::new();

    for (i, sexpr) in sexprs.iter().enumerate() {
        res.push_str(&sexpr.pretty(line_len));
        if i < sexprs.len() - 1 {
            res.push_str("\n\n");
        } else {
            res.push('\n');
        }
    }

    res
}

impl fmt::Display for NodeFFI {
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
            MimKind::MetaVar => f.write_str("metavar"),
            MimKind::Root => f.write_str("root"),
            MimKind::Scope => f.write_str("scope"),
            MimKind::Cons => f.write_str("cons"),
            MimKind::Nil => f.write_str("nil"),
            MimKind::Num => f.write_str(&self.num.to_string()),
            MimKind::Symbol => f.write_str(&self.symbol),
            _ => todo!(),
        }
    }
}

pub trait FFI {
    fn to_ffi(&self) -> RecExprFFI;
}

pub trait FFIInner {
    fn to_ffi(&self) -> NodeFFI {
        Default::default()
    }
    fn to_ffi_with_childs(&self, _children: &[usize]) -> NodeFFI {
        Default::default()
    }
}

impl FFI for RecExpr<Mim> {
    fn to_ffi(&self) -> RecExprFFI {
        let nodes = self.iter().map(|n| n.to_ffi()).collect();
        RecExprFFI { nodes }
    }
}

impl FFIInner for Mim {
    fn to_ffi(&self) -> NodeFFI {
        fn new_node_ffi(
            kind: MimKind,
            children: &[Id],
            num: Option<u64>,
            symbol: Option<String>,
        ) -> NodeFFI {
            let converted_ids = children.iter().map(|id| usize::from(*id) as u32).collect();

            NodeFFI {
                kind,
                children: converted_ids,
                num: num.unwrap_or_default(),
                symbol: symbol.unwrap_or_default(),
                slot: String::new(),
            }
        }

        match self {
            Mim::Let(children) => new_node_ffi(MimKind::Let, children, None, None),
            Mim::Lam(children) => new_node_ffi(MimKind::Lam, children, None, None),
            Mim::Con(children) => new_node_ffi(MimKind::Con, children, None, None),
            Mim::App(children) => new_node_ffi(MimKind::App, children, None, None),
            Mim::Var(children) => new_node_ffi(MimKind::Var, children, None, None),
            Mim::Lit(children) => new_node_ffi(MimKind::Lit, children, None, None),
            Mim::Pack(children) => new_node_ffi(MimKind::Pack, children, None, None),
            Mim::Tuple(children) => new_node_ffi(MimKind::Tuple, children, None, None),
            Mim::Extract(children) => new_node_ffi(MimKind::Extract, children, None, None),
            Mim::Insert(children) => new_node_ffi(MimKind::Insert, children, None, None),
            Mim::Rule(children) => new_node_ffi(MimKind::Rule, children, None, None),
            Mim::Inj(children) => new_node_ffi(MimKind::Inj, children, None, None),
            Mim::Merge(children) => new_node_ffi(MimKind::Merge, children, None, None),
            Mim::Axm(children) => new_node_ffi(MimKind::Axm, children, None, None),
            Mim::Match(children) => new_node_ffi(MimKind::Match, children, None, None),
            Mim::Proxy(children) => new_node_ffi(MimKind::Proxy, children, None, None),
            Mim::Join(children) => new_node_ffi(MimKind::Join, children, None, None),
            Mim::Meet(children) => new_node_ffi(MimKind::Meet, children, None, None),
            Mim::Bot(child) => new_node_ffi(MimKind::Bot, &[*child], None, None),
            Mim::Top(child) => new_node_ffi(MimKind::Top, &[*child], None, None),
            Mim::Arr(children) => new_node_ffi(MimKind::Arr, children, None, None),
            Mim::Sigma(children) => new_node_ffi(MimKind::Sigma, children, None, None),
            Mim::Cn(child) => new_node_ffi(MimKind::Cn, &[*child], None, None),
            Mim::Pi(children) => new_node_ffi(MimKind::Pi, children, None, None),
            Mim::Idx(child) => new_node_ffi(MimKind::Idx, &[*child], None, None),
            Mim::Hole(child) => new_node_ffi(MimKind::Hole, &[*child], None, None),
            Mim::Type(child) => new_node_ffi(MimKind::Type, &[*child], None, None),
            Mim::Reform(child) => new_node_ffi(MimKind::Type, &[*child], None, None),
            Mim::Num(n) => new_node_ffi(MimKind::Num, &[], Some(*n), None),
            Mim::Symbol(s) => new_node_ffi(MimKind::Symbol, &[], None, Some(s.clone())),
        }
    }
}

impl FFI for RecExprSlotted<MimSlotted> {
    fn to_ffi(&self) -> RecExprFFI {
        fn to_ffi_internal(
            rec_expr: &RecExprSlotted<MimSlotted>,
            nodes: &mut Vec<NodeFFI>,
            added: &mut HashMap<NodeFFI, usize>,
        ) -> usize {
            let child_ids: Vec<usize> = rec_expr
                .children
                .iter()
                .map(|child| to_ffi_internal(child, nodes, added))
                .collect();

            let new_node = rec_expr.node.to_ffi_with_childs(&child_ids);

            if added.contains_key(&new_node) {
                return *added.get(&new_node).unwrap();
            }

            let id = nodes.len();
            nodes.push(new_node);
            id
        }

        let mut nodes: Vec<NodeFFI> = Vec::new();
        let mut added = HashMap::<NodeFFI, usize>::new();
        to_ffi_internal(self, &mut nodes, &mut added);
        RecExprFFI { nodes }
    }
}

impl FFIInner for MimSlotted {
    fn to_ffi_with_childs(&self, children: &[usize]) -> NodeFFI {
        fn new_node_ffi(
            kind: MimKind,
            children: &[usize],
            num: Option<u64>,
            symbol: Option<String>,
            slot: Option<String>,
        ) -> NodeFFI {
            let converted_ids = children.iter().map(|id| *id as u32).collect();

            NodeFFI {
                kind,
                children: converted_ids,
                num: num.unwrap_or_default(),
                symbol: symbol.unwrap_or_default(),
                slot: slot.unwrap_or_default(),
            }
        }

        match &self {
            MimSlotted::Let(bind) => new_node_ffi(
                MimKind::Let,
                children,
                None,
                None,
                Some(format!("{}", bind.slot)),
            ),
            MimSlotted::Lam(.., bind) => new_node_ffi(
                MimKind::Lam,
                children,
                None,
                None,
                Some(format!("{}", bind.slot)),
            ),
            MimSlotted::Con(.., bind) => new_node_ffi(
                MimKind::Con,
                children,
                None,
                None,
                Some(format!("{}", bind.slot)),
            ),
            MimSlotted::App(..) => new_node_ffi(MimKind::App, children, None, None, None),
            MimSlotted::Var(slot) => new_node_ffi(
                MimKind::Var,
                children,
                None,
                None,
                Some(format!("{}", slot)),
            ),
            MimSlotted::Lit(..) => new_node_ffi(MimKind::Lit, children, None, None, None),
            MimSlotted::Pack(..) => new_node_ffi(MimKind::Pack, children, None, None, None),
            MimSlotted::Tuple(..) => new_node_ffi(MimKind::Tuple, children, None, None, None),
            MimSlotted::Extract(..) => new_node_ffi(MimKind::Extract, children, None, None, None),
            MimSlotted::Insert(..) => new_node_ffi(MimKind::Insert, children, None, None, None),
            MimSlotted::Rule(..) => new_node_ffi(MimKind::Rule, children, None, None, None),
            MimSlotted::Inj(..) => new_node_ffi(MimKind::Inj, children, None, None, None),
            MimSlotted::Merge(..) => new_node_ffi(MimKind::Merge, children, None, None, None),
            MimSlotted::Axm(..) => new_node_ffi(MimKind::Axm, children, None, None, None),
            MimSlotted::Match(..) => new_node_ffi(MimKind::Match, children, None, None, None),
            MimSlotted::Proxy(..) => new_node_ffi(MimKind::Proxy, children, None, None, None),
            MimSlotted::Join(..) => new_node_ffi(MimKind::Join, children, None, None, None),
            MimSlotted::Meet(..) => new_node_ffi(MimKind::Meet, children, None, None, None),
            MimSlotted::Bot(..) => new_node_ffi(MimKind::Bot, children, None, None, None),
            MimSlotted::Top(..) => new_node_ffi(MimKind::Top, children, None, None, None),
            MimSlotted::Arr(..) => new_node_ffi(MimKind::Arr, children, None, None, None),
            MimSlotted::Sigma(..) => new_node_ffi(MimKind::Sigma, children, None, None, None),
            MimSlotted::Cn(..) => new_node_ffi(MimKind::Cn, children, None, None, None),
            MimSlotted::Pi(..) => new_node_ffi(MimKind::Pi, children, None, None, None),
            MimSlotted::Idx(..) => new_node_ffi(MimKind::Idx, children, None, None, None),
            MimSlotted::Hole(..) => new_node_ffi(MimKind::Hole, children, None, None, None),
            MimSlotted::Type(..) => new_node_ffi(MimKind::Type, children, None, None, None),
            MimSlotted::Reform(..) => new_node_ffi(MimKind::Type, children, None, None, None),
            MimSlotted::MetaVar(..) => new_node_ffi(MimKind::MetaVar, children, None, None, None),
            MimSlotted::Root(..) => new_node_ffi(MimKind::Root, children, None, None, None),
            MimSlotted::Scope(..) => new_node_ffi(MimKind::Scope, children, None, None, None),
            MimSlotted::Cons(..) => new_node_ffi(MimKind::Cons, children, None, None, None),
            MimSlotted::Nil() => new_node_ffi(MimKind::Nil, children, None, None, None),
            MimSlotted::Num(n) => new_node_ffi(MimKind::Num, children, Some(*n), None, None),
            MimSlotted::Symbol(s) => {
                new_node_ffi(MimKind::Symbol, children, None, Some(s.to_string()), None)
            }
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
                MimKind::Let => {
                    if !node.slot.is_empty() {
                        vec.insert(1, Sexpr::String(node.slot.clone()))
                    }
                }
                MimKind::Lam => {
                    if !node.slot.is_empty() {
                        vec.insert(vec.len() - 1, Sexpr::String(node.slot.clone()))
                    }
                }
                MimKind::Con => {
                    if !node.slot.is_empty() {
                        vec.insert(vec.len() - 1, Sexpr::String(node.slot.clone()))
                    }
                }
                MimKind::Var => {
                    if !node.slot.is_empty() {
                        vec.insert(1, Sexpr::String(node.slot.clone()))
                    }
                }
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
