use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    O0, // No optimizations
    O1, // Basic optimizations
    O2, // Moderate optimizations
    O3, // Aggressive optimizations
    Os, // Size optimizations
}

/// Compiler optimizer for Abjad
pub struct Optimizer {
    /// Optimization level
    level: OptimizationLevel,
    
    /// Constant values
    constants: HashMap<String, Literal>,
}

impl Optimizer {
    /// Create a new optimizer
    pub fn new(level: OptimizationLevel) -> Self {
        Optimizer {
            level,
            constants: HashMap::new(),
        }
    }

    /// Optimize an AST
    pub fn optimize(&mut self, ast: &mut AST) -> Result<()> {
        match self.level {
            OptimizationLevel::O0 => return Ok(()),
            OptimizationLevel::O1 => self.optimize_basic(ast)?,
            OptimizationLevel::O2 => {
                self.optimize_basic(ast)?;
                self.optimize_moderate(ast)?;
            }
            OptimizationLevel::O3 => {
                self.optimize_basic(ast)?;
                self.optimize_moderate(ast)?;
                self.optimize_aggressive(ast)?;
            }
            OptimizationLevel::Os => {
                self.optimize_basic(ast)?;
                self.optimize_size(ast)?;
            }
        }
        Ok(())
    }

    /// Basic optimizations (O1)
    fn optimize_basic(&mut self, ast: &mut AST) -> Result<()> {
        for statement in &mut ast.statements {
            self.optimize_statement_basic(statement)?;
        }
        Ok(())
    }

    /// Moderate optimizations (O2)
    fn optimize_moderate(&mut self, ast: &mut AST) -> Result<()> {
        // Constant propagation
        self.constant_propagation(ast)?;
        
        // Dead code elimination
        self.dead_code_elimination(ast)?;
        
        Ok(())
    }

    /// Aggressive optimizations (O3)
    fn optimize_aggressive(&mut self, ast: &mut AST) -> Result<()> {
        // Loop unrolling
        self.loop_unrolling(ast)?;
        
        // Function inlining
        self.function_inlining(ast)?;
        
        Ok(())
    }

    /// Size optimizations (Os)
    fn optimize_size(&mut self, ast: &mut AST) -> Result<()> {
        // Inline small functions
        self.function_inlining(ast)?;
        
        // Remove unused code
        self.dead_code_elimination(ast)?;
        
        Ok(())
    }

