use std::rc::Rc;

/// Enum representing a literal token in the DDL
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i32),
    Dbl(f64),
    Str(String),
}

impl Literal {
    pub fn is_str_literal(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }

    /// Clones the string value if `self` is a `Literal::Str`, otherwise
    /// returns `None`.
    pub fn get_str(&self) -> Option<&String> {
        match self {
            Self::Str(val) => Some(val),
            _ => None,
        }
    }

    pub fn is_int_literal(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    /// Clones the int value if `self` is a `Literal::Int`, otherwise returns
    /// `None`.
    pub fn get_int(&self) -> Option<i32> {
        match self {
            Self::Int(val) => Some(val.clone()),
            _ => None,
        }
    }

    pub fn is_dbl_literal(&self) -> bool {
        match self {
            Self::Dbl(_) | Self::Int(_) => true,
            _ => false,
        }
    }

    /// Clones the double value if `self` is a `Literal::Dbl`, otherwise returns
    /// `None`.
    pub fn get_dbl(&self) -> Option<f64> {
        match self {
            Self::Dbl(val) => Some(val.clone()),
            Self::Int(val) => Some(*val as f64),
            _ => None,
        }
    }
}

/// A doubly-ended range of type `T`.
#[derive(Debug, PartialEq)]
pub struct Range<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

/// Type alias over a `Range<T> where T = i32`
pub type IntRange = Range<i32>;

/// Type alias over a `Range<T> where T = f64`
pub type DblRange = Range<f64>;

/// The basic types available in the application.
#[derive(Debug, PartialEq)]
pub enum ParentType {
    Int(IntRange),
    Str(IntRange),
    Dbl(DblRange),
    Ident(String),
}

/// Derived data types in the applicaiton.
#[derive(Debug, PartialEq)]
pub struct DType {
    pub parent: ParentType,
    pub nullable: bool,
}

#[derive(Debug, PartialEq)]
pub struct ColumnSchema {
    pub column_name: String,
    pub dtype: DType,
    pub default_value: Option<Literal>,
}

#[derive(Debug, PartialEq)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<ColumnSchema>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    TypeDef(String, Rc<ParentType>),
    Table(Rc<TableSchema>),
}

#[derive(Debug, PartialEq)]
pub struct SpreadsheetSchema {
    pub stmts: Vec<Stmt>,
}
