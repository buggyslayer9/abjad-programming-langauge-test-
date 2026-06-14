//! I/O operations module for the Abjad standard library

use std::fs;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;

/// Represents a file for reading and writing
pub struct File {
    path: String,
}

impl File {
    /// Opens a file for reading
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        fs::File::open(path)?;
        Ok(File {
            path: path.as_ref().to_string_lossy().to_string(),
        })
    }

    /// Creates a new file for writing
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        fs::File::create(path)?;
        Ok(File {
            path: path.as_ref().to_string_lossy().to_string(),
        })
    }

    /// Reads all content from the file
    pub fn read_all(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Writes content to the file
    pub fn write(&self, content: &str) -> io::Result<()> {
        fs::write(&self.path, content)
    }

    /// Appends content to the file
    pub fn append(&self, content: &str) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&self.path)?;
        file.write_all(content.as_bytes())
    }

    /// Returns true if the file exists
    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Deletes the file
    pub fn delete(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }

    /// Returns the file size in bytes
    pub fn size(&self) -> io::Result<u64> {
        fs::metadata(&self.path).map(|m| m.len())
    }
}

/// Represents an input stream
pub struct InputStream {
    reader: Box<dyn Read>,
}

impl InputStream {
    /// Creates a new input stream from standard input
    pub fn stdin() -> Self {
        InputStream {
            reader: Box::new(io::stdin()),
        }
    }

    /// Creates a new input stream from a file
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        Ok(InputStream {
            reader: Box::new(file),
        })
    }

    /// Reads a line from the input stream
    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        let mut reader = BufReader::new(&mut self.reader);
        reader.read_line(&mut line)?;
        Ok(line.trim_end().to_string())
    }

    /// Reads all content from the input stream
    pub fn read_all(&mut self) -> io::Result<String> {
        let mut content = String::new();
        self.reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Reads bytes from the input stream
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buffer)
    }
}

/// Represents an output stream
pub struct OutputStream {
    writer: Box<dyn Write>,
}

impl OutputStream {
    /// Creates a new output stream to standard output
    pub fn stdout() -> Self {
        OutputStream {
            writer: Box::new(io::stdout()),
        }
    }

    /// Creates a new output stream to standard error
    pub fn stderr() -> Self {
        OutputStream {
            writer: Box::new(io::stderr()),
        }
    }

    /// Creates a new output stream to a file
    pub fn to_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = fs::File::create(path)?;
        Ok(OutputStream {
            writer: Box::new(file),
        })
    }

    /// Writes content to the output stream
    pub fn write(&mut self, content: &str) -> io::Result<()> {
        self.writer.write_all(content.as_bytes())
    }

    /// Writes a line to the output stream
    pub fn write_line(&mut self, content: &str) -> io::Result<()> {
        self.writer.write_all(format!("{}\n", content).as_bytes())
    }

    /// Flushes the output stream
    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Terminal I/O operations
pub struct Terminal;

impl Terminal {
    /// Reads a line from the terminal
    pub fn read_line() -> io::Result<String> {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(line.trim_end().to_string())
    }

    /// Reads a password from the terminal (without echoing)
    pub fn read_password() -> io::Result<String> {
        // In a real implementation, this would use platform-specific methods
        // to disable terminal echoing
        Self::read_line()
    }

    /// Writes to the terminal
    pub fn write(content: &str) -> io::Result<()> {
        print!("{}", content);
        io::stdout().flush()
    }

    /// Writes a line to the terminal
    pub fn write_line(content: &str) -> io::Result<()> {
        println!("{}", content);
        Ok(())
    }

    /// Clears the terminal screen
    pub fn clear() -> io::Result<()> {
        // ANSI escape code to clear screen
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush()
    }

    /// Moves the cursor to the specified position
    pub fn move_cursor(row: u16, col: u16) -> io::Result<()> {
        print!("\x1B[{};{}H", row, col);
        io::stdout().flush()
    }

    /// Hides the cursor
    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1B[?25l");
        io::stdout().flush()
    }

    /// Shows the cursor
    pub fn show_cursor() -> io::Result<()> {
        print!("\x1B[?25h");
        io::stdout().flush()
    }

    /// Gets the terminal size
    pub fn size() -> io::Result<(u16, u16)> {
        terminal_size::terminal_size()
            .map(|(w, h)| (w.0, h.0))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get terminal size"))
    }
}

/// Directory operations
pub struct Directory;

impl Directory {
    /// Creates a new directory
    pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir(path)
    }

    /// Creates a directory and all parent directories
    pub fn create_all(path: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    /// Removes a directory
    pub fn remove(path: impl AsRef<Path>) -> io::Result<()> {
        fs::remove_dir(path)
    }

    /// Removes a directory and all its contents
    pub fn remove_all(path: impl AsRef<Path>) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    /// Returns true if the directory exists
    pub fn exists(path: impl AsRef<Path>) -> bool {
        Path::new(path.as_ref()).exists()
    }

    /// Lists the contents of a directory
    pub fn list(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
        fs::read_dir(path)?
            .map(|entry| entry.map(|e| e.path().to_string_lossy().to_string()))
            .collect()
    }

    /// Returns the current working directory
    pub fn current() -> io::Result<String> {
        fs::canonicalize(".")
            .map(|p| p.to_string_lossy().to_string())
    }

    /// Changes the current working directory
    pub fn change(path: impl AsRef<Path>) -> io::Result<()> {
        std::env::set_current_dir(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_operations() {
        let path = PathBuf::from("/tmp/test_abjad_file.txt");
        
        // Create and write
        let file = File::create(&path).unwrap();
        file.write("hello world").unwrap();
        
        // Read
        let file = File::open(&path).unwrap();
        let content = file.read_all().unwrap();
        assert_eq!(content, "hello world");
        
        // Cleanup
        file.delete().unwrap();
    }

    #[test]
    fn test_directory_operations() {
        let path = PathBuf::from("/tmp/test_abjad_dir");
        
        // Create
        Directory::create(&path).unwrap();
        assert!(Directory::exists(&path));
        
        // Cleanup
        Directory::remove(&path).unwrap();
    }

    #[test]
    fn test_terminal_write() {
        Terminal::write("test").unwrap();
        Terminal::write_line("test line").unwrap();
    }
}
