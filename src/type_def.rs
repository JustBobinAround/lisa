use std::{sync::Arc, ops::Deref};

#[derive(Debug, Ord, Eq, PartialOrd, Clone)]
pub enum Type {
    TypeDef {
        name: Arc<String>,
        type_def: Arc<Type>,
    },
    Generic,
    None,
    Bool,
    Int,
    Uint,
    Char,
    Float,
    String,
    Array {
        array_type: Arc<Type>,
    },
    Struct {
        pairs: Vec<Arc<Type>>,
    },
    Function {
        param_type: Arc<Type>,
        return_type: Arc<Type>,
    },
    Optional {
        type_def: Arc<Type>,
    },
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.get_sig() == other.get_sig()
    }
}
impl Type {
    pub fn reduce(t: Arc<Type>) -> Arc<Type> {
        match &*t {
            Type::Function { param_type, return_type } => {
                return_type.clone()
            }
            _ => {
                t
            }
        }
    }
    pub fn get_sig(&self) -> u64 {
        self.hash_structure(0)
    }


    fn hash_structure(&self, state: u64) -> u64 {
        use Type::*;

        fn combine_hash(state: u64, value: u64) -> u64 {
            state.wrapping_mul(31).wrapping_add(value)
        }

        match self {
            TypeDef { name: _, type_def } => {
                type_def.deref().hash_structure(state)
            }
            Generic => {
                combine_hash(state, 1000000001)
            }
            None => {
                combine_hash(state, 1000000002)
            }
            Bool => {
                combine_hash(state, 1000000003)
            }
            Int => {
                combine_hash(state, 1000000004)
            }
            Uint => {
                combine_hash(state, 1000000005)
            }
            Char => {
                combine_hash(state, 1000000006)
            }
            Float => {
                combine_hash(state, 1000000007)
            }
            String => {
                combine_hash(state, 1000000008)
            }
            Array { array_type } => {
                let state = combine_hash(state, 1000000009);
                array_type.hash_structure(state)
            }
            Struct { pairs } => {
                let state = combine_hash(state, 1000000010);
                let mut sorted_pairs = pairs.clone();
                sorted_pairs.sort();
                sorted_pairs.into_iter().fold(state, |acc, pair| pair.hash_structure(acc))
            }
            Function { param_type, return_type } => {
                let state = combine_hash(state, 1000000011);
                let state = param_type.hash_structure(state);
                return_type.hash_structure(state)
            }
            Optional { type_def } => {
                let state = combine_hash(state, 1000000012);
                type_def.hash_structure(state)
            }
        }
    }
}

