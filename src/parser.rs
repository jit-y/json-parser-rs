mod value;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::iter::Peekable;
use value::Value;

pub struct Parser<'c> {
    lexer: Peekable<Lexer<'c>>,
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Value> {
        match self.next_token() {
            None => return Err(anyhow!("err")),
            Some(tok) => match &tok.token_type {
                TokenType::Null => self.parse_null(),
                TokenType::String(val) => self.parse_string(val.clone()),
                TokenType::Number(val) => self.parse_number(*val),
                TokenType::LBracket => self.parse_array(),
                _ => return Err(anyhow!("unexpected token {:?}", tok)),
            },
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
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
        let mut res = vec![];

        while let Some(tok) = self.peek_token() {
            match tok.token_type {
                TokenType::RBracket => {
                    self.next_token();
                    break;
                }
                TokenType::Comma => {
                    self.next_token();
                    if let Some(tok) = self.peek_token() {
                        if tok.token_type == TokenType::RBracket {
                            return Err(anyhow!("Invalid token {}", tok.literal));
                        }
                    }
                }
                _ => {
                    let v = self.parse()?;
                    res.push(v);
                }
            }
        }

        Ok(Value::Array(res))
    }

    fn parse_object(&mut self) -> Result<Value> {
        unimplemented!()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tests: Vec<(&str, Value)> = vec![
            (r#""aaa""#, Value::String("aaa".to_string())),
            (r#"1e10"#, Value::Number(1e10_f64)),
            (
                r#"["aaa", 1e10]"#,
                Value::Array(vec![
                    Value::String("aaa".to_string()),
                    Value::Number(1e10_f64),
                ]),
            ),
        ];

        for (t, e) in tests {
            let l = Lexer::new(t);
            let mut p = Parser::new(l);
            let v = p.parse();

            dbg!(&v);

            assert!(v.is_ok());
            assert_eq!(v.unwrap(), e);
        }
    }
}
