use crate::mim_egg::Mim;
use crate::mim_slotted::MimSlotted;
use crate::{equality_saturate, equality_saturate_slotted, node_ffi_str, pretty, pretty_slotted};
use bridge::{MimKind, NodeFFI, RecExprFFI};
use egg::{Id, RecExpr};
use slotted_egraphs::RecExpr as RecExprSlotted;
use slotted_egraphs::{Id as IdSlotted, Language};
use std::collections::{BTreeMap, HashMap};
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

    #[derive(Debug, Hash)]
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

    #[derive(Debug, PartialEq, Hash, Eq, Clone)]
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
            MimKind::Scope => f.write_str("scope"),
            MimKind::Cons => f.write_str("cons"),
            MimKind::Nil => f.write_str("nil"),
            MimKind::Num => f.write_str(&self.num.to_string()),
            MimKind::Symbol => f.write_str(&self.symbol),
            _ => todo!(),
        }
    }
}

pub fn to_ffi(rec_expr: &RecExpr<Mim>) -> RecExprFFI {
    let mut nodes = Vec::new();

    for node in rec_expr {
        match node {
            Mim::Let(children) => nodes.push(new_node_ffi(MimKind::Let, children, None, None)),
            Mim::Lam(children) => nodes.push(new_node_ffi(MimKind::Lam, children, None, None)),
            Mim::Con(children) => nodes.push(new_node_ffi(MimKind::Con, children, None, None)),
            Mim::App(children) => nodes.push(new_node_ffi(MimKind::App, children, None, None)),
            Mim::Var(children) => nodes.push(new_node_ffi(MimKind::Var, children, None, None)),
            Mim::Lit(children) => nodes.push(new_node_ffi(MimKind::Lit, children, None, None)),
            Mim::Pack(children) => nodes.push(new_node_ffi(MimKind::Pack, children, None, None)),
            Mim::Tuple(children) => nodes.push(new_node_ffi(MimKind::Tuple, children, None, None)),
            Mim::Extract(children) => {
                nodes.push(new_node_ffi(MimKind::Extract, children, None, None))
            }
            Mim::Insert(children) => {
                nodes.push(new_node_ffi(MimKind::Insert, children, None, None))
            }
            Mim::Rule(children) => nodes.push(new_node_ffi(MimKind::Rule, children, None, None)),
            Mim::Inj(children) => nodes.push(new_node_ffi(MimKind::Inj, children, None, None)),
            Mim::Merge(children) => nodes.push(new_node_ffi(MimKind::Merge, children, None, None)),
            Mim::Axm(children) => nodes.push(new_node_ffi(MimKind::Axm, children, None, None)),
            Mim::Match(children) => nodes.push(new_node_ffi(MimKind::Match, children, None, None)),
            Mim::Proxy(children) => nodes.push(new_node_ffi(MimKind::Proxy, children, None, None)),

            Mim::Join(children) => nodes.push(new_node_ffi(MimKind::Join, children, None, None)),
            Mim::Meet(children) => nodes.push(new_node_ffi(MimKind::Meet, children, None, None)),
            Mim::Bot(child) => nodes.push(new_node_ffi(MimKind::Bot, &[*child], None, None)),
            Mim::Top(child) => nodes.push(new_node_ffi(MimKind::Top, &[*child], None, None)),
            Mim::Arr(children) => nodes.push(new_node_ffi(MimKind::Arr, children, None, None)),
            Mim::Sigma(children) => nodes.push(new_node_ffi(MimKind::Sigma, children, None, None)),
            Mim::Cn(child) => nodes.push(new_node_ffi(MimKind::Cn, &[*child], None, None)),
            Mim::Pi(children) => nodes.push(new_node_ffi(MimKind::Pi, children, None, None)),
            Mim::Idx(child) => nodes.push(new_node_ffi(MimKind::Idx, &[*child], None, None)),
            Mim::Hole(child) => nodes.push(new_node_ffi(MimKind::Hole, &[*child], None, None)),
            Mim::Type(child) => nodes.push(new_node_ffi(MimKind::Type, &[*child], None, None)),
            Mim::Reform(child) => nodes.push(new_node_ffi(MimKind::Type, &[*child], None, None)),

            Mim::Num(n) => nodes.push(new_node_ffi(MimKind::Num, &[], Some(*n), None)),
            Mim::Symbol(s) => nodes.push(new_node_ffi(MimKind::Symbol, &[], None, Some(s.clone()))),
        }
    }

    RecExprFFI { nodes }
}

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

