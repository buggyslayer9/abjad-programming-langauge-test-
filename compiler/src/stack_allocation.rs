use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Stack allocation analyzer for the Abjad programming language
pub struct StackAllocationAnalyzer {
    /// Track stack variables
    stack_variables: HashMap<String, StackVariable>,
    
    /// Current stack offset
    stack_offset: usize,
    
    /// Current scope depth
    scope_depth: usize,
}

/// Information about a stack variable
#[derive(Debug, Clone)]
pub struct StackVariable {
    pub name: String,
    pub type_annotation: Type,
    pub offset: usize,
    pub size: usize,
    pub scope: usize,
}

impl StackAllocationAnalyzer {
    /// Create a new stack allocation analyzer
    pub fn new() -> Self {
        StackAllocationAnalyzer {
            stack_variables: HashMap::new(),
            stack_offset: 0,
            scope_depth: 0,
        }
    }

    /// Analyze an entire AST for stack allocation
    pub fn analyze(&mut self, ast: &AST) -> Result<StackAllocationInfo> {
        for statement in &ast.statements {
            self.analyze_statement(statement)?;
        }
        
        Ok(StackAllocationInfo {
            variables: self.stack_variables.clone(),
            total_size: self.stack_offset,
            max_depth: self.scope_depth,
        })
    }

    /// Analyze a statement for stack allocation
    fn analyze_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => self.analyze_expression(expr)?,
            Statement::Let { name, type_annotation, value, .. } => {
                self.analyze_expression(value)?;
                
                let ty = type_annotation.clone().unwrap_or_else(|| {
                    self.infer_type(value)
                });
                
                let size = self.type_size(&ty);
                let offset = self.stack_offset;
                
                self.stack_variables.insert(name.clone(), StackVariable {
                    name: name.clone(),
                    type_annotation: ty,
                    offset,
                    size,
                    scope: self.scope_depth,
                });
                
                self.stack_offset += size;
            }
            Statement::Assignment { target, value } => {
                self.analyze_expression(target)?;
                self.analyze_expression(value)?;
            }
            Statement::Function(func) => {
                self.scope_depth += 1;
                let old_offset = self.stack_offset;
                self.stack_offset = 0;  // New stack frame for function
                
                for stmt in &func.body {
                    self.analyze_statement(stmt)?;
                }
                
                self.stack_offset = old_offset;  // Restore stack offset
                self.scope_depth -= 1;
            }
            Statement::Struct(_) => {}
            Statement::Enum(_) => {}
            Statement::Import(_) => {}
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Analyze an expression for stack allocation
    fn analyze_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Literal(_) => {}
            Expression::Identifier(name) => {
                // Check if variable is on stack
                if let Some(var) = self.stack_variables.get(name) {
                    if var.scope > self.scope_depth {
                        return Err(AbjadError::syntax(format!(
                            "Variable '{}' is out of scope",
                            name
                        )));
                    }
                }
            }
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)?;
            }
            Expression::Unary { operand, .. } => {
                self.analyze_expression(operand)?;
            }
            Expression::Call { function, arguments } => {
                self.analyze_expression(function)?;
                for arg in arguments {
                    self.analyze_expression(arg)?;
                }
            }
            Expression::ArrayLiteral(elements) => {
                for elem in elements {
                    self.analyze_expression(elem)?;
                }
                // Arrays are heap-allocated, not stack
            }
            Expression::Index { array, index } => {
                self.analyze_expression(array)?;
                self.analyze_expression(index)?;
            }
            Expression::TupleLiteral(elements) => {
                for elem in elements {
                    self.analyze_expression(elem)?;
                }
            }
            Expression::Block(statements) => {
                self.scope_depth += 1;
                for stmt in statements {
                    self.analyze_statement(stmt)?;
                }
                self.scope_depth -= 1;
            }
            Expression::If { condition, then_branch, else_branch } => {
                self.analyze_expression(condition)?;
                self.analyze_expression(then_branch)?;
                if let Some(else_br) = else_branch {
                    self.analyze_expression(else_br)?;
                }
            }
            Expression::Match { value, arms } => {
                self.analyze_expression(value)?;
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        self.analyze_expression(guard)?;
                    }
                    self.analyze_expression(&arm.body)?;
                }
            }
            Expression::Loop { body } => {
                self.analyze_expression(body)?;
            }
            Expression::While { condition, body } => {
                self.analyze_expression(condition)?;
                self.analyze_expression(body)?;
            }
            Expression::For { iterable, body, .. } => {
                self.analyze_expression(iterable)?;
                self.analyze_expression(body)?;
            }
            Expression::Break(value) => {
                if let Some(v) = value {
                    self.analyze_expression(v)?;
                }
            }
            Expression::Continue => {}
            Expression::Return(value) => {
                if let Some(v) = value {
                    self.analyze_expression(v)?;
                }
            }
            Expression::Cast { value, .. } => {
                self.analyze_expression(value)?;
            }
            Expression::Parenthesized(expr) => {
                self.analyze_expression(expr)?;
            }
            Expression::Assignment { .. } => {
                // Handled in analyze_statement
            }
        }
        Ok(())
    }

    /// Infer the type of an expression
    fn infer_type(&self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(Literal::Integer(_)) => Type::I32,
            Expression::Literal(Literal::Float(_)) => Type::F64,
            Expression::Literal(Literal::String(_)) => Type::String,
            Expression::Literal(Literal::Char(_)) => Type::Char,
            Expression::Literal(Literal::Boolean(_)) => Type::Bool,
            Expression::ArrayLiteral(elements) => {
                if let Some(first) = elements.first() {
                    let elem_type = self.infer_type(first);
                    Type::Array(Box::new(elem_type), Some(elements.len()))
                } else {
                    Type::Array(Box::new(Type::I32), None)
                }
            }
            _ => Type::I32,  // Default
        }
    }

    /// Get the size of a type in bytes
    fn type_size(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I32 | Type::U32 | Type::F32 => 4,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::Bool => 1,
            Type::Char => 4,
            Type::String => 8,  // Pointer to string data
            Type::Array(elem_type, size) => {
                let elem_size = self.type_size(elem_type);
                elem_size * size.unwrap_or(0)
            }
            Type::Tuple(types) => {
                types.iter().map(|t| self.type_size(t)).sum()
            }
            Type::Slice(_) => 8,  // Pointer
            Type::Function(_, _) => 8,  // Function pointer
            Type::Named(_) => 8,  // Assume pointer for user-defined types
            Type::Generic(_) => 8,
        }
    }
}

/// Information about stack allocation
#[derive(Debug, Clone)]
pub struct StackAllocationInfo {
    pub variables: HashMap<String, StackVariable>,
    pub total_size: usize,
    pub max_depth: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_stack_allocation_simple() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = StackAllocationAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert_eq!(info.variables.len(), 1);
        assert!(info.total_size > 0);
    }

    #[test]
    fn test_stack_allocation_multiple() {
        let source = "متغير أ = ١٠; متغير ب = ٢٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = StackAllocationAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert_eq!(info.variables.len(), 2);
    }

    #[test]
    fn test_stack_allocation_function() {
        let source = "دالة مثال() { متغير أ = ١٠; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = StackAllocationAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.max_depth > 0);
    }
}
