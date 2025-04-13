use docx_rs::{Run, RunFonts};

use super::{
    structural::ApplicableStyles,
    style::{Style, StyleError},
};

/// Chunk of text attached to a certain style
#[derive(Debug, Clone)]
pub struct StyledText {
    pub text: String,
    pub style: Style,
}

impl Default for StyledText {
    fn default() -> Self {
        StyledText {
            text: String::new(),
            style: Style::default(),
        }
    }
}

impl StyledText {
    pub fn new(text: String, style: Style) -> Self {
        StyledText { text, style }
    }

    pub fn apply_to_raw(&self) -> docx_rs::Run {
        let mut run = Run::new().add_text(&self.text);

        run = run.fonts(RunFonts::new().ascii(self.style.font()));
        run = run.size(self.style.size() as usize);
        // docx-rs Run::color expects hex string without the leading '#'
        run = run.color(&self.style.font_color()[1..]);
        if self.style.bold() {
            run = run.bold();
        }
        if self.style.italic() {
            run = run.italic();
        }
        if let Some(u_style) = self.style.underline() {
            run = run.underline(format!("{}", u_style).as_str());
        }
        if let Some(highlight) = self.style.highlight_color() {
            // docx-rs Run::highlight expects hex string without the leading '#'
            run = run.highlight(&highlight[1..]);
        }

        run
    }

    // TODO: this is just an initial idea.
    pub fn apply_style_tagging(&self) -> String {
        format!("[[{}]]{}[[/{}]]", self.style, self.text, self.style)
    }

    /// Change self style of written section calling on certain commands
    // TODO: Maybe this would be optimal receiving an enum
    pub fn change_style(&mut self, command: ApplicableStyles) -> Result<(), StyleError> {
        self.style = match command {
            ApplicableStyles::Bold => self.style.clone().switch_bold(),
            ApplicableStyles::Italic => self.style.clone().switch_italic(),
            ApplicableStyles::Underline(style_opt) => self.style.clone().set_underline(style_opt),
            ApplicableStyles::Size(n) => self.style.clone().change_size(n),
            ApplicableStyles::Color(s) => self.style.clone().change_font_color(s.to_string())?,
            ApplicableStyles::Highlight(s) => {
                self.style.clone().change_font_highlight(s.clone())?
            }
            ApplicableStyles::Font(s) => self.style.clone().change_font(s.to_string())?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stylemgr::structural::ApplicableStyles;
    use crate::stylemgr::style::{Style, UnderlineStyle};

    #[test]
    fn test_styled_text_new() {
        let style = Style::default().switch_bold();
        let text = "Hello".to_string();
        let st = StyledText::new(text.clone(), style.clone());

        assert_eq!(st.text, text);
        assert!(st.style.bold());
        assert!(!st.style.italic()); // Check against default
    }

    #[test]
    fn test_styled_text_default() {
        let st = StyledText::default();
        assert_eq!(st.text, "");
        // Check against Style::new() defaults
        assert!(!st.style.bold());
        assert!(!st.style.italic());
        assert_eq!(st.style.size(), 11);
        assert_eq!(st.style.font(), "Arial");
        assert_eq!(st.style.font_color(), "#000000");
    }

    #[test]
    fn test_apply_style_tagging() {
        let style = Style::default().switch_bold().change_size(14);
        let text = "World".to_string();
        let st = StyledText::new(text.clone(), style);

        // Expected format depends on Style's Display impl
        let expected_tag = "bold;pt(14);Arial;fc(#000000)";
        let expected_output = format!("[[{}]]{}[[/{}]]", expected_tag, text, expected_tag);

        assert_eq!(st.apply_style_tagging(), expected_output);
    }

    #[test]
    fn test_change_style_simple() {
        let mut st = StyledText::new("Test".to_string(), Style::default());

        assert!(!st.style.bold());
        let result = st.change_style(ApplicableStyles::Bold);
        assert!(result.is_ok());
        assert!(st.style.bold());

        assert!(!st.style.italic());
        let result = st.change_style(ApplicableStyles::Italic);
        assert!(result.is_ok());
        assert!(st.style.italic());
        assert!(st.style.bold()); // Previous style should persist

        assert_eq!(st.style.size(), 11);
        let result = st.change_style(ApplicableStyles::Size(16));
        assert!(result.is_ok());
        assert_eq!(st.style.size(), 16);
    }

    #[test]
    fn test_change_style_color_valid() {
        let mut st = StyledText::new("Color".to_string(), Style::default());
        let result = st.change_style(ApplicableStyles::Color("#112233".to_string()));
        assert!(result.is_ok());
        assert_eq!(st.style.font_color(), "#112233");
    }

    #[test]
    fn test_change_style_color_invalid() {
        let mut st = StyledText::new("Color".to_string(), Style::default());
        let original_color = st.style.font_color().to_string();

        let result = st.change_style(ApplicableStyles::Color("InvalidHex".to_string()));
        assert!(result.is_err());
        // Check that the style was not changed
        assert_eq!(st.style.font_color(), original_color);
        assert!(matches!(
            result.unwrap_err(),
            StyleError::InvalidHexColor(_)
        ));
    }

    #[test]
    fn test_change_style_font_invalid() {
        let mut st = StyledText::new("Font".to_string(), Style::default());
        let original_font = st.style.font().to_string();

        let result = st.change_style(ApplicableStyles::Font(
            "DefinitelyNotAFontName123".to_string(),
        ));
        assert!(result.is_err());
        // Check that the style was not changed
        assert_eq!(st.style.font(), original_font);
        assert!(matches!(result.unwrap_err(), StyleError::FontNotFound(_)));
    }

    // Optional: Basic check for apply_to_raw
    #[test]
    fn test_apply_to_raw_runs() {
        let st = StyledText::new("Test Run".to_string(), Style::default());
        let _run = st.apply_to_raw(); // Prefixed with _ to mark as unused
        // Basic check: Ensure it returns a Run object. More detailed checks are complex.
        assert!(std::any::TypeId::of::<Run>() == std::any::TypeId::of::<docx_rs::Run>());
        // We can't easily check the internal state of the Run without more work.
        // println!("apply_to_raw produced a Run: {:?}", run); // Requires Run to implement Debug - Commented out
    }

    #[test]
    fn test_change_style_underline() {
        let mut st = StyledText::new("Underline".to_string(), Style::default());

        assert_eq!(st.style.underline(), None);
        let result = st.change_style(ApplicableStyles::Underline(Some(UnderlineStyle::Double)));
        assert!(result.is_ok());
        assert_eq!(st.style.underline(), Some(&UnderlineStyle::Double));
    }
}
