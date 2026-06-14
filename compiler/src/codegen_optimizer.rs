use crate::error::{AbjadError, Result};

/// Code generation optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeGenOptimizationLevel {
    /// No optimizations
    None,
    /// Basic optimizations
    Basic,
    /// Moderate optimizations
    Moderate,
    /// Aggressive optimizations
    Aggressive,
}

/// Code generation optimizer for Abjad
pub struct CodeGenOptimizer {
    /// Optimization level
    level: CodeGenOptimizationLevel,
    
    /// Enable LLVM optimizations
    enable_llvm_optimizations: bool,
    
    /// Enable Cranelift optimizations
    enable_cranelift_optimizations: bool,
    
    /// Enable register allocation
    enable_register_allocation: bool,
}

impl CodeGenOptimizer {
    /// Create a new code generation optimizer
    pub fn new(level: CodeGenOptimizationLevel) -> Self {
        CodeGenOptimizer {
            level,
            enable_llvm_optimizations: true,
            enable_cranelift_optimizations: false,
            enable_register_allocation: true,
        }
    }

    /// Set optimization level
    pub fn set_level(&mut self, level: CodeGenOptimizationLevel) {
        self.level = level;
    }

    /// Enable LLVM optimizations
    pub fn enable_llvm_optimizations(&mut self, enabled: bool) {
        self.enable_llvm_optimizations = enabled;
    }

    /// Enable Cranelift optimizations
    pub fn enable_cranelift_optimizations(&mut self, enabled: bool) {
        self.enable_cranelift_optimizations = enabled;
    }

    /// Enable register allocation
    pub fn enable_register_allocation(&mut self, enabled: bool) {
        self.enable_register_allocation = enabled;
    }

    /// Get optimization flags for LLVM
    pub fn get_llvm_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        
        match self.level {
            CodeGenOptimizationLevel::None => {
                flags.push("-O0".to_string());
            }
            CodeGenOptimizationLevel::Basic => {
                flags.push("-O1".to_string());
            }
            CodeGenOptimizationLevel::Moderate => {
                flags.push("-O2".to_string());
            }
            CodeGenOptimizationLevel::Aggressive => {
                flags.push("-O3".to_string());
            }
        }
        
        if self.enable_llvm_optimizations {
            flags.push("--llvm-opt".to_string());
        }
        
        if self.enable_register_allocation {
            flags.push("--regalloc".to_string());
        }
        
        flags
    }

    /// Get optimization flags for Cranelift
    pub fn get_cranelift_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        
        if self.enable_cranelift_optimizations {
            flags.push("--cranelift-opt".to_string());
        }
        
        match self.level {
            CodeGenOptimizationLevel::None => {
                flags.push("-O0".to_string());
            }
            CodeGenOptimizationLevel::Basic => {
                flags.push("-O1".to_string());
            }
            CodeGenOptimizationLevel::Moderate => {
                flags.push("-O2".to_string());
            }
            CodeGenOptimizationLevel::Aggressive => {
                flags.push("-O3".to_string());
            }
        }
        
        flags
    }

    /// Optimize generated code
    pub fn optimize(&self) -> Result<String> {
        let mut optimizations = Vec::new();
        
        match self.level {
            CodeGenOptimizationLevel::None => {
                optimizations.push("No code generation optimizations");
            }
            CodeGenOptimizationLevel::Basic => {
                optimizations.push("Basic code generation optimizations");
            }
            CodeGenOptimizationLevel::Moderate => {
                optimizations.push("Moderate code generation optimizations");
            }
            CodeGenOptimizationLevel::Aggressive => {
                optimizations.push("Aggressive code generation optimizations");
            }
        }
        
        if self.enable_llvm_optimizations {
            optimizations.push("LLVM optimizations enabled");
        }
        
        if self.enable_cranelift_optimizations {
            optimizations.push("Cranelift optimizations enabled");
        }
        
        if self.enable_register_allocation {
            optimizations.push("Register allocation enabled");
        }
        
        Ok(optimizations.join("\n"))
    }

    /// Get optimization level
    pub fn level(&self) -> CodeGenOptimizationLevel {
        self.level
    }

    /// Check if LLVM optimizations are enabled
    pub fn llvm_optimizations_enabled(&self) -> bool {
        self.enable_llvm_optimizations
    }

    /// Check if Cranelift optimizations are enabled
    pub fn cranelift_optimizations_enabled(&self) -> bool {
        self.enable_cranelift_optimizations
    }

    /// Check if register allocation is enabled
    pub fn register_allocation_enabled(&self) -> bool {
        self.enable_register_allocation
    }
}

impl Default for CodeGenOptimizer {
    fn default() -> Self {
        Self::new(CodeGenOptimizationLevel::Moderate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_optimizer_default() {
        let optimizer = CodeGenOptimizer::default();
        assert_eq!(optimizer.level(), CodeGenOptimizationLevel::Moderate);
        assert!(optimizer.llvm_optimizations_enabled());
        assert!(!optimizer.cranelift_optimizations_enabled());
    }

    #[test]
    fn test_codegen_optimizer_llvm_flags() {
        let optimizer = CodeGenOptimizer::new(CodeGenOptimizationLevel::Basic);
        let flags = optimizer.get_llvm_flags();
        assert!(flags.contains(&"-O1".to_string()));
        assert!(flags.contains(&"--llvm-opt".to_string()));
    }

    #[test]
    fn test_codegen_optimizer_cranelift_flags() {
        let mut optimizer = CodeGenOptimizer::new(CodeGenOptimizationLevel::Basic);
        optimizer.enable_cranelift_optimizations(true);
        let flags = optimizer.get_cranelift_flags();
        assert!(flags.contains(&"-O1".to_string()));
        assert!(flags.contains(&"--cranelift-opt".to_string()));
    }

    #[test]
    fn test_codegen_optimizer_optimize() {
        let optimizer = CodeGenOptimizer::new(CodeGenOptimizationLevel::Aggressive);
        let result = optimizer.optimize().unwrap();
        assert!(result.contains("Aggressive"));
    }
}
