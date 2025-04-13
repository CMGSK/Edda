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
        write!(
            f,
            "{}",
            match self {
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
            }
        )
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

#[cfg(test)]
mod tests {
    use super::*; // Import items from the outer module (Style, StyleError)

    #[test]
    fn test_style_new_defaults() {
        let style = Style::new();
        assert_eq!(style.bold(), false);
        assert_eq!(style.italic(), false);
        assert_eq!(style.underline(), None);
        assert_eq!(style.size(), 11);
        assert_eq!(style.font(), "Arial");
        assert_eq!(style.font_color(), "#000000");
        assert_eq!(style.highlight_color(), None);
    }

    #[test]
    fn test_style_toggles() {
        let style = Style::new();
        assert_eq!(style.bold(), false);
        let style = style.switch_bold();
        assert_eq!(style.bold(), true);
        let style = style.switch_bold();
        assert_eq!(style.bold(), false);

        let style = style.switch_italic();
        assert_eq!(style.italic(), true);
        let style = style.set_underline(Some(UnderlineStyle::Single));
        assert_eq!(style.underline(), Some(&UnderlineStyle::Single));
    }

    #[test]
    fn test_style_change_size() {
        let style = Style::new().change_size(14);
        assert_eq!(style.size(), 14);
    }

    #[test]
    fn test_style_change_font_color_valid() {
        let result = Style::new().change_font_color("#FF00AA".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().font_color(), "#FF00AA");
    }

    #[test]
    fn test_style_change_font_color_invalid_hex() {
        let result = Style::new().change_font_color("FF00AA".to_string()); // Missing #
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StyleError::InvalidHexColor(_)
        ));

        let result = Style::new().change_font_color("#12345".to_string()); // Too short
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StyleError::InvalidHexColor(_)
        ));

        let result = Style::new().change_font_color("#GGHHII".to_string()); // Invalid chars
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StyleError::InvalidHexColor(_)
        ));
    }

    #[test]
    fn test_style_change_font_highlight() {
        let result = Style::new().change_font_highlight(Some("#FFFF00".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().highlight_color(), Some("#FFFF00"));

        let result = result.unwrap().change_font_highlight(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().highlight_color(), None);

        let result = Style::new().change_font_highlight(Some("Invalid".to_string()));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StyleError::InvalidHexColor(_)
        ));
    }

    #[test]
    fn test_style_change_font_valid() {
        // Assuming common fonts are available. Might fail in minimal environments.
        let result = Style::new().change_font("Times New Roman".to_string());
        // This check depends on the font being installed on the system running tests
        if result.is_ok() {
            assert_eq!(result.unwrap().font(), "Times New Roman");
        } else {
            // If font isn't found, don't fail the test, just acknowledge
            println!("Test skipped: 'Times New Roman' not found.");
        }
    }

    #[test]
    fn test_style_change_font_invalid() {
        let result = Style::new().change_font("DefinitelyNotAFontName123".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StyleError::FontNotFound(_)));
    }

    #[test]
    fn test_style_display_format() {
        let style = Style::new();
        assert_eq!(format!("{}", style), "pt(11);Arial;fc(#000000)");

        let style = style.switch_bold().switch_italic();
        assert_eq!(format!("{}", style), "bold;italic;pt(11);Arial;fc(#000000)");

        let style = style
            .change_font_highlight(Some("#00FF00".to_string()))
            .unwrap();
        assert_eq!(
            format!("{}", style),
            "bold;italic;hc(#00FF00);pt(11);Arial;fc(#000000)"
        );

        let style = Style::new()
            .set_underline(Some(UnderlineStyle::Single))
            .change_size(20);
        assert_eq!(
            format!("{}", style),
            "underline(single);pt(20);Arial;fc(#000000)"
        );
    }
}
