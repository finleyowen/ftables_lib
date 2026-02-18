use std::{collections::HashMap, rc::Rc};

use crate::core::schema::{SharedDataType, SharedTableSchema};

/// Enum representing a literal token in the DDL
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i32),
    Dbl(f64),
    Str(Rc<str>),
}

impl Literal {
    pub fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }

    /// Clones the string value if `self` is a `Literal::Str`, otherwise
    /// returns `None`.
    pub fn get_str(&self) -> Option<Rc<str>> {
        match self {
            Self::Str(val) => Some(val.clone()),
            _ => None,
        }
    }

    pub fn is_i32(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    /// Clones the int value if `self` is a `Literal::Int`, otherwise returns
    /// `None`.
    pub fn get_i32(&self) -> Option<i32> {
        match self {
            Self::Int(val) => Some(val.clone()),
            _ => None,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Self::Dbl(_) | Self::Int(_) => true,
            _ => false,
        }
    }

    /// Clones the double value if `self` is a `Literal::Dbl`, otherwise returns
    /// `None`.
    pub fn get_f64(&self) -> Option<f64> {
        match self {
            Self::Dbl(val) => Some(val.clone()),
            Self::Int(val) => Some(*val as f64),
            _ => None,
        }
    }
}

/// A symbol in the symbol table; the value of a variable.
#[derive(Debug)]
pub enum Symbol {
    TableSchema(SharedTableSchema),
    DataType(SharedDataType),
}

/// A table or map where the keys are variable names/identifiers and the values
/// are of type [Symbol].
pub type SymbolTable = HashMap<Rc<str>, Symbol>;

/// A statement in the DDL.
#[derive(Debug)]
pub enum Stmt {
    TableSchema(SharedTableSchema),
    DataType(Rc<str>, SharedDataType),
}
