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

// TODO: Merge and make for types
impl Analysis<MimSlotted> for MimSlottedAnalysis {
    type Data = AnalysisData;

    fn make(_eg: &EGraph<MimSlotted, Self>, _enode: &MimSlotted) -> Self::Data {
        AnalysisData::default()
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
