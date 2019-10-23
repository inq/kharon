#[derive(Debug)]
pub struct Command<A> {
    pub name: String,
    pub action: A,
}

impl<A> Command<A> {
    pub fn new(name: String, action: A) -> Self {
        Self { name, action }
    }
}
