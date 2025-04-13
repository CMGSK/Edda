use serde::Serialize;
use std::fmt::Write;
use std::io::Read;
use std::path::Path;
use std::{fs::File, io};
use thiserror::Error;

use docx_rs::{DocumentChild, Docx, Paragraph, ParagraphChild, ReaderError, RunChild, read_docx};

use crate::stylemgr::structural::StyledParagraph;
#[allow(unused_imports)]
use crate::stylemgr::style::Style;
#[allow(unused_imports)]
use crate::stylemgr::text::StyledText;

#[derive(Debug, Error)]
pub enum DocumentErr {
    #[error("Could not open document: {0}")]
    OpenDocxError(std::io::Error),
    #[error("Could not read the docx document: {0}")]
    ReadDocxError(ReaderError),
}

impl From<std::io::Error> for DocumentErr {
    fn from(value: std::io::Error) -> Self {
        DocumentErr::OpenDocxError(value)
    }
}
impl From<ReaderError> for DocumentErr {
    fn from(value: ReaderError) -> Self {
        DocumentErr::ReadDocxError(value)
    }
}

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

    pub fn load_parsed_docx<P: AsRef<Path>>(path: P) -> Result<Self, DocumentErr> {
        let docx = Self::load_from_docx(path)?;
        let mut buf = Vec::new();

        for dx_paragraph in docx.document.children {
            let mut raw_buf = Vec::new();
            if let DocumentChild::Paragraph(paragraph) = dx_paragraph {
                for dx_run in paragraph.children {
                    if let ParagraphChild::Run(run) = dx_run {
                        let mut text = String::new();
                        text = run.children.iter().fold(text, |acc, x| {
                            if let RunChild::Text(x) = x {
                                format!("{acc} {}", x.text.trim())
                            } else {
                                acc
                            }
                        });
                        let run = run.run_property;
                        let bold = run.bold.is_some();
                        let italics = run.italic.is_some();

                        // TODO: We need to figure out how to get this `val` from a docx with docx-rs crate without
                        // going nuts in the process. Thanks for the extensive documentation, @bokuweb >:|
                        // I came up with `get_run_property_serde` but that's just awful and risky to say the least.
                        let size = Self::get_run_property_serde(&run.sz);
                        let fonts = Self::get_run_property_serde(&run.fonts); // This is specially obscure
                        let color = Self::get_run_property_serde(&run.color);
                        let underline = Self::get_run_property_serde(&run.underline);
                        let highlight = Self::get_run_property_serde(&run.highlight);

                        let style =
                            Style::new(Some(bold), Some(italics), None, None, None, None, None);
                        raw_buf.push(StyledText::new(text, style));
                    }
                }
            }
            buf.push(StyledParagraph { raw: raw_buf });
        }

        //TODO: load metadata from the docx file

        Ok(Self {
            content: buf,
            metadata: Metadata::default(),
        })
    }

    /// This function is highly discouraged and will be removed once there's a safe workaround to it.
    fn get_run_property_serde<T: Serialize>(prop: &Option<T>) -> Option<String> {
        match prop {
            Some(prop) => match serde_json::to_value(prop).unwrap_or_default().get("val") {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            None => None,
        }
    }

    fn load_from_docx<P: AsRef<Path>>(path: P) -> Result<Docx, DocumentErr> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf);
        let doc = read_docx(&buf)?;

        Ok(doc)
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

        let style1 = Style::default();
        let style2 = Style::default().switch_bold();

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

        let style1_tag = format!("{}", Style::default());
        let style2_tag = format!("{}", Style::default().switch_bold());

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
