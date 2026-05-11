use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::analysis::{AnalysisData, MimSlottedAnalysis};
use slotted_egraphs::*;

/***********************************************************/
/* Conversion from type-annotated RecExpr to TypedRecExpr  */
/***********************************************************/

type TypeExpr = RecExpr<MimSlotted>;

#[derive(Debug, Clone)]
pub struct TypedRecExpr {
    node: MimSlotted,
    children: Vec<TypedRecExpr>,
    type_: TypeExpr,
}

pub(crate) fn extract_type_annotations(rec_expr: &RecExpr<MimSlotted>) -> TypedRecExpr {
    if let MimSlotted::TypeWrap(..) = rec_expr.node {
        let type_expr = rec_expr.children[0].clone();
        let expr = &rec_expr.children[1];
        let mut stripped = extract_type_annotations(expr);
        stripped.type_ = type_expr;

        // Instead of the actual type, we give var nodes a hole type
        // to be inferred later on by the mim compiler. This is because
        // all vars are represented with the same singleton var eclass
        // and we can't store different vars' types on this single eclass.
        if let MimSlotted::Var(_slot) = expr.node {
            stripped.type_ = hole();
        }

        return stripped;
    }

    TypedRecExpr {
        node: rec_expr.node.clone(),
        children: rec_expr
            .children
            .iter()
            .map(extract_type_annotations)
            .collect(),
        type_: hole(),
    }
}

pub(crate) fn add_expr_typed(
    eg: &mut EGraph<MimSlotted, MimSlottedAnalysis>,
    rec_expr: TypedRecExpr,
) -> AppliedId {
    let mut node = rec_expr.node;
    let mut child_ids = node.applied_id_occurrences_mut();

    for (i, child) in rec_expr.children.into_iter().enumerate() {
        *(child_ids[i]) = add_expr_typed(eg, child);
    }

    let eclass_applied_id = eg.add(node);

    let eclass_id = eclass_applied_id.id;
    let analysis_data = eg.analysis_data_mut(eclass_id);
    analysis_data.type_ = rec_expr.type_;

    eclass_applied_id
}

/***********************************************************/
/*  Analysis maintaining type information on eclasses      */
/***********************************************************/

pub type TypeData = TypeExpr;

// Returns the ast size of a term, useful for comparing type expr sizes in merge
pub(crate) fn term_size(type_expr: &TypeExpr) -> usize {
    fn size(type_expr: &TypeExpr) -> usize {
        1 + type_expr.children.iter().map(size).sum::<usize>()
    }
    size(type_expr)
}

// Returns the number of holes in a type expr, also useful for comparison in merge
pub(crate) fn hole_amount(type_expr: &TypeExpr) -> usize {
    fn holes(type_expr: &TypeExpr) -> usize {
        if let MimSlotted::Hole(..) = type_expr.node {
            1 + type_expr.children.iter().map(holes).sum::<usize>()
        } else {
            type_expr.children.iter().map(holes).sum::<usize>()
        }
    }
    holes(type_expr)
}

// This is a placeholder for a type that is as of yet unknown.
// The type inference built into the mim compiler is able to later
// infer the types of these holes from the context they appear in.
//
// We could also leave the level of the type as a hole and
// then use world.mut_hole_type() which leaves the level as a hole
// as well and infers it later on. (not sure if we need this though)
//
// (hole (type (lit 0 Univ)))  --  Hole(*)
pub(crate) fn hole() -> TypeExpr {
    TypeExpr {
        node: MimSlotted::Hole(AppliedId::null()),
        children: vec![TypeExpr {
            node: MimSlotted::Type(AppliedId::null()),
            children: vec![TypeExpr {
                node: MimSlotted::Lit(AppliedId::null(), AppliedId::null()),
                children: vec![
                    TypeExpr {
                        node: MimSlotted::Num(0),
                        children: vec![],
                    },
                    TypeExpr {
                        node: MimSlotted::Symbol("Univ".into()),
                        children: vec![],
                    },
                ],
            }],
        }],
    }
}

