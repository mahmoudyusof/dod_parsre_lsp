use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read, Write},
    iter::Peekable, str::Bytes,
};

#[derive(Debug)]
pub enum TokenKind {
    SEMICOLON, COLON, DCOLON, LPAREN, RPAREN, LCURLY, RCURLY, LSQUARE, RSQUARE, COMMA, DOT,

    ADD, SUB, MUL, DIV, MOD,

    INT(i32), FLOAT(f32), STRING(String), IDENTIFIER(String), KEYWORD(String),

    BANG,
    ASSIGNE,

    EQ, GT, LT, GTEQ, LTEQ, NEQ,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub column: u32
}


impl Clone for TokenKind {
    fn clone(&self) -> Self {
        match self {
            TokenKind::SEMICOLON => TokenKind::SEMICOLON,
            TokenKind::COLON => TokenKind::COLON,
            TokenKind::DCOLON => TokenKind::DCOLON,
            TokenKind::LPAREN => TokenKind::LPAREN,
            TokenKind::RPAREN => TokenKind::RPAREN,
            TokenKind::LCURLY => TokenKind::LCURLY,
            TokenKind::RCURLY => TokenKind::RCURLY,
            TokenKind::LSQUARE => TokenKind::LSQUARE,
            TokenKind::RSQUARE => TokenKind::RSQUARE,
            TokenKind::COMMA => TokenKind::COMMA,
            TokenKind::DOT => TokenKind::DOT,
            TokenKind::ADD => TokenKind::ADD,
            TokenKind::SUB => TokenKind::SUB,
            TokenKind::MUL => TokenKind::MUL,
            TokenKind::DIV => TokenKind::DIV,
            TokenKind::MOD => TokenKind::MOD,
            TokenKind::INT(val) => TokenKind::INT(*val),
            TokenKind::FLOAT(val) => TokenKind::FLOAT(*val),
            TokenKind::STRING(val) => TokenKind::STRING(val.clone()),
            TokenKind::IDENTIFIER(val) => TokenKind::IDENTIFIER(val.clone()),
            TokenKind::KEYWORD(val) => TokenKind::KEYWORD(val.clone()),
            TokenKind::BANG => TokenKind::BANG,
            TokenKind::ASSIGNE => TokenKind::ASSIGNE,
            TokenKind::EQ => TokenKind::EQ,
            TokenKind::GT => TokenKind::GT,
            TokenKind::LT => TokenKind::LT,
            TokenKind::GTEQ => TokenKind::GTEQ,
            TokenKind::LTEQ => TokenKind::LTEQ,
            TokenKind::NEQ => TokenKind::NEQ,
            TokenKind::EOF => TokenKind::EOF,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            kind: self.kind.clone(),
            line: self.line.clone(),
            column: self.column.clone()
        }
    }
}

pub struct Tokenizer<'a> {
    iterator: Peekable<Bytes<'a>>,
    current_token: Option<Token>,
    current_line: u32,
    current_col: u32
}

impl<'a> Tokenizer<'a> {
    //pub fn new(file: File) -> Self {
    //    let reader = BufReader::new(file);
    //    let iterator = reader.bytes().peekable();
    //
    //    let mut tokenizer = Tokenizer {
    //        iterator,
    //        current_token: None,
    //        current_line: 1,
    //        current_col: 1
    //    };
    //
    //    tokenizer.parse_token();
    //    tokenizer
    //}

    pub fn tokenize(code: &'a str) -> Vec<Token> {
        let codeText = code.bytes().peekable();
        let mut tokenizer = Tokenizer {
            iterator: codeText,
            current_token: None,
            current_col: 1,
            current_line: 1
        };

        tokenizer.parse_token();

        let mut tokens = vec![];
        for token in tokenizer {
            tokens.push(token);
        }
        tokens
    }

