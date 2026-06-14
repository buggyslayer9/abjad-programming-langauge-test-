use crate::ast::*;
use crate::error::{AbjadError, Result};

#[cfg(feature = "llvm")]
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    types::BasicTypeEnum,
    values::{FunctionValue, BasicValueEnum},
    IntPredicate, FloatPredicate,
};

/// Code generator for the Abjad programming language
pub struct CodeGenerator {
    #[cfg(feature = "llvm")]
    context: Context,
    #[cfg(feature = "llvm")]
    module: Module,
    #[cfg(feature = "llvm")]
    builder: Builder,
}

impl CodeGenerator {
    /// Create a new code generator
    #[cfg(feature = "llvm")]
    pub fn new(module_name: &str) -> Self {
        let context = Context::create();
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        CodeGenerator {
            context,
            module,
            builder,
        }
    }

    /// Create a new code generator (without LLVM support)
    #[cfg(not(feature = "llvm"))]
    pub fn new(_module_name: &str) -> Self {
        CodeGenerator {}
    }

    /// Generate LLVM IR from an AST
    #[cfg(feature = "llvm")]
    pub fn generate(&mut self, ast: &AST) -> Result<String> {
        for statement in &ast.statements {
            self.compile_statement(statement)?;
        }
        
        Ok(self.module.print_to_string().to_string())
    }

    /// Generate LLVM IR from an AST (without LLVM support)
    #[cfg(not(feature = "llvm"))]
    pub fn generate(&mut self, _ast: &AST) -> Result<String> {
        Err(AbjadError::internal("LLVM backend not enabled. Compile with --features llvm"))
    }

