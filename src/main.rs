#![feature(maybe_uninit_ref)]
mod logger;

use dope::executor::{self, Executor};

#[macro_use]
extern crate failure;
extern crate macros;

mod command;
mod term;
mod view;

async fn main_async(executor: &executor::Handle) -> Result<(), failure::Error> {
    use std::io::Write;
    use term::Input;
    use termion::raw::IntoRawMode;

    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut view = {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        let screen = termion::screen::AlternateScreen::from(stdout);
        view::View::new(screen)?
    };
    view.render()?;
    let mut cmd = command::Handler::new();

    use futures::StreamExt;
    let mut reader = term::reader(executor.reactor()?)?;
    'outer: loop {
        match reader.next().await.unwrap()? {
            Input::Timer => {
                println!("TIMER");
            }
            Input::Keyboard(key) => {
                if let Some(action) = cmd.handle_key(key) {
                    write!(view.output, "{:?}", action).unwrap();
                    view.flush()?;
                    if let command::Action::Quit = action {
                        break 'outer;
                    }
                }
            }
            res => {
                view.resize()?; // TODO: Move to Sigwinch
                write!(view.output, "{:?}", res).unwrap();
                view.flush()?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), failure::Error> {
    color_backtrace::install();
    //log::set_logger(&logger::Logger).unwrap();
    //log::set_max_level(log::LevelFilter::Debug);

    let executor = Executor::new()?;
    let handle = executor.handle();
    executor.block_on(main_async(&handle)).unwrap()?;

    Ok(())
}