    fn get_word(&mut self, byte: u8) -> Token {
        let mut word = String::new();
        word.push(char::from(byte));

        // define a hashset of keywords
        let keywords: HashSet<&str> = HashSet::from([
            "if", "else", "while", "for", "return", "break", "continue", "true", "false", "null",
            "int", "float", "string", "bool", "void", "let", "const", "function",
        ]);

        loop {
            match self.iterator.peek() {
                Some(byte) => {
                    if char::from(*byte).is_alphanumeric() || *byte == b'_' {
                        word.push(char::from(*byte));
                        self.next_byte();
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if keywords.contains(word.as_str()) {
            return Token {
                kind: TokenKind::KEYWORD(word),
                line: self.current_line,
                column: self.current_col
            };
        } else {
            return Token {
                kind: TokenKind::IDENTIFIER(word),
                line: self.current_line,
                column: self.current_col
            };
        }
    }

    fn get_number(&mut self, byte: u8) -> (String, bool) {
        let mut word = String::new();
        word.push(char::from(byte));
        let mut is_float = false;

        loop {
            match self.iterator.peek() {
                Some(byte) => {
                    if *byte == b'.' {
                        word.push(char::from(*byte));
                        self.next_byte();
                        is_float = true;
                    } else if char::from(*byte).is_numeric() {
                        word.push(char::from(*byte));
                        self.next_byte();
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        return (word, is_float);
    }

    fn get_string(&mut self) -> String {
        let mut string = String::new();

        loop {
            match self.iterator.peek() {
                Some(b'"') => {
                    self.next_byte();
                    break;
                }
                Some(byte) => {
                    string.push(char::from(*byte));
                    self.next_byte();
                }
                _ => {
                    break;
                }
            }
        }

        string
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.iterator.next();
        if let Some(byte) = byte {
            match byte {
                b'\n' => {
                    self.current_col = 0;
                    self.current_line += 1;
                    return Some(b'\n');
                },
                byte => {
                    self.current_col += 1;
                    return Some(byte);
                }
            }
        } else {
            return None;
        }
    }

    pub fn peek_token(&self) -> Option<Token> {
        return self.current_token.clone();
    }

    fn parse_token(&mut self) {
        if let Some(token) = &self.current_token // borrowing
        {
            match token.kind {
                TokenKind::EOF => {

                    self.current_token = None;
                    return;
                },
                _ => {}
            }
        }



        let byte = self.next_byte();

        if let None = byte {
            self.current_token = Some(Token {
                kind: TokenKind::EOF,
                line: self.current_line,
                column: self.current_col
            });
            return;
        }

        let byte = byte.unwrap();

        if byte == b';' {
            self.current_token = Some(Token {
                kind: TokenKind::SEMICOLON,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b':' {
            match self.iterator.peek() {
                Some(b':') => {
                    self.current_token = Some(Token {
                        kind: TokenKind::DCOLON,
                        line: self.current_line,
                        column: self.current_col
                    });
                    self.next_byte();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::COLON,
                        line: self.current_line,
                        column: self.current_col
                    });
                }
            }
        } else if byte == b',' {
            self.current_token = Some(Token {
                kind: TokenKind::COMMA,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'(' {
            self.current_token = Some(Token {
                kind: TokenKind::LPAREN,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b')' {
            self.current_token = Some(Token {
                kind: TokenKind::RPAREN,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'{' {
            self.current_token = Some(Token {
                kind: TokenKind::LCURLY,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'}' {
            self.current_token = Some(Token {
                kind: TokenKind::RCURLY,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'[' {
            self.current_token = Some(Token {
                kind: TokenKind::LSQUARE,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b']' {
            self.current_token = Some(Token {
                kind: TokenKind::RSQUARE,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'.' {
            self.current_token = Some(Token {
                kind: TokenKind::DOT,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'!' {
            match self.iterator.peek() {
                Some(b'=') => {
                    self.current_token = Some(Token {
                        kind: TokenKind::NEQ,
                        line: self.current_line,
                        column: self.current_col
                    });
                    self.next_byte();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::BANG,
                        line: self.current_line,
                        column: self.current_col
                    })
                }
            }
        } else if byte == b'=' {
            match self.iterator.peek() {
                Some(b'=') => {
                    self.current_token = Some(Token {
                        kind: TokenKind::EQ,
                        line: self.current_line,
                        column: self.current_col
                    });
                    self.next_byte();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::ASSIGNE,
                        line: self.current_line,
                        column: self.current_col
                    })
                }
            }
        } else if byte == b'<' {
            match self.iterator.peek() {
                Some(b'=') => {
                    self.current_token = Some(Token {
                        kind: TokenKind::LTEQ,
                        line: self.current_line,
                        column: self.current_col
                    });
                    self.next_byte();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::LT,
                        line: self.current_line,
                        column: self.current_col
                    })
                }
            }
        } else if byte == b'>' {
            match self.iterator.peek() {
                Some(b'=') => {
                    self.current_token = Some(Token {
                        kind: TokenKind::GTEQ,
                        line: self.current_line,
                        column: self.current_col
                    });
                    self.next_byte();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::GT,
                        line: self.current_line,
                        column: self.current_col
                    })
                }
            }
        } else if byte == b'"' {
            let string = self.get_string();
            self.current_token = Some(Token {
                kind: TokenKind::STRING(string),
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'+' {
            self.current_token = Some(Token {
                kind: TokenKind::ADD,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'-' {
            self.current_token = Some(Token {
                kind: TokenKind::SUB,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'*' {
            self.current_token = Some(Token {
                kind: TokenKind::MUL,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'/' {
            self.current_token = Some(Token {
                kind: TokenKind::DIV,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b'%' {
            self.current_token = Some(Token {
                kind: TokenKind::MOD,
                line: self.current_line,
                column: self.current_col
            });
        } else if byte == b' ' || byte == b'\t' || byte == b'\r' || byte == b'\n' {
            self.parse_token();
        } else if char::from(byte).is_numeric() {
            let (value, is_float) = self.get_number(byte);
            if is_float {
                let val: f32 = match value.parse() {
                    Ok(v) => v,
                    Err(_) => 0.0,
                };

                self.current_token = Some(Token {
                    kind: TokenKind::FLOAT(val),
                    line: self.current_line,
                    column: self.current_col
                });
            } else {
                let val: i32 = match value.parse() {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                self.current_token = Some(Token {
                    kind: TokenKind::INT(val),
                    line: self.current_line,
                    column: self.current_col
                });
            }
        } else if byte == b'_' || char::from(byte).is_alphabetic() {
            let token = self.get_word(byte);
            self.current_token = Some(token);
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.current_token.clone();
        self.parse_token();
        token
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
