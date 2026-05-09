use crate::mim_slotted::{MimSlotted, TypeExpr};
use slotted_egraphs::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MimSlottedAnalysis;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct AnalysisData {
    pub type_: Option<TypeExpr>,
}

// We compare type expressions by their ast sizes so we
// can do a quick comparison of equivalent types in the
// analysis and pick the one with a smaller ast size
fn term_size(type_expr: &TypeExpr) -> usize {
    fn size(type_expr: &TypeExpr) -> usize {
        1 + type_expr.children.iter().map(size).sum::<usize>()
    }
    size(type_expr)
}

// This is a placeholder for a type that is as of yet unknown.
// The type inference built into the mim compiler is able to later
// infer the types of these holes from the context they appear in.
//
// (hole (lit 0 Univ))  --  Hole(*)
fn hole() -> RecExpr<MimSlotted> {
    RecExpr {
        node: MimSlotted::Hole(AppliedId::null()),
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
    }
}

impl Analysis<MimSlotted> for MimSlottedAnalysis {
    type Data = AnalysisData;

    fn make(eg: &EGraph<MimSlotted, Self>, enode: &MimSlotted) -> Self::Data {
        match enode {
            // typeof[(lam $x (scope <filter> <body>))] = typeof($x) -> typeof(body)
            MimSlotted::Lam(var_bind) => {
                let var_scope_id = &var_bind.elem;
                let enodes = eg.enodes_applied(var_scope_id);
                let var_scope = enodes.first().expect("Failed to get var scope node");

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
            // typeof[(app <callee> <arg>)] = typeof(callee-codomain)
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
            // TODO: Make types for other variants
            _ => AnalysisData { type_: None },
        }
    }

    // We are making the assumption here that terms are already
    // well-typed. (terms emitted from the mim compiler are already well-typed
    // and we do our best to correctly type terms that are newly introduced by
    // rewrite-rules) So whenever we are merging two eclasses associated with type-data,
    // we assume they are equivalent representations of the same type and just
    // merge the type with the smaller term-size into the eclass.
    fn merge(l: Self::Data, r: Self::Data) -> Self::Data {
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

    fn modify(_eg: &mut EGraph<MimSlotted, Self>, _id: Id) {}
}
