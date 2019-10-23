use common::Key;

#[derive(Debug)]
pub enum Input {
    Keyboard(Key),
    Single(usize),
    Pair(usize, usize),
    Timer,
    Sigwinch,
}

impl Input {
    pub fn parse(input: &str) -> (Option<Input>, String) {
        let mut it = input.chars();
        let res = match it.next() {
            Some('\x1b') => match it.next() {
                Some('[') => Input::try_csi(&mut it),
                Some(c) => Some(Input::Keyboard(Key::Meta(c as char))),
                None => Some(Input::Keyboard(Key::Esc)),
            },
            Some(c) => Some(Input::Keyboard(Key::from_char(c))),
            _ => None,
        };
        (res.map(Input::normalize), it.collect())
    }

    #[inline]
    fn normalize(self) -> Input {
        match self {
            Input::Single(1) => Input::Keyboard(Key::Home),
            Input::Single(4) => Input::Keyboard(Key::End),
            Input::Single(5) => Input::Keyboard(Key::PageUp),
            Input::Single(6) => Input::Keyboard(Key::PageDown),
            etc => etc,
        }
    }

    #[inline]
    fn try_int(it: &mut std::str::Chars, seed: usize) -> Option<(usize, char)> {
        let mut res = seed;
        for c in it {
            if c >= '0' && c <= '9' {
                res = res * 10 + c.to_digit(10).unwrap() as usize;
            } else {
                return Some((res, c));
            }
        }
        None
    }

    #[inline]
    fn try_int_pair(it: &mut std::str::Chars, seed: usize) -> Option<Input> {
        match Input::try_int(it, seed) {
            Some((y, ';')) => {
                // x;yR
                if let Some((x, 'R')) = Input::try_int(it, seed) {
                    Some(Input::Pair(x, y))
                } else {
                    None
                }
            }
            Some((n, '~')) => {
                // n~
                Some(Input::Single(n))
            }
            _ => None,
        }
    }

    #[inline]
    fn try_csi(s: &mut std::str::Chars) -> Option<Input> {
        if let Some(c) = s.next() {
            match c {
                '0'..='9' => Input::try_int_pair(s, c.to_digit(10).unwrap() as usize),
                'A' => Some(Input::Keyboard(Key::Up)),
                'B' => Some(Input::Keyboard(Key::Down)),
                'C' => Some(Input::Keyboard(Key::Right)),
                'D' => Some(Input::Keyboard(Key::Left)),
                'H' => Some(Input::Keyboard(Key::Home)),
                'F' => Some(Input::Keyboard(Key::End)),
                _ => None,
            }
        } else {
            Some(Input::Keyboard(Key::Esc))
        }
    }
}
