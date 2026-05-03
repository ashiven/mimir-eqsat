#[allow(unused_imports)]
use eqsat_rs::{
    eqsat_egg, eqsat_slotted,
    ffi::bridge::{CostFn, RuleSet},
    pretty_ffi,
};
use std::fs;

fn main() {
    let example = fs::read_to_string("./examples/loop.slotted").expect("Failed to read file.");
    let nodes = eqsat_slotted(&example, vec![], CostFn::AstSize);

    print!("{}", pretty_ffi(nodes, 80));
}
