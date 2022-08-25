use crate::lang::lifetime::{LifeTime, ScopeLoc};
use crate::lang::types::{FerrumGenericsTable, FerrumType, GenericTemplate};

/// Wraps a variable declaration in ferrum source code.
/// It contains the name of the variable that is being declared, rather it is mutable or not and
/// the type of the variable.
#[derive(Clone, Debug)]
pub struct FerrumVarDeclare {
    name: String,
    ty: FerrumType,
    loc: ScopeLoc,
    mutable: bool,
}

pub struct FerrumVariableHandle {
    name: String,
    declare: ScopeLoc,
    mutable: bool,
    has_data: bool,
    is_alive: bool,
}

pub struct FerrumVariable {
    name: String,
    mutable: bool,
    lifetime: LifeTime
}




pub struct FerrumVarDeclareTemplate {
    name: String,
    id: usize,
    loc: ScopeLoc,
    mutable: bool,
}

impl GenericTemplate for FerrumVarDeclareTemplate {
    type Final = FerrumVarDeclare;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        Some(FerrumVarDeclare {
            name: self.name.clone(),
            ty: table[self.id].clone(),
            loc: self.loc,
            mutable: self.mutable
        })
    }
}
