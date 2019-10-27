mod buffer;
use std::io::Write;

pub struct View<T: Write> {
    pub output: T,
    width: u16,
    height: u16,
}

impl<T: Write> View<T> {
    pub fn new(output: T) -> std::io::Result<Self> {
        let (width, height) = termion::terminal_size()?;
        Ok(View {
            output,
            width,
            height,
        })
    }

    pub fn resize(&mut self) -> std::io::Result<String> {
        let (width, height) = termion::terminal_size()?;
        self.width = width;
        self.height = height;
        self.render()
    }

    pub fn render(&mut self) -> std::io::Result<String> {
        use buffer::{Buffer, Palette};
        use termion::{color, cursor};

        let mut buffer = Buffer::new();
        let bg = color::Rgb(0xdd, 0xcc, 0xcc);
        let fg = color::Rgb(0x33, 0x00, 0x00);
        let palette = Palette::new(bg, fg);
        let inverted = Palette::new(fg, bg);
        buffer.erase(self.width as usize, self.height as usize, palette);
        let (_brush, rendered) = buffer.render(1, 1, inverted);
        Ok(format!("{}{}", rendered, cursor::Goto(3, 3),))
    }
}
