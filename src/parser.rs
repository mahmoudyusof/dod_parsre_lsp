use std::{iter::Peekable, slice::Iter};

use crate::types::{diagnostics::Diagnostic, Position, Range};

use super::tokenizer::{Token, TokenKind};

pub enum Stmt{
    EOF,
    VariableDeclaration(String, Expr),
    IFSTATEMENT(Expr, Vec<Stmt>),
    EXPR(Expr),
}
pub enum Expr {  
    INTEGERLITERAL(i32),
    FLOATLITERAL(f32),
    IDENTIFIER(String),
    BINARYEXPR(Box<Expr>, Box<Expr>, Token),
    PARENTHESIZED(Box<Expr>),
    UNARY(Box<Expr>)
}


pub struct Parser<'a>{
    tokenizer: Peekable<Iter<'a, Token>>,
    pub program: Vec<Stmt>,
    pub diagnostics: Vec<Diagnostic>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser {
            tokenizer: tokens.iter().peekable(),
            program: Vec::new(),
            diagnostics: Vec::new()
        }
    }


    fn parse_stmt(&mut self) -> Result<Stmt, Diagnostic> {
        let token = self.tokenizer.peek();
        match token {
            Some(token) => match &token.kind {
                TokenKind::EOF => {
                    self.tokenizer.next();
                    return Ok(Stmt::EOF);
                }
                TokenKind::KEYWORD(keyword) => {
                    if keyword == "let" {
                        let stmt = self.parse_variable_declaration()?;
                        return Ok(stmt);
                    } else if keyword == "if" {
                        // consume if
                        let previous = self.tokenizer.next().unwrap();
                        //parse paren
                        let token = self.tokenizer.next();
                        match token {
                            Some(token) => {
                                match token.kind {
                                    TokenKind::LPAREN => {},
                                    _ => {
                                        let diagnostic = Diagnostic {
                                            range: Range {
                                                start: Position { line: token.line - 1, character: token.column - 1 },
                                                end: Position { line: token.line - 1, character: token.column }
                                            },
                                            severity: 1,
                                            message: format_args!("invalid syntax at line {:?} for if statement, expected '(' at column {:?} found {:?}", token.line, token.column, token.kind).to_string() 
                                        };
                                        return Err(diagnostic);
                                    }
                                }
                            },
                            None => {
                                let diagnostic = Diagnostic {
                                    range: Range {
                                        start: Position { line: previous.line - 1, character: previous.column + 1 },
                                        end: Position { line: previous.line - 1, character: previous.column + 2 }
                                    },
                                    severity: 1,
                                    message: format_args!("invalid syntax at line {:?} for if statement, expected '(' at column {:?} found 'EOF'", previous.line, previous.column + 2).to_string() 
                                };
                                return Err(diagnostic);
                            }
                        }
                        //parse expr
                        let expr = self.parse_expr()?;
                        //parse paren
                        let token = self.tokenizer.next();

                        match token {
                            Some(token) => {
                                match token.kind {
                                    TokenKind::RPAREN => {},
                                    _ => {

                                        let diagnostic = Diagnostic {
                                            range: Range {
                                                start: Position { line: token.line - 1, character: token.column - 1 },
                                                end: Position { line: token.line - 1, character: token.column }
                                            },
                                            severity: 1,
                                            message: format_args!("invalid syntax at line {:?} for if statement, expected ')' at column {:?} found {:?}", token.line, token.column, token.kind).to_string() 
                                        };
                                        return Err(diagnostic);
                                    }
                                }
                            },
                            None => {
                                let diagnostic = Diagnostic {
                                    range: Range {
                                        start: Position { line: previous.line - 1, character: previous.column + 1 },
                                        end: Position { line: previous.line - 1, character: previous.column + 2 }
                                    },
                                    severity: 1,
                                    message: format_args!("invalid syntax at line {:?} for if statement, expected '(' at column {:?} found 'EOF'", previous.line, previous.column + 2).to_string() 
                                };
                                return Err(diagnostic);
                            }
                        };

                        let token = self.tokenizer.next();

                        match token {
                            Some(token) => {
                                match token.kind {
                                    TokenKind::LCURLY => {},
                                    _ => {

                                        let diagnostic = Diagnostic {
                                            range: Range {
                                                start: Position { line: token.line - 1, character: token.column - 1 },
                                                end: Position { line: token.line - 1, character: token.column }
                                            },
                                            severity: 1,
                                            message: format_args!("invalid syntax at line {:?} for if statement, expected '{{' at column {:?} found {:?}", token.line, token.column, token.kind).to_string() 
                                        };
                                        return Err(diagnostic);
                                    }
                                }
                            },
                            None => {
                                let diagnostic = Diagnostic {
                                    range: Range {
                                        start: Position { line: previous.line - 1, character: previous.column + 1 },
                                        end: Position { line: previous.line - 1, character: previous.column + 2 }
                                    },
                                    severity: 1,
                                    message: format_args!("invalid syntax at line {:?} for if statement, expected '(' at column {:?} found 'EOF'", previous.line, previous.column + 2).to_string() 
                                };
                                return Err(diagnostic);
                            }
                        }

                        //parse statements
                        let mut statements = Vec::new();
                        loop {
                            let statement = self.parse_stmt()?;
                            statements.push(statement);
                            let token = self.tokenizer.peek();
                            if let Some(token) = token {
                                if let TokenKind::RCURLY = token.kind {
                                    break;
                                }

                                if let TokenKind::EOF = token.kind {
                                    
                                    let diagnostic = Diagnostic {
                                        range: Range {
                                            start: Position { line: previous.line - 1, character: previous.column + 1 },
                                            end: Position { line: previous.line - 1, character: previous.column + 2 }
                                        },
                                        severity: 1,
                                        message: format_args!("invalid syntax at line {:?} for if statement, expected ')' at column {:?} found 'EOF'", previous.line, previous.column + 2).to_string() 
                                    };
                                    return Err(diagnostic);
                                }
                            } else {
                                let diagnostic = Diagnostic {
                                    range: Range {
                                        start: Position { line: previous.line - 1, character: previous.column + 1 },
                                        end: Position { line: previous.line - 1, character: previous.column + 2 }
                                    },
                                    severity: 1,
                                    message: format_args!("invalid syntax at line {:?} for if statement, expected '(' at column {:?} found 'EOF'", previous.line, previous.column + 2).to_string() 
                                };
                                return Err(diagnostic);
                            }
                        }
                        // consume closing curly
                        self.tokenizer.next().unwrap();
                        return Ok(Stmt::IFSTATEMENT(expr, statements));
                    } else {
                        return Err(Diagnostic {
                            range: Range {
                                start: Position {line: token.line - 1, character: token.column - 1},
                                end:  Position {line: token.line - 1, character: token.column },
                            },
                            severity: 1,
                            message:format_args!("unimplemented keyword, '{:?}'", keyword).to_string() 
                        });
                    }
                }
                _ => {
                    let expr = self.parse_expr()?;
                    return Ok(Stmt::EXPR(expr));
                }
            },
            None => return Ok(Stmt::EOF)
        }
    }


    pub fn parse(&mut self) {
        loop {
            match self.parse_stmt() {
                Ok(stmt) => {
                    match stmt {
                        Stmt::EOF => { break; },
                        _ => { self.program.push(stmt); }
                    }
                },
                Err (diagnostic) => {
                    self.diagnostics.push(diagnostic);
                    break;
                }
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Stmt, Diagnostic> {
        // consume let or const
        self.tokenizer.next().unwrap();
        // consume identifier
        let identifier = self.tokenizer.next().unwrap();
        let identifier = match &identifier.kind {
            TokenKind::IDENTIFIER(a) => {
                a
            }
            _ => {
                return Err(Diagnostic {
                    range: Range {
                        start: Position {line: identifier.line, character: identifier.column},
                        end:  Position {line: identifier.line, character: identifier.column + 1},
                    },
                    severity: 1,
                    message:format_args!("unimplemented keyword, '{:?}'", identifier.kind).to_string() 
                })
            }
        };

        // consume equal character
        self.tokenizer.next().unwrap();
        // parse expression
        let expression = self.parse_expr()?;
        // consume semicolon
        self.tokenizer.next().unwrap();
        return Ok(Stmt::VariableDeclaration(identifier.to_string(), expression));
    }

    fn parse_expr(&mut self) -> Result<Expr, Diagnostic>{
        let expr = self.parse_comparison_expr()?;
        return Ok(expr);
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr, Diagnostic>{
        let mut left = self.parse_additive_expr()?;

        loop {
            let token = self.tokenizer.peek();
            match token {
                Some(token) => match token.kind {
                    TokenKind::LT | TokenKind::LTEQ | TokenKind::GTEQ | TokenKind::GT | TokenKind::EQ | TokenKind::NEQ => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_additive_expr()?;
                        left = Expr::BINARYEXPR(Box::new(left), Box::new(right), operator.clone());
                    }
                    _ => {
                        return Ok(left);
                    }
                },
                _ => {
                    return Err(Diagnostic {
                        range: Range {
                            start: Position {line: 0, character: 0},
                            end:  Position {line: 0, character: 1},
                        },
                        severity: 1,
                        message: "unexpected end of file".to_string() 
                    });
                },
            }
        }
    }

    fn parse_additive_expr(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_mult_expr()?;

        loop {
            let token = self.tokenizer.peek();
            match token {
                Some(token) => match token.kind {
                    TokenKind::ADD | TokenKind::SUB => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_mult_expr()?;
                        left = Expr::BINARYEXPR(Box::new(left), Box::new(right), operator.clone());
                    }
                    _ => {
                        return Ok(left);
                    }
                },
                _ => {
                    return Err(Diagnostic {
                        range: Range {
                            start: Position {line: 0, character: 0},
                            end:  Position {line: 0, character: 1},
                        },
                        severity: 1,
                        message: "unexpected end of file".to_string() 
                    });
                }
            }
        }
    }


    fn parse_mult_expr(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.parse_unary_expr()?;

        loop {
            let token = self.tokenizer.peek();
            match token {
                Some(token) => match token.kind {
                    TokenKind::MUL | TokenKind::DIV | TokenKind::MOD => {
                        let operator = self.tokenizer.next().unwrap();
                        let right = self.parse_unary_expr()?;
                        left = Expr::BINARYEXPR(Box::new(left), Box::new(right), operator.clone());
                    }
                    _ => {
                        return Ok(left);
                    }
                },
                _ => {

                    return Err(Diagnostic {
                        range: Range {
                            start: Position {line: 0, character: 0},
                            end:  Position {line: 0, character: 1},
                        },
                        severity: 1,
                        message: "unexpected end of file".to_string() 
                    });
                },
            }
        }
    }


    fn parse_parenthesized_exp(&mut self) -> Result<Expr, Diagnostic>{
        let token = self.tokenizer.peek();
        let stmt = match token {
            Some(token) => match token.kind {
                TokenKind::LPAREN => {
                    self.tokenizer.next();
                    let stmt = self.parse_comparison_expr()?;
                    let stmt = Expr::PARENTHESIZED(Box::new(stmt));
                    let _closing = self.tokenizer.next();
                    stmt
                },
                _ => {
                    let expr = self.parse_primary_expr()?;
                    return Ok(expr);
                }
            },
            None => { 

                    return Err(Diagnostic {
                        range: Range {
                            start: Position {line: 0, character: 0},
                            end:  Position {line: 0, character: 1},
                        },
                        severity: 1,
                        message: "unexpected end of file".to_string() 
                    });
            }
        };
        return Ok(stmt);
    }


    fn parse_unary_expr(&mut self) -> Result<Expr, Diagnostic>{
        let token = self.tokenizer.peek();
        let stmt = match token {
            Some(token) => match token.kind {
                TokenKind::BANG => {
                    self.tokenizer.next().unwrap();
                    let operand = self.parse_parenthesized_exp()?;
                    Expr::UNARY(Box::new(operand))
                },
                _ => {
                    let expr = self.parse_parenthesized_exp()?;
                    return Ok(expr);
                }
            },
            None => {

                return Err(Diagnostic {
                    range: Range {
                        start: Position {line: 0, character: 0},
                        end:  Position {line: 0, character: 1},
                    },
                    severity: 1,
                    message: "unexpected end of file".to_string() 
                });
            }
        };
        return Ok(stmt);
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, Diagnostic>{
        let token = self.tokenizer.peek();

        let stmt = match token {
            Some(token) => match &token.kind {
                TokenKind::IDENTIFIER(a) => {
                    self.tokenizer.next();
                    Expr::IDENTIFIER(a.to_string())
                }
                TokenKind::INT(a) => {
                    self.tokenizer.next();
                    Expr::INTEGERLITERAL(*a)
                }
                TokenKind::FLOAT(a) => {
                    self.tokenizer.next();
                    Expr::FLOATLITERAL(*a)
                }
                _ => {
                    return Err(Diagnostic {
                        range: Range {
                            start: Position {line: 0, character: 0},
                            end:  Position {line: 0, character: 1},
                        },
                        severity: 1,
                        message: format_args!("unexpected token {:?} expected 'identifier | number | string'", token.kind).to_string() 
                    });
                },
            },
            None => {
                return Err(Diagnostic {
                    range: Range {
                        start: Position {line: 0, character: 0},
                        end:  Position {line: 0, character: 1},
                    },
                    severity: 1,
                    message: "unexpected end of file".to_string() 
                });
            },
        };

        return Ok(stmt);
    }
}
