use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Bytes, Read},
    iter::Peekable,
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

pub struct Token {
    pub kind: TokenKind,
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
        }
    }
}

pub struct Tokenizer {
    iterator: Peekable<Bytes<BufReader<File>>>,
    current_token: Option<Token>,
}

impl Tokenizer {
    pub fn new(file: File) -> Self {
        let reader = BufReader::new(file);
        let iterator = reader.bytes().peekable();

        let mut tokenizer = Tokenizer {
            iterator,
            current_token: None,
        };

        tokenizer.parse_token();
        tokenizer
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
                Some(Ok(byte)) => {
                    if char::from(*byte).is_alphanumeric() || *byte == b'_' {
                        word.push(char::from(*byte));
                        self.iterator.next();
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
            };
        } else {
            return Token {
                kind: TokenKind::IDENTIFIER(word),
            };
        }
    }

    fn get_number(&mut self, byte: u8) -> (String, bool) {
        let mut word = String::new();
        word.push(char::from(byte));
        let mut is_float = false;

        loop {
            match self.iterator.peek() {
                Some(Ok(byte)) => {
                    if *byte == b'.' {
                        word.push(char::from(*byte));
                        self.iterator.next();
                        is_float = true;
                    } else if char::from(*byte).is_numeric() {
                        word.push(char::from(*byte));
                        self.iterator.next();
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
                Some(Ok(b'"')) => {
                    self.iterator.next();
                    break;
                }
                Some(Ok(byte)) => {
                    string.push(char::from(*byte));
                    self.iterator.next();
                }
                _ => {
                    break;
                }
            }
        }

        string
    }

    pub fn peek_token(&self) -> Option<Token> {
        return self.current_token.clone();
    }

    fn parse_token(&mut self) {
        if let Some(Token {
            kind: TokenKind::EOF,
        }) = self.current_token
        {
            self.current_token = None;
            return;
        }
        let byte = self.iterator.next();

        if let None = byte {
            self.current_token = Some(Token {
                kind: TokenKind::EOF,
            });
            return;
        }

        let byte = byte.unwrap().expect("error trying to read character");

        if byte == b';' {
            self.current_token = Some(Token {
                kind: TokenKind::SEMICOLON,
            });
        } else if byte == b':' {
            match self.iterator.peek() {
                Some(Ok(b':')) => {
                    self.current_token = Some(Token {
                        kind: TokenKind::DCOLON,
                    });
                    self.iterator.next();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::COLON,
                    });
                }
            }
        } else if byte == b',' {
            self.current_token = Some(Token {
                kind: TokenKind::COMMA,
            });
        } else if byte == b'(' {
            self.current_token = Some(Token {
                kind: TokenKind::LPAREN,
            });
        } else if byte == b')' {
            self.current_token = Some(Token {
                kind: TokenKind::RPAREN,
            });
        } else if byte == b'{' {
            self.current_token = Some(Token {
                kind: TokenKind::LCURLY,
            });
        } else if byte == b'}' {
            self.current_token = Some(Token {
                kind: TokenKind::RCURLY,
            });
        } else if byte == b'[' {
            self.current_token = Some(Token {
                kind: TokenKind::LSQUARE,
            });
        } else if byte == b']' {
            self.current_token = Some(Token {
                kind: TokenKind::RSQUARE,
            });
        } else if byte == b'.' {
            self.current_token = Some(Token {
                kind: TokenKind::DOT,
            });
        } else if byte == b'!' {
            match self.iterator.peek() {
                Some(Ok(b'=')) => {
                    self.current_token = Some(Token {
                        kind: TokenKind::NEQ,
                    });
                    self.iterator.next();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::BANG,
                    })
                }
            }
        } else if byte == b'=' {
            match self.iterator.peek() {
                Some(Ok(b'=')) => {
                    self.current_token = Some(Token {
                        kind: TokenKind::EQ,
                    });
                    self.iterator.next();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::ASSIGNE,
                    })
                }
            }
        } else if byte == b'<' {
            match self.iterator.peek() {
                Some(Ok(b'=')) => {
                    self.current_token = Some(Token {
                        kind: TokenKind::LTEQ,
                    });
                    self.iterator.next();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::LT,
                    })
                }
            }
        } else if byte == b'>' {
            match self.iterator.peek() {
                Some(Ok(b'=')) => {
                    self.current_token = Some(Token {
                        kind: TokenKind::GTEQ,
                    });
                    self.iterator.next();
                }
                _ => {
                    self.current_token = Some(Token {
                        kind: TokenKind::GT,
                    })
                }
            }
        } else if byte == b'"' {
            let string = self.get_string();
            self.current_token = Some(Token {
                kind: TokenKind::STRING(string),
            });
        } else if byte == b'+' {
            self.current_token = Some(Token {
                kind: TokenKind::ADD,
            });
        } else if byte == b'-' {
            self.current_token = Some(Token {
                kind: TokenKind::SUB,
            });
        } else if byte == b'*' {
            self.current_token = Some(Token {
                kind: TokenKind::MUL,
            });
        } else if byte == b'/' {
            self.current_token = Some(Token {
                kind: TokenKind::DIV,
            });
        } else if byte == b'%' {
            self.current_token = Some(Token {
                kind: TokenKind::MOD,
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
                });
            } else {
                let val: i32 = match value.parse() {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                self.current_token = Some(Token {
                    kind: TokenKind::INT(val),
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

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
