use crate::{
    core::schema::{
        ColumnSchema, DBL_TYPE_NAME, DblDataType, INT_TYPE_NAME, IntDataType,
        STR_TYPE_NAME, SpreadsheetSchema, StrDataType, TableSchema,
    },
    json::ToJson,
    ql::lex::Literal,
};
use serde_json::{Number, Value, json};

impl ToJson for Literal {
    fn to_json(&self) -> Value {
        match self {
            Self::Int(val) => {
                Value::Number(Number::from_i128(*val as i128).unwrap())
            }
            Self::Dbl(val) => Value::Number(Number::from_f64(*val).unwrap()),
            Self::Str(val) => Value::String(val.to_string()),
        }
    }
}

impl ToJson for IntDataType {
    fn to_json(&self) -> Value {
        json!({"super": INT_TYPE_NAME, "nullable": self.nullable, "min": self.min, "max": self.max})
    }
}

impl ToJson for DblDataType {
    fn to_json(&self) -> Value {
        json!({"super": DBL_TYPE_NAME, "nullable": self.nullable, "min": self.min, "max": self.max})
    }
}

impl ToJson for StrDataType {
    fn to_json(&self) -> Value {
        json!({"super": STR_TYPE_NAME, "nullable": self.nullable, "min": self.min, "max": self.max})
    }
}

impl ToJson for ColumnSchema {
    fn to_json(&self) -> Value {
        match &self.default_value {
            Some(val) => {
                json!({
                    "column_type": self.column_type.to_json(),
                    "default_value": val.to_json()
                })
            }
            None => {
                json!({
                    "column_type": self.column_type.to_json()
                })
            }
        }
    }
}

impl ToJson for TableSchema {
    fn to_json(&self) -> Value {
        json!({
            "columns": Value::Object(
                self.column_names
                    .iter()
                    .map(|name| (name.to_string(), self.columns[name].to_json()))
                    .collect::<serde_json::Map<String, Value>>(),
            )
        })
    }
}

impl ToJson for SpreadsheetSchema {
    fn to_json(&self) -> Value {
        json!({
            "tables": Value::Object(self.table_names
                .iter()
                .map(|name| (name.to_string(), self.tables[name].to_json()))
                .collect::<serde_json::Map<String, Value>>())
        })
    }
}
