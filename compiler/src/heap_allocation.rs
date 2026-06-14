use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Heap allocation analyzer for the Abjad programming language
pub struct HeapAllocationAnalyzer {
    /// Track heap allocations
    heap_allocations: HashMap<String, HeapAllocation>,
    
    /// Track allocation sites
    allocation_sites: Vec<AllocationSite>,
    
    /// Current allocation ID
    allocation_id: usize,
}

/// Information about a heap allocation
#[derive(Debug, Clone)]
pub struct HeapAllocation {
    pub id: usize,
    pub name: String,
    pub type_annotation: Type,
    pub size: usize,
    pub allocation_type: HeapAllocationType,
}

/// Type of heap allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapAllocationType {
    DynamicArray,
    BoxedValue,
    SharedData,
    CustomAllocator,
}

/// Information about an allocation site
#[derive(Debug, Clone)]
pub struct AllocationSite {
    pub id: usize,
    pub location: String,
    pub allocation_type: HeapAllocationType,
}

impl HeapAllocationAnalyzer {
    /// Create a new heap allocation analyzer
    pub fn new() -> Self {
        HeapAllocationAnalyzer {
            heap_allocations: HashMap::new(),
            allocation_sites: Vec::new(),
            allocation_id: 0,
        }
    }

    /// Analyze an entire AST for heap allocation
    pub fn analyze(&mut self, ast: &AST) -> Result<HeapAllocationInfo> {
        for statement in &ast.statements {
            self.analyze_statement(statement)?;
        }
        
        Ok(HeapAllocationInfo {
            allocations: self.heap_allocations.clone(),
            sites: self.allocation_sites.clone(),
            total_allocations: self.allocation_id,
        })
    }

    /// Analyze a statement for heap allocation
    fn analyze_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => self.analyze_expression(expr)?,
            Statement::Let { name, type_annotation, value, .. } => {
                self.analyze_expression(value)?;
                
                // Check if this is a heap allocation
                if self.is_heap_allocation(value) {
                    let ty = type_annotation.clone().unwrap_or_else(|| {
                        self.infer_type(value)
                    });
                    
                    let size = self.type_size(&ty);
                    let allocation_type = self.determine_allocation_type(value);
                    
                    let id = self.allocation_id;
                    self.allocation_id += 1;
                    
                    self.heap_allocations.insert(name.clone(), HeapAllocation {
                        id,
                        name: name.clone(),
                        type_annotation: ty,
                        size,
                        allocation_type,
                    });
                    
                    self.allocation_sites.push(AllocationSite {
                        id,
                        location: format!("variable: {}", name),
                        allocation_type,
                    });
                }
            }
            Statement::Assignment { target, value } => {
                self.analyze_expression(target)?;
                self.analyze_expression(value)?;
            }
            Statement::Function(func) => {
                for stmt in &func.body {
                    self.analyze_statement(stmt)?;
                }
            }
            Statement::Struct(_) => {}
            Statement::Enum(_) => {}
            Statement::Import(_) => {}
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Analyze an expression for heap allocation
    fn analyze_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Literal(_) => {}
            Expression::Identifier(_) => {}
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
                // Arrays are heap-allocated
                self.track_heap_allocation("array_literal", HeapAllocationType::DynamicArray);
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
                for stmt in statements {
                    self.analyze_statement(stmt)?;
                }
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

    /// Check if an expression results in heap allocation
    fn is_heap_allocation(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::ArrayLiteral(_))
    }

    /// Determine the type of heap allocation
    fn determine_allocation_type(&self, expr: &Expression) -> HeapAllocationType {
        match expr {
            Expression::ArrayLiteral(_) => HeapAllocationType::DynamicArray,
            _ => HeapAllocationType::BoxedValue,
        }
    }

    /// Track a heap allocation
    fn track_heap_allocation(&mut self, location: &str, allocation_type: HeapAllocationType) {
        let id = self.allocation_id;
        self.allocation_id += 1;
        
        self.allocation_sites.push(AllocationSite {
            id,
            location: location.to_string(),
            allocation_type,
        });
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
            _ => Type::I32,
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
            Type::String => 8,
            Type::Array(elem_type, size) => {
                let elem_size = self.type_size(elem_type);
                elem_size * size.unwrap_or(0)
            }
            Type::Tuple(types) => {
                types.iter().map(|t| self.type_size(t)).sum()
            }
            Type::Slice(_) => 8,
            Type::Function(_, _) => 8,
            Type::Named(_) => 8,
            Type::Generic(_) => 8,
        }
    }
}

/// Information about heap allocation
#[derive(Debug, Clone)]
pub struct HeapAllocationInfo {
    pub allocations: HashMap<String, HeapAllocation>,
    pub sites: Vec<AllocationSite>,
    pub total_allocations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_heap_allocation_array() {
        let source = "متغير مصفوفة = [١، ٢، ٣];";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = HeapAllocationAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.total_allocations > 0);
    }

    #[test]
    fn test_heap_allocation_multiple() {
        let source = "متغير مصفوفة_١ = [١، ٢]; متغير مصفوفة_٢ = [٣، ٤];";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = HeapAllocationAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.total_allocations >= 2);
    }
}
