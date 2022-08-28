use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::lang::types::{FerrumGenerics, FerrumGenericsTable, FerrumType, GenericTemplate, Namespace};


/// Wrapper for Ferrum struct types.
#[derive(Debug)]
pub struct FerrumStruct {
    /// The name of the struct
    name: String,
    /// The fields within the struct
    fields: HashMap<String, FerrumStructMember>,
    /// A unique fingerprint. The fingerprint is intended for use in struct types that have been
    /// generated from a generic template. It must be unique for every data type of the template
    /// type that has been generated using a unique generics table. Usually, this is achieved
    /// by hashing the generic data types.
    generic_fingerprint: u64,
    namespace: Namespace,
    size: usize,
}

impl PartialEq for FerrumStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.namespace == other.namespace
            && self.generic_fingerprint == other.generic_fingerprint
    }
}

impl Hash for FerrumStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.namespace.hash(state);
        self.generic_fingerprint.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
struct FerrumStructMember {
    name: String,
    ty: FerrumType,
    offset: usize,
}



impl FerrumStruct {
    /// Returns true, if the struct has been generated from a `FerrumStructTemplate` using the
    /// specified generics table.
    pub fn matches_generics(&self, generics: &FerrumGenericsTable) -> bool {
        self.generic_fingerprint == *generics.fingerprint()
    }

    /// Recalculates the alignment of member fields for the struct.
    ///
    /// # Alignment
    /// At the moment, everything in the struct will be tightly packed and in an arbitrary order.
    /// In the future, additional alignment options will be available.
    pub fn align(&mut self) {
        let mut offset = 0usize;
        self.fields.iter_mut().for_each(|(_, f)| {
            f.offset = offset;
            offset += f.ty.size();
        });
        self.size = offset;
    }

    /// Returns the byte size of the struct
    pub fn size(&self) -> usize {
        self.size
    }
}





/// The ferrum struct template can be used to generate a `FerrumStruct` data type from a templated
/// struct raw type with the specified `FerrumGenericsTable`.
pub struct FerrumStructTemplate {
    name: String,
    namespace: Namespace,
    generics: FerrumGenerics,
    fields: HashMap<String, FerrumStructMember>,
    generic_fields: HashMap<String, FerrumStructTemplateMember>,
}

struct FerrumStructTemplateMember {
    name: String,
    generic_type: usize,
}

impl GenericTemplate for FerrumStructTemplateMember {
    type Final = FerrumStructMember;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        Some(FerrumStructMember {
            name: self.name.clone(),
            ty: table[self.generic_type].clone(),
            offset: 0,
        })
    }
}

impl GenericTemplate for FerrumStructTemplate {
    type Final = FerrumStruct;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let mut fields = self.fields.clone();
        for (name, field) in self.generic_fields.iter() {
            fields.insert(name.clone(), field.generate_type(table)?);
        }

        let mut s = FerrumStruct {
            name: self.name.clone(),
            fields,
            generic_fingerprint: *table.fingerprint(),
            namespace: self.namespace.clone(),
            size: 0,
        };
        // make alignment
        s.align();

        Some(s)
    }
}
