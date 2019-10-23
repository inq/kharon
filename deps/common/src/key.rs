use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Meta(char),
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Esc,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid input string: {}", _0)]
    InvalidStr(String),
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(value: &str) -> Result<Key, Error> {
        let chars: Vec<_> = value.chars().collect();
        match chars.as_slice() {
            ['C', '-', key] => Ok(Key::Ctrl(*key)),
            ['M', '-', key] => Ok(Key::Meta(*key)),
            [key] => Ok(Key::Char(*key)),
            _ => Err(Error::InvalidStr(value.to_string())),
        }
    }
}

impl Key {
    // CR = Ctrl('m')
    // LF = Ctrl('j')
    // Delete = Char('\x7f')
    pub fn from_char(c: char) -> Key {
        if c as u32 <= 26 {
            Key::Ctrl((c as u8 + b'a' - 1) as char)
        } else {
            Key::Char(c)
        }
    }

    pub fn identifier(self) -> String {
        match self {
            Key::Char(c) => format!("00{}", c),
            Key::Ctrl(c) => format!("01{}", c),
            Key::Meta(c) => format!("02{}", c),
            Key::Up => "03".to_string(),
            Key::Down => "04".to_string(),
            Key::Left => "05".to_string(),
            Key::Right => "06".to_string(),
            Key::Home => "07".to_string(),
            Key::End => "08".to_string(),
            Key::PageUp => "09".to_string(),
            Key::PageDown => "10".to_string(),
            Key::Esc => "11".to_string(),
        }
    }
}
