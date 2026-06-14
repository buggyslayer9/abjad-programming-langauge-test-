use crate::ast::*;
use crate::error::{AbjadError, Result};
use std::collections::HashMap;

/// Memory layout analyzer for the Abjad programming language
pub struct MemoryLayoutAnalyzer {
    /// Track memory regions
    regions: Vec<MemoryRegion>,
    
    /// Track struct layouts
    struct_layouts: HashMap<String, StructLayout>,
    
    /// Track enum layouts
    enum_layouts: HashMap<String, EnumLayout>,
}

/// Represents a memory region
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub region_type: RegionType,
}

/// Type of memory region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    Stack,
    Heap,
    Static,
    Code,
}

/// Layout information for a struct
#[derive(Debug, Clone)]
pub struct StructLayout {
    pub name: String,
    pub fields: Vec<FieldLayout>,
    pub size: usize,
    pub alignment: usize,
}

/// Layout information for a field
#[derive(Debug, Clone)]
pub struct FieldLayout {
    pub name: String,
    pub offset: usize,
    pub size: usize,
}

/// Layout information for an enum
#[derive(Debug, Clone)]
pub struct EnumLayout {
    pub name: String,
    pub variants: Vec<VariantLayout>,
    pub size: usize,
    pub tag_size: usize,
}

/// Layout information for a variant
#[derive(Debug, Clone)]
pub struct VariantLayout {
    pub name: String,
    pub size: usize,
}

impl MemoryLayoutAnalyzer {
    /// Create a new memory layout analyzer
    pub fn new() -> Self {
        MemoryLayoutAnalyzer {
            regions: Vec::new(),
            struct_layouts: HashMap::new(),
            enum_layouts: HashMap::new(),
        }
    }

    /// Analyze an entire AST for memory layout
    pub fn analyze(&mut self, ast: &AST) -> Result<MemoryLayoutInfo> {
        // Add stack region
        self.regions.push(MemoryRegion {
            name: "stack".to_string(),
            start: 0,
            end: 0,
            region_type: RegionType::Stack,
        });

        // Add heap region
        self.regions.push(MemoryRegion {
            name: "heap".to_string(),
            start: 0,
            end: 0,
            region_type: RegionType::Heap,
        });

        // Add static region
        self.regions.push(MemoryRegion {
            name: "static".to_string(),
            start: 0,
            end: 0,
            region_type: RegionType::Static,
        });

        for statement in &ast.statements {
            self.analyze_statement(statement)?;
        }

        Ok(MemoryLayoutInfo {
            regions: self.regions.clone(),
            struct_layouts: self.struct_layouts.clone(),
            enum_layouts: self.enum_layouts.clone(),
        })
    }

    /// Analyze a statement for memory layout
    fn analyze_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => self.analyze_expression(expr)?,
            Statement::Let { .. } => {}
            Statement::Assignment { target, value } => {
                self.analyze_expression(target)?;
                self.analyze_expression(value)?;
            }
            Statement::Function(func) => {
                for stmt in &func.body {
                    self.analyze_statement(stmt)?;
                }
            }
            Statement::Struct(struct_) => {
                let layout = self.compute_struct_layout(struct_);
                self.struct_layouts.insert(struct_.name.clone(), layout);
            }
            Statement::Enum(enum_) => {
                let layout = self.compute_enum_layout(enum_);
                self.enum_layouts.insert(enum_.name.clone(), layout);
            }
            Statement::Import(_) => {}
            Statement::Empty => {}
        }
        Ok(())
    }

    /// Analyze an expression for memory layout
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

    /// Compute the layout of a struct
    fn compute_struct_layout(&self, struct_: &StructDeclaration) -> StructLayout {
        let mut fields = Vec::new();
        let mut offset = 0;
        let mut max_alignment = 1;

        for field in &struct_.fields {
            let size = self.type_size(&field.type_annotation);
            let alignment = self.type_alignment(&field.type_annotation);
            
            // Align offset to field alignment
            offset = (offset + alignment - 1) & !(alignment - 1);
            
            fields.push(FieldLayout {
                name: field.name.clone(),
                offset,
                size,
            });

            offset += size;
            max_alignment = max_alignment.max(alignment);
        }

        // Align struct size to max alignment
        let size = (offset + max_alignment - 1) & !(max_alignment - 1);

        StructLayout {
            name: struct_.name.clone(),
            fields,
            size,
            alignment: max_alignment,
        }
    }

    /// Compute the layout of an enum
    fn compute_enum_layout(&self, enum_: &EnumDeclaration) -> EnumLayout {
        let mut variants = Vec::new();
        let mut max_variant_size = 0;

        for variant in &enum_.variants {
            let size: usize = variant.fields.iter()
                .map(|t| self.type_size(t))
                .sum();
            
            variants.push(VariantLayout {
                name: variant.name.clone(),
                size,
            });

            max_variant_size = max_variant_size.max(size);
        }

        // Tag size for discriminant
        let tag_size = if enum_.variants.len() <= 256 {
            1
        } else if enum_.variants.len() <= 65536 {
            2
        } else {
            4
        };

        // Total size is tag + max variant size
        let size = tag_size + max_variant_size;

        EnumLayout {
            name: enum_.name.clone(),
            variants,
            size,
            tag_size,
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
            Type::Slice(_) => 16,  // Pointer + length
            Type::Tuple(types) => {
                types.iter().map(|t| self.type_size(t)).sum()
            }
            Type::Function(_, _) => 8,
            Type::Named(name) => {
                // Check if it's a struct or enum
                if let Some(layout) = self.struct_layouts.get(name) {
                    layout.size
                } else if let Some(layout) = self.enum_layouts.get(name) {
                    layout.size
                } else {
                    8  // Default pointer size
                }
            }
            Type::Generic(_) => 8,
        }
    }

    /// Get the alignment of a type in bytes
    fn type_alignment(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 | Type::Bool => 1,
            Type::I16 | Type::U16 => 2,
            Type::I32 | Type::U32 | Type::F32 => 4,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::Char => 4,
            Type::String => 8,
            Type::Array(elem_type, _) => self.type_alignment(elem_type),
            Type::Slice(_) => 8,
            Type::Tuple(types) => {
                types.iter().map(|t| self.type_alignment(t)).max().unwrap_or(1)
            }
            Type::Function(_, _) => 8,
            Type::Named(name) => {
                if let Some(layout) = self.struct_layouts.get(name) {
                    layout.alignment
                } else if let Some(layout) = self.enum_layouts.get(name) {
                    layout.tag_size
                } else {
                    8
                }
            }
            Type::Generic(_) => 8,
        }
    }
}

/// Information about memory layout
#[derive(Debug, Clone)]
pub struct MemoryLayoutInfo {
    pub regions: Vec<MemoryRegion>,
    pub struct_layouts: HashMap<String, StructLayout>,
    pub enum_layouts: HashMap<String, EnumLayout>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_memory_layout_struct() {
        let source = "هيكل النقطة { س: عشري٦٤، ص: عشري٦٤ }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = MemoryLayoutAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.struct_layouts.contains_key("النقطة"));
    }

    #[test]
    fn test_memory_layout_enum() {
        let source = "تعداد الحالة { نشط، غير_نشط }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = MemoryLayoutAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.enum_layouts.contains_key("الحالة"));
    }

    #[test]
    fn test_memory_layout_regions() {
        let source = "متغير أ = ١٠;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut analyzer = MemoryLayoutAnalyzer::new();
        let info = analyzer.analyze(&ast).unwrap();
        
        assert!(info.regions.len() >= 3);  // stack, heap, static
    }
}
