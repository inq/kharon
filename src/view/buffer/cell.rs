use crate::view::buffer::Palette;

#[derive(Clone)]
pub struct Cell {
    chr: char,
    palette: Palette,
}

impl Cell {
    pub fn new(chr: char, palette: Palette) -> Self {
        Cell { chr, palette }
    }

    pub fn render(&self, current: Palette) -> (Palette, String) {
        (
            self.palette,
            format!("{}{}", self.palette.render(current), self.chr),
        )
    }
}
