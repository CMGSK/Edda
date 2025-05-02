use std::fmt::Write;
use std::path::Path;
use std::{fs::File, io};

use docx_rs::{Docx, Paragraph};
use thiserror::Error;
use crate::stylemgr::structural::StyledParagraph;
#[allow(unused_imports)]
use crate::stylemgr::style::Style;
#[allow(unused_imports)]
use crate::stylemgr::text::StyledText;

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Document packaging error: {0}")]
    DocxPackaging(String),
}


pub struct Document {
    content: Vec<StyledParagraph>,
    metadata: Metadata,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Metadata {
    title: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
    category: Option<String>,
    version: Option<String>,
    status: Option<String>,
    language: Option<String>,
    keywords: Option<Vec<String>>,
}

impl Metadata {
    /// Creates a new Metadata instance with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Sets the authors
    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        self.authors = Some(authors);
        self
    }

    /// Sets the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Sets the category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
    
    /// Sets the version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    
    /// Sets the status
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
    
    /// Sets the language
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
    
    /// Sets the keywords
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = Some(keywords);
        self
    }
    
    // Getters
    
    /// Returns the title of the document
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Returns the authors of the document, if set
    pub fn authors(&self) -> Option<&Vec<String>> {
        self.authors.as_ref()
    }
    
    /// Returns the description of the document, if set
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    
    /// Returns the category of the document, if set
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }
    
    /// Returns the version of the document, if set
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    
    /// Returns the status of the document, if set
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }
    
    /// Returns the language of the document, if set
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
    
    /// Returns the keywords of the document, if set
    pub fn keywords(&self) -> Option<&Vec<String>> {
        self.keywords.as_ref()
    }
}


