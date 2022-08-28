use std::error::Error;
use std::fmt::{Debug, Display, format, Formatter, Write};
use crate::lang::types::FerrumType;
use crate::lang::variable::VarLoc;

pub enum CompileError {
    DataTypeMismatch(FerrumType, FerrumType),
    LifetimeMismatch,
    IllegalMutBorrow,
    IllegalSharedBorrow,
    IllegalBorrowState,
    DataNotMutable(VarLoc),
    AlreadyAssigned(VarLoc),
    IllegalDataSource(String),
    VariableNotInitialized(VarLoc),
    ModifiedBorrowedData(VarLoc),
    UnknownVariable(VarLoc),
}

pub type CompileResult<T> = Result<T, CompileError>;

impl Debug for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::DataTypeMismatch(got, exp) => {
                f.write_str(&format!("got {:?} while {:?} was expected", got, exp))
            }
            CompileError::LifetimeMismatch => {
                f.write_str("Variable does not life long enough")
            }
            CompileError::IllegalMutBorrow => {
                f.write_str("Cannot borrow as mut")
            }
            CompileError::IllegalSharedBorrow => {
                f.write_str("Cannot borrow shared")
            }
            CompileError::IllegalBorrowState => {
                f.write_str("Illegal borrow checker state. This is a compiler error")
            }
            CompileError::DataNotMutable(v) => {
                f.write_str(&format!("Variable {v:?} is not mutable"))
            }
            CompileError::AlreadyAssigned(v) => {
                f.write_str(&format!("Cannot assign data to {v:?} twice"))
            }
            CompileError::IllegalDataSource(s) => {
                f.write_str(&format!("Expected {s} data source"))
            }
            CompileError::VariableNotInitialized(v) => {
                f.write_str(&format!("Tried to assign data to uninitialized variable {v:?}"))
            }
            CompileError::ModifiedBorrowedData(v) => {
                f.write_str(&format!("Cannot modify {v:?} while it is borrowed"))
            }
            CompileError::UnknownVariable(v) => {
                f.write_str(&format!("Variable {v:?} not found in current scope"))
            }
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CompileError {

}