pub fn to_ffi_slotted(rec_expr: &RecExprSlotted<MimSlotted>) -> RecExprFFI {
    let mut idxmap = BTreeMap::<usize, NodeFFI>::new();
    to_ffi_slotted_internal(rec_expr, &mut idxmap);

    let mut vars = BTreeMap::<usize, NodeFFI>::new();
    let mut var_uses = HashMap::<usize, Vec<(usize, NodeFFI)>>::new();
    let root_id = idxmap.len();
    analyze_var_uses(root_id, rec_expr, &mut var_uses, &mut vars);

    // Since slotted-egraphs seems to represent (var $1) (var $2) ... all as the same Var node
    // with the same Id, we have to manually insert Var nodes for every single slot into the idxmap.
    // This is to ensure that references to different vars will not all be to the the same node.
    if !vars.is_empty() {
        // 1) Shift up indices in idxmap (both the keys and the ffi nodes' child indices)
        let var_start_idx = *vars.keys().min().expect("Failed to get var start idx");
        let var_count = vars.len() - 1; // Not counting the var already in idxmap
        shift_indices(var_start_idx, var_count, &mut idxmap, &mut var_uses);

        // 2) Insert the vars above the var start idx (we made space for them in the previous step)
        idxmap.extend(vars.clone());

        // 3) Go over idxmap and adjust child indices according to var_uses
        adjust_var_uses(&mut idxmap, &mut var_uses, &vars);
    }

    let nodes = idxmap.values().cloned().collect();
    RecExprFFI { nodes }
}

fn shift_indices(
    offset: usize,
    shift_amount: usize,
    idxmap: &mut BTreeMap<usize, NodeFFI>,
    var_uses: &mut HashMap<usize, Vec<(usize, NodeFFI)>>,
) {
    let shift_children = move |children: &mut Vec<u32>| {
        children.iter_mut().for_each(|c| {
            if *c > (offset as u32) {
                *c += shift_amount as u32;
            }
        })
    };

    let shifted_idxmap: BTreeMap<usize, NodeFFI> = idxmap
        .iter_mut()
        .map(|(idx, node)| {
            let mut new_node = node.clone();
            shift_children(&mut new_node.children);

            if *idx > offset {
                (*idx + shift_amount, new_node)
            } else {
                (*idx, new_node)
            }
        })
        .collect();

    *idxmap = shifted_idxmap;

    let shifted_var_uses = var_uses
        .iter_mut()
        .map(|(idx, uses)| {
            let new_uses = uses.clone();

            if *idx > offset {
                (*idx + shift_amount, new_uses)
            } else {
                (*idx, new_uses)
            }
        })
        .collect();

    *var_uses = shifted_var_uses;
}

// var_uses: Parent Id -> Vec<(Child Idx, Child Node)>
// - Maps every ffi node to the var nodes they use (have as children)
//   and the index at which they have them as child nodes
//
// vars: Var Id -> Var Node
// - Stores every unique variable along with its Id
fn analyze_var_uses(
    curr_id: usize,
    rec_expr: &RecExprSlotted<MimSlotted>,
    var_uses: &mut HashMap<usize, Vec<(usize, NodeFFI)>>,
    vars: &mut BTreeMap<usize, NodeFFI>,
) {
    for (child_idx, child) in rec_expr.children.iter().enumerate() {
        let child_id = rec_expr.node.applied_id_occurrences()[child_idx].id.0;

        if let MimSlotted::Var(_) = child.node {
            let child_node = to_node_ffi_slotted(child);
            let parent_uses = var_uses.entry(curr_id).or_default();
            parent_uses.push((child_idx, child_node.clone()));

            if !vars.values().any(|v| *v == child_node) {
                vars.insert(child_id + vars.len(), child_node);
            }
        }
        analyze_var_uses(child_id, child, var_uses, vars);
    }
}

