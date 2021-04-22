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

    pub fn join_multicast_v4(
        &self,
        _multiaddr: &Ipv4Addr,
        _interface: &Ipv4Addr,
    ) -> io::Result<()> {
        todo!("`UdpSocket::join_multicast_v4`");
    }

    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
        todo!("`UdpSocket::join_multicast_v6`");
    }

    pub fn leave_multicast_v4(
        &self,
        _multiaddr: &Ipv4Addr,
        _interface: &Ipv4Addr,
    ) -> io::Result<()> {
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
