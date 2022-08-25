use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::process::Output;
use std::rc::Rc;
use crate::lang::r#enum::FerrumEnum;
use crate::lang::r#struct::FerrumStruct;
use crate::lang::tuple::FerrumTuple;


pub type Namespace = String;



/// Ferrum Types
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum FerrumType {
    /// Elementary types. These include number types.
    /// Elementary types are limited to 256 bytes.
    Elementary(u8),
    Struct(Rc<FerrumStruct>),
    Enum(Rc<FerrumEnum>),
    Tuple(Rc<FerrumTuple>),
    Ref(Box<FerrumType>),
    Ptr(Box<FerrumType>),

    MutRef(Box<FerrumType>),
    MutPtr(Box<FerrumType>),
}

/// A generic template is a template structure that can be used to generate templated functions,
/// structs or enums with the requested generics table.
pub trait GenericTemplate {
    type Final : Sized;
    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final>;
}

/// Contains matching explicit type descriptions for `FerrumGenericType`s. This is supplied during
/// compilation when a type or function with a specific set of generics is requested.
#[derive(Clone, PartialEq, Hash)]
pub struct FerrumGenericsTable {
    table: Vec<FerrumType>,
    fingerprint: u64,
}

/// Contains a vector of `FerrumGenericType`s.
pub struct FerrumGenerics {
    types: Vec<FerrumGenericType>
}

/// A generic type is a type name with a generic type id. These ids can be indexed into a
/// `FerrumGenericsTable` to get the explicit `FerrumType` that is being used inplace of the
/// generic.
pub struct FerrumGenericType {
    name: String,
    id: usize,
}



impl FerrumGenericsTable {
    pub fn new(table: Vec<FerrumType>) -> Self {
        let mut s = DefaultHasher::new();
        table.hash(&mut s);

        FerrumGenericsTable {
            table,
            fingerprint: s.finish()
        }
    }

    /// Gets the generic finger print for this specific generic table.
    pub fn fingerprint(&self) -> &u64 {
        &self.fingerprint
    }

    /// Generates a new generics table by joining two tables together. This is useful for nested
    /// generics.
    pub fn join(&self, other: &FerrumGenericsTable) -> Self {
        let mut vec = self.table.clone();
        for i in other.table.iter() {
            vec.push(i.clone());
        }

        let mut s = DefaultHasher::new();
        vec.hash(&mut s);

        FerrumGenericsTable {
            table: vec,
            fingerprint: s.finish()
        }
    }
}

impl Index<usize> for FerrumGenericsTable {
    type Output = FerrumType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}


impl GenericTemplate for FerrumGenericType {
    type Final = FerrumType;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        Some(table[self.id].clone())
    }
}


/// A template type is a type that is either a real (finished) type, or a templated type.
pub enum TemplateType<T: Sized, TT: GenericTemplate<Final = T>>{
    Typed(T),
    Templated(TT),
}

impl<T, TT> TemplateType<T, TT>
where
    T: Sized + Clone,
    TT: GenericTemplate<Final = T> {

    /// Makes the template type into a typed type. If the template type is an instance of
    /// `TemplateType::Typed`, the final type is simply returned. Otherwise, if the template type
    /// is an instance of `TemplateType::Templated`, the template is used to generate the final
    /// type.
    pub fn try_generate_type(&self, table: &FerrumGenericsTable) -> Option<T> {
        match self {
            Self::Typed(t) => Some(t.clone()),
            Self::Templated(tt) => tt.generate_type(table)
        }
    }
}