    /// Optimize a statement (basic)
    fn optimize_statement_basic(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::Let { name, value, .. } => {
                self.optimize_expression_basic(value)?;
                
                // Constant folding
                if let Expression::Literal(lit) = value {
                    self.constants.insert(name.clone(), lit.clone());
                }
            }
            Statement::Assignment { target, value } => {
                self.optimize_expression_basic(target)?;
                self.optimize_expression_basic(value)?;
            }
            Statement::Function(func) => {
                for stmt in &mut func.body {
                    self.optimize_statement_basic(stmt)?;
                }
            }
            Statement::Expression(expr) => {
                self.optimize_expression_basic(expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Optimize an expression (basic)
    fn optimize_expression_basic(&mut self, expr: &mut Expression) -> Result<()> {
        match expr {
            Expression::Binary { op, left, right } => {
                self.optimize_expression_basic(left)?;
                self.optimize_expression_basic(right)?;
                
                // Constant folding
                if let (Expression::Literal(l), Expression::Literal(r)) = (&**left, &**right) {
                    if let Some(result) = self.fold_constant(*op, l, r) {
                        *expr = Expression::Literal(result);
                    }
                }
            }
            Expression::Unary { op, operand } => {
                self.optimize_expression_basic(operand)?;
                
                // Constant folding
                if let Expression::Literal(lit) = &**operand {
                    if let Some(result) = self.fold_constant_unary(*op, lit) {
                        *expr = Expression::Literal(result);
                    }
                }
            }
            Expression::Call { function, arguments } => {
                self.optimize_expression_basic(function)?;
                for arg in arguments {
                    self.optimize_expression_basic(arg)?;
                }
            }
            Expression::ArrayLiteral(elements) => {
                for elem in elements {
                    self.optimize_expression_basic(elem)?;
                }
            }
            Expression::Index { array, index } => {
                self.optimize_expression_basic(array)?;
                self.optimize_expression_basic(index)?;
            }
            Expression::TupleLiteral(elements) => {
                for elem in elements {
                    self.optimize_expression_basic(elem)?;
                }
            }
            Expression::Block(statements) => {
                for stmt in statements {
                    self.optimize_statement_basic(stmt)?;
                }
            }
            Expression::If { condition, then_branch, else_branch } => {
                self.optimize_expression_basic(condition)?;
                self.optimize_expression_basic(then_branch)?;
                if let Some(else_br) = else_branch {
                    self.optimize_expression_basic(else_br)?;
                }
            }
            Expression::Loop { body } => {
                self.optimize_expression_basic(body)?;
            }
            Expression::While { condition, body } => {
                self.optimize_expression_basic(condition)?;
                self.optimize_expression_basic(body)?;
            }
            Expression::For { iterable, body, .. } => {
                self.optimize_expression_basic(iterable)?;
                self.optimize_expression_basic(body)?;
            }
            Expression::Break(value) => {
                if let Some(v) = value {
                    self.optimize_expression_basic(v)?;
                }
            }
            Expression::Return(value) => {
                if let Some(v) = value {
                    self.optimize_expression_basic(v)?;
                }
            }
            Expression::Cast { value, .. } => {
                self.optimize_expression_basic(value)?;
            }
            Expression::Parenthesized(expr) => {
                self.optimize_expression_basic(expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Fold constant binary operations
    fn fold_constant(&self, op: BinaryOperator, left: &Literal, right: &Literal) -> Option<Literal> {
        match (left, right) {
            (Literal::Integer(l), Literal::Integer(r)) => {
                match op {
                    BinaryOperator::Add => Some(Literal::Integer(l + r)),
                    BinaryOperator::Sub => Some(Literal::Integer(l - r)),
                    BinaryOperator::Mul => Some(Literal::Integer(l * r)),
                    BinaryOperator::Div => Some(Literal::Integer(l / r)),
                    BinaryOperator::Mod => Some(Literal::Integer(l % r)),
                    _ => None,
                }
            }
            (Literal::Float(l), Literal::Float(r)) => {
                match op {
                    BinaryOperator::Add => Some(Literal::Float(l + r)),
                    BinaryOperator::Sub => Some(Literal::Float(l - r)),
                    BinaryOperator::Mul => Some(Literal::Float(l * r)),
                    BinaryOperator::Div => Some(Literal::Float(l / r)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Fold constant unary operations
    fn fold_constant_unary(&self, op: UnaryOperator, operand: &Literal) -> Option<Literal> {
        match operand {
            Literal::Integer(n) => {
                match op {
                    UnaryOperator::Negate => Some(Literal::Integer(-n)),
                    _ => None,
                }
            }
            Literal::Float(n) => {
                match op {
                    UnaryOperator::Negate => Some(Literal::Float(-n)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Constant propagation
    fn constant_propagation(&mut self, ast: &mut AST) -> Result<()> {
        for statement in &mut ast.statements {
            if let Statement::Let { name, value, .. } = statement {
                if let Expression::Identifier(var_name) = &**value {
                    if let Some(lit) = self.constants.get(var_name) {
                        *value = Box::new(Expression::Literal(lit.clone()));
                    }
                }
            }
        }
        Ok(())
    }

    /// Dead code elimination
    fn dead_code_elimination(&mut self, ast: &mut AST) -> Result<()> {
        // Remove unreachable code after return
        let mut in_return = false;
        ast.statements.retain(|stmt| {
            if in_return {
                return false;
            }
            if matches!(stmt, Statement::Return(_)) {
                in_return = true;
            }
            true
        });
        Ok(())
    }

    /// Loop unrolling
    fn loop_unrolling(&mut self, ast: &mut AST) -> Result<()> {
        // Simple loop unrolling for small loops
        for statement in &mut ast.statements {
            if let Statement::Expression(expr) = statement {
                if let Expression::Loop { body } = expr {
                    if let Expression::Block(statements) = &**body {
                        if statements.len() <= 4 {
                            // Unroll small loops
                            *expr = body.clone();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Function inlining
    fn function_inlining(&mut self, ast: &mut AST) -> Result<()> {
        // Inline small functions
        // This is a simplified implementation
        Ok(())
    }

    /// Get optimization level
    pub fn level(&self) -> OptimizationLevel {
        self.level
    }

    /// Set optimization level
    pub fn set_level(&mut self, level: OptimizationLevel) {
        self.level = level;
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(OptimizationLevel::O2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_default() {
        let optimizer = Optimizer::default();
        assert_eq!(optimizer.level(), OptimizationLevel::O2);
    }

    #[test]
    fn test_constant_folding() {
        let mut optimizer = Optimizer::new(OptimizationLevel::O1);
        
        let mut expr = Expression::Binary {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Literal(Literal::Integer(2))),
            right: Box::new(Expression::Literal(Literal::Integer(3))),
        };
        
        optimizer.optimize_expression_basic(&mut expr).unwrap();
        
        assert!(matches!(expr, Expression::Literal(Literal::Integer(5))));
    }

    #[test]
    fn test_constant_folding_float() {
        let mut optimizer = Optimizer::new(OptimizationLevel::O1);
        
        let mut expr = Expression::Binary {
            op: BinaryOperator::Mul,
            left: Box::new(Expression::Literal(Literal::Float(2.0))),
            right: Box::new(Expression::Literal(Literal::Float(3.0))),
        };
        
        optimizer.optimize_expression_basic(&mut expr).unwrap();
        
        assert!(matches!(expr, Expression::Literal(Literal::Float(6.0))));
    }

    #[test]
    fn test_unary_negation() {
        let mut optimizer = Optimizer::new(OptimizationLevel::O1);
        
        let mut expr = Expression::Unary {
            op: UnaryOperator::Negate,
            operand: Box::new(Expression::Literal(Literal::Integer(5))),
        };
        
        optimizer.optimize_expression_basic(&mut expr).unwrap();
        
        assert!(matches!(expr, Expression::Literal(Literal::Integer(-5))));
    }
}
