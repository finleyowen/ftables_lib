use std::rc::Rc;

use crate::{
    core::schema::{
        ColumnSchema, DataType, DblDataType, IntDataType, StrDataType,
        TableSchema,
    },
    ddl::{
        core::{Stmt, Symbol, SymbolTable},
        lex::Token,
    },
};
use rlrl::parse::{ParseResult, TokenQueue};
pub trait Parse: Sized {
    fn parse(
        tq: &TokenQueue<Token>,
        symtable: &mut SymbolTable,
    ) -> ParseResult<Self>;
}

impl Parse for IntDataType {
    fn parse(
        tq: &TokenQueue<Token>,
        _symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        // create a mutable copy
        let mut tq = tq.clone();

        tq.consume_eq(Token::OAngle)?;

        // consume min
        let min = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_i32().unwrap())
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::Comma)?;

        // consume max
        let max = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_i32().unwrap())
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::CAngle)?;

        // consume ?
        let nullable = tq.consume_eq(Token::QMark).is_ok();

        // done
        Ok((IntDataType::new(min, max, nullable), tq.get_idx()))
    }
}

impl Parse for DblDataType {
    fn parse(
        tq: &TokenQueue<Token>,
        _symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        // create a mutable copy
        let mut tq = tq.clone();

        tq.consume_eq(Token::OAngle)?;

        // consume min
        let min = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_f64().unwrap())
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::Comma)?;

        // consume max
        let max = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_f64().unwrap())
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::CAngle)?;

        // consume ?
        let nullable = tq.consume_eq(Token::QMark).is_ok();

        // done
        Ok((DblDataType::new(min, max, nullable), tq.get_idx()))
    }
}

impl Parse for StrDataType {
    fn parse(
        tq: &TokenQueue<Token>,
        _symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        // create a mutable copy
        let mut tq = tq.clone();

        tq.consume_eq(Token::OAngle)?;

        // consume min
        let min = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_i32().unwrap().try_into()?)
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::Comma)?;

        // consume max
        let max = match tq.clone().peek_matching(|token| token.is_literal()) {
            Ok(token) => {
                tq.increment();
                let literal = token.get_literal().unwrap();
                if literal.is_i32() {
                    Some(literal.get_i32().unwrap().try_into()?)
                } else {
                    return Err(anyhow::anyhow!("Couldn't parse int literal!"));
                }
            }
            Err(_) => None,
        };

        tq.consume_eq(Token::CAngle)?;

        // consume ?
        let nullable = tq.consume_eq(Token::QMark).is_ok();

        // done
        Ok((StrDataType::new(min, max, nullable), tq.get_idx()))
    }
}

fn parse_data_type(
    tq: &TokenQueue<Token>,
    symtable: &mut SymbolTable,
) -> ParseResult<Rc<dyn DataType>> {
    let mut tq = tq.clone();

    let ident_tok =
        tq.consume_matching(|tok| tok.is_ident_or_str_literal_tok())?;
    let ident = ident_tok
        .get_ident_or_str_literal()
        .ok_or(anyhow::anyhow!("Expected an identifier!"))?;

    match &ident as &str {
        "int" => {
            let (dtype, end) = IntDataType::parse(&tq, symtable)?;
            return Ok((Rc::new(dtype), end));
        }
        "dbl" => {
            let (dtype, end) = DblDataType::parse(&tq, symtable)?;
            return Ok((Rc::new(dtype), end));
        }
        "str" => {
            let (dtype, end) = StrDataType::parse(&tq, symtable)?;
            return Ok((Rc::new(dtype), end));
        }
        _ => {
            if let Some(Symbol::DataType(dtype)) = symtable.get(&ident) {
                return Ok((dtype.clone(), tq.get_idx()));
            } else {
                return Err(anyhow::anyhow!(
                    "Unrecognised type name {}",
                    ident
                ));
            }
        }
    }
}

