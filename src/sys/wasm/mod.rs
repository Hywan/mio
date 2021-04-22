#![allow(unused)]

pub(crate) mod wasi;

cfg_os_poll! {
    use crate::Token;
    use std::io;

    macro_rules! io_err {
        ($expr:expr) => {
            io::Error::new(io::ErrorKind::Other, $expr)
        }
    }

    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> Self {
            Self
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            f(io)
        }
    }

    mod selector;

    pub(crate) use self::selector::{event, Event, Events, Selector};

    cfg_net! {
        pub(crate) mod net;
        pub(crate) mod tcp;
        pub(crate) mod udp;
    }

    #[derive(Debug)]
    pub(crate) struct Waker;

    impl Waker {
        pub(crate) fn new(_: &Selector, _: Token) -> io::Result<Waker> {
            Ok(Self)
        }

        pub(crate) fn wake(&self) -> io::Result<()> {
            todo!("`Waker::wake`");
        }
    }
}
