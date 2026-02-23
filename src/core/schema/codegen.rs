use std::fmt::Display;

use crate::{
    core::schema::{
        ColumnSchema, DBL_TYPE_NAME, DblDataType, INT_TYPE_NAME, IntDataType,
        STR_TYPE_NAME, SpreadsheetSchema, StrDataType, TableSchema,
    },
    ql::Stmt,
};

impl Display for IntDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = match self.min {
            Some(val) => &val.to_string(),
            None => "",
        };

        let max = match self.max {
            Some(val) => &val.to_string(),
            None => "",
        };

        write!(
            f,
            "{}<{}, {}>{}",
            INT_TYPE_NAME,
            min,
            max,
            if self.nullable { "?" } else { "" }
        )
    }
}

impl Display for DblDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = match self.min {
            Some(val) => &val.to_string(),
            None => "",
        };

        let max = match self.max {
            Some(val) => &val.to_string(),
            None => "",
        };

        write!(
            f,
            "{}<{}, {}>{}",
            DBL_TYPE_NAME,
            min,
            max,
            if self.nullable { "?" } else { "" }
        )
    }
}

impl Display for StrDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = match self.min {
            Some(val) => &val.to_string(),
            None => "",
        };

        let max = match self.max {
            Some(val) => &val.to_string(),
            None => "",
        };

        write!(
            f,
            "{}<{}, {}>{}",
            STR_TYPE_NAME,
            min,
            max,
            if self.nullable { "?" } else { "" }
        )
    }
}

impl Display for ColumnSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.default_value {
            Some(val) => write!(f, "{} = {}", self.column_type, val),
            None => write!(f, "{}", self.column_type),
        }
    }
}

impl Display for TableSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column_strs: Vec<String> = self
            .column_names
            .iter()
            .map(|name| format!("{name}: {}", self.columns[name]))
            .collect();
        let columns_str = column_strs.join(", ");

        write!(f, "({})", columns_str)
    }
}

impl Display for SpreadsheetSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in &self.table_names {
            writeln!(f, "table {name} {};", self.tables[name])?;
        }
        Ok(())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TableSchema(name, schema) => {
                write!(f, "table {name} {schema};",)
            }
            Self::TypeDef(type_name, data_type) => {
                write!(f, "type {type_name} {data_type};")
            }
        }
    }
}
