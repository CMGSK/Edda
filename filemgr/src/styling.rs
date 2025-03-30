use std::{collections::VecDeque, fmt};

use docx_rs::{Bold, Italic};
use font_kit::source::SystemSource;

pub enum ApplicableStyles {
    Bold,
    Italic,
    Underline,
    Size(u8),
    Font(String),
    Color(String),
    Highlight(Option<String>)
}

/// Collection of text chunks with its own styles
pub struct StyledParagraph {
    raw: VecDeque<StyledText>,
}

impl StyledParagraph {
    fn new() -> Self {
        StyledParagraph {
            raw: Vec::new().into(),
        }
    }

    fn parse_raw_tagged_text(text: &str) -> Self {
        todo!()
    }
}

/// Chunk of text attached to a certain style
pub struct StyledText {
    pub text: String,
    pub style: Style,
}

/// A defined Style for a chunk of text.
#[derive(Clone)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub size: u8,
    pub font: String,
    pub font_color: String,
    pub highlight_color: Option<String>,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bold {
            write!(f, "bold;")?;
        }
        if self.italic {
            write!(f, "italic;")?;
        }
        if self.underline {
            write!(f, "underline;")?;
        }
        if self.highlight_color.is_some() {
            write!(f, "hc({});", self.highlight_color.clone().unwrap())?;
        }

        write!(f, "pt({});{};fc({})", self.size, self.font, self.font_color)
    }
}

impl Style {
    fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            size: 11,
            font: "Arial".into(),
            font_color: "#FFFFFF".into(),
            highlight_color: None,
        }
    }

    fn switch_bold(mut self) -> Self {
        self.bold = !self.bold;
        self
    }

    fn switch_italic(mut self) -> Self {
        self.italic = !self.italic;
        self
    }

    fn switch_underline(mut self) -> Self {
        self.underline = !self.underline;
        self
    }

    fn change_size(mut self, new_size: u8) -> Self {
        self.size = new_size;
        self
    }

    fn change_font_color(mut self, new_color: String) -> Result<Self, ()> {
        check_hex(&new_color)?;

        self.font_color = new_color;
        Ok(self)
    }

    fn change_font_highlight(mut self, new_color: Option<String>) -> Result<Self, ()> {
        if new_color.is_some() {
            check_hex(&new_color.clone().unwrap())?;
        }

        self.highlight_color = new_color;
        Ok(self)
    }

    fn change_font(mut self, new_font: String) -> Result<Self, ()> {
        check_font(&new_font)?;

        self.font = new_font;
        Ok(self)
    }
}

/// Check if the string is a valid HEX color code. They can be # + 6 or 8 depending on alpha channel use
fn check_hex(s: &str) -> Result<(), ()> {
    if s.starts_with('#')
        || (s.len() != 7 || s.len() != 9)
        || s.chars().skip(1).all(|x| x.is_ascii_hexdigit())
    {
        return Err(());
    }
    Ok(())
}

/// Check if the selected font exists in the system
fn check_font(s: &str) -> Result<(), ()> {
    if SystemSource::new().select_family_by_name(s).is_err() {
        return Err(());
    }
    Ok(())
}

impl StyledText {
    pub fn new() -> Self {
        StyledText {
            text: String::new(),
            style: Style::new(),
        }
    }

    // TODO: this is just an initial idea.
    pub fn apply_style_tagging(self) -> String {
        format!("[[{}]]{}[[/{}]]", self.style, self.text, self.style)
    }

    /// Change self style of written section calling on certain commands
    // TODO: Maybe this would be optimal receiving an enum
    pub fn change_style(mut self, command: ApplicableStyles) {
        let rollback = self.style.clone();
        let new_style = match &command {
            ApplicableStyles::Bold => Ok(self.style.switch_bold()),
            ApplicableStyles::Italic => Ok(self.style.switch_italic()),
            ApplicableStyles::Underline => Ok(self.style.switch_underline()),
            ApplicableStyles::Size(n) => Ok(self.style.change_size(*n)),
            ApplicableStyles::Color(s) => self.style.change_font_color(s.to_string()),
            ApplicableStyles::Highlight(s) => self.style.change_font_highlight(s.clone()),
            ApplicableStyles::Font(s) => self.style.change_font(s.to_string()),
        };

        self.style = new_style.unwrap_or(rollback);
    }
}
