use crate::Mim::*;
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
        let c = |id: &Id| egraph[*id].data.clone();

        match enode {
            App([callee, arg]) => {
                if let Some(Symbol(s)) = c(callee)
                    && s == "%core.nat.add"
                    && let Some(Tuple(t)) = c(arg)
                    && let [t1, t2] = &*t
                    && let Some(Lit(l1)) = c(t1)
                    && let Some(Lit(l2)) = c(t2)
                    && let Some(Num(n1)) = c(&l1[0])
                    && let Some(Num(n2)) = c(&l2[0])
                {
                    // TODO: This needs to be made into a literal (maybe inside of modify?)
                    return Some(Num(n1 + n2));
                }
                None
            }
            _ => None,
        }
    }

    // checks if there is a constant associated with an eclass, and if
    // it is, creates a new enode in the egraph for that constant and merges
    // it with the eclass it is associated with
    fn modify(egraph: &mut EGraph<Mim, Self>, id: Id) {
        if let Some(c) = egraph[id].data.clone() {
            let const_id = egraph.add(c);
            // let new_lit = egraph.add(Lit(Box::new([const_id])))
            egraph.union(id, const_id);
        }
    }
}

pub fn rules() -> Vec<Rewrite<Mim, MimAnalysis>> {
    let mut rules = Vec::new();

    rules.extend(core::rules());
    rules.extend(math::rules());

    rules
}