fn adjust_var_uses(
    idxmap: &mut BTreeMap<usize, NodeFFI>,
    var_uses: &mut HashMap<usize, Vec<(usize, NodeFFI)>>,
    vars: &BTreeMap<usize, NodeFFI>,
) {
    let vars_rev: HashMap<NodeFFI, usize> = vars.iter().map(|(k, v)| (v.clone(), *k)).collect();

    let adjusted = idxmap
        .iter_mut()
        .map(|(idx, node)| {
            let mut new_node = node.clone();

            if let Some(var_uses) = var_uses.get(idx) {
                for (child_idx, child_node) in var_uses {
                    let new_idx = *vars_rev.get(child_node).expect("did not find var id");
                    new_node.children[*child_idx] = new_idx as u32;
                }
            }

            (*idx, new_node)
        })
        .collect();

    *idxmap = adjusted;
}

fn to_ffi_slotted_internal(
    rec_expr: &RecExprSlotted<MimSlotted>,
    idxmap: &mut BTreeMap<usize, NodeFFI>,
) {
    to_ffi_slotted_internal_(rec_expr, idxmap);
    let root_node = to_node_ffi_slotted(rec_expr);
    let root_id = idxmap.len();
    idxmap.insert(root_id, root_node);
}

fn to_ffi_slotted_internal_(
    rec_expr: &RecExprSlotted<MimSlotted>,
    idxmap: &mut BTreeMap<usize, NodeFFI>,
) {
    for (child_id, child) in rec_expr
        .node
        .applied_id_occurrences()
        .iter()
        .zip(&rec_expr.children)
    {
        let child_node = to_node_ffi_slotted(child);
        idxmap.insert(child_id.id.0, child_node);
        to_ffi_slotted_internal_(child, idxmap);
    }
}

