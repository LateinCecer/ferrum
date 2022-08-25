use crate::lang::types::{FerrumGenerics, FerrumGenericsTable, FerrumGenericType, FerrumType, GenericTemplate, TemplateType};

#[derive(Debug, PartialEq, Hash)]
pub struct FerrumTuple {
    members: Vec<FerrumType>,
}

pub struct FerrumTupleTemplate {
    members: Vec<TemplateType<FerrumType, FerrumGenericType>>
}

impl GenericTemplate for FerrumTupleTemplate {
    type Final = FerrumTuple;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let members: Vec<_> = self.members.iter()
            .map(|m| m.try_generate_type(table).unwrap())
            .collect();
        Some(FerrumTuple {
            members
        })
    }
}
