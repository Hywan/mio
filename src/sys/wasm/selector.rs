use crate::sys::net::{socket_pre_accept, socket_wait};
use crate::sys::wasi::{
    __wasi_errno_t, __wasi_event_fd_readwrite_t, __wasi_event_t, __wasi_event_u,
    __wasi_subscription_fs_readwrite_t, __wasi_subscription_t, __wasi_subscription_u,
    sys::poll_oneoff, __WASI_ESUCCESS, __WASI_EVENTTYPE_FD_READ, __WASI_EVENTTYPE_FD_WRITE,
};
use crate::{Interest, Token};
use slab::Slab;
use std::cell::RefCell;
use std::convert::TryInto;
use std::io;
use std::num::TryFromIntError;
use std::os::wasi::io::RawFd;
#[cfg(debug_assertions)]
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use wasio::types::{CancellationToken, UserContext};

#[cfg(debug_assertions)]
static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub(crate) struct RegisteredFd {
    fd: RawFd,
    token: Token,
    interests: Interest,
    cancellation_token: CancellationToken,
}

pub(crate) struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    register: RefCell<Slab<RegisteredFd>>,
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
        let user_context = UserContext(token.0.try_into().unwrap());
        let mut cancellation_token = CancellationToken(0);

        self.register
            .try_borrow_mut()
            .map_err(|_| io_err!("Cannot borrow the register as mutable"))?
            .insert(RegisteredFd {
                fd,
                token,
                interests,
                cancellation_token,
            });

        Ok(())
    }

    pub(crate) fn reregister(
        &self,
        _fd: RawFd,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<()> {
        todo!("`Selector::reregister`");
    }

    pub(crate) fn deregister(&self, fd: RawFd) -> io::Result<()> {
        let mut register = self
            .register
            .try_borrow_mut()
            .map_err(|_| io_err!("Cannot borrow the register as mutable"))?;

        let index = register
            .iter()
            .find_map(
                |(index, RegisteredFd { fd: current_fd, .. })| {
                    if current_fd == &fd {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .ok_or_else(|| {
                io_err!(format!(
                    "Cannot deregister the file descriptor `{}` because it is not registered",
                    fd
                ))
            })?;

        // TODO: do something with `cancellation_token`

        let _value = register.remove(index);

        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(crate) fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn select(&self, events: &mut Events, _timeout: Option<Duration>) -> io::Result<()> {
        println!("•");

        let mut err = 0;
        let mut user_context = UserContext(0);
        println!("waiting…");
        unsafe { socket_wait(&mut err, &mut user_context) };

        if err != 0 {
            return Err(io_err!(format!("`Selector::select` failed with `{}`", err)));
        }

        dbg!(&user_context);
        let retrieved_token = Token(user_context.0.try_into().unwrap());

        *events = self
            .register
            .try_borrow()
            .map_err(|_| io_err!("Cannot borrow the register as mutable"))?
            .iter()
            .filter_map(|(_, registered_fd)| {
                dbg!(&registered_fd);
                if registered_fd.token != retrieved_token {
                    None
                } else {
                    Some(registered_fd)
                }
            })
            .map(|RegisteredFd { fd, token, .. }| Event {
                wasi_errno: err,
                interest: Interest::READABLE.add(Interest::WRITABLE),
                token: retrieved_token,
            })
            .collect();

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
    use crate::sys::wasi::{
        __WASI_ECONNABORTED, __WASI_ECONNREFUSED, __WASI_ENOTCONN, __WASI_ESUCCESS,
    };
    use crate::sys::Event;
    use crate::Token;
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

    pub(crate) fn is_error(event: &Event) -> bool {
        event.wasi_errno != __WASI_ESUCCESS
    }

    // TODO
    pub(crate) fn is_read_closed(event: &Event) -> bool {
        is_readable(event)
            && ((event.wasi_errno == __WASI_ECONNABORTED)
                || (event.wasi_errno == __WASI_ECONNREFUSED)
                || (event.wasi_errno == __WASI_ENOTCONN))
    }

    // TODO
    pub(crate) fn is_write_closed(event: &Event) -> bool {
        is_writable(event)
            && ((event.wasi_errno == __WASI_ECONNABORTED)
                || (event.wasi_errno == __WASI_ECONNREFUSED)
                || (event.wasi_errno == __WASI_ENOTCONN))
    }

    pub(crate) fn is_priority(_event: &Event) -> bool {
        false
    }

    pub(crate) fn is_aio(event: &Event) -> bool {
        event.interest.is_aio()
    }

    pub(crate) fn is_lio(event: &Event) -> bool {
        event.interest.is_lio()
    }

    // TODO
    pub(crate) fn debug_details(
        _formatter: &mut fmt::Formatter<'_>,
        _event: &Event,
    ) -> fmt::Result {
        Ok(())
    }
}