fn to_node_ffi_slotted(rec_expr: &RecExprSlotted<MimSlotted>) -> NodeFFI {
    match &rec_expr.node {
        MimSlotted::Let(bind) => new_node_ffi_slotted(
            MimKind::Let,
            &[bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Lam(ext, name, dom, codom, bind) => new_node_ffi_slotted(
            MimKind::Lam,
            &[ext.id, name.id, dom.id, codom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Con(ext, name, dom, bind) => new_node_ffi_slotted(
            MimKind::Con,
            &[ext.id, name.id, dom.id, bind.elem.id],
            None,
            None,
            Some(format!("{}", bind.slot)),
        ),
        MimSlotted::Scope(filter, body) => {
            new_node_ffi_slotted(MimKind::Scope, &[filter.id, body.id], None, None, None)
        }
        MimSlotted::App(callee, arg) => {
            new_node_ffi_slotted(MimKind::App, &[callee.id, arg.id], None, None, None)
        }
        MimSlotted::Var(slot) => {
            new_node_ffi_slotted(MimKind::Var, &[], None, None, Some(format!("{}", slot)))
        }
        MimSlotted::Lit(val, type_) => {
            new_node_ffi_slotted(MimKind::Lit, &[val.id, type_.id], None, None, None)
        }
        MimSlotted::Pack(arity, body) => {
            new_node_ffi_slotted(MimKind::Pack, &[arity.id, body.id], None, None, None)
        }
        MimSlotted::Tuple(elem_cons) => {
            new_node_ffi_slotted(MimKind::Tuple, &[elem_cons.id], None, None, None)
        }
        MimSlotted::Extract(tuple, index) => {
            new_node_ffi_slotted(MimKind::Extract, &[tuple.id, index.id], None, None, None)
        }
        MimSlotted::Insert(tuple, index, value) => new_node_ffi_slotted(
            MimKind::Insert,
            &[tuple.id, index.id, value.id],
            None,
            None,
            None,
        ),
        MimSlotted::Rule(name, meta_var, lhs, rhs, guard) => new_node_ffi_slotted(
            MimKind::Rule,
            &[name.id, meta_var.id, lhs.id, rhs.id, guard.id],
            None,
            None,
            None,
        ),
        MimSlotted::Inj(type_, val) => {
            new_node_ffi_slotted(MimKind::Inj, &[type_.id, val.id], None, None, None)
        }
        MimSlotted::Merge(type_, type_cons) => {
            new_node_ffi_slotted(MimKind::Merge, &[type_.id, type_cons.id], None, None, None)
        }
        MimSlotted::Axm(name, type_) => {
            new_node_ffi_slotted(MimKind::Axm, &[name.id, type_.id], None, None, None)
        }
        MimSlotted::Match(op_cons) => {
            new_node_ffi_slotted(MimKind::Match, &[op_cons.id], None, None, None)
        }
        MimSlotted::Proxy(type_, pass, tag, op_cons) => new_node_ffi_slotted(
            MimKind::Proxy,
            &[type_.id, pass.id, tag.id, op_cons.id],
            None,
            None,
            None,
        ),
        MimSlotted::Join(type_cons) => {
            new_node_ffi_slotted(MimKind::Join, &[type_cons.id], None, None, None)
        }
        MimSlotted::Meet(type_cons) => {
            new_node_ffi_slotted(MimKind::Meet, &[type_cons.id], None, None, None)
        }
        MimSlotted::Bot(type_) => new_node_ffi_slotted(MimKind::Bot, &[type_.id], None, None, None),
        MimSlotted::Top(type_) => new_node_ffi_slotted(MimKind::Top, &[type_.id], None, None, None),
        MimSlotted::Arr(arity, body) => {
            new_node_ffi_slotted(MimKind::Arr, &[arity.id, body.id], None, None, None)
        }
        MimSlotted::Sigma(type_cons) => {
            new_node_ffi_slotted(MimKind::Sigma, &[type_cons.id], None, None, None)
        }
        MimSlotted::Cn(domain) => new_node_ffi_slotted(MimKind::Cn, &[domain.id], None, None, None),
        MimSlotted::Pi(domain, codomain) => {
            new_node_ffi_slotted(MimKind::Pi, &[domain.id, codomain.id], None, None, None)
        }
        MimSlotted::Idx(size) => new_node_ffi_slotted(MimKind::Idx, &[size.id], None, None, None),
        MimSlotted::Hole(type_) => {
            new_node_ffi_slotted(MimKind::Hole, &[type_.id], None, None, None)
        }
        MimSlotted::Type(level) => {
            new_node_ffi_slotted(MimKind::Type, &[level.id], None, None, None)
        }
        MimSlotted::Reform(meta_type) => {
            new_node_ffi_slotted(MimKind::Type, &[meta_type.id], None, None, None)
        }

        MimSlotted::Cons(elem, next) => {
            new_node_ffi_slotted(MimKind::Cons, &[elem.id, next.id], None, None, None)
        }
        MimSlotted::Nil() => new_node_ffi_slotted(MimKind::Nil, &[], None, None, None),

        MimSlotted::Num(n) => new_node_ffi_slotted(MimKind::Num, &[], Some(*n), None, None),
        MimSlotted::Symbol(s) => {
            new_node_ffi_slotted(MimKind::Symbol, &[], None, Some(s.to_string()), None)
        }
    }
}

fn new_node_ffi_slotted(
    kind: MimKind,
    children: &[IdSlotted],
    num: Option<u64>,
    symbol: Option<String>,
    slot: Option<String>,
) -> NodeFFI {
    let converted_ids = children.iter().map(|id| id.0 as u32).collect();

    NodeFFI {
        kind,
        children: converted_ids,
        num: num.unwrap_or_default(),
        symbol: symbol.unwrap_or_default(),
        slot: slot.unwrap_or_default(),
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
