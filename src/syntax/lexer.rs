use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Agent,
    Skill,
    Identifier(String),
    String(String),
    Number(u32),
    Bool(bool),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPos>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<TokenWithPos>> {
        self.skip_whitespace();
        if self.pos >= self.chars.len() {
            return Ok(None);
        }

        let line = self.line;
        let column = self.column;
        let ch = self.chars[self.pos];

        let token = if ch == '#' {
            self.skip_comment();
            return self.next_token();
        } else if ch == ':' {
            self.advance();
            Token::Colon
        } else if ch == '{' {
            self.advance();
            Token::LBrace
        } else if ch == '}' {
            self.advance();
            Token::RBrace
        } else if ch == '[' {
            self.advance();
            Token::LBracket
        } else if ch == ']' {
            self.advance();
            Token::RBracket
        } else if ch == ',' {
            self.advance();
            Token::Comma
        } else if ch == '"' {
            self.advance();
            let s = self.read_string()?;
            Token::String(s)
        } else if ch.is_ascii_digit() {
            let n = self.read_number();
            Token::Number(n)
        } else if ch == 't'
            && self.peek() == Some('r')
            && self.chars[self.pos + 1..].first() == Some(&'u')
        {
            self.advance();
            self.advance();
            self.advance();
            Token::Bool(true)
        } else if ch == 'f'
            && self.peek() == Some('a')
            && self.chars[self.pos + 1..].first() == Some(&'l')
        {
            self.advance();
            self.advance();
            self.advance();
            self.advance();
            Token::Bool(false)
        } else if ch.is_alphabetic() || ch == '_' {
            let ident = self.read_identifier();
            match ident.as_str() {
                "agent" => Token::Agent,
                "skill" => Token::Skill,
                _ => Token::Identifier(ident),
            }
        } else {
            return Err(anyhow!(
                "Unexpected character '{}' at line {}, column {}",
                ch,
                self.line,
                self.column
            ));
        };

        Ok(Some(TokenWithPos {
            token,
            line,
            column,
        }))
    }

    fn advance(&mut self) {
        if self.pos < self.chars.len() {
            if self.chars[self.pos] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
            self.advance();
        }
    }

    fn read_string(&mut self) -> Result<String> {
        let mut s = String::new();
        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            s.push(self.chars[self.pos]);
            self.advance();
        }
        if self.pos >= self.chars.len() {
            return Err(anyhow!("Unterminated string at line {}", self.line));
        }
        self.advance();
        Ok(s)
    }

    fn read_number(&mut self) -> u32 {
        let mut n = 0u32;
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            n = n * 10 + (self.chars[self.pos] as u32 - '0' as u32);
            self.advance();
        }
        n
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while self.pos < self.chars.len()
            && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_')
        {
            ident.push(self.chars[self.pos]);
            self.advance();
        }
        ident
    }
}
