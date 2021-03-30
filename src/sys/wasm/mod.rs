cfg_os_poll! {
    use std::io;
    use crate::Token;

    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> Self {
            Self
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            todo!("`IoSourceState::do_io`");
        }
    }

    mod selector {
        use std::io;
        use crate::{Token, Interest};
        use crate::sys::Events;
        use std::time::Duration;
        use std::os::wasi::io::RawFd;

        pub(crate) struct Selector;

        impl Selector {
            pub(crate) fn new() -> io::Result<Self> {
                todo!("`Selector::new`");
            }

            pub(crate) fn register(&self, _fd: RawFd, _token: Token, _interests: Interest) -> io::Result<()> {
                todo!("`Selector::register`");
            }

            pub(crate) fn reregister(&self, _fd: RawFd, _token: Token, _interests: Interest) -> io::Result<()> {
                todo!("`Selector::reregister`");
            }

            pub(crate) fn deregister(&self, _fd: RawFd) -> io::Result<()> {
                todo!("`Selector::deregister`");
            }

            pub(crate) fn id(&self) -> usize {
                todo!("`Selector::id`");
            }

            pub(crate) fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
                todo!("``Selector::select");
            }

            pub(crate) fn try_clone(&self) -> io::Result<Self> {
                todo!("`Selector::try_clone`");
            }

            #[cfg(debug_assertions)]
            pub(crate) fn register_waker(&self) -> bool {
                todo!("`Selector::register_waker`");
            }
        }

        #[derive(Clone)]
        pub(crate) struct Event {
            foo: usize,
        };
        pub(crate) type Events = Vec<Event>;

        pub(crate) mod event {
            use crate::sys::Event;
            use crate::Token;

            pub fn token(_event: &Event) -> Token {
                todo!("`_event::token`");
            }

            pub fn is_readable(_event: &Event) -> bool {
                todo!("`_event::is_readable`");
            }

            pub fn is_writable(_event: &Event) -> bool {
                todo!("`_event::is_writable`");
            }

            pub fn is_error(_event: &Event) -> bool {
                todo!("`_event::is_error`");
            }

            pub fn is_read_closed(_event: &Event) -> bool {
                todo!("`_event::is_read_closed`");
            }

            pub fn is_write_closed(_event: &Event) -> bool {
                todo!("`_event::is_write_closed`");
            }

            pub fn is_priority(_event: &Event) -> bool {
                todo!("`event::is_priority`");
            }
        }
    }

    pub(crate) use self::selector::{event, Event, Events, Selector};

    cfg_net! {
        pub(crate) mod net {
            pub struct TcpListener;
            pub struct TcpStream;

            pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, Shutdown, SocketAddrV4, SocketAddrV6};
        }

        pub(crate) mod tcp {
            use crate::sys::net;
            use std::io;
            use std::net::SocketAddr;

            pub fn accept(_listener: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
                todo!("`tcp::accept`");
            }
        }

        pub(crate) mod udp {
        }
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
