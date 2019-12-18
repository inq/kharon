use failure::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::io::AsyncRead;

use crate::term::Input;

pub(in crate::term) struct Stdin {
    stdin: futures::io::BufReader<dope::io::Stdin>,
    data: String,
}

impl futures::Stream for Stdin {
    type Item = Result<Input, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut buf = std::mem::MaybeUninit::<[u8; 32]>::uninit();

        let mut mut_ref = Pin::as_mut(&mut self);
        let len = futures::ready!(AsyncRead::poll_read(
            Pin::new(&mut mut_ref.stdin),
            cx,
            unsafe { buf.get_mut() },
        ))?;
        if len > 0 {
            match String::from_utf8(unsafe { buf.get_mut() }[..len].to_vec()) {
                Ok(parsed) => {
                    mut_ref.data.push_str(&parsed);
                    if let Some(input) = mut_ref.consume() {
                        return Poll::Ready(Some(Ok(input)));
                    }
                }
                Err(err) => {
                    return Poll::Ready(Some(Err(failure::Error::from(err))));
                }
            }
        }
        Poll::Pending
    }
}

impl Stdin {
    pub(in crate::term) fn new(
        reactor: dope::executor::reactor::Handle,
    ) -> Result<Self, failure::Error> {
        Ok(Self {
            stdin: dope::io::stdin(reactor)?,
            data: String::new(),
        })
    }

    fn consume(&mut self) -> Option<Input> {
        let (input, remaining) = Input::parse(&self.data);
        self.data = remaining;
        input
    }
}
