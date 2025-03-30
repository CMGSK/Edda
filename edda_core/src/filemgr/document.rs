use std::{collections::VecDeque, fs::File, io};

use docx_rs::{Docx, Paragraph};

use crate::stylemgr::structural::StyledParagraph;

pub struct Document {
    content: VecDeque<StyledParagraph>,
    metadata: Metadata,
}

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
            content: VecDeque::new(),
            metadata: Metadata {
                title: title.into(),
                authors: None,
                description: None,
                category: None,
                version: None,
                status: None,
                language: None,
                keywords: None,
            },
        }
    }

    pub fn get_metadata(&self) -> String {
        format!("{:?}", self.metadata)
    }
    /// Get full document as string
    pub fn get_text(&self, tagged: bool) -> String {
        self.content
            .iter()
            .map(|sp| {
                sp.raw
                    .iter()
                    .map(|x| {
                        if tagged {
                            x.clone().apply_style_tagging()
                        } else {
                            x.text.clone()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn save_as_docx(&self, path: &str) -> io::Result<()> {
        let mut document = Docx::new();

        for styled_paragraph in &self.content {
            // A Paragraph is a block of text
            let mut docx_paragraph = Paragraph::new();

            for styled_text in &styled_paragraph.raw {
                // A Run is a segment of text inside paragraphs to distinguish styling
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
