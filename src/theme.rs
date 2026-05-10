use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            accent: Color::Cyan,
        }
    }
}