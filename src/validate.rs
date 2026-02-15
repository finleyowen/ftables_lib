use crate::core::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    rc::Rc,
};

const STR_LEN_MSG: &str = "String length can't be negative!";

pub enum Symbol {
    TypeDef(Rc<ParentType>),
    Table(Rc<TableSchema>),
}

pub struct SymbolTable(HashMap<String, Symbol>);

pub trait Validate {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()>;
}

impl<T> Validate for Range<T>
where
    T: Display + PartialOrd + Clone,
{
    fn validate(&self, _: &mut SymbolTable) -> anyhow::Result<()> {
        if let Some(min) = self.min.clone()
            && let Some(max) = self.max.clone()
            && min > max
        {
            return Err(anyhow::anyhow!(
                "Can't have min > max ({min} > {max})"
            ));
        }
        Ok(())
    }
}

impl Validate for ParentType {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        match &self {
            ParentType::Int(range) => range.validate(symbol_table),
            ParentType::Dbl(range) => range.validate(symbol_table),
            ParentType::Str(range) => {
                range.validate(symbol_table)?;
                if let Some(min) = range.min
                    && min < 0
                {
                    return Err(anyhow::anyhow!(STR_LEN_MSG));
                }

                if let Some(max) = range.max
                    && max < 0
                {
                    return Err(anyhow::anyhow!(STR_LEN_MSG));
                }

                Ok(())
            }
            ParentType::Ident(ident) => {
                if !symbol_table.0.contains_key(ident) {
                    return Err(anyhow::anyhow!("Unrecognised dtype {ident}"));
                }
                Ok(())
            }
        }
    }
}

impl Validate for ColumnSchema {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        self.dtype.parent.validate(symbol_table)?;
        Ok(())
    }
}

impl Validate for TableSchema {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        let mut column_names: HashSet<&str> = HashSet::new();
        column_names.reserve(self.columns.len());

        for column in &self.columns {
            let column_name = &column.column_name;
            if !column_names.insert(column_name) {
                return Err(anyhow::anyhow!(
                    "Can't have two columns named {column_name}!"
                ));
            }

            column.validate(symbol_table)?;
        }

        Ok(())
    }
}

impl Validate for Stmt {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        match self {
            Self::TypeDef(name, parent_type) => {
                if let Some(_) = symbol_table
                    .0
                    .insert(name.clone(), Symbol::TypeDef(parent_type.clone()))
                {
                    return Err(anyhow::anyhow!(
                        "Can't have two defined types named {name}"
                    ));
                }
                parent_type.validate(symbol_table)?;
                return Ok(());
            }
            Self::Table(table) => {
                if let Some(_) = symbol_table.0.insert(
                    table.table_name.clone(),
                    Symbol::Table(table.clone()),
                ) {
                    return Err(anyhow::anyhow!(
                        "Can't have two tables named {}",
                        table.table_name
                    ));
                }
                table.validate(symbol_table)?;
                return Ok(());
            }
        }
    }
}

impl Validate for SpreadsheetSchema {
    fn validate(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        for stmt in &self.stmts {
            stmt.validate(symbol_table)?;
        }
        Ok(())
    }
}

pub fn validate_prgm(prgm: &SpreadsheetSchema) -> anyhow::Result<SymbolTable> {
    let mut sym_table = SymbolTable(HashMap::new());
    prgm.validate(&mut sym_table)?;
    Ok(sym_table)
}
