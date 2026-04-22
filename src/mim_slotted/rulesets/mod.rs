use crate::ffi::bridge::RuleSet;
use crate::mim_slotted::MimSlotted;
use crate::mim_slotted::analysis::MimSlottedAnalysis;
use slotted_egraphs::Rewrite;

pub mod default;

pub fn get_rules(rulesets: Vec<RuleSet>) -> Vec<Rewrite<MimSlotted, MimSlottedAnalysis>> {
    let mut rules = Vec::new();

    #[allow(clippy::single_match)]
    for ruleset in rulesets {
        match ruleset {
            RuleSet::Default => rules.extend(default::rules()),
            _ => (),
        }
    }

    rules
}
