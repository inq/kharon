#![feature(maybe_uninit_ref)]
mod logger;

use dope::executor::{self, Executor};

#[macro_use]
extern crate failure;
extern crate macros;

mod buffer;
mod command;
mod term;
mod view;

async fn main_async(executor: &executor::Handle) -> Result<(), failure::Error> {
    use term::Input;
    use termion::raw::IntoRawMode;

    let mut view = {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        let screen = termion::screen::AlternateScreen::from(stdout);
        view::View::new(screen)?
    };
    let mut cmd = command::Handler::new();

    use futures::io::AsyncWriteExt;
    use futures::StreamExt;
    let mut reader = term::reader(executor.reactor()?)?;
    let mut writer = dope::io::stdout(executor.reactor()?)?;

    let res = view.render()?;
    writer.write(res.as_bytes()).await?;
    writer.flush().await?;
    log::warn!("W R I T E");

    'outer: loop {
        match reader.next().await.unwrap()? {
            Input::Timer => {
                println!("TIMER");
            }
            Input::Keyboard(key) => {
                if let Some(action) = cmd.handle_key(key) {
                    writer.write(format!("{:?}", action).as_bytes()).await?;
                    if let command::Action::Quit = action {
                        break 'outer;
                    }
                }
            }
            Input::Sigwinch => {
                log::warn!("SIGWINCH!");
                writer.write(view.resize()?.as_bytes()).await?;
            }
            res => {
                writer.write(format!("{:?}", res).as_bytes()).await?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), failure::Error> {
    color_backtrace::install();
    // log::set_logger(&logger::Logger).unwrap();
    // log::set_max_level(log::LevelFilter::Debug);

    let executor = Executor::new()?;
    let handle = executor.handle();
    executor.block_on(main_async(&handle)).unwrap()?;

    Ok(())
}
