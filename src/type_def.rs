use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, ops::Deref, sync::Arc};
#[derive(Debug, PartialEq,Ord,Eq,PartialOrd, Clone)]
pub enum Type {
    TypeDef{
        name: Arc<String>,
        type_def: Arc<Type>
    },
    None,
    Bool,
    Int,
    Uint,
    Char,
    Float,
    String,
    Array{
        array_type: Box<Type>
    },
    Struct {
        pairs: Vec<Type>
    },
    Function {
        param_type: Box<Type>,
        return_type: Box<Type>,
    },
    Optional {
        types: Vec<Type>
    }
}

impl Type {
    pub fn get_sig(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash_structure(&mut hasher);
        hasher.finish()
    }

    fn hash_structure<H: Hasher>(&mut self, state: &mut H) {
        use Type::*;

        match self {
            TypeDef { name: _, type_def } => {
                "TypeDef".hash(state);
                type_def.clone()
                    .deref()
                    .clone()
                    .hash_structure(state); // once again, I hate rust sometimes
            }
            None => {
                "None".hash(state);
            }
            Bool => {
                "Bool".hash(state);
            }
            Int => {
                "Int".hash(state);
            }
            Uint => {
                "Uint".hash(state);
            }
            Char => {
                "Char".hash(state);
            }
            Float => {
                "Float".hash(state);
            }
            String => {
                "String".hash(state);
            }
            Array { array_type } => {
                "Array".hash(state);
                array_type.hash_structure(state);
            }
            Struct { pairs } => {
                "Struct".hash(state);
                pairs.sort();
                for pair in pairs {
                    pair.hash_structure(state);
                }
            }
            Function { param_type, return_type } => {
                "Function".hash(state);
                param_type.hash_structure(state);
                return_type.hash_structure(state);
            }
            Optional { types } => {
                "Optional".hash(state);
                types.sort();
                for ty in types {
                    ty.hash_structure(state);
                }
            }
        }
    }
}

