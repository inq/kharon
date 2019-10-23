use crate::view::buffer::{Cell, Palette};

#[derive(Default, Clone)]
pub struct Line {
    cells: Vec<Cell>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn erase(&mut self, width: usize, palette: Palette) -> &mut Self {
        let empty_cell = Cell::new(' ', palette);
        self.cells.resize(width, empty_cell);
        self
    }

    pub fn render(&self, x: usize, y: usize, mut current: Palette) -> (Palette, String) {
        use termion::cursor;
        let mut res = vec![format!("{}", cursor::Goto(x as u16, y as u16))];
        for cell in self.cells.iter() {
            let (palette, rendered) = cell.render(current);
            current = palette;
            res.push(rendered);
        }
        (current, res.join(""))
    }
}
