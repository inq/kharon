mod cell;
mod line;
mod palette;

pub use cell::Cell;
pub use line::Line;
pub use palette::Palette;
pub use termion::color::Rgb as Color;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<Line>,
}

impl Buffer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn erase(&mut self, width: usize, height: usize, palette: Palette) -> &mut Self {
        self.lines.resize_with(height, Line::new);
        for line in self.lines.iter_mut() {
            line.erase(width, palette);
        }
        self
    }

    pub fn render(&self, x: usize, y: usize, mut current: Palette) -> (Palette, String) {
        let mut res = vec![];
        for (i, line) in self.lines.iter().enumerate() {
            let (palette, rendered) = line.render(x, y + i, current);
            current = palette;
            res.push(rendered);
        }
        (current, res.join(""))
    }
}
