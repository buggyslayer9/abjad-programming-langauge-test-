use crate::error::{AbjadError, Result};
use std::collections::HashMap;
use std::path::Path;

/// Lint rule severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Suggestion,
}

/// Lint message
#[derive(Debug, Clone)]
pub struct LintMessage {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub message: String,
    pub severity: Severity,
}

/// Lint rule
#[derive(Debug, Clone)]
pub struct LintRule {
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub enabled: bool,
}

/// Linter for Abjad
pub struct Linter {
    /// Lint rules
    rules: HashMap<String, LintRule>,
    
    /// Lint messages
    messages: Vec<LintMessage>,
    
    /// Strict mode
    strict: bool,
}

impl Linter {
    /// Create a new linter
    pub fn new() -> Self {
        let mut linter = Linter {
            rules: HashMap::new(),
            messages: Vec::new(),
            strict: false,
        };
        
        linter.register_default_rules();
        linter
    }

    /// Register default lint rules
    fn register_default_rules(&mut self) {
        self.register_rule(LintRule {
            name: "unused_variable".to_string(),
            description: "Detect unused variables".to_string(),
            severity: Severity::Warning,
            enabled: true,
        });
        
        self.register_rule(LintRule {
            name: "dead_code".to_string(),
            description: "Detect unreachable code".to_string(),
            severity: Severity::Warning,
            enabled: true,
        });
        
        self.register_rule(LintRule {
            name: "style_consistency".to_string(),
            description: "Check code style consistency".to_string(),
            severity: Severity::Info,
            enabled: true,
        });
        
        self.register_rule(LintRule {
            name: "naming_convention".to_string(),
            description: "Check naming conventions".to_string(),
            severity: Severity::Suggestion,
            enabled: true,
        });
    }

    /// Register a lint rule
    pub fn register_rule(&mut self, rule: LintRule) {
        self.rules.insert(rule.name.clone(), rule);
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, name: &str) -> Result<()> {
        if let Some(rule) = self.rules.get_mut(name) {
            rule.enabled = true;
            Ok(())
        } else {
            Err(AbjadError::internal(format!("Rule '{}' not found", name)))
        }
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, name: &str) -> Result<()> {
        if let Some(rule) = self.rules.get_mut(name) {
            rule.enabled = false;
            Ok(())
        } else {
            Err(AbjadError::internal(format!("Rule '{}' not found", name)))
        }
    }

    /// Set strict mode
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    /// Lint a file
    pub fn lint_file(&mut self, file: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file)
            .map_err(|e| AbjadError::internal(format!("Failed to read file: {}", e)))?;
        
