use crate::lang::types::{FerrumGenerics, FerrumGenericsTable, FerrumGenericType, FerrumType, GenericTemplate, TemplateType};

#[derive(Debug, PartialEq, Hash)]
pub struct FerrumTuple {
    members: Vec<FerrumTupleMember>,
    size: usize,
}

#[derive(Debug, PartialEq, Hash)]
struct FerrumTupleMember {
    ty: FerrumType,
    offset: usize,
}

impl FerrumTuple {

    /// Realigns the tuple types and recalculates the total byte size.
    pub fn align(&mut self) {
        let mut offset = 0usize;
        self.members.iter_mut().for_each(|m| {
            m.offset = offset;
            offset += m.ty.size();
        });
        self.size = offset;
    }

    /// Returns the total byte size of the tuple.
    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct FerrumTupleTemplate {
    members: Vec<TemplateType<FerrumType, FerrumGenericType>>
}

impl GenericTemplate for FerrumTupleTemplate {
    type Final = FerrumTuple;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let members: Vec<_> = self.members.iter()
            .map(|m| {
                FerrumTupleMember {
                    ty: m.try_generate_type(table).unwrap(),
                    offset: 0,
                }
            })
            .collect();

        let mut t = FerrumTuple {
            members,
            size: 0,
        };
        t.align();
        Some(t)
    }
}
