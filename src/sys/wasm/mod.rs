cfg_os_poll! {
    use crate::Token;
    use std::io;

    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> Self {
            Self
        }

        pub fn do_io<T, F, R>(&self, _f: F, _io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            todo!("`IoSourceState::do_io`");
        }
    }

    mod selector {
        use crate::{Token, Interest};
        use std::io;
        use std::os::wasi::io::RawFd;
        use std::time::Duration;

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

            #[allow(unused)]
            pub(crate) fn id(&self) -> usize {
                todo!("`Selector::id`");
            }

            pub(crate) fn select(&self, _events: &mut Events, _timeout: Option<Duration>) -> io::Result<()> {
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
        }
        pub(crate) type Events = Vec<Event>;

        pub(crate) mod event {
            use crate::Token;
            use crate::sys::Event;
            use std::fmt;

            pub(crate) fn token(_event: &Event) -> Token {
                todo!("`_event::token`");
            }

            pub(crate) fn is_readable(_event: &Event) -> bool {
                todo!("`_event::is_readable`");
            }

            pub(crate) fn is_writable(_event: &Event) -> bool {
                todo!("`_event::is_writable`");
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

            pub(crate) fn is_aio(_event: &Event) -> bool {
                todo!("`event::is_aio`");
            }

            pub(crate) fn is_lio(_event: &Event) -> bool {
                todo!("`event::is_lio`");
            }

            pub(crate) fn debug_details(_formatter: &mut fmt::Formatter<'_>, _event: &Event) -> fmt::Result {
                todo!("`event::debug_details`");
            }
        }
    }

    pub(crate) use self::selector::{event, Event, Events, Selector};

    cfg_net! {
        pub(crate) mod net {
            use std::io;
            use std::net::ToSocketAddrs;
            use std::os::wasi::io::{AsRawFd, RawFd};
            use std::time::Duration;

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
            pub struct TcpListener;

            impl TcpListener {
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
                    todo!("`TcpListener::as_raw_fd`");
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

            pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, Shutdown, SocketAddrV4, SocketAddrV6};
        }

        pub(crate) mod tcp {
            use crate::net::TcpKeepalive;
            use std::io;
            use std::net::SocketAddr;
            use std::time::Duration;

            pub use crate::sys::net::{TcpListener, TcpStream};
            pub type TcpSocket = usize;

            pub fn new_v4_socket() -> io::Result<TcpSocket> {
                todo!("`tcp::new_v4_socket`");
            }

            pub fn new_v6_socket() -> io::Result<TcpSocket> {
                todo!("`tcp::new_v6_socket`");
            }

            pub fn bind(_socket: TcpSocket, _addr: SocketAddr) -> io::Result<()> {
                todo!("`tcp::bind`");
            }

            pub fn connect(_socket: TcpSocket, _addr: SocketAddr) -> io::Result<TcpStream> {
                todo!("`tcp::connect`");
            }

            pub fn listen(_socket: TcpSocket, _backlog: u32) -> io::Result<TcpListener> {
                todo!("`tcp::listen`");
            }

            pub fn accept(_listener: &TcpListener) -> io::Result<(TcpStream, SocketAddr)> {
                todo!("`tcp::accept`");
            }

            pub fn set_reuseaddr(_socket: TcpSocket, _reuseaddr: bool) -> io::Result<()> {
                todo!("`tcp::set_reuseaddr`");
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
