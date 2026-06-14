use crate::ast::*;
use crate::error::{AbjadError, Result};
use crate::token::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

/// Parser for the Abjad programming language
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    /// Create a new parser from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Peek at the next token
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Consume the next token
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Check if the next token matches the expected token
    fn matches(&mut self, expected: Token) -> bool {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expected) {
                self.next();
                return true;
            }
        }
        false
    }

    /// Expect a specific token or return an error
    fn expect(&mut self, expected: Token) -> Result<Token> {
        if let Some(token) = self.next() {
            if std::mem::discriminant(&token) == std::mem::discriminant(&expected) {
                return Ok(token);
            }
        }
        Err(AbjadError::syntax(format!("Expected {:?}, found {:?}", expected, self.peek())))
    }

    /// Parse the entire input into an AST
    pub fn parse(&mut self) -> Result<AST> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if token == &Token::EOF {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        Ok(AST::new(statements))
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek() {
            Some(Token::Var) | Some(Token::Const) => self.parse_let_statement(),
            Some(Token::Function) => self.parse_function_declaration(),
            Some(Token::Struct) => self.parse_struct_declaration(),
            Some(Token::Enum) => self.parse_enum_declaration(),
            Some(Token::Import) => self.parse_import_statement(),
            Some(Token::Semicolon) => {
                self.next();
                Ok(Statement::Empty)
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.matches(Token::Semicolon) {
                    Ok(Statement::Expression(expr))
                } else {
                    Ok(Statement::Expression(expr))
                }
            }
        }
    }

    /// Parse a let/const statement
    fn parse_let_statement(&mut self) -> Result<Statement> {
        let mutable = self.matches(Token::Var);
        if !mutable {
            self.expect(Token::Const)?;
        }

        let name = match self.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(AbjadError::syntax("Expected identifier after let/const")),
        };

        let type_annotation = if self.matches(Token::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(Statement::Let {
            name,
            type_annotation,
            value,
            mutable,
        })
    }

    /// Parse a function declaration
    fn parse_function_declaration(&mut self) -> Result<Statement> {
        self.expect(Token::Function)?;

        let name = match self.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(AbjadError::syntax("Expected identifier after function")),
        };

        self.expect(Token::LeftParen)?;
        let parameters = self.parse_parameters()?;
        self.expect(Token::RightParen)?;

        let return_type = if self.matches(Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace)?;

        Ok(Statement::Function(FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        }))
    }

    /// Parse function parameters
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();

        while !self.matches(Token::RightParen) {
            let mutable = self.matches(Token::Mut);
            
            let name = match self.next() {
                Some(Token::Identifier(name)) => name,
                _ => return Err(AbjadError::syntax("Expected parameter name")),
            };

            self.expect(Token::Colon)?;
            let type_annotation = self.parse_type()?;

            parameters.push(Parameter {
                name,
                type_annotation,
                mutable,
            });

            if !self.matches(Token::Comma) {
                break;
            }
        }

        Ok(parameters)
    }

    /// Parse a struct declaration
    fn parse_struct_declaration(&mut self) -> Result<Statement> {
        self.expect(Token::Struct)?;

        let name = match self.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(AbjadError::syntax("Expected identifier after struct")),
        };

        self.expect(Token::LeftBrace)?;
        let fields = self.parse_fields()?;
        self.expect(Token::RightBrace)?;

        Ok(Statement::Struct(StructDeclaration { name, fields }))
    }

    /// Parse struct fields
    fn parse_fields(&mut self) -> Result<Vec<Field>> {
        let mut fields = Vec::new();

        while !self.matches(Token::RightBrace) {
            let mutable = self.matches(Token::Mut);
            
            let name = match self.next() {
                Some(Token::Identifier(name)) => name,
                _ => return Err(AbjadError::syntax("Expected field name")),
            };

            self.expect(Token::Colon)?;
            let type_annotation = self.parse_type()?;

            fields.push(Field {
                name,
                type_annotation,
                mutable,
            });

            self.expect(Token::Comma)?;
        }

        Ok(fields)
    }

    /// Parse an enum declaration
    fn parse_enum_declaration(&mut self) -> Result<Statement> {
        self.expect(Token::Enum)?;

        let name = match self.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(AbjadError::syntax("Expected identifier after enum")),
        };

        self.expect(Token::LeftBrace)?;
        let variants = self.parse_enum_variants()?;
        self.expect(Token::RightBrace)?;

        Ok(Statement::Enum(EnumDeclaration { name, variants }))
    }

    /// Parse enum variants
    fn parse_enum_variants(&mut self) -> Result<Vec<EnumVariant>> {
        let mut variants = Vec::new();

        while !self.matches(Token::RightBrace) {
            let name = match self.next() {
                Some(Token::Identifier(name)) => name,
                _ => return Err(AbjadError::syntax("Expected variant name")),
            };

            let fields = if self.matches(Token::LeftParen) {
                let mut types = Vec::new();
                while !self.matches(Token::RightParen) {
                    types.push(self.parse_type()?);
                    if !self.matches(Token::Comma) {
                        break;
                    }
                }
                types
            } else {
                Vec::new()
            };

            variants.push(EnumVariant { name, fields });

            if !self.matches(Token::Comma) {
                break;
            }
        }

        Ok(variants)
    }

    /// Parse an import statement
    fn parse_import_statement(&mut self) -> Result<Statement> {
        self.expect(Token::Import)?;

        let path = match self.next() {
            Some(Token::StringLiteral(path)) => path,
            _ => return Err(AbjadError::syntax("Expected string literal after import")),
        };

        self.expect(Token::Semicolon)?;

        Ok(Statement::Import(path))
    }

    /// Parse a block
    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.matches(Token::RightBrace) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment()
    }

    /// Parse assignment
    fn parse_assignment(&mut self) -> Result<Expression> {
        let left = self.parse_logical_or()?;

        if self.matches(Token::Equal) {
            let right = self.parse_assignment()?;
            Ok(Expression::Assignment {
                target: Box::new(left),
                value: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    /// Parse logical or
    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and()?;

        while self.matches(Token::Or) {
            let right = self.parse_logical_and()?;
            left = Expression::Binary {
                op: BinaryOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse logical and
    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;

        while self.matches(Token::And) {
            let right = self.parse_equality()?;
            left = Expression::Binary {
                op: BinaryOperator::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse equality
    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::EqualEqual => Some(BinaryOperator::Equal),
                Token::NotEqual => Some(BinaryOperator::NotEqual),
                _ => None,
            };

            if let Some(op) = op {
                self.next();
                let right = self.parse_comparison()?;
                left = Expression::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Less => Some(BinaryOperator::Less),
                Token::LessEqual => Some(BinaryOperator::LessEqual),
                Token::Greater => Some(BinaryOperator::Greater),
                Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                _ => None,
            };

            if let Some(op) = op {
                self.next();
                let right = self.parse_term()?;
                left = Expression::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse term (addition/subtraction)
    fn parse_term(&mut self) -> Result<Expression> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => Some(BinaryOperator::Add),
                Token::Minus => Some(BinaryOperator::Sub),
                _ => None,
            };

            if let Some(op) = op {
                self.next();
                let right = self.parse_factor()?;
                left = Expression::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse factor (multiplication/division)
    fn parse_factor(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Star => Some(BinaryOperator::Mul),
                Token::Slash => Some(BinaryOperator::Div),
                Token::Percent => Some(BinaryOperator::Mod),
                _ => None,
            };

            if let Some(op) = op {
                self.next();
                let right = self.parse_unary()?;
                left = Expression::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse unary expression
    fn parse_unary(&mut self) -> Result<Expression> {
        if let Some(token) = self.peek() {
            let op = match token {
                Token::Minus => Some(UnaryOperator::Negate),
                Token::Not => Some(UnaryOperator::Not),
                Token::Tilde => Some(UnaryOperator::BitwiseNot),
                Token::Ampersand => Some(UnaryOperator::Reference),
                Token::Star => Some(UnaryOperator::Dereference),
                _ => None,
            };

            if let Some(op) = op {
                self.next();
                let operand = self.parse_unary()?;
                return Ok(Expression::Unary {
                    op,
                    operand: Box::new(operand),
                });
            }
        }

        self.parse_primary()
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<Expression> {
        match self.peek() {
            Some(Token::Integer(n)) => {
                let n = *n;
                self.next();
                Ok(Expression::Literal(Literal::Integer(n)))
            }
            Some(Token::Float(n)) => {
                let n = *n;
                self.next();
                Ok(Expression::Literal(Literal::Float(n)))
            }
            Some(Token::StringLiteral(s)) => {
                let s = s.clone();
                self.next();
                Ok(Expression::Literal(Literal::String(s)))
            }
            Some(Token::CharLiteral(c)) => {
                let c = *c;
                self.next();
                Ok(Expression::Literal(Literal::Char(c)))
            }
            Some(Token::True) => {
                self.next();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Some(Token::False) => {
                self.next();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Some(Token::Null) => {
                self.next();
                Ok(Expression::Literal(Literal::Null))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.next();
                
                // Check if it's a function call
                if self.matches(Token::LeftParen) {
                    let arguments = self.parse_arguments()?;
                    self.expect(Token::RightParen)?;
                    Ok(Expression::Call {
                        function: Box::new(Expression::Identifier(name)),
                        arguments,
                    })
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            Some(Token::LeftParen) => {
                self.next();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(Expression::Parenthesized(Box::new(expr)))
            }
            Some(Token::LeftBracket) => {
                self.next();
                let elements = self.parse_array_elements()?;
                self.expect(Token::RightBracket)?;
                Ok(Expression::ArrayLiteral(elements))
            }
            _ => Err(AbjadError::syntax(format!("Unexpected token: {:?}", self.peek()))),
        }
    }

    /// Parse function call arguments
    fn parse_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut arguments = Vec::new();

        while !self.matches(Token::RightParen) {
            arguments.push(self.parse_expression()?);
            if !self.matches(Token::Comma) {
                break;
            }
        }

        Ok(arguments)
    }

    /// Parse array elements
    fn parse_array_elements(&mut self) -> Result<Vec<Expression>> {
        let mut elements = Vec::new();

        while !self.matches(Token::RightBracket) {
            elements.push(self.parse_expression()?);
            if !self.matches(Token::Comma) {
                break;
            }
        }

        Ok(elements)
    }

    /// Parse a type
    fn parse_type(&mut self) -> Result<Type> {
        match self.next() {
            Some(Token::I8) => Ok(Type::I8),
            Some(Token::I16) => Ok(Type::I16),
            Some(Token::I32) => Ok(Type::I32),
            Some(Token::I64) => Ok(Type::I64),
            Some(Token::U8) => Ok(Type::U8),
            Some(Token::U16) => Ok(Type::U16),
            Some(Token::U32) => Ok(Type::U32),
            Some(Token::U64) => Ok(Type::U64),
            Some(Token::F32) => Ok(Type::F32),
            Some(Token::F64) => Ok(Type::F64),
            Some(Token::Bool) => Ok(Type::Bool),
            Some(Token::String) => Ok(Type::String),
            Some(Token::Char) => Ok(Type::Char),
            Some(Token::Identifier(name)) => Ok(Type::Named(name)),
            _ => Err(AbjadError::syntax("Expected type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let source = "١ + ٢";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_let_statement() {
        let source = "متغير عدد: صحيح٣٢ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_const_statement() {
        let source = "ثابت PI: عشري٦٤ = ٣.١٤;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_function_declaration() {
        let source = "دالة جمع(أ: صحيح٣٢، ب: صحيح٣٢) -> صحيح٣٢ { أعد أ + ب }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_arabic_keyword_function() {
        let source = "دالة رئيسي() { }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_struct_declaration() {
        let source = "هيكل المستخدم { اسم: نص، العمر: صحيح٣٢ }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_enum_declaration() {
        let source = "تعداد الحالة { نشط، غير_نشط، معلق }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_import_statement() {
        let source = "استورد \"أبجد/مدخلات\";";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_complex_expression() {
        let source = "١ + ٢ * ٣";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let source = "(١ + ٢) * ٣";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_function_call() {
        let source = "جمع(١، ٢)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_array_literal() {
        let source = "[١، ٢، ٣]";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_boolean_literal() {
        let source = "صحيح و خطأ";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_string_literal() {
        let source = "\"مرحباً بالعالم\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_comparison() {
        let source = "١ < ٢ و ٣ > ٤";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_equality() {
        let source = "أ == ب";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_unary_negation() {
        let source = "-٥";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_unary_not() {
        let source = "ليس صحيح";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_parse_multiple_statements() {
        let source = r#"
            متغير أ = ١;
            متغير ب = ٢;
            دالة جمع() { أعد أ + ب }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.statements.len(), 3);
    }
}
