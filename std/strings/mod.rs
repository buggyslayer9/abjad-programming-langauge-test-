//! String handling module for the Abjad standard library

/// Represents a string in Abjad
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbjadString {
    pub data: String,
}

impl AbjadString {
    /// Creates a new AbjadString
    pub fn new(data: impl Into<String>) -> Self {
        AbjadString {
            data: data.into(),
        }
    }

    /// Returns the length of the string
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the string is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns true if the string contains the substring
    pub fn contains(&self, substring: &str) -> bool {
        self.data.contains(substring)
    }

    /// Returns true if the string starts with the prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.data.starts_with(prefix)
    }

    /// Returns true if the string ends with the suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.data.ends_with(suffix)
    }

    /// Converts the string to uppercase
    pub fn to_uppercase(&self) -> Self {
        AbjadString::new(self.data.to_uppercase())
    }

    /// Converts the string to lowercase
    pub fn to_lowercase(&self) -> Self {
        AbjadString::new(self.data.to_lowercase())
    }

    /// Trims whitespace from both ends
    pub fn trim(&self) -> Self {
        AbjadString::new(self.data.trim())
    }

    /// Trims whitespace from the left
    pub fn trim_left(&self) -> Self {
        AbjadString::new(self.data.trim_start())
    }

    /// Trims whitespace from the right
    pub fn trim_right(&self) -> Self {
        AbjadString::new(self.data.trim_end())
    }

    /// Splits the string by a delimiter
    pub fn split(&self, delimiter: char) -> Vec<AbjadString> {
        self.data
            .split(delimiter)
            .map(|s| AbjadString::new(s))
            .collect()
    }

    /// Replaces occurrences of a pattern
    pub fn replace(&self, from: &str, to: &str) -> Self {
        AbjadString::new(self.data.replace(from, to))
    }

    /// Returns a substring
    pub fn substring(&self, start: usize, end: usize) -> Self {
        AbjadString::new(&self.data[start..end])
    }

    /// Converts the string to an integer if possible
    pub fn to_int(&self) -> Option<i64> {
        self.data.parse().ok()
    }

    /// Converts the string to a float if possible
    pub fn to_float(&self) -> Option<f64> {
        self.data.parse().ok()
    }

    /// Converts the string to a boolean if possible
    pub fn to_bool(&self) -> Option<bool> {
        match self.data.as_str() {
            "صحيح" | "true" | "1" => Some(true),
            "خطأ" | "false" | "0" => Some(false),
            _ => None,
        }
    }

    /// Converts Arabic numerals to Western numerals
    pub fn arabic_to_western(&self) -> Self {
        let mut result = String::new();
        for c in self.data.chars() {
            match c {
                '٠' => result.push('0'),
                '١' => result.push('1'),
                '٢' => result.push('2'),
                '٣' => result.push('3'),
                '٤' => result.push('4'),
                '٥' => result.push('5'),
                '٦' => result.push('6'),
                '٧' => result.push('7'),
                '٨' => result.push('8'),
                '٩' => result.push('9'),
                _ => result.push(c),
            }
        }
        AbjadString::new(result)
    }

    /// Converts Western numerals to Arabic numerals
    pub fn western_to_arabic(&self) -> Self {
        let mut result = String::new();
        for c in self.data.chars() {
            match c {
                '0' => result.push('٠'),
                '1' => result.push('١'),
                '2' => result.push('٢'),
                '3' => result.push('٣'),
                '4' => result.push('٤'),
                '5' => result.push('٥'),
                '6' => result.push('٦'),
                '7' => result.push('٧'),
                '8' => result.push('٨'),
                '9' => result.push('٩'),
                _ => result.push(c),
            }
        }
        AbjadString::new(result)
    }

    /// Reverses the string
    pub fn reverse(&self) -> Self {
        AbjadString::new(self.data.chars().rev().collect::<String>())
    }

    /// Returns the character at the given index
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.data.chars().nth(index)
    }

    /// Returns the number of characters (graphemes)
    pub fn char_count(&self) -> usize {
        self.data.chars().count()
    }
}

impl From<String> for AbjadString {
    fn from(data: String) -> Self {
        AbjadString::new(data)
    }
}

impl From<&str> for AbjadString {
    fn from(data: &str) -> Self {
        AbjadString::new(data)
    }

impl std::fmt::Display for AbjadString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_creation() {
        let s = AbjadString::new("hello");
        assert_eq!(s.len(), 5);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_string_contains() {
        let s = AbjadString::new("hello world");
        assert!(s.contains("world"));
        assert!(!s.contains("foo"));
    }

    #[test]
    fn test_string_starts_ends() {
        let s = AbjadString::new("hello world");
        assert!(s.starts_with("hello"));
        assert!(s.ends_with("world"));
    }

    #[test]
    fn test_string_case_conversion() {
        let s = AbjadString::new("hello");
        let upper = s.to_uppercase();
        assert_eq!(upper.data, "HELLO");
        
        let lower = upper.to_lowercase();
        assert_eq!(lower.data, "hello");
    }

    #[test]
    fn test_string_trim() {
        let s = AbjadString::new("  hello  ");
        let trimmed = s.trim();
        assert_eq!(trimmed.data, "hello");
    }

    #[test]
    fn test_string_split() {
        let s = AbjadString::new("a,b,c");
        let parts = s.split(',');
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_string_replace() {
        let s = AbjadString::new("hello world");
        let replaced = s.replace("world", "abjad");
        assert_eq!(replaced.data, "hello abjad");
    }

    #[test]
    fn test_string_to_int() {
        let s = AbjadString::new("123");
        assert_eq!(s.to_int(), Some(123));
        
        let s2 = AbjadString::new("abc");
        assert_eq!(s2.to_int(), None);
    }

    #[test]
    fn test_string_to_float() {
        let s = AbjadString::new("3.14");
        assert_eq!(s.to_float(), Some(3.14));
    }

    #[test]
    fn test_string_to_bool() {
        let s = AbjadString::new("صحيح");
        assert_eq!(s.to_bool(), Some(true));
        
        let s2 = AbjadString::new("خطأ");
        assert_eq!(s2.to_bool(), Some(false));
    }

    #[test]
    fn test_arabic_to_western() {
        let s = AbjadString::new("١٢٣");
        let converted = s.arabic_to_western();
        assert_eq!(converted.data, "123");
    }

    #[test]
    fn test_western_to_arabic() {
        let s = AbjadString::new("123");
        let converted = s.western_to_arabic();
        assert_eq!(converted.data, "١٢٣");
    }

    #[test]
    fn test_string_reverse() {
        let s = AbjadString::new("hello");
        let reversed = s.reverse();
        assert_eq!(reversed.data, "olleh");
    }

    #[test]
    fn test_string_char_at() {
        let s = AbjadString::new("hello");
        assert_eq!(s.char_at(0), Some('h'));
        assert_eq!(s.char_at(4), Some('o'));
        assert_eq!(s.char_at(5), None);
    }

    #[test]
    fn test_string_char_count() {
        let s = AbjadString::new("hello");
        assert_eq!(s.char_count(), 5);
    }
}
