use std::fmt::Write;
use std::path::Path;
use std::{fs::File, io};

use docx_rs::{Docx, Paragraph};

use crate::stylemgr::structural::StyledParagraph;
#[allow(unused_imports)]
use crate::stylemgr::style::Style;
#[allow(unused_imports)]
use crate::stylemgr::text::StyledText;

pub struct Document {
    content: Vec<StyledParagraph>,
    metadata: Metadata,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
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

impl Document {
    /// Create a blank document
    pub fn new(title: &str) -> Self {
        Self {
            content: Vec::new(),
            metadata: Metadata {
                title: title.into(),
                ..Default::default()
            },
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
    /// Get full document as string
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

    pub fn save_as_docx<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
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
        document.build().pack(&mut file)?;

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
}