impl Parse for ColumnSchema {
    fn parse(
        tq: &TokenQueue<Token>,
        symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        let mut tq: TokenQueue<Token> = tq.clone();

        let column_name = tq
            .consume_matching(|tok| tok.is_ident_or_str_literal_tok())?
            .get_ident_or_str_literal()
            .ok_or(anyhow::anyhow!("Couldn't get column name!"))?
            .into();

        tq.consume_eq(Token::Colon)?;

        let column_type = tq.parse_with_mut(parse_data_type, symtable)?;

        Ok((ColumnSchema::new(column_name, column_type), tq.get_idx()))
    }
}

impl Parse for TableSchema {
    fn parse(
        tq: &TokenQueue<Token>,
        symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        let mut tq_mut = tq.clone();

        let table_name = tq
            .peek_matching(|tok| tok.is_ident_or_str_literal_tok())?
            .get_ident_or_str_literal()
            .ok_or(anyhow::anyhow!("Couldn't get table name!"))?;
        tq_mut.increment();

        tq_mut.consume_eq(Token::OParen)?;

        let mut columns = vec![];

        while let Ok(column) =
            tq_mut.parse_with_mut(ColumnSchema::parse, symtable)
        {
            columns.push(column);
            if tq_mut.consume_eq(Token::Comma).is_err() {
                break;
            }
        }

        tq_mut.consume_eq(Token::CParen)?;
        Ok((TableSchema::new(table_name, columns), tq_mut.get_idx()))
    }
}

impl Parse for Stmt {
    fn parse(
        tq: &TokenQueue<Token>,
        symtable: &mut SymbolTable,
    ) -> ParseResult<Self> {
        let mut tq = tq.clone();

        match tq.consume() {
            Ok(Token::TypeKwd) => {
                let type_name: Rc<str> = tq
                    .consume_matching(|tok| tok.is_ident_or_str_literal_tok())?
                    .get_ident_or_str_literal()
                    .ok_or(anyhow::anyhow!("Couldn't get type name!"))?
                    .into();

                let data_type = tq.parse_with_mut(parse_data_type, symtable)?;

                if let Some(_) = symtable.insert(
                    type_name.clone(),
                    Symbol::DataType(data_type.clone()),
                ) {
                    return Err(anyhow::anyhow!(
                        "Symbol {} is already assigned!",
                        &type_name
                    ));
                }

                Ok((Stmt::DataType(type_name, data_type), tq.get_idx()))
            }
            Ok(Token::TableKwd) => {
                let table_schema =
                    Rc::new(tq.parse_with_mut(TableSchema::parse, symtable)?);

                if let Some(_) = symtable.insert(
                    table_schema.get_name().clone(),
                    Symbol::TableSchema(table_schema.clone()),
                ) {
                    return Err(anyhow::anyhow!(
                        "Symbol {} is already assigned!",
                        &table_schema.get_name().clone()
                    ));
                }

                Ok((Stmt::TableSchema(table_schema), tq.get_idx()))
            }
            Ok(tok) => {
                dbg!(tok);
                Err(anyhow::anyhow!("Couldn't parse statement!"))
            }
            Err(_) => Err(anyhow::anyhow!("Couldn't parse statement!")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rlrl::parse::TokenQueue;

    use crate::ddl::{
        lex::{Token, setup_lexer},
        parse::{Parse, Stmt},
    };

    fn lex(s: &str) -> anyhow::Result<TokenQueue<Token>> {
        let lexer = setup_lexer();
        Ok(TokenQueue::from(lexer.lex(s)?))
    }

    #[test]
    fn parse_stmt_test() -> anyhow::Result<()> {
        let mut tq = lex("type myType int<1,5>")?;
        let mut symtable = HashMap::new();

        let stmt = tq.parse_with_mut(Stmt::parse, &mut symtable)?;
        dbg!(&stmt);
        dbg!(&symtable);

        Ok(())
    }
}