    /// Compile a statement
    #[cfg(feature = "llvm")]
    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
            }
            Statement::Let { name, type_annotation, value, .. } => {
                let value = self.compile_expression(value)?;
                let ty = self.compile_type(type_annotation.as_ref().unwrap_or(&Type::I32))?;
                
                // Create an alloca instruction
                let function = self.builder.get_insert_block()
                    .and_then(|block| block.get_parent())
                    .ok_or_else(|| AbjadError::internal("No function context"))?;
                
                let entry_block = function.get_entry_block()
                    .ok_or_else(|| AbjadError::internal("No entry block"))?;
                
                let builder = self.context.create_builder();
                builder.position_at_end(entry_block, &self.builder);
                
                let alloca = builder.build_alloca(ty, name)?;
                self.builder.build_store(alloca, value)?;
            }
            Statement::Assignment { target, value } => {
                let target = self.compile_expression(target)?;
                let value = self.compile_expression(value)?;
                
                // Build store instruction
                if let Some(target_ptr) = target.as_pointer_value() {
                    self.builder.build_store(target_ptr, value)?;
                }
            }
            Statement::Function(func) => {
                self.compile_function(func)?;
            }
            Statement::Struct(_) => {
                // Structs are handled in type compilation
            }
            Statement::Enum(_) => {
                // Enums are handled in type compilation
            }
            Statement::Import(_) => {
                // Imports are handled separately
            }
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Compile a function
    #[cfg(feature = "llvm")]
    fn compile_function(&mut self, func: &FunctionDeclaration) -> Result<()> {
        let return_type = self.compile_type(func.return_type.as_ref().unwrap_or(&Type::I32))?;
        
        let param_types: Vec<BasicTypeEnum> = func.parameters.iter()
            .map(|p| self.compile_type(&p.type_annotation))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|t| t.into())
            .collect();
        
        let fn_type = self.context.void_type().fn_type(&param_types, false);
        
        let function = self.module.add_function(&func.name, fn_type, None);
        
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        for stmt in &func.body {
            self.compile_statement(stmt)?;
        }
        
        self.builder.build_return(None)?;
        
        Ok(())
    }

    /// Compile an expression
    #[cfg(feature = "llvm")]
    fn compile_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum> {
        match expr {
            Expression::Literal(lit) => self.compile_literal(lit),
            Expression::Identifier(name) => {
                // Look up variable in symbol table
                Err(AbjadError::internal(format!("Variable not found: {}", name)))
            }
            Expression::Binary { op, left, right } => {
                let left = self.compile_expression(left)?;
                let right = self.compile_expression(right)?;
                self.compile_binary_op(op, left, right)
            }
            Expression::Unary { op, operand } => {
                let operand = self.compile_expression(operand)?;
                self.compile_unary_op(op, operand)
            }
            Expression::Call { function, arguments } => {
                let function = self.compile_expression(function)?;
                let args: Vec<BasicValueEnum> = arguments.iter()
                    .map(|a| self.compile_expression(a))
                    .collect::<Result<Vec<_>>>()?;
                
                if let Some(function_value) = function.as_function_value() {
                    let call_site = self.builder.build_call(function_value, &args, "call")?;
                    Ok(call_site.try_as_basic_value().unwrap())
                } else {
                    Err(AbjadError::internal("Cannot call non-function"))
                }
            }
            Expression::ArrayLiteral(elements) => {
                let elements: Vec<BasicValueEnum> = elements.iter()
                    .map(|e| self.compile_expression(e))
                    .collect::<Result<Vec<_>>>()?;
                
                // Create array
                let array_type = if let Some(first) = elements.first() {
                    first.get_type()
                } else {
                    return Err(AbjadError::internal("Empty array"));
                };
                
                let array = self.context.const_array(array_type, &elements);
                Ok(array.as_basic_value_enum())
            }
            Expression::Index { array, index } => {
                let array = self.compile_expression(array)?;
                let index = self.compile_expression(index)?;
                
                if let (Some(array_ptr), Some(index_int)) = (array.as_pointer_value(), index.as_int_value()) {
                    let ptr = unsafe {
                        self.builder.build_gep(array_ptr, &[index_int], "gep")?
                    };
                    let loaded = self.builder.build_load(self.context.i32_type(), ptr, "load")?;
                    Ok(loaded)
                } else {
                    Err(AbjadError::internal("Invalid array indexing"))
                }
            }
            Expression::TupleLiteral(elements) => {
                // Tuples are represented as structs
                let elements: Vec<BasicValueEnum> = elements.iter()
                    .map(|e| self.compile_expression(e))
                    .collect::<Result<Vec<_>>>()?;
                
                // Create struct constant
                let types: Vec<BasicTypeEnum> = elements.iter()
                    .map(|e| e.get_type().into())
                    .collect();
                
                let struct_type = self.context.struct_type(&types, false);
                let struct_value = self.context.const_struct(&elements, false);
                Ok(struct_value.as_basic_value_enum())
            }
            Expression::Block(statements) => {
                for stmt in statements {
                    self.compile_statement(stmt)?;
                }
                Ok(self.context.i32_type().const_zero().into())
            }
            Expression::If { condition, then_branch, else_branch } => {
                let condition = self.compile_expression(condition)?;
                let condition_int = self.builder.build_int_compare(
                    IntPredicate::NE,
                    condition.into_int_value().unwrap(),
                    self.context.i32_type().const_zero(),
                    "ifcond"
                )?;
                
                let function = self.builder.get_insert_block()
                    .and_then(|block| block.get_parent())
                    .ok_or_else(|| AbjadError::internal("No function context"))?;
                
                let then_block = self.context.append_basic_block(function, "then");
                let else_block = self.context.append_basic_block(function, "else");
                let merge_block = self.context.append_basic_block(function, "ifmerge");
                
                self.builder.build_conditional_branch(condition_int, then_block, else_block)?;
                
                self.builder.position_at_end(then_block);
                let then_value = self.compile_expression(then_branch)?;
                self.builder.build_br(merge_block)?;
                
                self.builder.position_at_end(else_block);
                let else_value = if let Some(else_br) = else_branch {
                    self.compile_expression(else_br)?
                } else {
                    self.context.i32_type().const_zero().into()
                };
                self.builder.build_br(merge_block)?;
                
                self.builder.position_at_end(merge_block);
                
                let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");
                phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
                
                Ok(phi.as_basic_value_enum())
            }
            _ => Err(AbjadError::internal("Expression not yet implemented")),
        }
    }

    /// Compile a literal
    #[cfg(feature = "llvm")]
    fn compile_literal(&self, lit: &Literal) -> Result<BasicValueEnum> {
        match lit {
            Literal::Integer(n) => Ok(self.context.i32_type().const_int(*n as u64, false).into()),
            Literal::Float(n) => Ok(self.context.f64_type().const_float(*n).into()),
            Literal::String(s) => {
                let string_const = self.context.const_string(s, false);
                Ok(string_const.as_basic_value_enum())
            }
            Literal::Char(c) => Ok(self.context.i8_type().const_int(*c as u64, false).into()),
            Literal::Boolean(b) => Ok(self.context.bool_type().const_int(*b as u64, false).into()),
            Literal::Null => Ok(self.context.i32_type().const_zero().into()),
        }
    }

    /// Compile a type
    #[cfg(feature = "llvm")]
    fn compile_type(&self, ty: &Type) -> Result<BasicTypeEnum> {
        match ty {
            Type::I8 => Ok(self.context.i8_type().into()),
            Type::I16 => Ok(self.context.i16_type().into()),
            Type::I32 => Ok(self.context.i32_type().into()),
            Type::I64 => Ok(self.context.i64_type().into()),
            Type::U8 => Ok(self.context.i8_type().into()),
            Type::U16 => Ok(self.context.i16_type().into()),
            Type::U32 => Ok(self.context.i32_type().into()),
            Type::U64 => Ok(self.context.i64_type().into()),
            Type::F32 => Ok(self.context.f32_type().into()),
            Type::F64 => Ok(self.context.f64_type().into()),
            Type::Bool => Ok(self.context.bool_type().into()),
            Type::Char => Ok(self.context.i8_type().into()),
            Type::String => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            Type::Array(elem_type, size) => {
                let elem = self.compile_type(elem_type)?;
                Ok(self.context.array_type(elem.into_array_type(), size.unwrap_or(0) as u32).into())
            }
            Type::Tuple(types) => {
                let field_types: Vec<BasicTypeEnum> = types.iter()
                    .map(|t| self.compile_type(t))
                    .collect::<Result<Vec<_>>>()?;
                Ok(self.context.struct_type(&field_types, false).into())
            }
            Type::Slice(elem_type) => {
                let elem = self.compile_type(elem_type)?;
                let ptr_type = elem.ptr_type(AddressSpace::default());
                Ok(ptr_type.into())
            }
            Type::Function(params, ret) => {
                let param_types: Vec<BasicTypeEnum> = params.iter()
                    .map(|t| self.compile_type(t))
                    .collect::<Result<Vec<_>>>()?;
                let ret_type = self.compile_type(ret)?;
                Ok(self.context.void_type().fn_type(&param_types, false).into())
            }
            Type::Named(_) => Ok(self.context.i32_type().into()),
            Type::Generic(_) => Ok(self.context.i32_type().into()),
        }
    }

    /// Compile a binary operation
    #[cfg(feature = "llvm")]
    fn compile_binary_op(&mut self, op: &BinaryOperator, left: BasicValueEnum, right: BasicValueEnum) -> Result<BasicValueEnum> {
        match op {
            BinaryOperator::Add => {
                let result = self.builder.build_int_add(
                    left.into_int_value().unwrap(),
                    right.into_int_value().unwrap(),
                    "add"
                )?;
                Ok(result.into())
            }
            BinaryOperator::Sub => {
                let result = self.builder.build_int_sub(
                    left.into_int_value().unwrap(),
                    right.into_int_value().unwrap(),
                    "sub"
                )?;
                Ok(result.into())
            }
            BinaryOperator::Mul => {
                let result = self.builder.build_int_mul(
                    left.into_int_value().unwrap(),
                    right.into_int_value().unwrap(),
                    "mul"
                )?;
                Ok(result.into())
            }
            BinaryOperator::Div => {
                let result = self.builder.build_int_signed_div(
                    left.into_int_value().unwrap(),
                    right.into_int_value().unwrap(),
                    "div"
                )?;
                Ok(result.into())
            }
            _ => Err(AbjadError::internal("Binary operation not yet implemented")),
        }
    }

    /// Compile a unary operation
    #[cfg(feature = "llvm")]
    fn compile_unary_op(&mut self, op: &UnaryOperator, operand: BasicValueEnum) -> Result<BasicValueEnum> {
        match op {
            UnaryOperator::Negate => {
                let zero = self.context.i32_type().const_zero();
                let result = self.builder.build_int_sub(
                    zero,
                    operand.into_int_value().unwrap(),
                    "neg"
                )?;
                Ok(result.into())
            }
            _ => Err(AbjadError::internal("Unary operation not yet implemented")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    #[cfg(feature = "llvm")]
    fn test_codegen_simple() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut codegen = CodeGenerator::new("test");
        let ir = codegen.generate(&ast);
        
        assert!(ir.is_ok());
    }

    #[test]
    #[cfg(not(feature = "llvm"))]
    fn test_codegen_no_llvm() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut codegen = CodeGenerator::new("test");
        let ir = codegen.generate(&ast);
        
        assert!(ir.is_err());
    }
}
