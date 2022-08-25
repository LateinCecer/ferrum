use std::collections::hash_map::DefaultHasher;
use crate::lang::types::{FerrumGenerics, FerrumGenericsTable, FerrumGenericType, FerrumType, GenericTemplate, Namespace, TemplateType};
use crate::lang::variable::{FerrumVarDeclare, FerrumVarDeclareTemplate};

pub struct FerrumFunction {
    header: FerrumFunctionHeader,
    body: FerrumFunctionBody,
}

struct FerrumFunctionHeader {
    name: String,
    namespace: Namespace,
    params: Vec<FerrumVarDeclare>,
    return_value: FerrumType,
    generic_fingerprint: u64,
}

struct FerrumFunctionBody {
    // TODO
}

pub struct FerrumFunctionPtr {
    function_id: Namespace,
    generic_fingerprint: u64,
}

impl FerrumFunctionHeader {
    pub fn matches_generics(&self, generics: &FerrumGenericsTable) -> bool {
        self.generic_fingerprint == *generics.fingerprint()
    }
}

impl FerrumFunction {
    pub fn matches_generics(&self, generics: &FerrumGenericsTable) -> bool {
        self.header.generic_fingerprint == *generics.fingerprint()
    }
}





pub struct FerrumFunctionTemplate {
    header: FerrumFunctionHeaderTemplate,
    body: FerrumFunctionBodyTemplate,
}

struct FerrumFunctionHeaderTemplate {
    name: String,
    namespace: Namespace,
    params: Vec<TemplateType<FerrumVarDeclare, FerrumVarDeclareTemplate>>,
    return_value: TemplateType<FerrumType, FerrumGenericType>,
    generics: FerrumGenerics,
}

struct FerrumFunctionBodyTemplate {
    // TODO
}

impl GenericTemplate for FerrumFunctionTemplate {
    type Final = FerrumFunction;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        Some(FerrumFunction {
            header: self.header.generate_type(table)?,
            body: self.body.generate_type(table)?,
        })
    }
}

impl GenericTemplate for FerrumFunctionBodyTemplate {
    type Final = FerrumFunctionBody;

    fn generate_type(&self, _table: &FerrumGenericsTable) -> Option<Self::Final> {
        // TODO
        Some(FerrumFunctionBody {})
    }
}

impl GenericTemplate for FerrumFunctionHeaderTemplate {
    type Final = FerrumFunctionHeader;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        let params: Vec<_>
            = self.params.iter().map(
            |par| par.try_generate_type(table).unwrap()
        ).collect();

        Some(FerrumFunctionHeader {
            name: self.name.clone(),
            namespace: self.namespace.clone(),
            params,
            return_value: self.return_value.try_generate_type(table)?,
            generic_fingerprint: *table.fingerprint(),
        })
    }
}