impl Document {
    /// Create a blank document with the specified title
    /// 
    /// # Arguments
    /// * `title` - The title of the document
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            content: Vec::new(),
            metadata: Metadata::new(title),
        }
    }

    /// Creates a new document with the given content and title
    /// 
    /// # Arguments
    /// * `title` - The title of the document
    /// * `content` - The initial content of the document
    pub fn with_content(title: impl Into<String>, content: Vec<StyledParagraph>) -> Self {
        Self {
            content,
            metadata: Metadata::new(title),
        }
    }

    /// Returns a reference to the document's metadata
    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
    
    /// Returns a mutable reference to the document's metadata
    pub fn get_metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
    
    /// Sets the document's metadata
    pub fn set_metadata(&mut self, metadata: Metadata) -> &mut Self {
        self.metadata = metadata;
        self
    }

    /// Adds a paragraph to the document
    /// 
    /// # Arguments
    /// * `paragraph` - The paragraph to add to the document
    pub fn add_paragraph(&mut self, paragraph: StyledParagraph) -> &mut Self {
        self.content.push(paragraph);
        self
    }
    
    /// Removes a paragraph at the specified index
    /// 
    /// # Arguments
    /// * `index` - The index of the paragraph to remove
    /// 
    /// # Returns
    /// The removed paragraph, or None if the index is out of bounds
    pub fn remove_paragraph(&mut self, index: usize) -> Option<StyledParagraph> {
        if index < self.content.len() {
            Some(self.content.remove(index))
        } else {
            None
        }
    }
    
    /// Returns a reference to the paragraph at the specified index
    /// 
    /// # Arguments
    /// * `index` - The index of the paragraph to get
    pub fn get_paragraph(&self, index: usize) -> Option<&StyledParagraph> {
        self.content.get(index)
    }
    
    /// Returns a mutable reference to the paragraph at the specified index
    /// 
    /// # Arguments
    /// * `index` - The index of the paragraph to get
    pub fn get_paragraph_mut(&mut self, index: usize) -> Option<&mut StyledParagraph> {
        self.content.get_mut(index)
    }
    
    /// Returns the number of paragraphs in the document
    pub fn paragraph_count(&self) -> usize {
        self.content.len()
    }
    
    /// Checks if the document is empty (contains no paragraphs)
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get full document as string
    /// 
    /// # Arguments
    /// * `tagged` - If true, includes style tags in the output
    pub fn get_text(&self, tagged: bool) -> String {
        let mut buffer = String::with_capacity(self.content.len() * 100);

        for sp in &self.content {
            for x in &sp.raw {
                if tagged {
                    let _ = write!(buffer, "{}", x.clone().apply_style_tagging());
                } else {
                    buffer.push_str(&x.text);
                }
            }
        }
        buffer
    }

    pub fn save_as_docx<P: AsRef<Path>>(&self, path: P) -> Result<(), DocumentError> {
        let mut document = Docx::new();

        for styled_paragraph in &self.content {
            let mut docx_paragraph = Paragraph::new();

            for styled_text in &styled_paragraph.raw {
                let run = styled_text.apply_to_raw();
                docx_paragraph = docx_paragraph.add_run(run);
            }

            document = document.add_paragraph(docx_paragraph);
        }

        let mut file = File::create(path)?;
        document
            .build()
            .pack(&mut file)
            .map_err(|e| DocumentError::DocxPackaging(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stylemgr::structural::StyledParagraph;
    use std::fs;

    // Helper to create a document with some content for testing
    fn create_test_document() -> Document {
        let mut doc = Document::new("Test Title");

        let style1 = Style::new();
        let style2 = Style::new().switch_bold();

        let mut para1 = StyledParagraph::new();
        para1.add(StyledText::new(
            "Paragraph 1, Sentence 1. ".to_string(),
            style1.clone(),
        ));
        para1.add(StyledText::new("Bold bit.".to_string(), style2.clone()));

        let mut para2 = StyledParagraph::new();
        para2.add(StyledText::new("Paragraph 2.".to_string(), style1.clone()));

        doc.content.push(para1);
        doc.content.push(para2);

        doc
    }

    #[test]
    fn test_document_new() {
        let title = "My Document";
        let doc = Document::new(title);
        assert!(doc.content.is_empty());
        assert_eq!(doc.metadata.title, title);
        assert!(doc.metadata.authors.is_none());
        assert!(doc.metadata.description.is_none());
        // ... check other metadata fields if needed ...
    }

    #[test]
    fn test_get_metadata() {
        let title = "Another Title";
        let doc = Document::new(title);
        let metadata_ref = doc.get_metadata();
        assert_eq!(metadata_ref.title, title);
    }

    #[test]
    fn test_get_text_untagged() {
        let doc = create_test_document();
        let expected_text = "Paragraph 1, Sentence 1. Bold bit.Paragraph 2.";
        assert_eq!(doc.get_text(false), expected_text);
    }

    #[test]
    fn test_get_text_tagged() {
        let doc = create_test_document();

        let style1_tag = format!("{}", Style::new());
        let style2_tag = format!("{}", Style::new().switch_bold());

        let expected_text = format!(
            "[[{0}]]Paragraph 1, Sentence 1. [[/{0}]][[{1}]]Bold bit.[[/{1}]][[{0}]]Paragraph 2.[[/{0}]]",
            style1_tag, style2_tag
        );

        assert_eq!(doc.get_text(true), expected_text);
    }

    #[test]
    fn test_get_text_empty() {
        let doc = Document::new("Empty Doc");
        assert_eq!(doc.get_text(false), "");
        assert_eq!(doc.get_text(true), "");
    }

    #[test]
    // Basic test to ensure save_as_docx runs and returns Ok.
    // Does not validate the .docx content.
    fn test_save_as_docx_runs() -> io::Result<()> {
        let doc = create_test_document();
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_document_save.docx");

        // Ensure file doesn't exist initially or clean up from previous run
        let _ = fs::remove_file(&file_path);

        let result = doc.save_as_docx(&file_path);
        assert!(result.is_ok());
        assert!(file_path.exists(), "File should have been created");

        // Clean up the created file
        fs::remove_file(&file_path)?;

        Ok(())
    }
    
    #[test]
    fn test_save_as_docx_io_error() {
        let doc = create_test_document();
        // Use a location that should be inaccessible for writing
        // This is OS-specific, but /dev/null/invalid on Unix or an invalid device path on Windows 
        // should generate an IO error
        #[cfg(unix)]
        let invalid_path = "/dev/null/invalid_path.docx";
        #[cfg(windows)]
        let invalid_path = "\\\\invalid-device\\nonexistent\\file.docx";
        
        let result = doc.save_as_docx(invalid_path);
        
        match result {
            Err(DocumentError::Io(e)) => {
                // Verify it's an IO error
                assert!(e.kind() == io::ErrorKind::NotFound || 
                       e.kind() == io::ErrorKind::PermissionDenied ||
                       e.kind() == io::ErrorKind::Other,
                       "Expected NotFound, PermissionDenied or Other error kind, got {:?}", e.kind());
            },
            _ => panic!("Expected an IO error, got {:?}", result),
        }
    }
    
    #[test]
    fn test_error_display() {
        // Test IO error display
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let doc_err = DocumentError::Io(io_err);
        assert!(format!("{}", doc_err).contains("IO error"));
        assert!(format!("{}", doc_err).contains("file not found"));
    
        // Test packaging error display
        let pkg_err = DocumentError::DocxPackaging("failed to package".into());
        assert!(format!("{}", pkg_err).contains("Document packaging error"));
        assert!(format!("{}", pkg_err).contains("failed to package"));
    }
    
    #[test]
    fn test_document_paragraph_manipulation() {
        let mut doc = Document::new("Test Doc");
        assert!(doc.is_empty());
        assert_eq!(doc.paragraph_count(), 0);
        
        // Add paragraphs
        let para1 = StyledParagraph::new();
        let para2 = StyledParagraph::new();
        
        doc.add_paragraph(para1);
        assert_eq!(doc.paragraph_count(), 1);
        assert!(!doc.is_empty());
        
        doc.add_paragraph(para2);
        assert_eq!(doc.paragraph_count(), 2);
        
        // Get paragraph
        let para_ref = doc.get_paragraph(0);
        assert!(para_ref.is_some());
        
        // Remove paragraph
        let removed = doc.remove_paragraph(0);
        assert!(removed.is_some());
        assert_eq!(doc.paragraph_count(), 1);
        
        // Out of bounds access
        assert!(doc.get_paragraph(10).is_none());
        assert!(doc.get_paragraph_mut(10).is_none());
        assert!(doc.remove_paragraph(10).is_none());
    }
    
    #[test]
    fn test_document_with_content() {
        let style = Style::new();
        
        let mut para1 = StyledParagraph::new();
        para1.add(StyledText::new("Test content".to_string(), style.clone()));
        
        let paras = vec![para1];
        
        let doc = Document::with_content("With Content Doc", paras);
        assert_eq!(doc.paragraph_count(), 1);
        assert!(!doc.is_empty());
        assert_eq!(doc.get_text(false), "Test content");
    }
    
    #[test]
    fn test_metadata_getters_and_setters() {
        let title = "Test Title";
        let authors = vec!["Author 1".to_string(), "Author 2".to_string()];
        let description = "Test Description";
        
        let metadata = Metadata::new(title)
            .with_authors(authors.clone())
            .with_description(description)
            .with_category("Test Category")
            .with_version("1.0.0")
            .with_status("Draft")
            .with_language("en-US")
            .with_keywords(vec!["test".to_string(), "example".to_string()]);
        
        // Test getters
        assert_eq!(metadata.title(), title);
        assert_eq!(metadata.authors().unwrap(), &authors);
        assert_eq!(metadata.description().unwrap(), description);
        assert_eq!(metadata.category().unwrap(), "Test Category");
        assert_eq!(metadata.version().unwrap(), "1.0.0");
        assert_eq!(metadata.status().unwrap(), "Draft");
        assert_eq!(metadata.language().unwrap(), "en-US");
        assert_eq!(metadata.keywords().unwrap(), &vec!["test".to_string(), "example".to_string()]);
        
        // Test with Document
        let mut doc = Document::new("Original Title");
        doc.set_metadata(metadata);
        
        let metadata_ref = doc.get_metadata();
        assert_eq!(metadata_ref.title(), "Test Title");
        
        // Test mutable metadata
        let metadata_mut = doc.get_metadata_mut();
        let new_metadata = Metadata::new("New Title");
        *metadata_mut = new_metadata;
        
        assert_eq!(doc.get_metadata().title(), "New Title");
    }
}
