mod input;

pub mod reader;
pub use input::Input;

pub use reader::build as reader;
// mod sys;

/*

pub struct Term {
    reader: reader::Reader,
    // kqueue: sys::Kqueue,
}

impl Term {
    pub fn new() -> Result<Self, failure::Error> {
        Ok(Self {
            reader: reader::Reader::new(),
            // kqueue: sys::Kqueue::new()?,
        })
    }


    pub async fn read_event(&mut self) -> Result<Input, failure::Error> {
        /*
        use std::ops::{Generator, GeneratorState};
        use std::pin::Pin;

        match Pin::new(&mut self.kqueue.gen).resume() {
            GeneratorState::Yielded(event) => match event {
                sys::Event::Stdin => await!(&mut self.reader),
                sys::Event::Timer => Ok(Input::Timer),
                sys::Event::Sigwinch => Ok(Input::Sigwinch),
                _ => Ok(Input::Dummy), // TODO: Implement me
            },
            GeneratorState::Complete(_) => Ok(Input::Dummy),
        }*/
    }
}
*/
