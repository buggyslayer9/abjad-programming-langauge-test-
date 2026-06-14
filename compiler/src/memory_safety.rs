use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Memory safety checker for the Abjad programming language
pub struct MemorySafetyChecker {
    /// Track allocated memory
    allocations: HashMap<String, AllocationInfo>,
    
    /// Track memory leaks
    potential_leaks: Vec<String>,
    
    /// Track dangling pointers
    dangling_pointers: Vec<String>,
}

/// Information about a memory allocation
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub name: String,
    pub allocation_type: AllocationType,
    pub size: Option<usize>,
    pub scope: usize,
}

/// Type of memory allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationType {
    Stack,
    Heap,
    Static,
}

impl MemorySafetyChecker {
    /// Create a new memory safety checker
    pub fn new() -> Self {
        MemorySafetyChecker {
            allocations: HashMap::new(),
            potential_leaks: Vec::new(),
            dangling_pointers: Vec::new(),
        }
    }

    /// Check an entire AST for memory safety violations
    pub fn check(&mut self, ast: &AST) -> Result<()> {
        for statement in &ast.statements {
            self.check_statement(statement)?;
        }
        
        // Check for memory leaks
        if !self.potential_leaks.is_empty() {
            return Err(AbjadError::internal(format!(
                "Potential memory leaks detected: {:?}",
                self.potential_leaks
            )));
        }
        
        // Check for dangling pointers
        if !self.dangling_pointers.is_empty() {
            return Err(AbjadError::internal(format!(
                "Dangling pointers detected: {:?}",
                self.dangling_pointers
            )));
        }
        
        Ok(())
    }

    /// Check a statement for memory safety violations
    fn check_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => self.check_expression(expr)?,
            Statement::Let { name, value, .. } => {
                self.check_expression(value)?;
                self.track_allocation(name.clone(), self.determine_allocation_type(value));
            }
            Statement::Assignment { target, value } => {
                self.check_expression(target)?;
                self.check_expression(value)?;
            }
            Statement::Function(func) => {
                for stmt in &func.body {
                    self.check_statement(stmt)?;
                }
            }
            Statement::Struct(_) => {}
            Statement::Enum(_) => {}
            Statement::Import(_) => {}
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Check an expression for memory safety violations
    fn check_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Literal(_) => {}
            Expression::Identifier(name) => {
                // Check if the variable is still allocated
                if !self.allocations.contains_key(name) {
                    return Err(AbjadError::syntax(format!(
                        "Use of potentially unallocated variable: {}",
                        name
                    )));
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
                // Arrays are heap-allocated
                self.track_heap_allocation();
            }
            Expression::Index { array, index } => {
                self.check_expression(array)?;
                self.check_expression(index)?;
                
                // Check for bounds (simplified)
                if let Expression::Literal(Literal::Integer(idx)) = &**index {
                    if *idx < 0 {
                        return Err(AbjadError::syntax("Array index cannot be negative"));
                    }
                }
            }
            Expression::TupleLiteral(elements) => {
                for elem in elements {
                    self.check_expression(elem)?;
                }
            }
            Expression::Block(statements) => {
                for stmt in statements {
                    self.check_statement(stmt)?;
                }
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
            // Assignment is a Statement, not an Expression
        }
        Ok(())
    }

    /// Determine the allocation type for a value
    fn determine_allocation_type(&self, expr: &Expression) -> AllocationType {
        match expr {
            Expression::ArrayLiteral(_) => AllocationType::Heap,
            Expression::Literal(_) => AllocationType::Stack,
            Expression::Identifier(_) => AllocationType::Stack,
            _ => AllocationType::Stack,
        }
    }

    /// Track a memory allocation
    fn track_allocation(&mut self, name: String, allocation_type: AllocationType) {
        self.allocations.insert(name.clone(), AllocationInfo {
            name: name.clone(),
            allocation_type,
            size: None,
            scope: 0,
        });
    }

    /// Track a heap allocation
    fn track_heap_allocation(&mut self) {
        // Track that a heap allocation occurred
        // In a real implementation, this would track specific allocations
    }

    /// Check for double allocation
    fn check_double_allocation(&self, name: &str) -> Result<()> {
        if self.allocations.contains_key(name) {
            return Err(AbjadError::syntax(format!(
                "Double allocation detected for: {}",
                name
            )));
        }
        Ok(())
    }

    /// Check for use-after-free
    fn check_use_after_free(&self, name: &str) -> Result<()> {
        if self.dangling_pointers.contains(&name.to_string()) {
            return Err(AbjadError::syntax(format!(
                "Use-after-free detected for: {}",
                name
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_memory_safety_simple() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut checker = MemorySafetyChecker::new();
        assert!(checker.check(&ast).is_ok());
    }

    #[test]
    fn test_memory_safety_array() {
        let source = "متغير مصفوفة = [١، ٢، ٣];";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut checker = MemorySafetyChecker::new();
        assert!(checker.check(&ast).is_ok());
    }

    #[test]
    fn test_memory_safety_bounds() {
        let source = "متغير مصفوفة = [١، ٢، ٣]; مصفوفة[١];";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut checker = MemorySafetyChecker::new();
        assert!(checker.check(&ast).is_ok());
    }
}
