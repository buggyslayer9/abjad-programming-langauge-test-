use crate::error::{AbjadError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub compile: bool,
    pub optimize: u8,
    pub check: bool,
    pub targets: Vec<Target>,
}

/// Build target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub path: String,
    pub r#type: String, // "executable" or "library"
}

/// Build system for Abjad
pub struct BuildSystem {
    /// Build configuration
    config: BuildConfig,
}

impl BuildSystem {
    /// Create a new build system
    pub fn new(config: BuildConfig) -> Self {
        BuildSystem { config }
    }

    /// Load build configuration from file
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| AbjadError::internal(format!("Failed to read build config: {}", e)))?;
        
        let config: BuildConfig = toml::from_str(&content)
            .map_err(|e| AbjadError::internal(format!("Failed to parse build config: {}", e)))?;
        
        Ok(BuildSystem::new(config))
    }

    /// Build the project
    pub fn build(&self, release: bool) -> Result<()> {
        println!("Building project...");
        
        for target in &self.config.targets {
            println!("  Building target: {}", target.name);
            
            match target.r#type.as_str() {
                "executable" => self.build_executable(target, release)?,
                "library" => self.build_library(target, release)?,
                _ => return Err(AbjadError::internal(format!("Unknown target type: {}", target.r#type))),
            }
        }
        
        println!("Build completed successfully");
        Ok(())
    }

    /// Build an executable target
    fn build_executable(&self, target: &Target, release: bool) -> Result<()> {
        if !self.config.compile {
            println!("    Compilation disabled, skipping");
            return Ok(());
        }
        
        let optimize_level = if release { self.config.optimize } else { 0 };
        
        // In a real implementation, this would call the compiler
        println!("    Compiling {} with optimization level {}", target.path, optimize_level);
        
        Ok(())
    }

    /// Build a library target
    fn build_library(&self, target: &Target, release: bool) -> Result<()> {
        if !self.config.compile {
            println!("    Compilation disabled, skipping");
            return Ok(());
        }
        
        let optimize_level = if release { self.config.optimize } else { 0 };
        
        // In a real implementation, this would call the compiler
        println!("    Compiling library {} with optimization level {}", target.path, optimize_level);
        
        Ok(())
    }

    /// Clean build artifacts
    pub fn clean(&self, all: bool) -> Result<()> {
        println!("Cleaning build artifacts...");
        
        if all {
            println!("  Removing target directory");
            let target_dir = Path::new("target");
            if target_dir.exists() {
                fs::remove_dir_all(target_dir)
                    .map_err(|e| AbjadError::internal(format!("Failed to remove target directory: {}", e)))?;
            }
        } else {
            println!("  Removing build artifacts");
        }
        
        println!("Clean completed");
        Ok(())
    }

    /// Run the project
    pub fn run(&self, args: &[String]) -> Result<()> {
        println!("Running project...");
        
        // Find the executable target
        let executable = self.config.targets.iter()
            .find(|t| t.r#type == "executable")
            .ok_or_else(|| AbjadError::internal("No executable target found"))?;
        
        let output_path = if Path::new("target/release").exists() {
            format!("target/release/{}", executable.name)
        } else {
            format!("target/debug/{}", executable.name)
        };
        
        let mut cmd = Command::new(&output_path);
        cmd.args(args);
        
        let status = cmd.status()
            .map_err(|e| AbjadError::internal(format!("Failed to run executable: {}", e)))?;
        
        if status.success() {
            Ok(())
        } else {
            Err(AbjadError::internal("Program exited with error"))
        }
    }

    /// Check the project
    pub fn check(&self) -> Result<()> {
        if !self.config.check {
            println!("Check disabled, skipping");
            return Ok(());
        }
        
        println!("Checking project...");
        
        for target in &self.config.targets {
            println!("  Checking target: {}", target.name);
            // In a real implementation, this would run type checking
        }
        
        println!("Check completed successfully");
        Ok(())
    }
}

impl Default for BuildSystem {
    fn default() -> Self {
        BuildSystem::new(BuildConfig {
            compile: true,
            optimize: 0,
            check: true,
            targets: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_system_default() {
        let bs = BuildSystem::default();
        assert!(bs.config.compile);
        assert_eq!(bs.config.optimize, 0);
        assert!(bs.config.check);
    }

    #[test]
    fn test_build_system_custom() {
        let config = BuildConfig {
            compile: false,
            optimize: 3,
            check: false,
            targets: vec![],
        };
        let bs = BuildSystem::new(config);
        assert!(!bs.config.compile);
        assert_eq!(bs.config.optimize, 3);
        assert!(!bs.config.check);
    }
}
