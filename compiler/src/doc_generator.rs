use crate::error::{AbjadError, Result};
use std::fs;
use std::path::Path;

/// Documentation generator for Abjad
pub struct DocGenerator {
    /// Output directory for documentation
    output_dir: String,
}

impl DocGenerator {
    /// Create a new documentation generator
    pub fn new(output_dir: Option<String>) -> Self {
        DocGenerator {
            output_dir: output_dir.unwrap_or_else(|| "target/doc".to_string()),
        }
    }

    /// Generate documentation for a project
    pub fn generate(&self, source_dir: &Path) -> Result<()> {
        println!("Generating documentation...");
        
        // Create output directory if it doesn't exist
        let output_path = Path::new(&self.output_dir);
        if !output_path.exists() {
            fs::create_dir_all(output_path)
                .map_err(|e| AbjadError::internal(format!("Failed to create output directory: {}", e)))?;
        }
        
        // Scan source directory for Abjad files
        let abjad_files = self.find_abjad_files(source_dir)?;
        
        println!("  Found {} Abjad files", abjad_files.len());
        
        // Generate documentation for each file
        for file in &abjad_files {
            self.generate_file_doc(file)?;
        }
        
        // Generate index
        self.generate_index(&abjad_files)?;
        
        println!("Documentation generated successfully to {}", self.output_dir);
        Ok(())
    }

    /// Find all Abjad files in a directory
    fn find_abjad_files(&self, dir: &Path) -> Result<Vec<String>> {
        let mut files = Vec::new();
        
        let entries = fs::read_dir(dir)
            .map_err(|e| AbjadError::internal(format!("Failed to read directory: {}", e)))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| AbjadError::internal(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively search subdirectories
                files.extend(self.find_abjad_files(&path)?);
            } else if path.extension().map_or(false, |ext| ext == "abjad") {
                files.push(path.to_string_lossy().to_string());
            }
        }
        
        Ok(files)
    }

    /// Generate documentation for a single file
    fn generate_file_doc(&self, file_path: &str) -> Result<()> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| AbjadError::internal(format!("Failed to read file: {}", e)))?;
        
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        // Extract documentation comments
        let docs = self.extract_docs(&content);
        
        // Generate HTML documentation
        let html = self.generate_html(file_name, &docs);
        
        // Write to output file
        let output_file = format!("{}/{}.html", self.output_dir, file_name);
        fs::write(&output_file, html)
            .map_err(|e| AbjadError::internal(format!("Failed to write documentation: {}", e)))?;
        
        println!("  Generated documentation for {}", file_name);
        Ok(())
    }

    /// Extract documentation comments from source code
    fn extract_docs(&self, content: &str) -> Vec<String> {
        let mut docs = Vec::new();
        let mut in_doc = false;
        let mut current_doc = String::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("///") {
                in_doc = true;
                let doc_text = trimmed[3..].trim();
                current_doc.push_str(doc_text);
                current_doc.push('\n');
            } else if in_doc {
                if trimmed.is_empty() || trimmed.starts_with("دالة") || trimmed.starts_with("هيكل") {
                    if !current_doc.is_empty() {
                        docs.push(current_doc.clone());
                        current_doc.clear();
                    }
                    in_doc = false;
                } else {
                    current_doc.push_str(trimmed);
                    current_doc.push('\n');
                }
            }
        }
        
        if !current_doc.is_empty() {
            docs.push(current_doc);
        }
        
        docs
    }

    /// Generate HTML documentation
    fn generate_html(&self, file_name: &str, docs: &[String]) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"ar\" dir=\"rtl\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("  <title>");
        html.push_str(file_name);
        html.push_str(" - توثيق أبجد</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; direction: rtl; margin: 40px; }\n");
        html.push_str("    h1 { color: #333; }\n");
        html.push_str("    .doc { margin: 20px 0; padding: 15px; background: #f5f5f5; border-radius: 5px; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <h1>");
        html.push_str(file_name);
        html.push_str("</h1>\n");
        
        for doc in docs {
            html.push_str("  <div class=\"doc\">\n");
            html.push_str("    <pre>\n");
            html.push_str(doc);
            html.push_str("    </pre>\n");
            html.push_str("  </div>\n");
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        html
    }

    /// Generate index page
    fn generate_index(&self, files: &[String]) -> Result<()> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"ar\" dir=\"rtl\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("  <title>توثيق أبjad</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; direction: rtl; margin: 40px; }\n");
        html.push_str("    h1 { color: #333; }\n");
        html.push_str("    ul { list-style-type: none; }\n");
        html.push_str("    li { margin: 10px 0; }\n");
        html.push_str("    a { color: #0066cc; text-decoration: none; }\n");
        html.push_str("    a:hover { text-decoration: underline; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <h1>توثيق أبجد</h1>\n");
        html.push_str("  <ul>\n");
        
        for file in files {
            let file_name = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            html.push_str("    <li><a href=\"");
            html.push_str(file_name);
            html.push_str(".html\">");
            html.push_str(file_name);
            html.push_str("</a></li>\n");
        }
        
        html.push_str("  </ul>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        let index_file = format!("{}/index.html", self.output_dir);
        fs::write(&index_file, html)
            .map_err(|e| AbjadError::internal(format!("Failed to write index: {}", e)))?;
        
        println!("  Generated index");
        Ok(())
    }

    /// Open documentation in browser
    pub fn open(&self) -> Result<()> {
        let index_file = format!("{}/index.html", self.output_dir);
        
        let path = Path::new(&index_file);
        if !path.exists() {
            return Err(AbjadError::internal("Documentation not found. Run 'abjad doc' first."));
        }
        
        // Open in default browser
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(&index_file)
                .spawn()
                .map_err(|e| AbjadError::internal(format!("Failed to open browser: {}", e)))?;
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(&index_file)
                .spawn()
                .map_err(|e| AbjadError::internal(format!("Failed to open browser: {}", e)))?;
        }
        
        #[cfg(target_os = "windows")]
        {
            Command::new("start")
                .arg(&index_file)
                .spawn()
                .map_err(|e| AbjadError::internal(format!("Failed to open browser: {}", e)))?;
        }
        
        println!("Opened documentation in browser");
        Ok(())
    }

    /// Publish documentation
    pub fn publish(&self, url: &str) -> Result<()> {
        println!("Publishing documentation to {}...", url);
        
        // In a real implementation, this would upload to a documentation hosting service
        Ok(())
    }
}

impl Default for DocGenerator {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_generator_default() {
        let dg = DocGenerator::default();
        assert_eq!(dg.output_dir, "target/doc");
    }

    #[test]
    fn test_doc_generator_custom() {
        let dg = DocGenerator::new(Some("/custom/doc".to_string()));
        assert_eq!(dg.output_dir, "/custom/doc");
    }

    #[test]
    fn test_extract_docs() {
        let dg = DocGenerator::default();
        let content = "/// This is a doc comment\ndالة مثال() {}";
        let docs = dg.extract_docs(content);
        assert_eq!(docs.len(), 1);
        assert!(docs[0].contains("This is a doc comment"));
    }
}
