use core::panic;

use super::tokenizer::{Token, TokenKind, Tokenizer};

pub enum Stmt{
    INTEGERLITERAL(i32),
    FLOATLITERAL(f32),
    IDENTIFIER(String),
    BINARYEXPR(Box<Stmt>, Box<Stmt>, Token),
    PARENTHESIZED(Box<Stmt>),
    UNARY(Box<Stmt>)
}


pub struct Parser {
    tokenizer: Tokenizer,
    pub program: Vec<Stmt>,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser {
            tokenizer,
            program: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Some(token) => match token.kind {
                    TokenKind::EOF => {
                        self.tokenizer.next();
                        break;
                    }
                    _ => {
                        let stmt = self.parse_expr();
                        self.program.push( stmt );
                    }
                },
                None => break,
            }
        }
    }

    fn parse_expr(&mut self) -> Stmt{
        return self.parse_comparison_expr();
    }

    fn parse_comparison_expr(&mut self) -> Stmt{
        let mut left = self.parse_additive_expr();

        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Some(token) => match token.kind {
                    TokenKind::LT | TokenKind::LTEQ | TokenKind::GTEQ | TokenKind::GT | TokenKind::EQ | TokenKind::NEQ => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_additive_expr();
                        left = Stmt::BINARYEXPR(Box::new(left), Box::new(right), operator);
                    }
                    _ => {
                        return left;
                    }
                },
                _ => panic!("end of file early"),
            }
        }
    }

    fn parse_additive_expr(&mut self) -> Stmt{
        let mut left = self.parse_mult_expr();

        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Some(token) => match token.kind {
                    TokenKind::ADD | TokenKind::SUB => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_mult_expr();
                        left = Stmt::BINARYEXPR(Box::new(left), Box::new(right), operator);
                    }
                    _ => {
                        return left;
                    }
                },
                _ => panic!("end of file early"),
            }
        }
    }


    fn parse_mult_expr(&mut self) -> Stmt {
        let mut left = self.parse_unary_expr();

        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Some(token) => match token.kind {
                    TokenKind::MUL | TokenKind::DIV | TokenKind::MOD => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_unary_expr();
                        left = Stmt::BINARYEXPR(Box::new(left), Box::new(right), operator);
                    }
                    _ => {
                        return left;
                    }
                },
                _ => panic!("end oif file early"),
            }
        }
    }


    fn parse_parenthesized_exp(&mut self) -> Stmt {
        let token = self.tokenizer.peek_token();
        let stmt = match token {
            Some(token) => match token.kind {
                TokenKind::LPAREN => {
                    self.tokenizer.next_token();
                    let stmt = self.parse_comparison_expr();
                    let stmt = Stmt::PARENTHESIZED(Box::new(stmt));
                    let _closing = self.tokenizer.next_token();
                    stmt
                },
                _ => { self.parse_primary_expr() }
            },
            None => { panic!("unknown token") }
        };
        return stmt;
    }


    fn parse_unary_expr(&mut self) -> Stmt {
        let token = self.tokenizer.peek_token();
        let stmt = match token {
            Some(token) => match token.kind {
                TokenKind::BANG => {
                    self.tokenizer.next().unwrap();
                    let operand = self.parse_parenthesized_exp();
                    Stmt::UNARY(Box::new(operand))
                },
                _ => { self.parse_parenthesized_exp() }
            },
            None => {
                panic!("Unknown token");
            }
        };
        return stmt;
    }

    fn parse_primary_expr(&mut self) -> Stmt{
        let token = self.tokenizer.peek_token();

        let stmt = match token {
            Some(token) => match token.kind {
                TokenKind::IDENTIFIER(a) => {
                    self.tokenizer.next();
                    Stmt::IDENTIFIER(a)
                }
                TokenKind::INT(a) => {
                    self.tokenizer.next();
                    Stmt::INTEGERLITERAL(a)
                }
                TokenKind::FLOAT(a) => {
                    self.tokenizer.next();
                    Stmt::FLOATLITERAL(a)
                }
                _ => panic!("Unknown Token"),
            },
            None => panic!("end of file early"),
        };

        return stmt;
    }
}
