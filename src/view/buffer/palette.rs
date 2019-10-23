use crate::view::buffer::Color;

#[derive(Clone, Copy)]
pub struct Palette {
    bg: Color,
    fg: Color,
}

impl Palette {
    pub fn new(bg: Color, fg: Color) -> Self {
        Self { bg, fg }
    }

    pub fn render(self, current: Self) -> String {
        use termion::color;

        format!(
            "{}{}",
            if self.bg != current.bg {
                format!("{}", color::Bg(self.bg))
            } else {
                String::new()
            },
            if self.fg != current.fg {
                format!("{}", color::Fg(self.fg))
            } else {
                String::new()
            },
        )
    }
}
