pub(crate) mod wasi {
    // Code is copied from `wasmer-wasi`.
    #![allow(non_camel_case_types)]

    use std::fmt;

    pub type __wasi_fd_t = u32;
    pub const __WASI_STDIN_FILENO: u32 = 0;
    pub const __WASI_STDOUT_FILENO: u32 = 1;
    pub const __WASI_STDERR_FILENO: u32 = 2;

    pub type __wasi_userdata_t = u64;

    pub type __wasi_eventtype_t = u8;
    pub const __WASI_EVENTTYPE_CLOCK: u8 = 0;
    pub const __WASI_EVENTTYPE_FD_READ: u8 = 1;
    pub const __WASI_EVENTTYPE_FD_WRITE: u8 = 2;

    pub type __wasi_clockid_t = u32;
    pub const __WASI_CLOCK_REALTIME: u32 = 0;
    pub const __WASI_CLOCK_MONOTONIC: u32 = 1;
    pub const __WASI_CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
    pub const __WASI_CLOCK_THREAD_CPUTIME_ID: u32 = 3;

    pub type __wasi_timestamp_t = u64;

    pub type __wasi_subclockflags_t = u16;
    pub const __WASI_SUBSCRIPTION_CLOCK_ABSTIME: u16 = 1 << 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct __wasi_subscription_clock_t {
        pub clock_id: __wasi_clockid_t,
        pub timeout: __wasi_timestamp_t,
        pub precision: __wasi_timestamp_t,
        pub flags: __wasi_subclockflags_t,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct __wasi_subscription_fs_readwrite_t {
        pub fd: __wasi_fd_t,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union __wasi_subscription_u {
        pub clock: __wasi_subscription_clock_t,
        pub fd_readwrite: __wasi_subscription_fs_readwrite_t,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __wasi_subscription_t {
        pub userdata: __wasi_userdata_t,
        pub type_: __wasi_eventtype_t,
        pub u: __wasi_subscription_u,
    }

    pub type __wasi_errno_t = u16;
    pub const __WASI_ESUCCESS: u16 = 0;

    pub type __wasi_filesize_t = u64;

    pub type __wasi_eventrwflags_t = u16;
    pub const __WASI_EVENT_FD_READWRITE_HANGUP: u16 = 1 << 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct __wasi_event_fd_readwrite_t {
        pub nbytes: __wasi_filesize_t,
        pub flags: __wasi_eventrwflags_t,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union __wasi_event_u {
        pub fd_readwrite: __wasi_event_fd_readwrite_t,
    }

    impl fmt::Debug for __wasi_event_u {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("__wasi_event_u")
                .field("fd_readwrite", unsafe { &self.fd_readwrite })
                .finish()
        }
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    pub struct __wasi_event_t {
        pub userdata: __wasi_userdata_t,
        pub error: __wasi_errno_t,
        pub type_: __wasi_eventtype_t,
        pub u: __wasi_event_u,
    }

    pub(crate) mod sys {
        #[link(wasm_import_module = "wasi_snapshot_preview1")]
        extern "C" {
            pub fn poll_oneoff(
                subscriptions: *const super::__wasi_subscription_t,
                events: *mut super::__wasi_event_t,
                subscriptions_len: u32,
                events_len: *mut u32,
            ) -> super::__wasi_errno_t;
        }
    }
}

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

    mod selector {
        use crate::{Token, Interest};
        use slab::Slab;
        use std::cell::RefCell;
        use std::convert::TryInto;
        use std::io;
        use std::num::TryFromIntError;
        use std::os::wasi::io::RawFd;
        #[cfg(debug_assertions)]
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        use std::time::Duration;
        use crate::sys::wasi::{
            __WASI_ESUCCESS,
            __WASI_EVENTTYPE_FD_READ,
            __WASI_EVENTTYPE_FD_WRITE,
            __wasi_errno_t,
            __wasi_event_fd_readwrite_t,
            __wasi_event_t,
            __wasi_event_u,
            __wasi_subscription_fs_readwrite_t,
            __wasi_subscription_t,
            __wasi_subscription_u,
            sys::poll_oneoff,
        };

        #[cfg(debug_assertions)]
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        pub(crate) struct Selector {
            #[cfg(debug_assertions)]
            id: usize,
            register: RefCell<Slab<(RawFd, Token, Interest)>>,
            #[cfg(debug_assertions)]
            has_waker: AtomicBool,
        }

        impl Selector {
            pub(crate) fn new() -> io::Result<Self> {
                Ok(Self {
                    #[cfg(debug_assertions)]
                    id: NEXT_ID.fetch_add(1, Ordering::Relax) + 1,
                    register: RefCell::new(Slab::new()),
                    #[cfg(debug_assertions)]
                    has_waker: AtomicBool::new(false),
                })
            }

            pub(crate) fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
                self
                    .register
                    .try_borrow_mut()
                    .map_err(|_| io_err!("Cannot borrow the register as mutable"))?
                    .insert((fd, token, interests));

                Ok(())
            }

            pub(crate) fn reregister(&self, _fd: RawFd, _token: Token, _interests: Interest) -> io::Result<()> {
                todo!("`Selector::reregister`");
            }

            pub(crate) fn deregister(&self, fd: RawFd) -> io::Result<()> {
                let mut register = self
                    .register
                    .try_borrow_mut()
                    .map_err(|_| io_err!("Cannot borrow the register as mutable"))?;

                let index = register
                    .iter()
                    .find_map(|(index, (current_fd, _token, _interests))| {
                        if current_fd == &fd {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| io_err!(format!("Cannot deregister the file descriptor `{:?}` because it is not registered", fd)))?;

                let _value = register.remove(index);

                Ok(())
            }

            #[cfg(debug_assertions)]
            pub(crate) fn id(&self) -> usize {
                self.id
            }

            pub(crate) fn select(&self, events: &mut Events, _timeout: Option<Duration>) -> io::Result<()> {
                let mut register = self
                    .register
                    .try_borrow_mut()
                    .map_err(|_| io_err!("Cannot borrow the register as mutable"))?;

                // Transform the items in the register into WASI subscriptions.
                let mut wasi_subscriptions = Vec::new();

                for (fd, token, interests) in register.drain() {
                    dbg!((&fd, &token, &interests));

                    wasi_subscriptions.push(__wasi_subscription_t {
                        userdata: Into::<usize>::into(token) as u64,
                        type_: if interests.is_readable() {
                            __WASI_EVENTTYPE_FD_READ
                        } else if interests.is_writable() {
                            __WASI_EVENTTYPE_FD_WRITE
                        } else {
                            return Err(io_err!(format!("Interest for file descriptor `{:?}` and token `{:?}` is not supported", fd, token)));
                        },
                        u: __wasi_subscription_u {
                            fd_readwrite: __wasi_subscription_fs_readwrite_t {
                                fd,
                            }
                        }
                    });
                }

                // Prepare empty events to be filled by `poll_oneoff`.
                let mut wasi_events = vec![
                    __wasi_event_t {
                        userdata: 0,
                        error: 0,
                        type_: 0,
                        u: __wasi_event_u {
                            fd_readwrite: __wasi_event_fd_readwrite_t {
                                nbytes: 0,
                                flags: 0,
                            }
                        }
                    };
                    wasi_subscriptions.len()
                ];

                let mut wasi_events_len: u32 = 0;

                // Let's call the `poll_oneoff` syscall.
                let result = unsafe { poll_oneoff(
                    wasi_subscriptions.as_ptr(),
                    wasi_events.as_mut_ptr(),
                    wasi_subscriptions.len() as u32,
                    &mut wasi_events_len as *mut _,
                ) };

                if result != __WASI_ESUCCESS {
                    return Err(io_err!(format!("Calling `poll_oneoff` returned `{:?}` (i.e. not a success)", result)));
                }

                if wasi_events_len != wasi_events.len() as u32 {
                    return Err(io_err!(format!("Unexpected number of events (expected `{:?}`, received `{:?}`)", wasi_events.len(), wasi_events_len)));
                }

                *events = wasi_events
                    .iter()
                    .map(|wasi_event| {
                        dbg!(wasi_event);

                        Ok(Event {
                            wasi_errno: wasi_event.error,
                            interest: match wasi_event.type_ {
                                __WASI_EVENTTYPE_FD_READ => Interest::READABLE,
                                __WASI_EVENTTYPE_FD_WRITE => Interest::WRITABLE,
                                ty => return Err(io_err!(format!("Invalid `__wasi_event_t.type_` value `{:?}`", ty))),
                            },
                            token: Token(wasi_event.userdata.try_into().map_err(|e: TryFromIntError| io_err!(e.to_string()))?),
                        })
                    })
                    .collect::<io::Result<Events>>()?;

                Ok(())
            }

            pub(crate) fn try_clone(&self) -> io::Result<Self> {
                todo!("`Selector::try_clone`");
            }

            #[cfg(debug_assertions)]
            pub(crate) fn register_waker(&self) -> bool {
                self.has_waker.swap(true, Ordering::AcqRel)
            }
        }

        #[derive(Clone)]
        pub(crate) struct Event {
            wasi_errno: __wasi_errno_t,
            interest: Interest,
            token: Token,
        }
        pub(crate) type Events = Vec<Event>;

        pub(crate) mod event {
            use crate::Token;
            use crate::sys::Event;
            use std::fmt;

            pub(crate) fn token(event: &Event) -> Token {
                event.token
            }

            pub(crate) fn is_readable(event: &Event) -> bool {
                event.interest.is_readable()
            }

            pub(crate) fn is_writable(event: &Event) -> bool {
                event.interest.is_writable()
            }

            pub(crate) fn is_error(_event: &Event) -> bool {
                todo!("`_event::is_error`");
            }

            pub(crate) fn is_read_closed(_event: &Event) -> bool {
                todo!("`_event::is_read_closed`");
            }

            pub(crate) fn is_write_closed(_event: &Event) -> bool {
                todo!("`_event::is_write_closed`");
            }

            pub(crate) fn is_priority(_event: &Event) -> bool {
                todo!("`event::is_priority`");
            }

            pub(crate) fn is_aio(event: &Event) -> bool {
                event.interest.is_aio()
            }

            pub(crate) fn is_lio(event: &Event) -> bool {
                event.interest.is_lio()
            }

            pub(crate) fn debug_details(_formatter: &mut fmt::Formatter<'_>, _event: &Event) -> fmt::Result {
                todo!("`event::debug_details`");
            }
        }
    }

    pub(crate) use self::selector::{event, Event, Events, Selector};

    cfg_net! {
        pub(crate) mod net {
            use crate::sys::tcp::TcpSocket;
            use std::io;
            use std::net::ToSocketAddrs;
            use std::os::wasi::io::{AsRawFd, RawFd};
            use std::time::Duration;

            pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, Shutdown, SocketAddrV4, SocketAddrV6};
            pub use wasio::sys::{socket_create, socket_bind, socket_listen};

            #[derive(Debug)]
            #[allow(unused)]
            pub struct Incoming<'a> {
                listener: &'a TcpListener,
            }

            impl<'a> Iterator for Incoming<'a> {
                type Item = io::Result<TcpStream>;

                fn next(&mut self) -> Option<io::Result<TcpStream>> {
                    todo!("Incoming::next`");
                }
            }

            #[derive(Debug)]
            pub struct TcpListener {
                socket: TcpSocket,
            }

            impl TcpListener {
                pub(in crate::sys) fn new(socket: TcpSocket) -> Self {
                    Self { socket }
                }

                pub fn bind<A: ToSocketAddrs>(_addr: A) -> io::Result<Self> {
                    todo!("`TcpListener::bind`");
                }

                pub fn local_addr(&self) -> io::Result<SocketAddr> {
                    todo!("`TcpListener::local_addr`");
                }

                pub fn try_clone(&self) -> io::Result<Self> {
                    todo!("`TcpListener::try_clone`");
                }

                pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
                    todo!("`TcpListener::accept`");
                }

                pub fn incoming(&self) -> Incoming<'_> {
                    todo!("`TcpListener::incoming`");
                }

                pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> {
                    todo!("`TcpListener::set_ttl`");
                }

                pub fn ttl(&self) -> io::Result<u32> {
                    todo!("`TcpListener::ttl`");
                }

                pub fn take_error(&self) -> io::Result<Option<io::Error>> {
                    todo!("`TcpListener::take_error`");
                }

                pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
                    todo!("`TcpListener::set_nonblocking`");
                }
            }

            impl AsRawFd for TcpListener {
                fn as_raw_fd(&self) -> RawFd {
                    self.socket.into()
                }
            }

            #[derive(Debug)]
            pub struct TcpStream;

            impl TcpStream {
                pub fn connect<A: ToSocketAddrs>(_addr: A) -> io::Result<Self> {
                    todo!("`TcpStream::connect`");
                }

                pub fn connect_timeout(_addr: &SocketAddr, _timeout: Duration) -> io::Result<Self> {
                    todo!("`TcpStream::connect_timeout`");
                }

                pub fn peer_addr(&self) -> io::Result<SocketAddr> {
                    todo!("`TcpStream::peer_addr`");
                }

                pub fn local_addr(&self) -> io::Result<SocketAddr> {
                    todo!("`TcpStream::local_addr`");
                }

                pub fn shutdown(&self, _how: Shutdown) -> io::Result<()> {
                    todo!("`TcpStream::shutdown`");
                }

                pub fn try_clone(&self) -> io::Result<Self> {
                    todo!("`TcpStream::try_clone`");
                }

                pub fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
                    todo!("`TcpStream::set_read_timeout`");
                }

                pub fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
                    todo!("`TcpStream::set_write_timeout`");
                }

                pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
                    todo!("`TcpStream::read_timeout`");
                }

                pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
                    todo!("`TcpStream::write_timeout`");
                }

                pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> {
                    todo!("`TcpStream::seek`");
                }

                pub fn set_nodelay(&self, _nodelay: bool) -> io::Result<()> {
                    todo!("`TcpStream::set_nodelay`");
                }

                pub fn nodelay(&self) -> io::Result<bool> {
                    todo!("`TcpStream::nodelay`");
                }

                pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> {
                    todo!("`TcpStream::set_ttl`");
                }

                pub fn ttl(&self) -> io::Result<u32> {
                    todo!("`TcpStream::ttl`");
                }

                pub fn take_error(&self) -> io::Result<Option<io::Error>> {
                    todo!("`TcpStream::take_error`");
                }

                pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
                    todo!("`TcpStream::set_nonblocking`");
                }
            }

            impl io::Read for TcpStream {
                fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                    todo!("`TcpStream::read`");
                }
            }

            impl io::Read for &TcpStream {
                fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                    todo!("`TcpStream::read`");
                }
            }

            impl io::Write for TcpStream {
                fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                    todo!("`TcpStream::write`");
                }

                fn flush(&mut self) -> io::Result<()> {
                    todo!("`TcpStream::flush`");
                }
            }

            impl io::Write for &TcpStream {
                fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                    todo!("`TcpStream::write`");
                }

                fn flush(&mut self) -> io::Result<()> {
                    todo!("`TcpStream::flush`");
                }
            }

            impl AsRawFd for TcpStream {
                fn as_raw_fd(&self) -> RawFd {
                    todo!("`TcpStream::as_raw_fd`");
                }
            }
        }

        pub(crate) mod tcp {
            use crate::net::TcpKeepalive;
            use crate::sys::net::{socket_create, socket_bind, socket_listen};
            use crate::sys::wasi::__wasi_fd_t;
            use std::io;
            use std::net::SocketAddr;
            use std::time::Duration;
            use wasio::types::{AF_INET, AF_INET6, SOCK_STREAM, SockaddrIn};

            pub use crate::sys::net::{TcpListener, TcpStream};
            pub type TcpSocket = __wasi_fd_t;

            pub fn new_v4_socket() -> io::Result<TcpSocket> {
                let mut fd: __wasi_fd_t = 0;
                let err = unsafe { socket_create(&mut fd, AF_INET, SOCK_STREAM, 0) };

                if err != 0 {
                    return Err(io_err!(format!("`tcp::socket_create` failed with `{}`", err)));
                }

                Ok(fd)
            }

            pub fn new_v6_socket() -> io::Result<TcpSocket> {
                let mut fd: __wasi_fd_t = 0;
                let err = unsafe { socket_create(&mut fd, AF_INET6, SOCK_STREAM, 0) };

                if err != 0 {
                    return Err(io_err!(format!("`tcp::socket_create` failed with `{}`", err)));
                }

                Ok(fd)
            }

            // IPv6 isn't supported for the moment.
            pub fn bind(socket: TcpSocket, addr: SocketAddr) -> io::Result<()> {
                let err = match addr {
                    SocketAddr::V4(v4) => {
                        let addr = SockaddrIn {
                            sin_family: AF_INET as _,
                            sin_port: v4.port().to_be(),
                            sin_addr: v4.ip().octets(),
                            sin_zero: [0; 8],
                        };

                        unsafe {
                            socket_bind(
                                socket,
                                &addr as *const _ as *const u8,
                                std::mem::size_of::<SockaddrIn>() as u32,
                            )
                        }
                    },
                    SocketAddr::V6(_v6) => unimplemented!("`tcp::bind` with IPv6`"),
                };

                if err != 0 {
                    return Err(io_err!(format!("`tcp::socket_bind` failed with `{}`", err)));
                }

                Ok(())
            }

            pub fn connect(_socket: TcpSocket, _addr: SocketAddr) -> io::Result<TcpStream> {
                todo!("`tcp::connect`");
            }

            // `backlog` isn't used for the moment.
            pub fn listen(socket: TcpSocket, _backlog: u32) -> io::Result<TcpListener> {
                let err = unsafe { socket_listen(socket) };

                if err != 0 {
                    return Err(io_err!(format!("`tcp::socket_listen` failed with `{}`", err)));
                }

                Ok(TcpListener::new(socket))
            }

            pub fn accept(_listener: &TcpListener) -> io::Result<(TcpStream, SocketAddr)> {
                todo!("`tcp::accept`");
            }

            pub fn set_reuseaddr(_socket: TcpSocket, _reuseaddr: bool) -> io::Result<()> {
                dbg!("`tcp::set_reuseaddr`");
                Ok(())
            }

            pub fn get_reuseaddr(_socket: TcpSocket) -> io::Result<bool> {
                todo!("`tcp::get_reuseaddr`");
            }

            pub fn set_linger(_socket: TcpSocket, _duration: Option<Duration>) -> io::Result<()> {
                todo!("`tcp::set_linger`");
            }

            pub fn get_linger(_socket: TcpSocket) -> io::Result<Option<Duration>> {
                todo!("`tcp::get_linger`");
            }

            pub fn set_recv_buffer_size(_socket: TcpSocket, _size: u32) -> io::Result<()> {
                todo!("`tcp::set_recv_buffer_size`");
            }

            pub fn get_recv_buffer_size(_socket: TcpSocket) -> io::Result<u32> {
                todo!("`tcp::get_recv_buffer_size`");
            }

            pub fn set_send_buffer_size(_socket: TcpSocket, _size: u32) -> io::Result<()> {
                todo!("`tcp::set_send_buffer_size`");
            }

            pub fn get_send_buffer_size(_socket: TcpSocket) -> io::Result<u32> {
                todo!("`tcp::get_send_buffer_size`");
            }

            pub fn set_keepalive(_socket: TcpSocket, _keepalive: bool) -> io::Result<()> {
                todo!("`tcp::set_keepalive`");
            }

            pub fn get_keepalive(_socket: TcpSocket) -> io::Result<bool> {
                todo!("`tcp::get_keepalive`");
            }

            pub fn set_keepalive_params(_socket: TcpSocket, _keepalive: TcpKeepalive) -> io::Result<()> {
                todo!("`tcp::set_keealive_params`");
            }

            #[allow(unused)]
            pub fn get_keepalive_params(_socket: TcpSocket) -> io::Result<TcpKeepalive> {
                todo!("`tcp::get_keealive_params`");
            }

            pub fn get_keepalive_time(_socket: TcpSocket) -> io::Result<Option<Duration>> {
                todo!("`tcp::get_keepalive_time`");
            }

            pub fn get_localaddr(_socket: TcpSocket) -> io::Result<SocketAddr> {
                todo!("`tcp::get_localaddr`");
            }

            pub fn close(_socket: TcpSocket) {
                todo!("`tcp::close`");
            }
        }

        pub(crate) mod udp {
            use crate::sys::net::{Ipv4Addr, Ipv6Addr};
            use std::io;
            use std::net::{SocketAddr, ToSocketAddrs};
            use std::os::wasi::io::{AsRawFd, RawFd};
            use std::time::Duration;

            #[derive(Debug)]
            pub struct UdpSocket;

            impl UdpSocket {
                pub fn bind<A: ToSocketAddrs>(_addr: A) -> io::Result<Self> {
                    todo!("`UdpSocket::bind`");
                }

                pub fn recv_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
                    todo!("`UdpSocket::recv_from`");
                }

                pub fn peek_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
                    todo!("`UdpSocket::recv_from`");
                }

                pub fn send_to<A: ToSocketAddrs>(&self, _buf: &[u8], _addr: A) -> io::Result<usize> {
                    todo!("`UpdSocket::send_to`");
                }

                pub fn peer_addr(&self) -> io::Result<SocketAddr> {
                    todo!("`UdpSocket::peer_addr`");
                }

                pub fn local_addr(&self) -> io::Result<SocketAddr> {
                    todo!("`UdpSocket::local_addr`");
                }

                pub fn try_clone(&self) -> io::Result<Self> {
                    todo!("`UdpSocket::try_clone`");
                }

                pub fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
                    todo!("`UdpSocket::set_read_timeout`");
                }

                pub fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
                    todo!("`UdpSocket::set_write_timeout`");
                }

                pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
                    todo!("`UdpSocket::read_timeout`");
                }

                pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
                    todo!("`UdpSocket::write_timeout`");
                }

                pub fn set_broadcast(&self, _broadcast: bool) -> io::Result<()> {
                    todo!("`UdpSocket::set_broadcast`");
                }

                pub fn broadcast(&self) -> io::Result<bool> {
                    todo!("`UdpSocket::broadcast`");
                }

                pub fn set_multicast_loop_v4(&self, _multicast_loop_v4: bool) -> io::Result<()> {
                    todo!("`UdpSocket::set_multicast_loop_v4`");
                }

                pub fn multicast_loop_v4(&self) -> io::Result<bool> {
                    todo!("`UdpSocket::multicast_loop_v4`");
                }

                pub fn set_multicast_ttl_v4(&self, _multicast_ttl_v4: u32) -> io::Result<()> {
                    todo!("`UdpSocket::set_multicast_ttl_v4`");
                }

                pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
                    todo!("`UdpSocket::multicast_ttl_v4`");
                }

                pub fn set_multicast_loop_v6(&self, _multicast_loop_v6: bool) -> io::Result<()> {
                    todo!("`UdpSocket::set_multicast_loop_v6`");
                }

                pub fn multicast_loop_v6(&self) -> io::Result<bool> {
                    todo!("`UdpSocket::multicast_loop_v6`");
                }

                pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> {
                    todo!("`UdpSocket::set_ttl`");
                }

                pub fn ttl(&self) -> io::Result<u32> {
                    todo!("`UdpSocket::ttl`");
                }

                pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
                    todo!("`UdpSocket::join_multicast_v4`");
                }

                pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
                    todo!("`UdpSocket::join_multicast_v6`");
                }

                pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
                    todo!("`UdpSocket::leave_multicast_v4`");
                }

                pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
                    todo!("`UdpSocket::leave_multicast_v6`");
                }

                pub fn take_error(&self) -> io::Result<Option<io::Error>> {
                    todo!("`UdpSocket::take_error`");
                }

                pub fn connect<A: ToSocketAddrs>(&self, _addr: A) -> io::Result<()> {
                    todo!("`UdpSocket::connect<_A: ToSocketAddrs>`");
                }

                pub fn send(&self, _buf: &[u8]) -> io::Result<usize> {
                    todo!("`UdpSocket::send`");
                }

                pub fn recv(&self, _buf: &mut [u8]) -> io::Result<usize> {
                    todo!("`UdpSocket::recv`");
                }

                pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> {
                    todo!("`UdpSocket::peek`");
                }

                pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
                    todo!("`UdpSocket::set_nonblocking`");
                }
            }

            impl AsRawFd for UdpSocket {
                fn as_raw_fd(&self) -> RawFd {
                    todo!("`UdpSocket::as_raw_fd`");
                }
            }

            pub fn bind(_addr: SocketAddr) -> io::Result<UdpSocket> {
                todo!("`udp::bind`");
            }

            pub fn only_v6(_socket: &UdpSocket) -> io::Result<bool> {
                todo!("`udp::only_v6`");
            }
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
