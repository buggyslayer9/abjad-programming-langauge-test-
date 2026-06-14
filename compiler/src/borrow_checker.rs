use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Borrow checker for the Abjad programming language
pub struct BorrowChecker {
    /// Track variable lifetimes
    lifetimes: HashMap<String, Lifetime>,
    
    /// Track active borrows
    active_borrows: Vec<BorrowInfo>,
    
    /// Current scope depth
    scope_depth: usize,
}

/// Represents a lifetime
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lifetime {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

/// Information about a borrow
#[derive(Debug, Clone)]
pub struct BorrowInfo {
    pub borrower: String,
    pub borrow_type: BorrowType,
    pub lifetime: String,
    pub scope: usize,
}

/// Type of borrow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowType {
    Immutable,
    Mutable,
}

impl BorrowChecker {
    /// Create a new borrow checker
    pub fn new() -> Self {
        BorrowChecker {
            lifetimes: HashMap::new(),
            active_borrows: Vec::new(),
            scope_depth: 0,
        }
    }

    /// Check an entire AST for borrow violations
    pub fn check(&mut self, ast: &AST) -> Result<()> {
        for statement in &ast.statements {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    /// Check a statement for borrow violations
    fn check_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => self.check_expression(expr)?,
            Statement::Let { name, value, .. } => {
                self.check_expression(value)?;
                self.lifetimes.insert(name.clone(), Lifetime {
                    name: name.clone(),
                    start: self.scope_depth,
                    end: self.scope_depth + 1,
                });
            }
            Statement::Assignment { target, value } => {
                self.check_expression(target)?;
                self.check_expression(value)?;
            }
            Statement::Function(func) => {
                self.scope_depth += 1;
                for stmt in &func.body {
                    self.check_statement(stmt)?;
                }
                self.scope_depth -= 1;
            }
            Statement::Struct(_) => {}
            Statement::Enum(_) => {}
            Statement::Import(_) => {}
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Check an expression for borrow violations
    fn check_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Literal(_) => {}
            Expression::Identifier(name) => {
                // Check if variable is still valid
                if let Some(lifetime) = self.lifetimes.get(name) {
                    if self.scope_depth > lifetime.end {
                        return Err(AbjadError::syntax(format!(
                            "Variable '{}' is out of scope",
                            name
                        )));
                    }
                }
            }
            Expression::Binary { left, right, .. } => {
                self.check_expression(left)?;
                self.check_expression(right)?;
            }
            Expression::Unary { operand, .. } => {
                self.check_expression(operand)?;
            }
            Expression::Call { function, arguments } => {
                self.check_expression(function)?;
                for arg in arguments {
                    self.check_expression(arg)?;
                }
            }
            Expression::ArrayLiteral(elements) => {
                for elem in elements {
                    self.check_expression(elem)?;
                }
            }
            Expression::Index { array, index } => {
                self.check_expression(array)?;
                self.check_expression(index)?;
            }
            Expression::TupleLiteral(elements) => {
                for elem in elements {
                    self.check_expression(elem)?;
                }
            }
            Expression::Block(statements) => {
                self.scope_depth += 1;
                for stmt in statements {
                    self.check_statement(stmt)?;
                }
                self.scope_depth -= 1;
            }
            Expression::If { condition, then_branch, else_branch } => {
                self.check_expression(condition)?;
                self.check_expression(then_branch)?;
                if let Some(else_br) = else_branch {
                    self.check_expression(else_br)?;
                }
            }
            Expression::Match { value, arms } => {
                self.check_expression(value)?;
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        self.check_expression(guard)?;
                    }
                    self.check_expression(&arm.body)?;
                }
            }
            Expression::Loop { body } => {
                self.check_expression(body)?;
            }
            Expression::While { condition, body } => {
                self.check_expression(condition)?;
                self.check_expression(body)?;
            }
            Expression::For { iterable, body, .. } => {
                self.check_expression(iterable)?;
                self.check_expression(body)?;
            }
            Expression::Break(value) => {
                if let Some(v) = value {
                    self.check_expression(v)?;
                }
            }
            Expression::Continue => {}
            Expression::Return(value) => {
                if let Some(v) = value {
                    self.check_expression(v)?;
                }
            }
            Expression::Cast { value, .. } => {
                self.check_expression(value)?;
            }
            Expression::Parenthesized(expr) => {
                self.check_expression(expr)?;
            }
            Expression::Assignment { .. } => {
                // Handled in check_statement
            }
        }
        Ok(())
    }

    /// Check if a variable has active borrows
    fn has_active_borrows(&self, name: &str) -> bool {
        self.active_borrows.iter().any(|b| b.borrower == name)
    }

    /// Check if there are any mutable borrows
    fn has_mutable_borrows(&self) -> bool {
        self.active_borrows.iter().any(|b| b.borrow_type == BorrowType::Mutable)
    }

    /// Add a borrow
    fn add_borrow(&mut self, borrower: String, borrow_type: BorrowType) -> Result<()> {
        // Check borrowing rules
        if borrow_type == BorrowType::Mutable {
            if self.has_mutable_borrows() {
                return Err(AbjadError::syntax(
                    "Cannot have multiple mutable borrows"
                ));
            }
            if self.has_active_borrows(&borrower) {
                return Err(AbjadError::syntax(
                    "Cannot borrow while already borrowed"
                ));
            }
        }

        self.active_borrows.push(BorrowInfo {
            borrower,
            borrow_type,
            lifetime: format!("'{}", self.scope_depth),
            scope: self.scope_depth,
        });

        Ok(())
    }

    /// Remove borrows that are out of scope
    fn remove_out_of_scope_borrows(&mut self) {
        self.active_borrows.retain(|b| b.scope >= self.scope_depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_borrow_check_simple() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut borrow_checker = BorrowChecker::new();
        assert!(borrow_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_borrow_check_function() {
        let source = "دالة مثال() { متغير أ = ١٠; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut borrow_checker = BorrowChecker::new();
        assert!(borrow_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_borrow_check_block() {
        let source = "{ متغير أ = ١٠; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut borrow_checker = BorrowChecker::new();
        assert!(borrow_checker.check(&ast).is_ok());
    }
}
