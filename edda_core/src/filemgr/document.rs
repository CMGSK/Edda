use std::fmt::Write;
use std::path::Path;
use std::{fs::File, io};

use crate::stylemgr::structural::StyledParagraph;
#[allow(unused_imports)]
use crate::stylemgr::style::Style;
#[allow(unused_imports)]
use crate::stylemgr::text::StyledText;
use docx_rs::{Docx, Paragraph};
use thiserror::Error;

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

    /// Sets the authors for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        self.authors = Some(authors);
        self
    }

    /// Sets the description for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the category for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the version for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the status for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Sets the language for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sets the keywords for the metadata. Consumes and returns `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used"]
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = Some(keywords);
        self
    }

    // --- Getters ---

    /// Returns a reference to the title string.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns an optional reference to the vector of author strings.
    pub fn authors(&self) -> Option<&Vec<String>> {
        self.authors.as_ref()
    }

    /// Returns an optional reference to the description string slice.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns an optional reference to the category string slice.
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Returns an optional reference to the version string slice.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns an optional reference to the status string slice.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Returns an optional reference to the language string slice.
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Returns an optional reference to the vector of keyword strings.
    pub fn keywords(&self) -> Option<&Vec<String>> {
        self.keywords.as_ref()
    }
}

impl Document {
    /// Creates a new, empty `Document` with the specified title.
    ///
    /// The document will have no paragraphs initially.
    ///
    /// # Arguments
    /// * `title` - The title for the new document. Accepts any type that implements `Into<String>`.
    #[must_use = "Creating a new Document does nothing unless assigned"]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            content: Vec::new(),
            metadata: Metadata::new(title),
        }
    }

    /// Creates a new `Document` with the given title and initial content.
    ///
    /// # Arguments
    /// * `title` - The title for the new document. Accepts any type that implements `Into<String>`.
    /// * `content` - A vector of `StyledParagraph`s to initialize the document with.
    #[must_use = "Creating a new Document does nothing unless assigned"]
    pub fn with_content(title: impl Into<String>, content: Vec<StyledParagraph>) -> Self {
        Self {
            content,
            metadata: Metadata::new(title),
        }
    }

    /// Returns an immutable reference to the document's `Metadata`.
    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns a mutable reference to the document's `Metadata`, allowing modification.
    pub fn get_metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    /// Replaces the document's existing `Metadata` with the provided one.
    /// Returns a mutable reference to `self` for chaining.
    #[must_use = "Method call does nothing unless the result is used (or you don't need chaining)"]
    pub fn set_metadata(&mut self, metadata: Metadata) -> &mut Self {
        self.metadata = metadata;
        self
    }

    /// Appends a `StyledParagraph` to the end of the document's content.
    /// Returns a mutable reference to `self` for chaining.
    ///
    /// # Arguments
    /// * `paragraph` - The `StyledParagraph` to add.
    #[must_use = "Method call does nothing unless the result is used (or you don't need chaining)"]
    pub fn add_paragraph(&mut self, paragraph: StyledParagraph) -> &mut Self {
        self.content.push(paragraph);
        self
    }

    /// Removes the `StyledParagraph` at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the paragraph to remove.
    ///
    /// # Returns
    /// The removed `StyledParagraph` if the index was valid, otherwise `None`.
    ///
    /// # Panics
    /// This method does not panic; it returns `None` for out-of-bounds indices.
    pub fn remove_paragraph(&mut self, index: usize) -> Option<StyledParagraph> {
        if index < self.content.len() {
            Some(self.content.remove(index))
        } else {
            None
        }
    }

    /// Returns an optional immutable reference to the `StyledParagraph` at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the paragraph to retrieve.
    ///
    /// # Returns
    /// `Some(&StyledParagraph)` if the index is valid, otherwise `None`.
    pub fn get_paragraph(&self, index: usize) -> Option<&StyledParagraph> {
        self.content.get(index)
    }

    /// Returns an optional mutable reference to the `StyledParagraph` at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the paragraph to retrieve mutably.
    ///
    /// # Returns
    /// `Some(&mut StyledParagraph)` if the index is valid, otherwise `None`.
    pub fn get_paragraph_mut(&mut self, index: usize) -> Option<&mut StyledParagraph> {
        self.content.get_mut(index)
    }

    /// Returns the total number of paragraphs currently in the document.
    pub fn paragraph_count(&self) -> usize {
        self.content.len()
    }

    /// Returns `true` if the document contains no paragraphs, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Renders the entire document content as a single `String`.
    ///
    /// # Arguments
    /// * `tagged` - If `true`, includes inline style tags (e.g., `[[Style...]]Text[[/Style...]]`)
    ///              in the output string. If `false`, only the raw text content is included.
    ///
    /// # Returns
    /// A `String` containing the concatenated text of all paragraphs.
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

    /// Saves the document content as a DOCX file to the specified path.
    ///
    /// This method iterates through the `StyledParagraph`s and `StyledText`s,
    /// converting them into the `docx_rs` representation and then packaging them
    /// into a .docx file.
    ///
    /// # Arguments
    /// * `path` - The file system path where the .docx file should be saved. Accepts any type
    ///            that implements `AsRef<Path>`.
    ///
    /// # Errors
    /// Returns `DocumentError::Io` if there's an issue creating or writing to the file.
    /// Returns `DocumentError::DocxPackaging` if `docx_rs` encounters an error during packaging.
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

        // Check content is empty
        assert!(doc.content.is_empty());
        assert!(doc.is_empty()); // Also check the helper method

        // Check metadata against a default instance with the same title
        let expected_metadata = Metadata {
            title: title.to_string(),
            ..Default::default()
        };
        assert_eq!(doc.metadata, expected_metadata);
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
                assert!(
                    e.kind() == io::ErrorKind::NotFound
                        || e.kind() == io::ErrorKind::PermissionDenied
                        || e.kind() == io::ErrorKind::Other,
                    "Expected NotFound, PermissionDenied or Other error kind, got {:?}",
                    e.kind()
                );
            }
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

        let _ = doc.add_paragraph(para1);
        assert_eq!(doc.paragraph_count(), 1);
        assert!(!doc.is_empty());

        let _ = doc.add_paragraph(para2);
        assert_eq!(doc.paragraph_count(), 2);

        // Get paragraph (immutable)
        let para_ref = doc.get_paragraph(0);
        assert!(para_ref.is_some());
        // Ensure it's the first one we added (though they are empty here)
        assert_eq!(para_ref.unwrap(), &StyledParagraph::new()); // Compare with a default empty one

        // Get paragraph (mutable) - Test modification
        let para_mut = doc.get_paragraph_mut(1);
        assert!(para_mut.is_some());
        let style = Style::new().switch_bold();
        let text = StyledText::new("Modified".to_string(), style);
        para_mut.unwrap().add(text.clone());

        // Verify modification via immutable getter
        let para_ref_modified = doc.get_paragraph(1);
        assert!(para_ref_modified.is_some());
        assert_eq!(para_ref_modified.unwrap().raw.len(), 1);
        assert_eq!(para_ref_modified.unwrap().raw[0], text);

        // Remove paragraph and verify return value
        // Re-create para1 for comparison, as the original was moved
        let para1_original = StyledParagraph::new();
        let removed = doc.remove_paragraph(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), para1_original); // Check if the correct paragraph was removed
        assert_eq!(doc.paragraph_count(), 1);

        // Check remaining paragraph is the modified one
        assert_eq!(doc.get_paragraph(0).unwrap().raw.len(), 1);
        assert_eq!(doc.get_paragraph(0).unwrap().raw[0], text);

        // Out of bounds access checks
        assert!(doc.get_paragraph(1).is_none()); // Index 1 is now out of bounds
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
        assert_eq!(
            metadata.keywords().unwrap(),
            &vec!["test".to_string(), "example".to_string()]
        );

        // Test with Document
        let mut doc = Document::new("Original Title");
        let _ = doc.set_metadata(metadata);

        let metadata_ref = doc.get_metadata();
        assert_eq!(metadata_ref.title(), "Test Title");

        // Test mutable metadata
        let metadata_mut = doc.get_metadata_mut();
        let new_metadata = Metadata::new("New Title");
        *metadata_mut = new_metadata;

        assert_eq!(doc.get_metadata().title(), "New Title");
    }
}