        self.lint_content(&content, file.to_string_lossy().to_string())
    }

    /// Lint content
    pub fn lint_content(&mut self, content: &str, file: String) -> Result<()> {
        self.messages.clear();
        
        for (line_num, line) in content.lines().enumerate() {
            self.lint_line(line, line_num + 1, &file);
        }
        
        Ok(())
    }

    /// Lint a single line
    fn lint_line(&mut self, line: &str, line_num: usize, file: &str) {
        // Check for unused variables (simplified)
        if line.contains("متغير") && !line.contains("=") {
            self.add_message(LintMessage {
                file: file.to_string(),
                line: line_num,
                column: 0,
                rule: "unused_variable".to_string(),
                message: "Variable may be unused".to_string(),
                severity: Severity::Warning,
            });
        }
        
        // Check for style consistency
        if line.contains("  ") {
            self.add_message(LintMessage {
                file: file.to_string(),
                line: line_num,
                column: 0,
                rule: "style_consistency".to_string(),
                message: "Multiple spaces detected".to_string(),
                severity: Severity::Info,
            });
        }
        
        // Check naming convention
        if line.contains("متغير") && line.chars().any(|c| c.is_ascii_uppercase()) {
            self.add_message(LintMessage {
                file: file.to_string(),
                line: line_num,
                column: 0,
                rule: "naming_convention".to_string(),
                message: "Variable names should use lowercase".to_string(),
                severity: Severity::Suggestion,
            });
        }
    }

    /// Add a lint message
    fn add_message(&mut self, message: LintMessage) {
        if let Some(rule) = self.rules.get(&message.rule) {
            if rule.enabled {
                self.messages.push(message);
            }
        }
    }

    /// Get lint messages
    pub fn get_messages(&self) -> &[LintMessage] {
        &self.messages
    }

    /// Get lint messages by severity
    pub fn get_messages_by_severity(&self, severity: Severity) -> Vec<&LintMessage> {
        self.messages
            .iter()
            .filter(|m| m.severity == severity)
            .collect()
    }

    /// Get all rules
    pub fn get_rules(&self) -> Vec<&LintRule> {
        self.rules.values().collect()
    }

    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&LintRule> {
        self.rules
            .values()
            .filter(|r| r.enabled)
            .collect()
    }

    /// Generate report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("Lint Report\n");
        report.push_str("===========\n\n");
        
        let errors = self.get_messages_by_severity(Severity::Error);
        let warnings = self.get_messages_by_severity(Severity::Warning);
        let infos = self.get_messages_by_severity(Severity::Info);
        let suggestions = self.get_messages_by_severity(Severity::Suggestion);
        
        if !errors.is_empty() {
            report.push_str(&format!("Errors ({}):\n", errors.len()));
            for msg in errors {
                report.push_str(&format!("  {}:{}:{} - {}\n", msg.file, msg.line, msg.column, msg.message));
            }
            report.push('\n');
        }
        
        if !warnings.is_empty() {
            report.push_str(&format!("Warnings ({}):\n", warnings.len()));
            for msg in warnings {
                report.push_str(&format!("  {}:{}:{} - {}\n", msg.file, msg.line, msg.column, msg.message));
            }
            report.push('\n');
        }
        
        if !infos.is_empty() {
            report.push_str(&format!("Info ({}):\n", infos.len()));
            for msg in infos {
                report.push_str(&format!("  {}:{}:{} - {}\n", msg.file, msg.line, msg.column, msg.message));
            }
            report.push('\n');
        }
        
        if !suggestions.is_empty() {
            report.push_str(&format!("Suggestions ({}):\n", suggestions.len()));
            for msg in suggestions {
                report.push_str(&format!("  {}:{}:{} - {}\n", msg.file, msg.line, msg.column, msg.message));
            }
            report.push('\n');
        }
        
        if self.messages.is_empty() {
            report.push_str("No lint issues found!\n");
        }
        
        report
    }

    /// Auto-fix issues
    pub fn auto_fix(&mut self, content: &str) -> Result<String> {
        let mut fixed = content.to_string();
        
        // Fix multiple spaces
        fixed = fixed.replace("  ", " ");
        
        Ok(fixed)
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.messages.iter().any(|m| m.severity == Severity::Error)
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.messages.iter().any(|m| m.severity == Severity::Warning)
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_default() {
        let linter = Linter::default();
        assert!(!linter.strict);
    }

    #[test]
    fn test_linter_rules() {
        let linter = Linter::new();
        let rules = linter.get_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_linter_enable_disable() {
        let mut linter = Linter::new();
        linter.disable_rule("unused_variable").unwrap();
        
        let rules = linter.get_enabled_rules();
        let enabled_rule_names: Vec<_> = rules.iter().map(|r| &r.name).collect();
        assert!(!enabled_rule_names.contains(&"unused_variable"));
        
        linter.enable_rule("unused_variable").unwrap();
        
        let rules = linter.get_enabled_rules();
        let enabled_rule_names: Vec<_> = rules.iter().map(|r| &r.name).collect();
        assert!(enabled_rule_names.contains(&"unused_variable"));
    }

    #[test]
    fn test_linter_content() {
        let mut linter = Linter::new();
        linter.lint_content("متغير أ = ١٠", "test.abjad".to_string()).unwrap();
        
        let messages = linter.get_messages();
        assert!(!messages.is_empty());
    }

    #[test]
    fn test_linter_report() {
        let mut linter = Linter::new();
        linter.lint_content("متغير أ = ١٠", "test.abjad".to_string()).unwrap();
        
        let report = linter.generate_report();
        assert!(report.contains("Lint Report"));
    }

    #[test]
    fn test_auto_fix() {
        let linter = Linter::new();
        let fixed = linter.auto_fix("متغير  أ  =  ١٠").unwrap();
        assert_eq!(fixed, "متغير أ = ١٠");
    }
}
