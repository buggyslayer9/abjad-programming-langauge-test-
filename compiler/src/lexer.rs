use crate::error::{AbjadError, Result};
use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

/// Lexer for the Abjad programming language
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from the input string
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    /// Get the current position
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Peek at the next character
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Consume the next character
    fn next(&mut self) -> Option<char> {
        let c = self.input.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    /// Check if the next character matches
    fn matches(&mut self, expected: char) -> bool {
        if let Some(&c) = self.peek() {
            if c == expected {
                self.next();
                return true;
            }
        }
        false
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Skip a comment
    fn skip_comment(&mut self) -> Result<()> {
        // Single-line comment
        if self.matches('/') && self.matches('/') {
            while let Some(&c) = self.peek() {
                if c == '\n' {
                    break;
                }
                self.next();
            }
            return Ok(());
        }

        // Multi-line comment
        if self.matches('/') && self.matches('*') {
            loop {
                if let Some(&c) = self.peek() {
                    if c == '*' && self.matches('*') && self.matches('/') {
                        break;
                    }
                    self.next();
                } else {
                    return Err(AbjadError::lexical("Unterminated multi-line comment"));
                }
            }
            return Ok(());
        }

        Ok(())
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self, first: char) -> Token {
        let mut identifier = String::from(first);
        
        while let Some(&c) = self.peek() {
            if c.is_alphabetic() || c == '_' || c.is_numeric() {
                identifier.push(self.next().unwrap());
            } else {
                break;
            }
        }

        // Check if it's a keyword
        match identifier.as_str() {
            "دالة" => Token::Function,
            "هيكل" => Token::Struct,
            "تعداد" => Token::Enum,
            "استورد" => Token::Import,
            "متغير" => Token::Var,
            "ثابت" => Token::Const,
            "أعد" => Token::Return,
            "إذا" => Token::If,
            "وإلا" => Token::Else,
            "مطابق" => Token::Match,
            "حلقة" => Token::For,
            "بينما" => Token::While,
            "لكل" => Token::ForEach,
            "كسر" => Token::Break,
            "تابع" => Token::Continue,
            "و" => Token::And,
            "أو" => Token::Or,
            "ليس" => Token::Not,
            "صحيح" => Token::True,
            "خطأ" => Token::False,
            "لا_شيء" => Token::Null,
            "حاول" => Token::Try,
            "التقط" => Token::Catch,
            "أطلق" => Token::Throw,
            "نفِّذ" => Token::Impl,
            "لـ" => Token::ForTrait,
            "في" => Token::In,
            "غير_متزامن" => Token::Async,
            "انتظر" => Token::Await,
            "نقل" => Token::Move,
            "استعارة" => Token::Borrow,
            "متغير" => Token::Mut,
            "مرجع" => Token::Ref,
            "ذات" => Token::Self_,
            "سمة" => Token::Trait,
            "عام" => Token::Generic,
            "حيث" => Token::Where,
            "صحيح٨" => Token::I8,
            "صحيح١٦" => Token::I16,
            "صحيح٣٢" => Token::I32,
            "صحيح٦٤" => Token::I64,
            "طبيعي٨" => Token::U8,
            "طبيعي١٦" => Token::U16,
            "طبيعي٣٢" => Token::U32,
            "طبيعي٦٤" => Token::U64,
            "عشري٣٢" => Token::F32,
            "عشري٦٤" => Token::F64,
            "منطقي" => Token::Bool,
            "نص" => Token::String,
            "حرف" => Token::Char,
            _ => Token::Identifier(identifier),
        }
    }

    /// Read a number (integer or float)
    fn read_number(&mut self, first: char) -> Result<Token> {
        let mut number = String::from(first);
        let mut is_float = false;

        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '.' {
                if c == '.' {
                    if is_float {
                        return Err(AbjadError::lexical("Multiple decimal points in number"));
                    }
                    is_float = true;
                }
                number.push(self.next().unwrap());
            } else {
                break;
            }
        }

        if is_float {
            Ok(Token::Float(number.parse().map_err(|_| {
                AbjadError::lexical("Invalid float literal")
            })?))
        } else {
            Ok(Token::Integer(number.parse().map_err(|_| {
                AbjadError::lexical("Invalid integer literal")
            })?))
        }
    }

    /// Read a string literal
    fn read_string(&mut self, quote: char) -> Result<Token> {
        let mut string = String::new();

        while let Some(&c) = self.peek() {
            if c == quote {
                self.next();
                break;
            }
            
            if c == '\\' {
                self.next();
                if let Some(escaped) = self.next() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        'u' => {
                            // Unicode escape sequence
                            let mut code = String::new();
                            for _ in 0..4 {
                                if let Some(&c) = self.peek() {
                                    code.push(self.next().unwrap());
                                }
                            }
                            let code_point = u32::from_str_radix(&code, 16)
                                .map_err(|_| AbjadError::lexical("Invalid Unicode escape"))?;
                            string.push(char::from_u32(code_point)
                                .ok_or_else(|| AbjadError::lexical("Invalid Unicode code point"))?);
                        }
                        _ => return Err(AbjadError::lexical("Invalid escape sequence")),
                    }
                }
            } else {
                string.push(self.next().unwrap());
            }
        }

        Ok(Token::StringLiteral(string))
    }

    /// Read a character literal
    fn read_char(&mut self) -> Result<Token> {
        let mut char_str = String::new();

        while let Some(&c) = self.peek() {
            if c == '\'' {
                self.next();
                break;
            }
            
            if c == '\\' {
                self.next();
                if let Some(escaped) = self.next() {
                    match escaped {
                        'n' => char_str.push('\n'),
                        't' => char_str.push('\t'),
                        '\\' => char_str.push('\\'),
                        '"' => char_str.push('"'),
                        '\'' => char_str.push('\''),
                        'u' => {
                            let mut code = String::new();
                            for _ in 0..4 {
                                if let Some(&c) = self.peek() {
                                    code.push(self.next().unwrap());
                                }
                            }
                            let code_point = u32::from_str_radix(&code, 16)
                                .map_err(|_| AbjadError::lexical("Invalid Unicode escape"))?;
                            char_str.push(char::from_u32(code_point)
                                .ok_or_else(|| AbjadError::lexical("Invalid Unicode code point"))?);
                        }
                        _ => return Err(AbjadError::lexical("Invalid escape sequence")),
                    }
                }
            } else {
                char_str.push(self.next().unwrap());
            }
        }

        let chars: Vec<char> = char_str.chars().collect();
        if chars.len() != 1 {
            return Err(AbjadError::lexical("Character literal must contain exactly one character"));
        }

        Ok(Token::CharLiteral(chars[0]))
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        // Check for comments
        if let Some(&'/') = self.peek() {
            self.skip_comment()?;
            return self.next_token();
        }

        let c = match self.next() {
            Some(c) => c,
            None => return Ok(Token::EOF),
        };

        match c {
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' | 
            '\u{0600}'..='\u{06FF}' | // Arabic
            '\u{0750}'..='\u{077F}' | // Arabic Supplement
            '\u{08A0}'..='\u{08FF}'   // Arabic Extended-A
            => Ok(self.read_identifier(c)),

            // Numbers
            '0'..='9' => self.read_number(c),

            // String literals
            '"' => self.read_string('"'),
            '\'' => self.read_char(),

            // Operators and delimiters
            '+' => {
                if self.matches('=') {
                    Ok(Token::PlusEqual)
                } else if self.matches('+') {
                    Ok(Token::Power)
                } else {
                    Ok(Token::Plus)
                }
            }
            '-' => {
                if self.matches('=') {
                    Ok(Token::MinusEqual)
                } else if self.matches('>') {
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Minus)
                }
            }
            '*' => {
                if self.matches('=') {
                    Ok(Token::StarEqual)
                } else {
                    Ok(Token::Star)
                }
            }
            '/' => {
                if self.matches('=') {
                    Ok(Token::SlashEqual)
                } else {
                    Ok(Token::Slash)
                }
            }
            '%' => {
                if self.matches('=') {
                    Ok(Token::PercentEqual)
                } else {
                    Ok(Token::Percent)
                }
            }
            '=' => {
                if self.matches('=') {
                    Ok(Token::EqualEqual)
                } else {
                    Ok(Token::Equal)
                }
            }
            '!' => {
                if self.matches('=') {
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Not)
                }
            }
            '<' => {
                if self.matches('=') {
                    Ok(Token::LessEqual)
                } else if self.matches('<') {
                    Ok(Token::LeftShift)
                } else {
                    Ok(Token::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    Ok(Token::GreaterEqual)
                } else if self.matches('>') {
                    Ok(Token::RightShift)
                } else {
                    Ok(Token::Greater)
                }
            }
            '&' => Ok(Token::Ampersand),
            '|' => {
                if self.matches('|') {
                    Ok(Token::Or)
                } else {
                    Ok(Token::Pipe)
                }
            }
            '^' => Ok(Token::Caret),
            '~' => Ok(Token::Tilde),
            '.' => {
                if self.matches('.') {
                    if self.matches('.') {
                        Ok(Token::TripleDot)
                    } else {
                        Ok(Token::DoubleDot)
                    }
                } else {
                    Ok(Token::Dot)
                }
            }
            ':' => {
                if self.matches(':') {
                    Ok(Token::DoubleColon)
                } else {
                    Ok(Token::Colon)
                }
            }
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semicolon),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),

            // Unknown character
            _ => Err(AbjadError::lexical(format!("Unexpected character: {}", c))),
        }
    }

    /// Skip to a synchronization point for error recovery
    fn skip_to_sync_point(&mut self) {
        // Skip until we find a semicolon, brace, or keyword
        while let Some(&c) = self.peek() {
            if c == ';' || c == '{' || c == '}' || c == '\n' {
                break;
            }
            self.next();
        }
    }

    /// Get the next token with error recovery
    pub fn next_token_with_recovery(&mut self) -> Token {
        match self.next_token() {
            Ok(token) => token,
            Err(_) => {
                // Skip to sync point and continue
                self.skip_to_sync_point();
                Token::EOF
            }
        }
    }

    /// Tokenize the entire input with error recovery
    pub fn tokenize_with_recovery(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token_with_recovery();
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_identifier() {
        let mut lexer = Lexer::new("اسم");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("اسم".to_string()));
    }

    #[test]
    fn test_function_keyword() {
        let mut lexer = Lexer::new("دالة");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Function);
    }

    #[test]
    fn test_integer_literal() {
        let mut lexer = Lexer::new("١٠");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Integer(10));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"مرحباً\"");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::StringLiteral("مرحباً".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * /");
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Minus);
        assert_eq!(lexer.next_token().unwrap(), Token::Star);
        assert_eq!(lexer.next_token().unwrap(), Token::Slash);
    }

    #[test]
    fn test_comment() {
        let mut lexer = Lexer::new("// هذا تعليق\nدالة");
        assert_eq!(lexer.next_token().unwrap(), Token::Function);
    }
}