pub(crate) fn make_type(
    eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    enode: &MimSlotted,
) -> AnalysisData {
    match enode {
        // typeof[(let $name (scope <definition> <expr>))]  = typeof(<expr>)
        MimSlotted::Let(..) => make_let_type(eg, enode),
        // typeof[(lam $x (scope <filter> <body>))]         = Pi(Hole(*), typeof(<body>))
        MimSlotted::Lam(..) => make_lam_type(eg, enode),
        // typeof[(app <callee> <arg>)]                     = typeof(<callee-codomain>)
        MimSlotted::App(..) => make_app_type(eg, enode),
        // typeof[(var $x)]                                 = Hole(*)
        MimSlotted::Var(..) => make_var_type(eg, enode),
        // typeof[(lit <val> <type>)]                       = <type>
        MimSlotted::Lit(..) => make_lit_type(eg, enode),
        // typeof[(pack <arity> <body>)]                    = Arr(<arity>, typeof(<body>))
        MimSlotted::Pack(..) => make_pack_type(eg, enode),
        // typeof[(tuple <elem-cons>)]                      = Sigma(<elem-type-cons>)
        MimSlotted::Tuple(..) => make_tuple_type(eg, enode),
        // typeof[(extract <tuple> <index>)]                = typeof(<extracted-elem>)
        MimSlotted::Extract(..) => make_extract_type(eg, enode),
        // typeof[(insert <tuple> <index> <value>)]         = typeof(<inserted-tuple>)
        MimSlotted::Insert(..) => make_insert_type(eg, enode),

        // TODO:
        // MimSlotted::Inj(..) = make_inj_type(eg, enode),
        // MimSlotted::Merge(..) = make_merge_type(eg, enode),
        _ => AnalysisData { type_: hole() },
    }
}

// We are making the assumption here that terms are already
// well-typed. (terms emitted from the mim compiler are already well-typed
// and we do our best to correctly type terms that are newly introduced by
// rewrite-rules) So whenever we are merging two eclasses associated with type-data,
// we assume they are equivalent representations of the same type and just
// merge the type with fewer holes and smaller term-size into the eclass.
pub(crate) fn merge_type(l: AnalysisData, r: AnalysisData) -> AnalysisData {
    let l_holes = hole_amount(&l.type_);
    let l_size = term_size(&l.type_);

    let r_holes = hole_amount(&r.type_);
    let r_size = term_size(&r.type_);

    if l_holes < r_holes {
        AnalysisData { type_: l.type_ }
    } else if l_holes > r_holes {
        AnalysisData { type_: r.type_ }
    } else if l_size < r_size {
        AnalysisData { type_: l.type_ }
    } else if l_size > r_size {
        AnalysisData { type_: r.type_ }
    } else {
        AnalysisData { type_: l.type_ }
    }
}

fn make_let_type(eg: &EGraph<MimSlotted, MimSlottedAnalysis>, enode: &MimSlotted) -> AnalysisData {
    let name_bind = if let MimSlotted::Let(name_bind) = enode {
        name_bind
    } else {
        panic!("Expected a let node")
    };

    let name_scope_id = eg.find_applied_id(&name_bind.elem);
    let enodes = eg.enodes_applied(&name_scope_id);
    let name_scope = enodes.first().unwrap_or_else(|| {
        eg.dump();
        panic!(
            "Failed to get name scope node at id: {}",
            name_scope_id.id.0
        )
    });

    let scope_child_ids = name_scope.applied_id_occurrences();
    let expr_id = scope_child_ids.get(1).expect("Failed to get expr id");
    let expr_type = eg.analysis_data(expr_id.id).type_.clone();

    AnalysisData { type_: expr_type }
}

