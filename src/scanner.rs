use crate::{error::ScanError, token::Token};

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            errors: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScanError>> {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::Eof { line: self.line });

        if self.errors.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.tokens.push(Token::LeftParen { line: self.line }),
            ')' => self.tokens.push(Token::RightParen { line: self.line }),

            '{' => self.tokens.push(Token::LeftBrace { line: self.line }),
            '}' => self.tokens.push(Token::RightBrace { line: self.line }),

            '.' => self.tokens.push(Token::Dot { line: self.line }),
            ',' => self.tokens.push(Token::Comma { line: self.line }),
            ';' => self.tokens.push(Token::Semicolon { line: self.line }),

            '+' => self.tokens.push(Token::Plus { line: self.line }),
            '-' => self.tokens.push(Token::Minus { line: self.line }),
            '*' => self.tokens.push(Token::Asterisk { line: self.line }),

            '/' => {
                if self.is_expected('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::Slash { line: self.line });
                }
            }

            '=' => {
                if self.is_expected('=') {
                    self.tokens.push(Token::EqualEqual { line: self.line });
                } else {
                    self.tokens.push(Token::Equal { line: self.line });
                }
            }
            '!' => {
                if self.is_expected('=') {
                    self.tokens.push(Token::NotEqual { line: self.line });
                } else {
                    self.tokens.push(Token::Bang { line: self.line });
                }
            }

            '<' => {
                if self.is_expected('=') {
                    self.tokens.push(Token::LessEqual { line: self.line });
                } else {
                    self.tokens.push(Token::LessThan { line: self.line });
                }
            }
            '>' => {
                if self.is_expected('=') {
                    self.tokens.push(Token::GreaterEqual { line: self.line });
                } else {
                    self.tokens.push(Token::GreaterThan { line: self.line });
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.errors
                        .push(ScanError::UnexpectedCharacter { line: self.line });
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        match self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .as_str()
        {
            "true" => self.tokens.push(Token::True { line: self.line }),
            "false" => self.tokens.push(Token::False { line: self.line }),
            "nil" => self.tokens.push(Token::Nil { line: self.line }),

            "and" => self.tokens.push(Token::And { line: self.line }),
            "or" => self.tokens.push(Token::Or { line: self.line }),

            "if" => self.tokens.push(Token::If { line: self.line }),
            "else" => self.tokens.push(Token::Else { line: self.line }),
            "for" => self.tokens.push(Token::For { line: self.line }),
            "while" => self.tokens.push(Token::While { line: self.line }),

            "class" => self.tokens.push(Token::Class { line: self.line }),
            "this" => self.tokens.push(Token::This { line: self.line }),
            "super" => self.tokens.push(Token::Super { line: self.line }),

            "fun" => self.tokens.push(Token::Fun { line: self.line }),
            "return" => self.tokens.push(Token::Return { line: self.line }),
            "var" => self.tokens.push(Token::Var { line: self.line }),
            "print" => self.tokens.push(Token::Print { line: self.line }),

            variable => self.tokens.push(Token::Identifier {
                line: self.line,
                literal: variable.into(),
            }),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_eof() {
            self.errors
                .push(ScanError::UnterminatedString { line: self.line });
            return;
        }

        self.advance();

        self.tokens.push(Token::String {
            line: self.line,
            literal: self.source[self.start + 1..self.current - 1]
                .iter()
                .collect(),
        });
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.tokens.push(Token::Number {
            line: self.line,
            literal: self.source[self.start..self.current].iter().collect(),
        });
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn is_expected(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\x00'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\x00'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ScanError, scanner::Scanner, token::Token};

    #[test]
    fn test_success() {
        let source = r#"
            ( )
            { }
            . , ;
            + - * /
            = !
            == != < <= > >=
            abc _a_b_c_ "" "abc" 123 123.0
            true false nil
            and or
            if else for while
            class this super
            fun return var print
        "#
        .trim_start();

        let actual = Scanner::new(source.into()).scan_tokens().unwrap();

        let expected = vec![
            Token::LeftParen { line: 1 },
            Token::RightParen { line: 1 },
            Token::LeftBrace { line: 2 },
            Token::RightBrace { line: 2 },
            Token::Dot { line: 3 },
            Token::Comma { line: 3 },
            Token::Semicolon { line: 3 },
            Token::Plus { line: 4 },
            Token::Minus { line: 4 },
            Token::Asterisk { line: 4 },
            Token::Slash { line: 4 },
            Token::Equal { line: 5 },
            Token::Bang { line: 5 },
            Token::EqualEqual { line: 6 },
            Token::NotEqual { line: 6 },
            Token::LessThan { line: 6 },
            Token::LessEqual { line: 6 },
            Token::GreaterThan { line: 6 },
            Token::GreaterEqual { line: 6 },
            Token::Identifier {
                line: 7,
                literal: "abc".into(),
            },
            Token::Identifier {
                line: 7,
                literal: "_a_b_c_".into(),
            },
            Token::String {
                line: 7,
                literal: "".into(),
            },
            Token::String {
                line: 7,
                literal: "abc".into(),
            },
            Token::Number {
                line: 7,
                literal: "123".into(),
            },
            Token::Number {
                line: 7,
                literal: "123.0".into(),
            },
            Token::True { line: 8 },
            Token::False { line: 8 },
            Token::Nil { line: 8 },
            Token::And { line: 9 },
            Token::Or { line: 9 },
            Token::If { line: 10 },
            Token::Else { line: 10 },
            Token::For { line: 10 },
            Token::While { line: 10 },
            Token::Class { line: 11 },
            Token::This { line: 11 },
            Token::Super { line: 11 },
            Token::Fun { line: 12 },
            Token::Return { line: 12 },
            Token::Var { line: 12 },
            Token::Print { line: 12 },
            Token::Eof { line: 13 },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comment() {
        let source = r#"
            // abc
            // "abc"
            // 123
            abc
            "abc"
            123
        "#
        .trim_start();

        let actual = Scanner::new(source.into()).scan_tokens().unwrap();

        let expected = vec![
            Token::Identifier {
                line: 4,
                literal: "abc".into(),
            },
            Token::String {
                line: 5,
                literal: "abc".into(),
            },
            Token::Number {
                line: 6,
                literal: "123".into(),
            },
            Token::Eof { line: 7 },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_failure() {
        let source = r#"
            ~ @ #
            "abc
        "#
        .trim_start();

        let actual = Scanner::new(source.into()).scan_tokens().unwrap_err();

        let expected = vec![
            ScanError::UnexpectedCharacter { line: 1 },
            ScanError::UnexpectedCharacter { line: 1 },
            ScanError::UnexpectedCharacter { line: 1 },
            ScanError::UnterminatedString { line: 3 },
        ];

        assert_eq!(actual, expected);
    }
}
