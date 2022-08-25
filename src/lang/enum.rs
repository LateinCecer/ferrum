use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::lang::types::{FerrumGenerics, FerrumGenericsTable, FerrumGenericType, FerrumType, GenericTemplate, Namespace, TemplateType};

#[derive(Debug)]
pub struct FerrumEnum {
    name: String,
    namespace: Namespace,
    members: HashMap<String, FerrumEnumMember>,
    generic_fingerprint: u64,
}

impl PartialEq for FerrumEnum {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.namespace == other.namespace
            && self.generic_fingerprint == other.generic_fingerprint
    }
}

impl Hash for FerrumEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.namespace.hash(state);
        self.generic_fingerprint.hash(state);
    }
}


#[derive(Debug)]
struct FerrumEnumMember {
    name: String,
    id: u8,
    args: Vec<FerrumType>,
}

impl FerrumEnum {
    pub fn matches_generics(&self, generics: &FerrumGenericsTable) -> bool {
        self.generic_fingerprint == *generics.fingerprint()
    }
}






struct FerrumEnumTemplate {
    name: String,
    namespace: Namespace,
    members: HashMap<String, FerrumEnumMemberTemplate>,
    generics: FerrumGenerics,
}

struct FerrumEnumMemberTemplate {
    name: String,
    id: u8,
    args: Vec<TemplateType<FerrumType, FerrumGenericType>>,
}

impl GenericTemplate for FerrumEnumTemplate {
    type Final = FerrumEnum;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let mut members = HashMap::new();
        for (name, mem) in self.members.iter() {
            members.insert(name.clone(), mem.generate_type(table)?);
        }

        Some(FerrumEnum {
            name: self.name.clone(),
            namespace: self.namespace.clone(),
            members,
            generic_fingerprint: *table.fingerprint(),
        })
    }
}

impl GenericTemplate for FerrumEnumMemberTemplate {
    type Final = FerrumEnumMember;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let mut args: Vec<_> = self.args.iter()
            .map(|arg| arg.try_generate_type(table).unwrap())
            .collect();

        Some(FerrumEnumMember {
            name: self.name.clone(),
            id: self.id,
            args,
        })
    }
}
