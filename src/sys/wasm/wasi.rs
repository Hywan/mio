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
pub const __WASI_E2BIG: u16 = 1;
pub const __WASI_EACCES: u16 = 2;
pub const __WASI_EADDRINUSE: u16 = 3;
pub const __WASI_EADDRNOTAVAIL: u16 = 4;
pub const __WASI_EAFNOSUPPORT: u16 = 5;
pub const __WASI_EAGAIN: u16 = 6;
pub const __WASI_EALREADY: u16 = 7;
pub const __WASI_EBADF: u16 = 8;
pub const __WASI_EBADMSG: u16 = 9;
pub const __WASI_EBUSY: u16 = 10;
pub const __WASI_ECANCELED: u16 = 11;
pub const __WASI_ECHILD: u16 = 12;
pub const __WASI_ECONNABORTED: u16 = 13;
pub const __WASI_ECONNREFUSED: u16 = 14;
pub const __WASI_ECONNRESET: u16 = 15;
pub const __WASI_EDEADLK: u16 = 16;
pub const __WASI_EDESTADDRREQ: u16 = 17;
pub const __WASI_EDOM: u16 = 18;
pub const __WASI_EDQUOT: u16 = 19;
pub const __WASI_EEXIST: u16 = 20;
pub const __WASI_EFAULT: u16 = 21;
pub const __WASI_EFBIG: u16 = 22;
pub const __WASI_EHOSTUNREACH: u16 = 23;
pub const __WASI_EIDRM: u16 = 24;
pub const __WASI_EILSEQ: u16 = 25;
pub const __WASI_EINPROGRESS: u16 = 26;
pub const __WASI_EINTR: u16 = 27;
pub const __WASI_EINVAL: u16 = 28;
pub const __WASI_EIO: u16 = 29;
pub const __WASI_EISCONN: u16 = 30;
pub const __WASI_EISDIR: u16 = 31;
pub const __WASI_ELOOP: u16 = 32;
pub const __WASI_EMFILE: u16 = 33;
pub const __WASI_EMLINK: u16 = 34;
pub const __WASI_EMSGSIZE: u16 = 35;
pub const __WASI_EMULTIHOP: u16 = 36;
pub const __WASI_ENAMETOOLONG: u16 = 37;
pub const __WASI_ENETDOWN: u16 = 38;
pub const __WASI_ENETRESET: u16 = 39;
pub const __WASI_ENETUNREACH: u16 = 40;
pub const __WASI_ENFILE: u16 = 41;
pub const __WASI_ENOBUFS: u16 = 42;
pub const __WASI_ENODEV: u16 = 43;
pub const __WASI_ENOENT: u16 = 44;
pub const __WASI_ENOEXEC: u16 = 45;
pub const __WASI_ENOLCK: u16 = 46;
pub const __WASI_ENOLINK: u16 = 47;
pub const __WASI_ENOMEM: u16 = 48;
pub const __WASI_ENOMSG: u16 = 49;
pub const __WASI_ENOPROTOOPT: u16 = 50;
pub const __WASI_ENOSPC: u16 = 51;
pub const __WASI_ENOSYS: u16 = 52;
pub const __WASI_ENOTCONN: u16 = 53;
pub const __WASI_ENOTDIR: u16 = 54;
pub const __WASI_ENOTEMPTY: u16 = 55;
pub const __WASI_ENOTRECOVERABLE: u16 = 56;
pub const __WASI_ENOTSOCK: u16 = 57;
pub const __WASI_ENOTSUP: u16 = 58;
pub const __WASI_ENOTTY: u16 = 59;
pub const __WASI_ENXIO: u16 = 60;
pub const __WASI_EOVERFLOW: u16 = 61;
pub const __WASI_EOWNERDEAD: u16 = 62;
pub const __WASI_EPERM: u16 = 63;
pub const __WASI_EPIPE: u16 = 64;
pub const __WASI_EPROTO: u16 = 65;
pub const __WASI_EPROTONOSUPPORT: u16 = 66;
pub const __WASI_EPROTOTYPE: u16 = 67;
pub const __WASI_ERANGE: u16 = 68;
pub const __WASI_EROFS: u16 = 69;
pub const __WASI_ESPIPE: u16 = 70;
pub const __WASI_ESRCH: u16 = 71;
pub const __WASI_ESTALE: u16 = 72;
pub const __WASI_ETIMEDOUT: u16 = 73;
pub const __WASI_ETXTBSY: u16 = 74;
pub const __WASI_EXDEV: u16 = 75;
pub const __WASI_ENOTCAPABLE: u16 = 76;

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

pub use wasio::types::__wasi_ciovec_t;

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
