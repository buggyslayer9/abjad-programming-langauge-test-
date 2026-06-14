use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Type checker for the Abjad programming language
pub struct TypeChecker {
    /// Symbol table for variables
    symbols: HashMap<String, Type>,
    
    /// Symbol table for functions
    functions: HashMap<String, FunctionType>,
    
    /// Symbol table for structs
    structs: HashMap<String, StructType>,
    
    /// Symbol table for enums
    enums: HashMap<String, EnumType>,
}

/// Represents a function type
#[derive(Debug, Clone)]
struct FunctionType {
    parameters: Vec<Type>,
    return_type: Type,
}

/// Represents a struct type
#[derive(Debug, Clone)]
struct StructType {
    fields: HashMap<String, Type>,
}

/// Represents an enum type
#[derive(Debug, Clone)]
struct EnumType {
    variants: HashMap<String, Vec<Type>>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        TypeChecker {
            symbols: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
        }
    }

    /// Type check an entire AST
    pub fn check(&mut self, ast: &AST) -> Result<()> {
        // First pass: collect declarations
        self.collect_declarations(ast)?;

        // Second pass: type check statements
        for statement in &ast.statements {
            self.check_statement(statement)?;
        }

        Ok(())
    }

    /// Collect declarations (functions, structs, enums)
    fn collect_declarations(&mut self, ast: &AST) -> Result<()> {
        for statement in &ast.statements {
            match statement {
                Statement::Function(func) => {
                    let return_type = func.return_type.clone().unwrap_or(Type::Named("Unit".to_string()));
                    let param_types: Vec<Type> = func.parameters.iter().map(|p| p.type_annotation.clone()).collect();
                    
                    self.functions.insert(
                        func.name.clone(),
                        FunctionType {
                            parameters: param_types,
                            return_type,
                        }
                    );
                }
                Statement::Struct(struct_) => {
                    let mut fields = HashMap::new();
                    for field in &struct_.fields {
                        fields.insert(field.name.clone(), field.type_annotation.clone());
                    }
                    
                    self.structs.insert(
                        struct_.name.clone(),
                        StructType { fields }
                    );
                }
                Statement::Enum(enum_) => {
                    let mut variants = HashMap::new();
                    for variant in &enum_.variants {
                        variants.insert(variant.name.clone(), variant.fields.clone());
                    }
                    
                    self.enums.insert(
                        enum_.name.clone(),
                        EnumType { variants }
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Type check a statement
    fn check_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
            }
            Statement::Let { name, type_annotation, value, mutable } => {
                let value_type = self.check_expression(value)?;
                
                if let Some(ty) = type_annotation {
                    if !self.types_compatible(&value_type, ty) {
                        return Err(AbjadError::type_error(format!(
                            "Type mismatch: expected {}, found {}",
                            ty, value_type
                        )));
                    }
                }
                
                self.symbols.insert(name.clone(), value_type);
            }
            Statement::Assignment { target, value } => {
                let target_type = self.check_expression(target)?;
                let value_type = self.check_expression(value)?;
                
                if !self.types_compatible(&target_type, &value_type) {
                    return Err(AbjadError::type_error(format!(
                        "Type mismatch in assignment: expected {}, found {}",
                        target_type, value_type
                    )));
                }
            }
            Statement::Function(func) => {
                // Create a new scope for the function
                let mut old_symbols = std::mem::take(&mut self.symbols);
                
                // Add parameters to symbol table
                for param in &func.parameters {
                    self.symbols.insert(param.name.clone(), param.type_annotation.clone());
                }
                
                // Type check function body
                for stmt in &func.body {
                    self.check_statement(stmt)?;
                }
                
                // Restore old symbols
                self.symbols = old_symbols;
            }
            Statement::Struct(_) => {
                // Already collected in collect_declarations
            }
            Statement::Enum(_) => {
                // Already collected in collect_declarations
            }
            Statement::Import(_) => {
                // Import statements are handled separately
            }
            Statement::Empty => {}
        }

        Ok(())
    }

    /// Type check an expression
    fn check_expression(&mut self, expr: &Expression) -> Result<Type> {
        match expr {
            Expression::Literal(lit) => Ok(self.type_of_literal(lit)),
            Expression::Identifier(name) => {
                self.symbols.get(name)
                    .cloned()
                    .ok_or_else(|| AbjadError::type_error(format!("Undefined variable: {}", name)))
            }
            Expression::Binary { op, left, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                self.check_binary_operation(op, &left_type, &right_type)
            }
            Expression::Unary { op, operand } => {
                let operand_type = self.check_expression(operand)?;
                self.check_unary_operation(op, &operand_type)
            }
            Expression::Call { function, arguments } => {
                let function_type = self.check_expression(function)?;
                self.check_function_call(&function_type, arguments)
            }
            Expression::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    return Ok(Type::Array(Box::new(Type::Named("Unknown".to_string())), None));
                }
                
                let first_type = self.check_expression(&elements[0])?;
                for elem in &elements[1..] {
                    let elem_type = self.check_expression(elem)?;
                    if !self.types_compatible(&first_type, &elem_type) {
                        return Err(AbjadError::type_error(format!(
                            "Array elements must have the same type: expected {}, found {}",
                            first_type, elem_type
                        )));
                    }
                }
                
                Ok(Type::Array(Box::new(first_type), Some(elements.len())))
            }
            Expression::Index { array, index } => {
                let array_type = self.check_expression(array)?;
                let index_type = self.check_expression(index)?;
                
                if !self.is_integer_type(&index_type) {
                    return Err(AbjadError::type_error("Array index must be an integer"));
                }
                
                match array_type {
                    Type::Array(element_type, _) => Ok(*element_type),
                    Type::Slice(element_type) => Ok(*element_type),
                    _ => Err(AbjadError::type_error("Cannot index non-array type")),
                }
            }
            Expression::TupleLiteral(elements) => {
                let mut types = Vec::new();
                for elem in elements {
                    types.push(self.check_expression(elem)?);
                }
                Ok(Type::Tuple(types))
            }
            Expression::Block(statements) => {
                for stmt in statements {
                    self.check_statement(stmt)?;
                }
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::If { condition, then_branch, else_branch } => {
                let condition_type = self.check_expression(condition)?;
                
                if !self.is_boolean_type(&condition_type) {
                    return Err(AbjadError::type_error("If condition must be a boolean"));
                }
                
                let then_type = self.check_expression(then_branch)?;
                
                if let Some(else_br) = else_branch {
                    let else_type = self.check_expression(else_br)?;
                    if !self.types_compatible(&then_type, &else_type) {
                        return Err(AbjadError::type_error(format!(
                            "If branches must have the same type: expected {}, found {}",
                            then_type, else_type
                        )));
                    }
                }
                
                Ok(then_type)
            }
            Expression::Match { value, arms } => {
                let value_type = self.check_expression(value)?;
                let mut arm_types = Vec::new();
                
                for arm in arms {
                    // Check pattern compatibility with value type
                    self.check_pattern(&arm.pattern, &value_type)?;
                    
                    // Check guard if present
                    if let Some(guard) = &arm.guard {
                        let guard_type = self.check_expression(guard)?;
                        if !self.is_boolean_type(&guard_type) {
                            return Err(AbjadError::type_error("Match guard must be a boolean"));
                        }
                    }
                    
                    let arm_type = self.check_expression(&arm.body)?;
                    arm_types.push(arm_type);
                }
                
                // All arms must have the same type
                if let Some(first_type) = arm_types.first() {
                    for arm_type in &arm_types[1..] {
                        if !self.types_compatible(first_type, arm_type) {
                            return Err(AbjadError::type_error("All match arms must have the same type"));
                        }
                    }
                    Ok(first_type.clone())
                } else {
                    Ok(Type::Named("Unit".to_string()))
                }
            }
            Expression::Loop { body } => {
                self.check_expression(body)?;
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::While { condition, body } => {
                let condition_type = self.check_expression(condition)?;
                
                if !self.is_boolean_type(&condition_type) {
                    return Err(AbjadError::type_error("While condition must be a boolean"));
                }
                
                self.check_expression(body)?;
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::For { pattern, iterable, body } => {
                let iterable_type = self.check_expression(iterable)?;
                
                // Check that iterable is an array or slice
                match iterable_type {
                    Type::Array(_, _) | Type::Slice(_) => {}
                    _ => return Err(AbjadError::type_error("For loop must iterate over array or slice")),
                }
                
                self.check_expression(body)?;
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::Break(value) => {
                if let Some(v) = value {
                    self.check_expression(v)?;
                }
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::Continue => Ok(Type::Named("Unit".to_string())),
            Expression::Return(value) => {
                if let Some(v) = value {
                    self.check_expression(v)?;
                }
                Ok(Type::Named("Unit".to_string()))
            }
            Expression::Cast { value, target_type } => {
                let value_type = self.check_expression(value)?;
                self.check_cast(&value_type, target_type)?;
                Ok(target_type.clone())
            }
            Expression::Parenthesized(expr) => self.check_expression(expr),
            Expression::Assignment { .. } => {
                // This should be handled in check_statement
                Ok(Type::Named("Unit".to_string()))
            }
        }
    }

    /// Check if two types are compatible
    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::I8, Type::I8) => true,
            (Type::I16, Type::I16) => true,
            (Type::I32, Type::I32) => true,
            (Type::I64, Type::I64) => true,
            (Type::U8, Type::U8) => true,
            (Type::U16, Type::U16) => true,
            (Type::U32, Type::U32) => true,
            (Type::U64, Type::U64) => true,
            (Type::F32, Type::F32) => true,
            (Type::F64, Type::F64) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Char, Type::Char) => true,
            (Type::Named(a), Type::Named(b)) if a == b => true,
            (Type::Array(t1, s1), Type::Array(t2, s2)) => {
                self.types_compatible(t1, t2) && s1 == s2
            }
            (Type::Slice(t1), Type::Slice(t2)) => self.types_compatible(t1, t2),
            (Type::Tuple(types1), Type::Tuple(types2)) => {
                if types1.len() != types2.len() {
                    return false;
                }
                types1.iter().zip(types2.iter()).all(|(t1, t2)| self.types_compatible(t1, t2))
            }
            _ => false,
        }
    }

    /// Get the type of a literal
    fn type_of_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Integer(_) => Type::I32,
            Literal::Float(_) => Type::F64,
            Literal::String(_) => Type::String,
            Literal::Char(_) => Type::Char,
            Literal::Boolean(_) => Type::Bool,
            Literal::Null => Type::Named("Null".to_string()),
        }
    }

    /// Check a binary operation
    fn check_binary_operation(&self, op: &BinaryOperator, left: &Type, right: &Type) -> Result<Type> {
        match op {
            BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => {
                if self.is_numeric_type(left) && self.is_numeric_type(right) {
                    Ok(self.promote_numeric_types(left, right))
                } else {
                    Err(AbjadError::type_error(format!(
                        "Binary operation requires numeric types: found {} and {}",
                        left, right
                    )))
                }
            }
            BinaryOperator::Power => {
                if self.is_numeric_type(left) && self.is_numeric_type(right) {
                    Ok(left.clone())
                } else {
                    Err(AbjadError::type_error("Power operation requires numeric types"))
                }
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::Less | BinaryOperator::LessEqual | BinaryOperator::Greater | BinaryOperator::GreaterEqual => {
                if self.types_compatible(left, right) {
                    Ok(Type::Bool)
                } else {
                    Err(AbjadError::type_error(format!(
                        "Comparison requires compatible types: found {} and {}",
                        left, right
                    )))
                }
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if self.is_boolean_type(left) && self.is_boolean_type(right) {
                    Ok(Type::Bool)
                } else {
                    Err(AbjadError::type_error("Logical operations require boolean types"))
                }
            }
            BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseOr | BinaryOperator::BitwiseXor | BinaryOperator::LeftShift | BinaryOperator::RightShift => {
                if self.is_integer_type(left) && self.is_integer_type(right) {
                    Ok(self.promote_numeric_types(left, right))
                } else {
                    Err(AbjadError::type_error("Bitwise operations require integer types"))
                }
            }
        }
    }

    /// Check a unary operation
    fn check_unary_operation(&self, op: &UnaryOperator, operand: &Type) -> Result<Type> {
        match op {
            UnaryOperator::Negate => {
                if self.is_numeric_type(operand) {
                    Ok(operand.clone())
                } else {
                    Err(AbjadError::type_error("Negation requires numeric type"))
                }
            }
            UnaryOperator::Not => {
                if self.is_boolean_type(operand) {
                    Ok(Type::Bool)
                } else {
                    Err(AbjadError::type_error("Not operation requires boolean type"))
                }
            }
            UnaryOperator::BitwiseNot => {
                if self.is_integer_type(operand) {
                    Ok(operand.clone())
                } else {
                    Err(AbjadError::type_error("Bitwise not requires integer type"))
                }
            }
            UnaryOperator::Dereference => {
                match operand {
                    Type::Slice(t) => Ok(*t.clone()),
                    _ => Err(AbjadError::type_error("Dereference requires slice type")),
                }
            }
            UnaryOperator::Reference => {
                Ok(Type::Slice(Box::new(operand.clone())))
            }
        }
    }

    /// Check a function call
    fn check_function_call(&self, function_type: &Type, arguments: &[Expression]) -> Result<Type> {
        match function_type {
            Type::Function(params, ret) => {
                if params.len() != arguments.len() {
                    return Err(AbjadError::type_error(format!(
                        "Function expects {} arguments, found {}",
                        params.len(),
                        arguments.len()
                    )));
                }
                
                // Note: We can't check argument types without the expressions
                // This would require a mutable reference to self to check expressions
                
                Ok(*ret.clone())
            }
            _ => Err(AbjadError::type_error("Cannot call non-function type")),
        }
    }

    /// Check a pattern
    fn check_pattern(&self, pattern: &Pattern, value_type: &Type) -> Result<()> {
        match pattern {
            Pattern::Wildcard => Ok(()),
            Pattern::Literal(lit) => {
                let lit_type = self.type_of_literal(lit);
                if self.types_compatible(&lit_type, value_type) {
                    Ok(())
                } else {
                    Err(AbjadError::type_error(format!(
                        "Pattern type mismatch: expected {}, found {}",
                        value_type, lit_type
                    )))
                }
            }
            Pattern::Identifier(_) => Ok(()),
            Pattern::Tuple(patterns) => {
                if let Type::Tuple(types) = value_type {
                    if patterns.len() != types.len() {
                        return Err(AbjadError::type_error("Tuple pattern arity mismatch"));
                    }
                    for (p, t) in patterns.iter().zip(types.iter()) {
                        self.check_pattern(p, t)?;
                    }
                    Ok(())
                } else {
                    Err(AbjadError::type_error("Cannot match tuple pattern against non-tuple type"))
                }
            }
            Pattern::Struct { name, fields } => {
                if let Some(struct_type) = self.structs.get(name) {
                    for (field_name, pattern) in fields {
                        if let Some(field_type) = struct_type.fields.get(field_name) {
                            self.check_pattern(pattern, field_type)?;
                        } else {
                            return Err(AbjadError::type_error(format!(
                                "Struct {} has no field {}",
                                name, field_name
                            )));
                        }
                    }
                    Ok(())
                } else {
                    Err(AbjadError::type_error(format!("Unknown struct: {}", name)))
                }
            }
            Pattern::Or(p1, p2) => {
                self.check_pattern(p1, value_type)?;
                self.check_pattern(p2, value_type)
            }
        }
    }

    /// Check a type cast
    fn check_cast(&self, from: &Type, to: &Type) -> Result<()> {
        // Allow numeric casts
        if self.is_numeric_type(from) && self.is_numeric_type(to) {
            return Ok(());
        }
        
        // Allow casting to/from pointers (not implemented yet)
        
        Err(AbjadError::type_error(format!(
            "Cannot cast from {} to {}",
            from, to
        )))
    }

    /// Check if a type is numeric
    fn is_numeric_type(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64 |
            Type::F32 | Type::F64
        )
    }

    /// Check if a type is an integer
    fn is_integer_type(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64
        )
    }

    /// Check if a type is boolean
    fn is_boolean_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Bool)
    }

    /// Promote numeric types to a common type
    fn promote_numeric_types(&self, t1: &Type, t2: &Type) -> Type {
        match (t1, t2) {
            (Type::F64, _) | (_, Type::F64) => Type::F64,
            (Type::F32, _) | (_, Type::F32) => Type::F32,
            (Type::I64, _) | (_, Type::I64) => Type::I64,
            (Type::I32, _) | (_, Type::I32) => Type::I32,
            (Type::I16, _) | (_, Type::I16) => Type::I16,
            (Type::I8, _) | (_, Type::I8) => Type::I8,
            (Type::U64, _) | (_, Type::U64) => Type::U64,
            (Type::U32, _) | (_, Type::U32) => Type::U32,
            (Type::U16, _) | (_, Type::U16) => Type::U16,
            (Type::U8, _) | (_, Type::U8) => Type::U8,
            _ => Type::I32, // Default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_type_check_simple_expression() {
        let source = "١ + ٢";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        assert!(type_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_type_check_let_statement() {
        let source = "متغير عدد: صحيح٣٢ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        assert!(type_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_type_check_function_declaration() {
        let source = "دالة جمع(أ: صحيح٣٢، ب: صحيح٣٢) -> صحيح٣٢ { أعد أ + ب }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        assert!(type_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_type_check_struct_declaration() {
        let source = "هيكل المستخدم { اسم: نص، العمر: صحيح٣٢ }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        assert!(type_checker.check(&ast).is_ok());
    }

    #[test]
    fn test_type_check_enum_declaration() {
        let source = "تعداد الحالة { نشط، غير_نشط، معلق }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        assert!(type_checker.check(&ast).is_ok());
    }
}
