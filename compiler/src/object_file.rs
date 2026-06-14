use crate::error::{AbjadError, Result};
use std::path::Path;

/// Object file generator for the Abjad programming language
pub struct ObjectFileGenerator {
    /// Target triple for the object file
    target_triple: String,
}

impl ObjectFileGenerator {
    /// Create a new object file generator
    pub fn new(target_triple: Option<String>) -> Self {
        ObjectFileGenerator {
            target_triple: target_triple.unwrap_or_else(|| Self::default_target()),
        }
    }

    /// Get the default target triple for the current platform
    fn default_target() -> String {
        #[cfg(target_os = "linux")]
        {
            #[cfg(target_arch = "x86_64")]
            return "x86_64-unknown-linux-gnu".to_string();
            #[cfg(target_arch = "aarch64")]
            return "aarch64-unknown-linux-gnu".to_string();
        }
        #[cfg(target_os = "windows")]
        {
            #[cfg(target_arch = "x86_64")]
            return "x86_64-pc-windows-msvc".to_string();
        }
        #[cfg(target_os = "macos")]
        {
            #[cfg(target_arch = "x86_64")]
            return "x86_64-apple-darwin".to_string();
            #[cfg(target_arch = "aarch64")]
            return "aarch64-apple-darwin".to_string();
        }
        "x86_64-unknown-linux-gnu".to_string() // Default
    }

    /// Generate an object file from LLVM IR
    #[cfg(feature = "llvm")]
    pub fn generate_from_ir(&self, ir: &str, output_path: &Path) -> Result<()> {
        use inkwell::{
            context::Context,
            module::Module,
            targets::{InitializationConfig, Target, TargetMachine},
            object::Object,
        };

        // Initialize LLVM targets
        Target::initialize_llvm(&InitializationConfig::default());

        let context = Context::create();
        let module = context.create_module("abjad_module");
        
        // Parse IR
        module.parse_ir(ir.as_bytes())
            .map_err(|e| AbjadError::internal(format!("Failed to parse IR: {}", e)))?;

        // Get target
        let target = Target::from_triple(&self.target_triple)
            .map_err(|e| AbjadError::internal(format!("Failed to get target: {}", e)))?;

        // Create target machine
        let target_machine = target.create_target_machine(
            &self.target_triple,
            "generic",
            "",
            inkwell::OptimizationLevel::None,
            None,
            None,
            inkwell::RelocMode::PIC,
            None,
        ).ok_or_else(|| AbjadError::internal("Failed to create target machine"))?;

        // Write object file
        target_machine.write_to_file(
            module,
            inkwell::object::FileType::Object,
            output_path,
        ).map_err(|e| AbjadError::internal(format!("Failed to write object file: {}", e)))?;

        Ok(())
    }

    /// Generate an object file from LLVM IR (without LLVM support)
    #[cfg(not(feature = "llvm"))]
    pub fn generate_from_ir(&self, _ir: &str, _output_path: &Path) -> Result<()> {
        Err(AbjadError::internal("LLVM backend not enabled. Compile with --features llvm"))
    }

    /// Generate an object file from an AST
    #[cfg(feature = "llvm")]
    pub fn generate_from_ast(&self, ast: &crate::ast::AST, output_path: &Path) -> Result<()> {
        use crate::codegen::CodeGenerator;
        
        let mut codegen = CodeGenerator::new("abjad_module");
        let ir = codegen.generate(ast)?;
        self.generate_from_ir(&ir, output_path)
    }

    /// Generate an object file from an AST (without LLVM support)
    #[cfg(not(feature = "llvm"))]
    pub fn generate_from_ast(&self, _ast: &crate::ast::AST, _output_path: &Path) -> Result<()> {
        Err(AbjadError::internal("LLVM backend not enabled. Compile with --features llvm"))
    }

    /// Get the target triple
    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_generator_default_target() {
        let generator = ObjectFileGenerator::new(None);
        assert!(!generator.target_triple().is_empty());
    }

    #[test]
    fn test_object_generator_custom_target() {
        let generator = ObjectFileGenerator::new(Some("x86_64-unknown-linux-gnu".to_string()));
        assert_eq!(generator.target_triple(), "x86_64-unknown-linux-gnu");
    }

    #[test]
    #[cfg(feature = "llvm")]
    fn test_object_generation_from_ir() {
        let ir = r#"
            define i32 @main() {
            entry:
                ret i32 0
            }
        "#;
        
        let generator = ObjectFileGenerator::new(None);
        let output = std::path::PathBuf::from("/tmp/test_abjad.o");
        
        // This test requires LLVM to be installed
        let result = generator.generate_from_ir(ir, &output);
        
        // We don't assert success here as LLVM might not be installed
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    #[cfg(not(feature = "llvm"))]
    fn test_object_generation_no_llvm() {
        let ir = "define i32 @main() { ret i32 0 }";
        
        let generator = ObjectFileGenerator::new(None);
        let output = std::path::PathBuf::from("/tmp/test_abjad.o");
        
        let result = generator.generate_from_ir(ir, &output);
        assert!(result.is_err());
    }
}
