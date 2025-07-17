use anyhow::{Result, Context};
use std::fs;

pub fn read_pdf(path: &str) -> Result<Vec<u8>> {
    fs::read(path).context("Failed to read PDF file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_read_pdf_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = b"test pdf content";
        temp_file.write_all(test_data).unwrap();
        
        let result = read_pdf(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(result, test_data);
    }
    
    #[test]
    fn test_read_pdf_file_not_found() {
        let result = read_pdf("/nonexistent/file.pdf");
        assert!(result.is_err());
    }
}