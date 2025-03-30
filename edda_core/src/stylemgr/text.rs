use docx_rs::{Run, RunFonts};

use super::{structural::ApplicableStyles, style::Style};

/// Chunk of text attached to a certain style
#[derive(Clone)]
pub struct StyledText {
    pub text: String,
    pub style: Style,
}

impl Default for StyledText {
    fn default() -> Self {
        StyledText {
            text: String::new(),
            style: Style::new(),
        }
    }
}

impl StyledText {

    pub fn new(text: String, style: Style) -> Self {
        StyledText { text, style }
    }

    pub fn apply_to_raw(&self) -> docx_rs::Run {
        let mut run = Run::new().add_text(&self.text);

        run = run.fonts(RunFonts::new().ascii(self.style.font.to_string()));
        run = run.size(self.style.size as usize);
        run = run.color(&self.style.font_color[1..]);
        if self.style.bold {
            run = run.bold();
        }
        if self.style.italic {
            run = run.italic();
        }
        if self.style.underline {
            // I'd love to find the fucking docx documentation to create an enum with underline types
            run = run.underline("single");
        }
        if self.style.highlight_color.is_some() {
            // I hate they ignore the # in the hex color so much it's unreal
            run = run.highlight(&self.style.highlight_color.as_ref().unwrap()[1..]);
        }

        run
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
