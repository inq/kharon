use libc;
use std::boxed::Box;
use std::os::unix::io::RawFd;
use std::pin::Pin;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "kqueue returned -1")]
    Kqueue,
    #[fail(display = "kevent returned -1")]
    Kevent,
}

#[derive(Debug)]
#[repr(i32)]
pub enum Event {
    Stdin = libc::STDIN_FILENO,
    Stdout = libc::STDOUT_FILENO,
    Sigwinch = libc::SIGWINCH,
    Timer = 0xbeef,
}

struct Builder {
    kq: RawFd,
    changes: Vec<libc::kevent>,
    events: Vec<libc::kevent>,
}

impl Builder {
    pub fn new() -> Result<Builder, Error> {
        let res = unsafe { libc::kqueue() };
        if res == -1 {
            return Err(Error::Kqueue);
        }
        Ok(Builder {
            kq: res,
            changes: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
        })
    }

    fn add_event(&mut self, ident: i32, filter: i16, aux: isize) {
        self.changes.push(libc::kevent {
            ident: ident as libc::uintptr_t,
            filter,
            flags: libc::EV_ADD,
            fflags: 0,
            data: aux,
            udata: ::std::ptr::null_mut(),
        })
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.add_event(Event::Stdin as i32, libc::EVFILT_READ, 0);
        self.add_event(Event::Stdout as i32, libc::EVFILT_WRITE, 0);
        self.add_event(Event::Sigwinch as i32, libc::EVFILT_SIGNAL, 0);
        self.add_event(Event::Timer as i32, libc::EVFILT_TIMER, 100);
        let res = unsafe {
            libc::kevent(
                self.kq,
                self.changes.as_ptr(),
                self.changes.len() as i32,
                ::std::ptr::null_mut(),
                0,
                &libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
            )
        };
        if res == -1 {
            Err(Error::Kevent)
        } else {
            Ok(())
        }
    }

    fn fetch_events(&mut self) -> Result<(), Error> {
        unsafe {
            let res = libc::kevent(
                self.kq,
                std::ptr::null(),
                0,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                &libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
            );
            if res == -1 {
                return Err(Error::Kevent);
            } else {
                self.events.set_len(res as usize);
            }
        }
        Ok(())
    }

    fn make_gen(
        mut self,
    ) -> impl std::ops::Generator<Return = Result<(), Error>, Yield = Event> + 'static {
        static move || loop {
            self.fetch_events()?;

            for e in &self.events {
                let event = match e.ident {
                    event if event == Event::Stdin as usize => Some(Event::Stdin),
                    event if event == Event::Stdout as usize => Some(Event::Stdout),
                    event if event == Event::Sigwinch as usize => Some(Event::Sigwinch),
                    event if event == Event::Timer as usize => Some(Event::Timer),
                    _ => None,
                };
                if let Some(event) = event {
                    yield event;
                }
            }
        }
    }
}

pub struct Kqueue {
    pub gen: Pin<Box<dyn std::ops::Generator<Return = Result<(), Error>, Yield = Event>>>,
}

impl Kqueue {
    pub fn new() -> Result<Kqueue, Error> {
        let mut builder = Builder::new()?;
        builder.init()?;
        Ok(Kqueue {
            gen: Box::pin(builder.make_gen()),
        })
    }
}
