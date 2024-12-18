use std::fs;
use std::io::{self, Write};

/// Reads the contents of a file from the given path.
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Writes the given content to a file at the given path.
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}

/// Example processing function: Converts the content to uppercase.
pub fn process_content(content: &str) -> String {
    content.to_uppercase()
}