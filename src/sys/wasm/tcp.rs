use crate::net::TcpKeepalive;
use crate::sys::net::{
    socket_accept, socket_bind, socket_create, socket_listen, socket_pre_accept, socket_recv,
    socket_wait,
};
use crate::sys::wasi::{__wasi_ciovec_t, __wasi_fd_t, __WASI_EAGAIN, __WASI_ESUCCESS};
use std::io;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use wasio::types::{CancellationToken, SockaddrIn, UserContext, AF_INET, AF_INET6, SOCK_STREAM};

pub use crate::sys::net::{TcpListener, TcpStream};
pub type TcpSocket = __wasi_fd_t;

pub fn new_v4_socket() -> io::Result<TcpSocket> {
    let mut fd: __wasi_fd_t = 0;
    let err = unsafe { socket_create(&mut fd, AF_INET, SOCK_STREAM, 0) };

    if err != 0 {
        return Err(io_err!(format!(
            "`tcp::socket_create` failed with `{}`",
            err
        )));
    }

    Ok(fd)
}

pub fn new_v6_socket() -> io::Result<TcpSocket> {
    let mut fd: __wasi_fd_t = 0;
    let err = unsafe { socket_create(&mut fd, AF_INET6, SOCK_STREAM, 0) };

    if err != 0 {
        return Err(io_err!(format!(
            "`tcp::socket_create` failed with `{}`",
            err
        )));
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
        }
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
        return Err(io_err!(format!(
            "`tcp::socket_listen` failed with `{}`",
            err
        )));
    }

    let mut cancellation_token = CancellationToken(0);
    let mut user_context = UserContext(0);

    let err = unsafe { socket_pre_accept(socket, user_context, &mut cancellation_token) };

    if err != 0 {
        return Err(io_err!(format!(
            "`tcp::socket_listen` pre-accepting failed with `{}`",
            err
        )));
    }

    Ok(TcpListener::new(socket))
}

pub fn accept(listener: &TcpListener) -> io::Result<(TcpStream, SocketAddr)> {
    let mut connection: __wasi_fd_t = 0;
    let mut address = SockaddrIn::default();

    println!("socket_accept");

    let err = unsafe {
        socket_accept(
            &mut connection,
            &mut address as *mut _ as *mut u8,
            mem::size_of::<SockaddrIn>() as u32,
        )
    };

    dbg!(&err);
    dbg!(&connection);

    let mut cancellation_token = CancellationToken(0);
    let mut user_context = UserContext(0);

    {
        let mut err =
            unsafe { socket_pre_accept(listener.socket, user_context, &mut cancellation_token) };

        if err != 0 {
            return Err(io_err!(format!(
                "`tcp::socket_pre_accept` failed with `{}`",
                err
            )));
        }
    }

    match err {
        __WASI_ESUCCESS => {
            let port = address.sin_port;

            #[cfg(target_endian = "little")]
            let port = port.to_le();

            let socket: TcpSocket = connection;
            let address = SocketAddr::new(IpAddr::V4(address.sin_addr.into()), port);

            Ok((TcpStream::new(socket, address), address))
        }

        __WASI_EAGAIN => Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "`tcp::socket_accept` returned __WASI_EAGAIN",
        )),

        _ => Err(io_err!(format!(
            "`tcp::socket_accept` failed with `{}`",
            err
        ))),
    }
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

// custom functions

pub fn read(socket: TcpSocket, buf: &mut [u8]) -> io::Result<usize> {
    let iov = __wasi_ciovec_t {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len() as u32,
    };
    let mut cancellation_token = CancellationToken(0);
    let mut read_buffer_length = 0;

    let err = unsafe {
        socket_recv(
            socket,
            &iov,
            1,
            0,
            &mut read_buffer_length,
            std::ptr::null_mut(),
            UserContext(0),
            &mut cancellation_token,
        )
    };

    if err != 0 {
        return Err(io_err!(format!("`TcpStream::read` failed with `{}`", err)));
    }

    Ok(read_buffer_length as usize)
}
