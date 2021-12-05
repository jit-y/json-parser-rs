mod value;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use anyhow::{anyhow, Result};
use value::Value;

pub struct Parser<'c> {
    lexer: Lexer<'c>,
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Value> {
        match self.lexer.next() {
            None => return Err(anyhow!("err")),
            Some(tok) => match tok.token_type {
                TokenType::Null => self.parse_null(),
                TokenType::String(val) => self.parse_string(val),
                TokenType::Number(val) => self.parse_number(val),
                TokenType::LBracket => self.parse_array(),
                _ => unimplemented!(),
            },
        }
    }

    fn parse_null(&mut self) -> Result<Value> {
        Ok(Value::Null)
    }

    fn parse_string(&mut self, val: String) -> Result<Value> {
        Ok(Value::String(val))
    }

    fn parse_number(&mut self, val: f64) -> Result<Value> {
        Ok(Value::Number(val))
    }

    fn parse_array(&mut self) -> Result<Value> {
        unimplemented!()
    }

    fn parse_object(&mut self) -> Result<Value> {
        unimplemented!()
    }
}
