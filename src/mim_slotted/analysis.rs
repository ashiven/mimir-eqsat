use crate::mim_slotted::{MimSlotted, TypeExpr};
use slotted_egraphs::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MimSlottedAnalysis;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct AnalysisData {
    pub type_: Option<TypeExpr>,
}

// TODO: Merge and make for types
impl Analysis<MimSlotted> for MimSlottedAnalysis {
    type Data = AnalysisData;

    fn make(_eg: &EGraph<MimSlotted, Self>, _enode: &MimSlotted) -> Self::Data {
        AnalysisData::default()
    }

    fn merge(_l: Self::Data, _r: Self::Data) -> Self::Data {
        AnalysisData::default()
    }

    fn modify(_eg: &mut EGraph<MimSlotted, Self>, _id: Id) {}
}