// TODO: Since we are working with continuation passing style I should
// probably give lambdas a type of Cn(Sigma(Hole(*), Cn(typeof(<body>))))
fn make_lam_type(eg: &EGraph<MimSlotted, MimSlottedAnalysis>, enode: &MimSlotted) -> AnalysisData {
    let var_bind = if let MimSlotted::Lam(var_bind) = enode {
        var_bind
    } else {
        panic!("Expected a lam node")
    };

    let var_scope_id = eg.find_applied_id(&var_bind.elem);
    let enodes = eg.enodes_applied(&var_scope_id);
    let var_scope = enodes.first().unwrap_or_else(|| {
        eg.dump();
        panic!("Failed to get var scope node at id: {}", var_scope_id.id.0)
    });

    let scope_child_ids = var_scope.applied_id_occurrences();
    let body_id = scope_child_ids.get(1).expect("Failed to get body id");

    let body_type = eg.analysis_data(body_id.id).type_.clone();

    AnalysisData {
        type_: TypeExpr {
            node: MimSlotted::Pi(AppliedId::null(), AppliedId::null()),
            children: vec![hole(), body_type],
        },
    }
}

fn make_app_type(eg: &EGraph<MimSlotted, MimSlottedAnalysis>, enode: &MimSlotted) -> AnalysisData {
    let callee = if let MimSlotted::App(callee, _arg) = enode {
        callee
    } else {
        panic!("Expected an app node")
    };

    let callee_type = eg.analysis_data(callee.id).type_.clone();

    if let TypeExpr {
        node: MimSlotted::Pi(..),
        children: pi_childs,
    } = callee_type
    {
        let codom_type = pi_childs.get(1).expect("Failed to get callee codomain");
        AnalysisData {
            type_: codom_type.clone(),
        }
    } else {
        AnalysisData { type_: hole() }
    }
}

// I guess we should always give var a hole type because all vars
// are represented with the same var eclass and therefore we can't
// associate the different variables' types with this eclass.
// So we should just hope that the mim compiler will be able to
// resolve all of these var holes itself.
fn make_var_type(
    _eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    _enode: &MimSlotted,
) -> AnalysisData {
    AnalysisData { type_: hole() }
}

fn make_lit_type(eg: &EGraph<MimSlotted, MimSlottedAnalysis>, enode: &MimSlotted) -> AnalysisData {
    let type_ = if let MimSlotted::Lit(_val, type_) = enode {
        type_
    } else {
        panic!("Expected a lit node")
    };

    let type_id = eg.find_applied_id(type_);
    let type_ = eg.get_syn_expr(&type_id);
    AnalysisData { type_ }
}

fn make_pack_type(eg: &EGraph<MimSlotted, MimSlottedAnalysis>, enode: &MimSlotted) -> AnalysisData {
    let (arity, body) = if let MimSlotted::Pack(arity, body) = enode {
        (arity, body)
    } else {
        panic!("Expected a pack node")
    };

    let arity_id = eg.find_applied_id(arity);
    let arity = eg.get_syn_expr(&arity_id);
    let body_type = eg.analysis_data(body.id).type_.clone();

    AnalysisData {
        type_: TypeExpr {
            node: MimSlotted::Arr(AppliedId::null(), AppliedId::null()),
            children: vec![arity, body_type],
        },
    }
}

