use crate::error::{AbjadError, Result};
use std::path::Path;
use std::process::Command;

/// Linker for the Abjad programming language
pub struct Linker {
    /// Linker to use (cc, ld, etc.)
    linker: String,
    
    /// Target triple
    target_triple: String,
}

impl Linker {
    /// Create a new linker
    pub fn new(linker: Option<String>, target_triple: Option<String>) -> Self {
        Linker {
            linker: linker.unwrap_or_else(|| Self::default_linker()),
            target_triple: target_triple.unwrap_or_else(|| Self::default_target()),
        }
    }

    /// Get the default linker for the current platform
    fn default_linker() -> String {
        #[cfg(target_os = "linux")]
        return "cc".to_string();
        #[cfg(target_os = "windows")]
        return "link.exe".to_string();
        #[cfg(target_os = "macos")]
        return "clang".to_string();
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        "cc".to_string() // Default
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
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        "x86_64-unknown-linux-gnu".to_string() // Default
    }

    /// Link object files into an executable
    pub fn link(&self, object_files: &[&Path], output_path: &Path, libraries: &[&str]) -> Result<()> {
        let mut cmd = Command::new(&self.linker);
        
        // Add object files
        for obj_file in object_files {
            cmd.arg(obj_file);
        }
        
        // Add output flag
        cmd.arg("-o").arg(output_path);
        
        // Add libraries
        for lib in libraries {
            cmd.arg(format!("-l{}", lib));
        }
        
        // Add platform-specific flags
        self.add_platform_flags(&mut cmd);
        
        // Execute linker
        let output = cmd.output()
            .map_err(|e| AbjadError::internal(format!("Failed to execute linker: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AbjadError::internal(format!("Linking failed: {}", stderr)));
        }
        
        Ok(())
    }

    /// Add platform-specific flags to the linker command
    fn add_platform_flags(&self, cmd: &mut Command) {
        #[cfg(target_os = "linux")]
        {
            cmd.arg("-lm");  // Link math library
        }
        
        #[cfg(target_os = "windows")]
        {
            cmd.arg("/SUBSYSTEM:CONSOLE");
        }
        
        #[cfg(target_os = "macos")]
        {
            cmd.arg("-framework").arg("CoreFoundation");
        }
    }

    /// Get the linker being used
    pub fn linker(&self) -> &str {
        &self.linker
    }

    /// Get the target triple
    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }
}

impl Default for Linker {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_default() {
        let linker = Linker::default();
        assert!(!linker.linker().is_empty());
        assert!(!linker.target_triple().is_empty());
    }

    #[test]
    fn test_linker_custom() {
        let linker = Linker::new(Some("gcc".to_string()), Some("x86_64-unknown-linux-gnu".to_string()));
        assert_eq!(linker.linker(), "gcc");
        assert_eq!(linker.target_triple(), "x86_64-unknown-linux-gnu");
    }
}
