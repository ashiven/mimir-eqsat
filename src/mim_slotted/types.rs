use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::analysis::{AnalysisData, MimSlottedAnalysis};
use slotted_egraphs::*;

/* Conversion from type-annotated RecExpr to TypedRecExpr */

type TypeExpr = RecExpr<MimSlotted>;

#[derive(Debug, Clone)]
pub struct TypedRecExpr {
    node: MimSlotted,
    children: Vec<TypedRecExpr>,
    type_: Option<TypeExpr>,
}

pub(crate) fn extract_type_annotations(rec_expr: &RecExpr<MimSlotted>) -> TypedRecExpr {
    if let MimSlotted::TypeWrap(..) = rec_expr.node {
        let type_expr = rec_expr.children[0].clone();
        let expr = &rec_expr.children[1];
        let mut stripped = extract_type_annotations(expr);
        stripped.type_ = Some(type_expr);

        // Instead of the actual type, we give var nodes a hole type
        // to be inferred later on by the mim compiler. This is because
        // all vars are represented with the same singleton var eclass
        // and we can't store different vars' types on this single eclass.
        if let MimSlotted::Var(_slot) = expr.node {
            stripped.type_ = Some(hole());
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
        type_: None,
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

/* Analysis maintaining type information on eclasses */

pub type TypeData = Option<TypeExpr>;

// We compare type expressions by their ast sizes so we
// can do a quick comparison of equivalent types in the
// analysis and pick the one with a smaller ast size
pub(crate) fn term_size(type_expr: &TypeExpr) -> usize {
    fn size(type_expr: &TypeExpr) -> usize {
        1 + type_expr.children.iter().map(size).sum::<usize>()
    }
    size(type_expr)
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
pub(crate) fn hole() -> RecExpr<MimSlotted> {
    RecExpr {
        node: MimSlotted::Hole(AppliedId::null()),
        children: vec![RecExpr {
            node: MimSlotted::Type(AppliedId::null()),
            children: vec![RecExpr {
                node: MimSlotted::Lit(AppliedId::null(), AppliedId::null()),
                children: vec![
                    RecExpr {
                        node: MimSlotted::Num(0),
                        children: vec![],
                    },
                    RecExpr {
                        node: MimSlotted::Symbol("Univ".into()),
                        children: vec![],
                    },
                ],
            }],
        }],
    }
}

pub(crate) fn type_make(
    eg: &EGraph<MimSlotted, MimSlottedAnalysis>,
    enode: &MimSlotted,
) -> AnalysisData {
    match enode {
        // typeof[(let $name (scope <definition> <expr>))] = typeof(<expr>)
        MimSlotted::Let(name_bind) => {
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

            // typeof(expr)
            if let Some(expr_type) = eg.analysis_data(expr_id.id).type_.clone() {
                AnalysisData {
                    type_: Some(expr_type.clone()),
                }
            }
            // Hole[typeof(expr)]
            else {
                AnalysisData {
                    type_: Some(hole()),
                }
            }
        }
        // TODO: Since we are working with continuation passing style I should
        // probably give lambdas a type of Cn(Sigma(Hole(*), Cn(typeof(<body>))))
        //
        // typeof[(lam $x (scope <filter> <body>))] = Pi(typeof($x), typeof(<body>))
        MimSlotted::Lam(var_bind) => {
            let var_scope_id = eg.find_applied_id(&var_bind.elem);
            let enodes = eg.enodes_applied(&var_scope_id);
            let var_scope = enodes.first().unwrap_or_else(|| {
                eg.dump();
                panic!("Failed to get var scope node at id: {}", var_scope_id.id.0)
            });

            let scope_child_ids = var_scope.applied_id_occurrences();
            let body_id = scope_child_ids.get(1).expect("Failed to get body id");

            // Hole[typeof(var)] -> typeof(body)
            if let Some(body_type) = eg.analysis_data(body_id.id).type_.clone() {
                AnalysisData {
                    type_: Some(RecExpr {
                        node: MimSlotted::Pi(AppliedId::null(), AppliedId::null()),
                        children: vec![hole(), body_type],
                    }),
                }
            // Hole[typeof(var)] -> Hole[typeof(body)]
            } else {
                AnalysisData {
                    type_: Some(RecExpr {
                        node: MimSlotted::Pi(AppliedId::null(), AppliedId::null()),
                        children: vec![hole(), hole()],
                    }),
                }
            }
        }
        // typeof[(app <callee> <arg>)] = typeof(<callee-codomain>)
        MimSlotted::App(callee, _arg) => {
            let callee_type = eg.analysis_data(callee.id).type_.clone();
            // typeof(callee-codomain)
            if let Some(RecExpr {
                node: MimSlotted::Pi(..),
                children: pi_childs,
            }) = callee_type
            {
                let codom_type = pi_childs.get(1).expect("Failed to get callee codomain");
                AnalysisData {
                    type_: Some(codom_type.clone()),
                }
            }
            // Hole[typeof(callee-codomain)]
            else {
                AnalysisData {
                    type_: Some(hole()),
                }
            }
        }
        // I guess we should always give var a hole type because all vars
        // are represented with the same var eclass and therefore we can't
        // associate the different variables' types with this eclass.
        // So we should just hope that the mim compiler will be able to
        // resolve all of these var holes itself.
        //
        // typeof[(var $x)] = Hole(*)
        MimSlotted::Var(_slot) => AnalysisData {
            type_: Some(hole()),
        },
        // typeof[(lit <val> <type>)] = type
        MimSlotted::Lit(_val, type_) => {
            let type_id = eg.find_applied_id(type_);
            let type_ = eg.get_syn_expr(&type_id);
            AnalysisData { type_: Some(type_) }
        }
        /*
        // TODO: Make types for other variants
        // (pack <arity> <body>)
        Pack(AppliedId, AppliedId) = "pack",
        // (tuple <elem-cons>)
        Tuple(AppliedId) = "tuple",
        // (extract <tuple> <index>)
        Extract(AppliedId, AppliedId) = "extract",
        // (insert <tuple> <index> <value>)
        Insert(AppliedId, AppliedId, AppliedId) = "insert",
        // (inj <type> <value>)
        Inj(AppliedId, AppliedId) = "inj",
        // (merge <type> <type-cons>)
        Merge(AppliedId, AppliedId) = "merge",
        */
        _ => AnalysisData { type_: None },
    }
}

// We are making the assumption here that terms are already
// well-typed. (terms emitted from the mim compiler are already well-typed
// and we do our best to correctly type terms that are newly introduced by
// rewrite-rules) So whenever we are merging two eclasses associated with type-data,
// we assume they are equivalent representations of the same type and just
// merge the type with fewer holes and smaller term-size into the eclass.
pub(crate) fn type_merge(l: AnalysisData, r: AnalysisData) -> AnalysisData {
    match (l.type_, r.type_) {
        (None, None) => AnalysisData { type_: None },
        (None, Some(type_r)) => AnalysisData {
            type_: Some(type_r),
        },
        (Some(type_l), None) => AnalysisData {
            type_: Some(type_l),
        },
        (Some(type_l), Some(type_r)) => {
            let merged_type = if term_size(&type_l) < term_size(&type_r) {
                type_l
            } else {
                type_r
            };
            AnalysisData {
                type_: Some(merged_type),
            }
        }
    }
}