fn make_tuple_type(
    eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    enode: &MimSlotted,
) -> AnalysisData {
    let elem_cons = if let MimSlotted::Tuple(elem_cons) = enode {
        elem_cons
    } else {
        panic!("Expected a tuple node")
    };

    let elem_cons_id = eg.find_applied_id(elem_cons);
    let enodes = eg.enodes_applied(&elem_cons_id);
    let elem_cons = enodes.first().expect("Failed to get tuple elem cons");

    let mut elem_types: Vec<TypeExpr> = Vec::new();

    let mut curr_cons = elem_cons.clone();
    while let MimSlotted::Cons(elem, next) = curr_cons {
        let curr_elem_id = eg.find_applied_id(&elem);
        let curr_elem_type = eg.analysis_data(curr_elem_id.id).type_.clone();
        elem_types.push(curr_elem_type);

        let enodes = eg.enodes_applied(&next);
        let next_cons = enodes
            .first()
            .expect("Failed to get next elem cons")
            .clone();
        curr_cons = next_cons;
    }

    let mut elem_type_cons = TypeExpr {
        node: MimSlotted::Nil(),
        children: vec![],
    };

    if elem_types.is_empty() {
        AnalysisData {
            type_: TypeExpr {
                node: MimSlotted::Sigma(AppliedId::null()),
                children: vec![elem_type_cons],
            },
        }
    } else {
        while let Some(elem_type) = elem_types.pop() {
            elem_type_cons = TypeExpr {
                node: MimSlotted::Cons(AppliedId::null(), AppliedId::null()),
                children: vec![elem_type, elem_type_cons],
            }
        }
        AnalysisData {
            type_: TypeExpr {
                node: MimSlotted::Sigma(AppliedId::null()),
                children: vec![elem_type_cons],
            },
        }
    }
}

fn get_literal(lit_expr: &RecExpr<MimSlotted>) -> u64 {
    let lit_val = lit_expr.children.first().expect("Expected literal value");

    if let MimSlotted::Symbol(s) = lit_val.node {
        match s.as_str() {
            "ff" => 0,
            "tt" => 1,
            "i1" => 2,
            "i8" => 0x100,
            "i16" => 0x10000,
            "i32" => 0x100000000,
            _ => panic!("Unknown literal alias"),
        }
    } else if let MimSlotted::Num(n) = lit_val.node {
        n
    } else {
        panic!("Expected literal value to be a symbol or a number");
    }
}

fn cons_elem_at(cons_expr: &RecExpr<MimSlotted>, index: u64) -> RecExpr<MimSlotted> {}

fn make_extract_type(
    eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    enode: &MimSlotted,
) -> AnalysisData {
    let (tuple, index) = if let MimSlotted::Extract(tuple, index) = enode {
        (tuple, index)
    } else {
        panic!("Expected an extract node")
    };
    let tuple_type = eg.analysis_data(tuple.id).type_.clone();
    let index_id = eg.find_applied_id(index);
    let index = eg.get_syn_expr(&index_id);

    let mut extract_type = hole();

    // Extract from pack
    if let TypeExpr {
        node: MimSlotted::Arr(..),
        children: arr_childs,
    } = tuple_type
    {
        extract_type = arr_childs.get(1).expect("Expected array body").clone()
    // Extract from tuple with literal index
    } else if let TypeExpr {
        node: MimSlotted::Sigma(..),
        children: sigma_childs,
    } = tuple_type
        && let RecExpr {
            node: MimSlotted::Lit(..),
            ..
        } = index
    {
        let sigma_elem_cons = sigma_childs.first().expect("Expected sigma elem cons");
        let index_literal = get_literal(&index);
        extract_type = cons_elem_at(sigma_elem_cons, index_literal);
    }

    AnalysisData {
        type_: extract_type,
    }
}

// TODO: work on types not values
fn make_insert_type(
    eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    enode: &MimSlotted,
) -> AnalysisData {
    let (tuple, _index, _value) = if let MimSlotted::Insert(tuple, index, value) = enode {
        (tuple, index, value)
    } else {
        panic!("Expected an insert node")
    };
    let tuple_id = eg.find_applied_id(tuple);
    let enodes = eg.enodes_applied(&tuple_id);
    let tuple_node = enodes.first().expect("Expected extract tuple");

    let mut insert_type = hole();

    // We can easily infer the type of an insert into a pack literal but for
    // any other inserts we can't reasonably do so, so we just return a hole.
    // It would be possible to infer the type for more complex inserts as well
    // but I hope it will not be needed for now.
    if let MimSlotted::Pack(..) = tuple_node {
        let pack_type = eg.analysis_data(tuple.id).type_.clone();
        insert_type = pack_type;
    }

    AnalysisData { type_: insert_type }
}
