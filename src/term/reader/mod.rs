mod stdin;

use crate::term::Input;
use stdin::Stdin;

pub fn build(
    reactor: dope::executor::reactor::Handle,
) -> Result<impl futures::Stream<Item = Result<Input, failure::Error>>, failure::Error> {
    use futures::StreamExt;

    let stdin = Stdin::new(reactor.clone())?;
    let timer = dope::timer::Timer::start(reactor.clone(), chrono::Duration::seconds(1))?
        .map(|()| Ok(Input::Timer));
    Ok(futures::stream::select(timer, stdin))
}
