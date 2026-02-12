use crate::Mim::*;
use crate::core::*;
use crate::*;

pub mod core;
pub mod math;

define_language! {
    pub enum Mim {
        // app (callee, arg)
        "app" = App([Id; 2]),
        "lam" = Lam([Id; 2]),
        // con (name, argtuple, body)
        "con" = Con([Id; 3]),

        // var (name, type)
        "var" = Var([Id; 2]),
        // lit (value, <type>)
        "lit" = Lit(Box<[Id]>),

        "tuple" = Tuple(Box<[Id]>),
        "extract" = Extract([Id; 2]),
        "ins" = Ins([Id; 3]),

        // TYPES
        "sigma" = Sigma(Box<[Id]>),
        "arr" = Arr([Id; 2]),
        "cn" = Cn(Id),
        "idx" = Idx(Id),

        Num(i32), Symbol(String),
    }
}

#[derive(Default)]
pub struct MimAnalysis;
impl Analysis<Mim> for MimAnalysis {
    type Data = Option<Mim>;

    fn merge(&mut self, a: &mut Self::Data, b: Self::Data) -> DidMerge {
        if a.is_none() && b.is_some() {
            *a = b;
            DidMerge(true, false)
        } else {
            DidMerge(false, false)
        }
    }

    fn make(egraph: &mut EGraph<Mim, Self>, enode: &Mim) -> Self::Data {
        if let Some(folded) = fold_core(egraph, enode) {
            return Some(folded);
        }

        None
    }

    fn modify(egraph: &mut EGraph<Mim, Self>, id: Id) {
        if let Some(c) = egraph[id].data.clone() {
            let const_id = egraph.add(c);
            let lit_id = egraph.add(Lit(Box::new([const_id])));
            egraph.union(id, lit_id);
        }
    }
}

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let mut rules = Vec::new();

    rules.extend(core::rules());
    rules.extend(math::rules());

    rules
}
