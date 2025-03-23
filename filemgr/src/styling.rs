use font_kit::source::SystemSource;

pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub size: u8,
    pub font: String,
    pub font_color: String,
    pub highlight_color: Option<String>,
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

fn check_hex(s: &str) -> Result<(), ()> {
    if s.starts_with('#') || (s.len() != 7) || s.chars().skip(1).all(|x| x.is_ascii_hexdigit()) {
        return Err(());
    }
    Ok(())
}

fn check_font(s: &str) -> Result<(), ()> {
    if SystemSource::new().select_family_by_name(s).is_err() {
        return Err(());
    }
    Ok(())
}