use std::mem;
use crate::lang::compiler::FerrumCompiler;
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::lifetime::{LifeTime};
use crate::lang::types::{FerrumGenericsTable, FerrumType, GenericTemplate};

pub enum DataSource {
    Location(DataLoc),
    Reference(DataLoc, VarLoc, bool),
    MutReference(DataLoc, VarLoc, bool),
}

impl DataSource {
    pub fn new_ref(&mut self, new_loc: VarLoc) -> CompileResult<()> {
        match self {
            DataSource::Reference(_, loc, invalidated) => {
                *loc = new_loc;
                *invalidated = false;
                Ok(())
            },
            _ => Err(CompileError::IllegalDataSource(String::from("shared reference")))
        }
    }

    pub fn new_mut_ref(&mut self, new_loc: VarLoc) -> CompileResult<()> {
        match self {
            DataSource::MutReference(_, loc, invalidated) => {
                *loc = new_loc;
                *invalidated = false;
                Ok(())
            },
            _ => Err(CompileError::IllegalDataSource(String::from("mutable reference")))
        }
    }

    pub fn remove_dependencies(&mut self, compiler: &mut FerrumCompiler) -> CompileResult<()> {
        match self {
            DataSource::Location(_) => Ok(()),
            DataSource::Reference(_, loc, invalidated) => {
                *invalidated = true;
                compiler.deref_var(loc)
            },
            DataSource::MutReference(_, loc, invalidated) => {
                *invalidated = true;
                compiler.deref_mut_var(loc)
            },
        }
    }

    pub fn is_heap_data(&self) -> bool {
        match self {
            DataSource::Location(data) => !data.is_stack,
            _ => false,
        }
    }

    pub fn is_heap_ref(&self, compiler: &mut FerrumCompiler) -> CompileResult<bool> {
        let loc = match self {
            DataSource::Reference(_, loc, _) => loc,
            DataSource::MutReference(_, loc, _) => loc,
            _ => return Ok(false)
        };

        let var = compiler.find_var(&loc.name, loc.stack_frame)
            .ok_or(CompileError::UnknownVariable(loc.clone()))?;
        Ok(var.is_on_heap())
    }

    pub fn is_local_ref(&self, compiler: &mut FerrumCompiler) -> CompileResult<bool> {
        Ok(!self.is_heap_ref(compiler)?)
    }
}

impl Drop for DataSource {
    fn drop(&mut self) {
        // check if the dropped data source contains a valid reference to an other variable and
        // panic if that is true!
        // match self {
        //     DataSource::Reference(_, _, invalidated) if !*invalidated => {
        //         panic!("A validate shared data reference has been dropped!");
        //     },
        //     DataSource::MutReference(_, _, invalidated) if !*invalidated => {
        //         panic!("A validate mutable data reference has been dropped!");
        //     },
        //     _ => (),
        // }
    }
}

pub enum BorrowState {
    None,
    Mut,
    Shared(usize),
}

impl BorrowState {
    pub fn inc_shared(&mut self) -> CompileResult<()> {
        match self {
            BorrowState::None => {
                *self = BorrowState::Shared(1);
                Ok(())
            },
            BorrowState::Mut => Err(CompileError::IllegalSharedBorrow),
            BorrowState::Shared(counter) => {
                *counter += 1;
                Ok(())
            }
        }
    }

    pub fn dec_shared(&mut self) -> CompileResult<()> {
        match self {
            BorrowState::Shared(counter) => {
                if *counter == 0 {
                    return Err(CompileError::IllegalBorrowState);
                }

                *counter -= 1;
                if *counter == 0 {
                    *self = BorrowState::None;
                }
                Ok(())
            },
            _ => Err(CompileError::IllegalBorrowState),
        }
    }

    pub fn borrow_mut(&mut self) -> CompileResult<()> {
        match self {
            BorrowState::None => {
                *self = BorrowState::Mut;
                Ok(())
            },
            _ => Err(CompileError::IllegalMutBorrow),
        }
    }

