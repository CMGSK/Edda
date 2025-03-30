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


        self.style = new_style.unwrap_or(rollback);
    }
}
