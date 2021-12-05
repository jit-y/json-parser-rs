mod value;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use anyhow::{anyhow, Result};
use value::Value;

pub struct Parser<'c> {
    lexer: Lexer<'c>,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Self {
        let mut parser = Self {
            lexer: lexer,
            current_token: None,
            next_token: None,
        };

        parser.read_token();
        parser.read_token();

        parser
    }

    pub fn parse(&mut self) -> Result<Value> {
        match &self.current_token {
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

    fn read_token(&mut self) {
        std::mem::swap(&mut self.next_token, &mut self.current_token);
        self.next_token = self.lexer.next();
    }

    fn parse_null(&mut self) -> Result<Value> {
        self.read_token();
        Ok(Value::Null)
    }

    fn parse_string(&mut self, val: String) -> Result<Value> {
        self.read_token();
        Ok(Value::String(val))
    }

    fn parse_number(&mut self, val: f64) -> Result<Value> {
        self.read_token();
        Ok(Value::Number(val))
    }

    fn parse_array(&mut self) -> Result<Value> {
        let mut res = vec![];

        self.read_token();

        while let Some(tok) = &self.current_token {
            match tok.token_type {
                TokenType::RBracket => {
                    self.read_token();
                    break;
                }
                TokenType::Comma => match &self.next_token {
                    Some(tok) => {
                        if tok.token_type == TokenType::LBracket {
                            return Err(anyhow!("invalid"));
                        }
                        self.read_token();
                    }
                    None => return Err(anyhow!("invalid")),
                },
                _ => {
                    let v = self.parse()?;
                    res.push(v);
                    self.read_token();
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
