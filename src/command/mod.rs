use common::Key;

macros::command_map!(
    "C-x C-c": ("exit", "Quit")
    "C-x C-w": ("save-buffer", "Save")
);

pub struct Handler {
    state: State,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            state: State::initial(),
        }
    }

    pub fn handle_key(&mut self, key: Key) -> Option<Action> {
        match self.state.handle_key(key) {
            Response::More(state) => {
                self.state = state;
                None
            }
            Response::Done(command) => {
                self.state = State::initial();
                Some(command.action)
            }
            Response::Empty => {
                println!("Invalid key!");
                self.state = State::initial();
                None
            }
        }
    }
}
