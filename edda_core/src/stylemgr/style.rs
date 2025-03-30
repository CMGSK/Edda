use std::fmt;
use thiserror::Error;

use font_kit::{error::SelectionError, source::SystemSource};

#[derive(Debug, Error)]
pub enum StyleError {
    #[error("Invalid HEX color format: '{0}'")]
    InvalidHexColor(String),
    #[error("Font not found: '{0}'")]
    FontNotFound(String),
    #[error("Failed to query system fonts for '{0}': {1}")]
    FontQueryError(String, SelectionError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnderlineStyle {
    Single,
    Words,
    Double,
    Thick,
    Dotted,
    DottedHeavy,
    Dash,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DashDotHeavy,
    DotDotDash,
    DashDotDotHeavy,
    Wave,
    WavyHeavy,
    WavyDouble,
    // Note: "none" is represented by Option::None in the Style struct
}

impl fmt::Display for UnderlineStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            UnderlineStyle::Single => "single",
            UnderlineStyle::Words => "words",
            UnderlineStyle::Double => "double",
            UnderlineStyle::Thick => "thick",
            UnderlineStyle::Dotted => "dotted",
            UnderlineStyle::DottedHeavy => "dottedHeavy",
            UnderlineStyle::Dash => "dash",
            UnderlineStyle::DashedHeavy => "dashedHeavy",
            UnderlineStyle::DashLong => "dashLong",
            UnderlineStyle::DashLongHeavy => "dashLongHeavy",
            UnderlineStyle::DotDash => "dotDash",
            UnderlineStyle::DashDotHeavy => "dashDotHeavy",
            UnderlineStyle::DotDotDash => "dotDotDash",
            UnderlineStyle::DashDotDotHeavy => "dashDotDotHeavy",
            UnderlineStyle::Wave => "wave",
            UnderlineStyle::WavyHeavy => "wavyHeavy",
            UnderlineStyle::WavyDouble => "wavyDouble",
        })
    }
}

/// A defined Style for a chunk of text.
#[derive(Debug, Clone)]
pub struct Style {
    bold: bool,
    italic: bool,
    underline: Option<UnderlineStyle>,
    size: u8,
    font: String,
    font_color: String,
    highlight_color: Option<String>,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bold {
            write!(f, "bold;")?;
        }
        if self.italic {
            write!(f, "italic;")?;
        }
        if let Some(u_style) = &self.underline {
            write!(f, "underline({});", u_style)?;
        }
        if let Some(color) = &self.highlight_color {
            write!(f, "hc({});", color)?;
        }

        write!(f, "pt({});{};fc({})", self.size, self.font, self.font_color)
    }
}

impl Style {
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: None,
            size: 11,
            font: "Arial".into(),
            font_color: "#000000".into(),
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

    pub fn set_underline(mut self, style: Option<UnderlineStyle>) -> Self {
        self.underline = style;
        self
    }

    pub fn change_size(mut self, new_size: u8) -> Self {
        self.size = new_size;
        self
    }

    pub fn change_font_color(mut self, new_color: String) -> Result<Self, StyleError> {
        check_hex(&new_color)?;

        self.font_color = new_color;
        Ok(self)
    }

    pub fn change_font_highlight(mut self, new_color: Option<String>) -> Result<Self, StyleError> {
        if let Some(color) = &new_color {
            check_hex(color)?;
        }

        self.highlight_color = new_color;
        Ok(self)
    }

    pub fn change_font(mut self, new_font: String) -> Result<Self, StyleError> {
        check_font(&new_font)?;

        self.font = new_font;
        Ok(self)
    }

    // Getters for private fields
    pub fn bold(&self) -> bool {
        self.bold
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn underline(&self) -> Option<&UnderlineStyle> {
        self.underline.as_ref()
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn font(&self) -> &str {
        &self.font
    }

    pub fn font_color(&self) -> &str {
        &self.font_color
    }

    pub fn highlight_color(&self) -> Option<&str> {
        self.highlight_color.as_deref() // Returns Option<&str>
    }
}

/// Check if the string is a valid HEX color code. They can be # + 6 or 8 depending on alpha channel use
fn check_hex(s: &str) -> Result<(), StyleError> {
    if !s.starts_with('#') {
        return Err(StyleError::InvalidHexColor(s.to_string()));
    }
    let hex_part = &s[1..];
    if !(hex_part.len() == 6 || hex_part.len() == 8) {
        return Err(StyleError::InvalidHexColor(s.to_string()));
    }
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(StyleError::InvalidHexColor(s.to_string()));
    }
    Ok(())
}

/// Check if the selected font exists in the system
fn check_font(s: &str) -> Result<(), StyleError> {
    match SystemSource::new().select_family_by_name(s) {
        Ok(_) => Ok(()),
        Err(SelectionError::NotFound) => Err(StyleError::FontNotFound(s.to_string())),
        Err(e) => Err(StyleError::FontQueryError(s.to_string(), e)),
    }
}

    }
}
