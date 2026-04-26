#[allow(unused_imports)]
use mimir_eqsat::{
    equality_saturate, equality_saturate_slotted,
    ffi::bridge::{CostFn, RuleSet},
};
use std::fs;

fn main() {
    let example = fs::read_to_string("./examples/fun.slotted").expect("Failed to read file.");
    let nodes = equality_saturate_slotted(&example, vec![RuleSet::Default], CostFn::AstSize);

    for node in nodes.iter() {
        println!("{}", node.pretty(80));
    }
}
