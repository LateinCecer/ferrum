use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::bytecode::chunk::Chunk;
use crate::lang::error::CompileResult;
use crate::lang::variable::{DataLoc, FerrumVariable, VarLoc};

pub struct FerrumCompiler {
    chunk: Chunk,
    scopes: Vec<StackScope>,
}

struct StackScope {
    vars: HashMap<String, Vec<FerrumVariable>>,
    sp: usize,
}

impl FerrumCompiler {
    pub fn find_global(&self, name: &str) -> Option<&FerrumVariable> {
        todo!()
    }

    pub fn find_global_mut(&mut self, name: &str) -> Option<&mut FerrumVariable> {
        todo!()
    }

    pub fn find_var(&self, name: &str, lvl: usize) -> Option<&FerrumVariable> {
        for i in (0..lvl).rev() {
            let var = self.scopes[i].get_var(name);
            match var {
                Some(var) => return Some(var),
                None => (),
            }
        }
        self.find_global(name)
    }

    pub fn find_var_mut(&mut self, name: &str, lvl: usize) -> Option<&mut FerrumVariable> {
        for i in (0..lvl).rev() {
            match self.scopes[i].get_var(name) {
                Some(_) => {
                    return self.scopes[i].get_var_mut(name)
                },
                None => (),
            }
        }
        self.find_global_mut(name)
    }

    pub fn deref_var(&mut self, loc: &VarLoc) -> CompileResult<()> {
        match self.find_var_mut(&loc.name, loc.stack_frame) {
            Some(var) => var.borrow_state.dec_shared(),
            _ => Ok(())
        }
    }

    pub fn deref_mut_var(&mut self, loc: &VarLoc) -> CompileResult<()> {
        match self.find_var_mut(&loc.name, loc.stack_frame) {
            Some(var) => var.borrow_state.free_mut(),
            _ => Ok(())
        }
    }

    /// Writes instructions to increase the runtime shared reference count of a heap variable.
    pub fn inc_heap_rc(&mut self, var: &FerrumVariable) {
        todo!()
    }

    /// Writes instructions to decrease the runtime shared reference count of a heap variable.
    pub fn dec_heap_rc(&mut self, var: &FerrumVariable) {
        todo!()
    }

    /// Writes instructions to grab a mutable reference to a heap variable.
    pub fn grab_heap_mut(&mut self, var: &FerrumVariable) {
        todo!()
    }

    /// Writes instructions to drop a mutable reference to a heap variable.
    pub fn drop_heap_mut(&mut self, var: &FerrumVariable) {
        todo!()
    }
}

impl StackScope {
    pub fn alloc_data_loc(&mut self, size: usize) -> DataLoc {
        let loc = DataLoc {
            loc: self.sp,
            size,
            is_stack: true,
        };
        self.sp += size;
        loc
    }

    /// Adds a new variable to the stack scope.
    pub fn add_var(&mut self, var: FerrumVariable) {
        match self.vars.entry(var.name().to_owned()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(var);
            },
            Entry::Vacant(e) => {
                let mut v = Vec::new();
                v.push(var);
                e.insert(v);
            }
        }
    }

    /// Returns a shared reference to the most recent variable with the specified name.
    pub fn get_var(&self, name: &str) -> Option<&FerrumVariable> {
        match self.vars.get(name) {
            Some(e) => {
                e.last()
            },
            None => {
                None
            }
        }
    }

    /// Returns a mutable reference to the most recent variable with the specified name.
    pub fn get_var_mut(&mut self, name: &str) -> Option<&mut FerrumVariable> {
        match self.vars.get_mut(name) {
            Some(e) => {
                e.last_mut()
            },
            None => {
                None
            }
        }
    }
}
