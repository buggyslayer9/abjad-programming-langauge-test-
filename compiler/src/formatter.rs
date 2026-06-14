use crate::error::{AbjadError, Result};
use std::path::Path;

/// Formatter configuration
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub line_width: usize,
    pub trailing_comma: bool,
    pub align_comments: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig {
            indent_size: 4,
            use_tabs: false,
            line_width: 80,
            trailing_comma: true,
            align_comments: true,
        }
    }
}

/// Code formatter for Abjad
pub struct Formatter {
    config: FormatterConfig,
}

impl Formatter {
    /// Create a new formatter with default configuration
    pub fn new() -> Self {
        Formatter {
            config: FormatterConfig::default(),
        }
    }

    /// Create a new formatter with custom configuration
    pub fn with_config(config: FormatterConfig) -> Self {
        Formatter { config }
    }

    /// Format a file
    pub fn format_file(&self, file: &Path) -> Result<String> {
        let content = std::fs::read_to_string(file)
            .map_err(|e| AbjadError::internal(format!("Failed to read file: {}", e)))?;
        
        self.format_content(&content)
    }

    /// Format content
    pub fn format_content(&self, content: &str) -> Result<String> {
        let mut formatted = String::new();
        let mut indent_level = 0;
        let in_block = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            // Decrease indent for closing braces
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }
            
            // Add indentation
            let indent = if self.config.use_tabs {
                "\t".repeat(indent_level)
            } else {
                " ".repeat(indent_level * self.config.indent_size)
            };
            
            formatted.push_str(&indent);
            formatted.push_str(trimmed);
            
            // Increase indent for opening braces
            if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
                indent_level += 1;
                in_block = true;
            }
            
            formatted.push('\n');
        }
        
        Ok(formatted)
    }

    /// Format and write to file
    pub fn format_file_in_place(&self, file: &Path) -> Result<()> {
        let formatted = self.format_file(file)?;
        std::fs::write(file, formatted)
            .map_err(|e| AbjadError::internal(format!("Failed to write file: {}", e)))?;
        Ok(())
    }

    /// Check if file is formatted
    pub fn check_file(&self, file: &Path) -> Result<bool> {
        let original = std::fs::read_to_string(file)
            .map_err(|e| AbjadError::internal(format!("Failed to read file: {}", e)))?;
        
        let formatted = self.format_content(&original)?;
        
        Ok(original == formatted)
    }

    /// Format a directory recursively
    pub fn format_directory(&self, dir: &Path) -> Result<Vec<String>> {
        let mut formatted_files = Vec::new();
        
        let entries = std::fs::read_dir(dir)
            .map_err(|e| AbjadError::internal(format!("Failed to read directory: {}", e)))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| AbjadError::internal(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                formatted_files.extend(self.format_directory(&path)?);
            } else if path.extension().map_or(false, |ext| ext == "abjad") {
                self.format_file_in_place(&path)?;
                formatted_files.push(path.to_string_lossy().to_string());
            }
        }
        
        Ok(formatted_files)
    }

    /// Get configuration
    pub fn config(&self) -> &FormatterConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: FormatterConfig) {
        self.config = config;
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_default() {
        let formatter = Formatter::default();
        assert_eq!(formatter.config().indent_size, 4);
        assert!(!formatter.config().use_tabs);
    }

    #[test]
    fn test_formatter_custom_config() {
        let config = FormatterConfig {
            indent_size: 2,
            use_tabs: true,
            line_width: 120,
            trailing_comma: false,
            align_comments: false,
        };
        
        let formatter = Formatter::with_config(config);
        assert_eq!(formatter.config().indent_size, 2);
        assert!(formatter.config().use_tabs);
    }

    #[test]
    fn test_format_content() {
        let formatter = Formatter::default();
        let content = "متغير أ = ١٠\nمتغير ب = ٢٠";
        let formatted = formatter.format_content(content).unwrap();
        
        assert!(formatted.contains("متغير أ = ١٠"));
        assert!(formatted.contains("متغير ب = ٢٠"));
    }

    #[test]
    fn test_format_with_indentation() {
        let formatter = Formatter::default();
        let content = "دالة مثال() {\nمتغير أ = ١٠\n}";
        let formatted = formatter.format_content(content).unwrap();
        
        assert!(formatted.contains("    متغير أ = ١٠"));
    }

    #[test]
    fn test_check_file() {
        let formatter = Formatter::default();
        let content = "متغير أ = ١٠";
        
        // Create a temporary file
        let temp_file = Path::new("/tmp/test_abjad_format.abjad");
        std::fs::write(temp_file, content).unwrap();
        
        // Check if formatted
        let is_formatted = formatter.check_file(temp_file).unwrap();
        assert!(is_formatted);
        
        // Cleanup
        std::fs::remove_file(temp_file).unwrap();
    }
}
