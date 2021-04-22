use crate::sys::tcp::{read, TcpSocket};
use std::io;
use std::net::ToSocketAddrs;
use std::os::wasi::io::{AsRawFd, RawFd};
use std::time::Duration;
use wasio::types::{CancellationToken, UserContext};

pub use std::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6};
pub use wasio::sys::{
    socket_accept, socket_bind, socket_create, socket_listen, socket_pre_accept, socket_recv,
    wait as socket_wait,
};

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
    pub(in crate::sys) socket: TcpSocket,
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
pub struct TcpStream {
    socket: TcpSocket,
    address: SocketAddr,
}

impl TcpStream {
    pub(in crate::sys) fn new(socket: TcpSocket, address: SocketAddr) -> Self {
        Self { socket, address }
    }

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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(self.socket, buf)
    }
}

impl io::Read for &TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(self.socket, buf)
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
        self.socket.into()
    }
}
