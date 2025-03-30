use std::fmt;

use font_kit::source::SystemSource;

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
    pub fn new() -> Self {
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

    pub fn switch_bold(mut self) -> Self {
        self.bold = !self.bold;
        self
    }

    pub fn switch_italic(mut self) -> Self {
        self.italic = !self.italic;
        self
    }

    pub fn switch_underline(mut self) -> Self {
        self.underline = !self.underline;
        self
    }

    pub fn change_size(mut self, new_size: u8) -> Self {
        self.size = new_size;
        self
    }

    pub fn change_font_color(mut self, new_color: String) -> Result<Self, ()> {
        check_hex(&new_color)?;

        self.font_color = new_color;
        Ok(self)
    }

    pub fn change_font_highlight(mut self, new_color: Option<String>) -> Result<Self, ()> {
        if new_color.is_some() {
            check_hex(&new_color.clone().unwrap())?;
        }

        self.highlight_color = new_color;
        Ok(self)
    }

    pub fn change_font(mut self, new_font: String) -> Result<Self, ()> {
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
