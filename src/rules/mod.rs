use crate::*;

pub mod core;
pub mod math;

define_language! {
    pub enum Mim {
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

        Nat(i32), Ident(String),
    }
}

pub fn rules() -> Vec<Rewrite<Mim, ()>> {
    let mut rules = Vec::new();

    rules.extend(core::rules());
    rules.extend(math::rules());

    rules
}