    pub fn free_mut(&mut self) -> CompileResult<()> {
        match self {
            BorrowState::Mut => {
                *self = BorrowState::None;
                Ok(())
            },
            _ => Err(CompileError::IllegalBorrowState)
        }
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            BorrowState::None => false,
            _ => true,
        }
    }

    pub fn is_borrowed_mut(&self) -> bool {
        match self {
            BorrowState::Mut => true,
            _ => false,
        }
    }

    pub fn is_borrowed_shared(&self) -> bool {
        match self {
            BorrowState::Shared(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarLoc {
    pub stack_frame: usize,
    pub name: String
}

#[derive(Debug)]
pub struct DataLoc {
    pub is_stack: bool,
    pub loc: usize,
    pub size: usize,
}

/// Wraps a variable declaration in ferrum source code.
/// It contains the name of the variable that is being declared, rather it is mutable or not and
/// the type of the variable.
#[derive(Clone, Debug)]
pub struct FerrumVarDeclare {
    name: String,
    ty: FerrumType,
    mutable: bool,
}

pub struct FerrumVariableHandle {
    name: String,
    declare: DataLoc,
    ty: FerrumType,
    mutable: bool,
    has_data: bool,
    is_alive: bool,
}

pub struct FerrumVariable {
    name: String,
    ty: FerrumType,
    pub loc: Option<DataSource>,
    mutable: bool,
    pub lifetime: LifeTime,
    pub borrow_state: BorrowState,
    stack_lvl: usize,
    do_invalidate: bool,
}

impl FerrumVariableHandle {
    pub fn new(
        name: String, ty: FerrumType, loc: DataLoc, mutable: bool, has_data: bool, is_alive: bool
    ) -> Self {
        FerrumVariableHandle {
            name, ty, declare: loc, mutable, has_data, is_alive
        }
    }
}

impl FerrumVariable {
    pub fn new(
        name: String, stack_lvl: usize, ty: FerrumType, loc: DataSource, mutable: bool, lifetime: LifeTime, do_invalidate: bool,
    ) -> Self {
        FerrumVariable {
            name,
            ty,
            loc: Some(loc),
            borrow_state: BorrowState::None,
            stack_lvl,
            mutable,
            lifetime,
            do_invalidate,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &FerrumType {
        &self.ty
    }

    pub fn is_mutable(&self) -> &bool {
        &self.mutable
    }

    pub fn is_valid(&self) -> bool {
        self.loc.is_some()
    }

    pub fn is_ref(&self) -> bool {
        match &self.loc {
            Some(DataSource::Reference(_, _, _)) => true,
            _ => false
        }
    }

    /// If the data source of this variable is a reference to an other variable, this function
    /// will decrement the reference counter for that other variable. Otherwise, this does nothing.
    pub fn try_invalidate_data(&mut self, compiler: &mut FerrumCompiler) -> CompileResult<()> {
        if self.borrow_state.is_borrowed() {
            return Err(CompileError::ModifiedBorrowedData(self.create_ref()));
        }

        if !self.do_invalidate {
            return Ok(())
        }

        match &mut self.loc {
            Some(s) => s.remove_dependencies(compiler),
            None => Ok(())
        }
    }

    /// Takes over the ownership of data shepherded by an other variable. Through this, the `other`
    /// Variable gets invalidated.
    pub fn take_ownership(&mut self, other: &mut Self, compiler: &mut FerrumCompiler) -> CompileResult<()> {
        if self.ty != other.ty {
            return Err(CompileError::DataTypeMismatch(
                self.ty.clone(), other.ty.clone()
            ));
        }

        self.try_invalidate_data(compiler)?;
        self.loc = None;
        mem::swap(&mut self.loc, &mut other.loc);
        Ok(())
    }

    /// Swaps data ownership with an other variable.
    pub fn swap(&mut self, other: &mut Self) -> CompileResult<()> {
        if self.ty != other.ty {
            return Err(CompileError::DataTypeMismatch(
                self.ty.clone(), other.ty.clone())
            );
        }
        mem::swap(&mut self.loc, &mut other.loc);
        Ok(())
    }

    pub fn borrow(&mut self, other: &mut Self, compiler: &mut FerrumCompiler) -> CompileResult<()> {
        match &self.ty {
            FerrumType::Ref(t) if **t == other.ty => {

                self.try_invalidate_data(compiler)?;
                other.borrow_state.inc_shared()?;
                self.do_invalidate = other.is_on_stack();
                if !self.do_invalidate {
                    compiler.inc_heap_rc(other);
                }

                match &mut self.loc {
                    Some(s) => s.new_ref(other.create_ref()),
                    None => Err(CompileError::VariableNotInitialized(self.create_ref()))
                }
            },
            FerrumType::MutRef(t) if **t == other.ty => {
                if !other.is_mutable() {
                    return Err(CompileError::DataNotMutable(other.create_ref()));
                }

                self.try_invalidate_data(compiler)?;
                other.borrow_state.borrow_mut()?;
                self.do_invalidate = other.is_on_stack();
                if !self.do_invalidate {
                    compiler.grab_heap_mut(other);
                }

                match &mut self.loc {
                    Some(s) => s.new_mut_ref(other.create_ref()),
                    None => Err(CompileError::VariableNotInitialized(self.create_ref())),
                }
            }
            _ => Err(CompileError::DataTypeMismatch(
                self.ty.clone(),
                FerrumType::Ref(Box::new(other.ty.clone())))
            )
        }
    }

    pub fn assign_data(&mut self, source: DataSource) -> CompileResult<()> {
        match &self.loc {
            None => {
                self.loc = Some(source);
                Ok(())
            }
            _ => Err(CompileError::AlreadyAssigned(self.create_ref()))
        }
    }

    pub fn create_ref(&self) -> VarLoc {
        VarLoc {
            stack_frame: self.stack_lvl,
            name: self.name.clone(),
        }
    }

    pub fn holds_data(&self) -> bool {
        match &self.loc {
            Some(DataSource::Location(_)) => true,
            _ => false,
        }
    }

    pub fn holds_ref(&self) -> bool {
        match &self.loc {
            Some(DataSource::Reference(_, _, _)) => true,
            Some(DataSource::MutReference(_, _, _)) => true,
            _ => false,
        }
    }

    pub fn is_on_heap(&self) -> bool {
        match &self.loc {
            None => false,
            Some(d) => d.is_heap_data()
        }
    }

    pub fn is_on_stack(&self) -> bool {
        match &self.loc {
            None => true,
            Some(d) => !d.is_heap_data()
        }
    }
}




pub struct FerrumVarDeclareTemplate {
    name: String,
    id: usize,
    mutable: bool,
}

impl GenericTemplate for FerrumVarDeclareTemplate {
    type Final = FerrumVarDeclare;

    fn generate_type(&self, table: &FerrumGenericsTable) -> Option<Self::Final> {
        Some(FerrumVarDeclare {
            name: self.name.clone(),
            ty: table[self.id].clone(),
            mutable: self.mutable
        })
    }
}
