use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::types::{TypeData, type_make, type_merge};
use slotted_egraphs::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MimSlottedAnalysis;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct AnalysisData {
    pub type_: TypeData,
}

impl Analysis<MimSlotted> for MimSlottedAnalysis {
    type Data = AnalysisData;

    fn make(eg: &EGraph<MimSlotted, Self>, enode: &MimSlotted) -> Self::Data {
        type_make(eg, enode)
    }

    fn merge(l: Self::Data, r: Self::Data) -> Self::Data {
        type_merge(l, r)
    }

    fn modify(_eg: &mut EGraph<MimSlotted, Self>, _id: Id) {}
}
