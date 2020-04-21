use std::io::{Error, ErrorKind};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;

use nix::errno::Errno;
use nix::sys::socket::{sendmsg, ControlMessage, MsgFlags};
use nix::sys::uio::IoVec;

pub(crate) fn padding_for_8_bytes(value: usize) -> usize {
    padding_for_n_bytes(value, 8)
}

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

/// Similar to std Write.write_all(), but handle ancillary Fds with sendmsg().
pub(crate) fn write_all(socket: &UnixStream, mut buf: &[u8], fds: &[RawFd]) -> std::io::Result<()> {
    let mut cmsg = vec![ControlMessage::ScmRights(fds)];

    while !buf.is_empty() {
        let iov = [IoVec::from_slice(buf)];

        match sendmsg(socket.as_raw_fd(), &iov, &cmsg, MsgFlags::empty(), None) {
            Ok(0) => {
                return Err(Error::new(
                    ErrorKind::WriteZero,
                    "failed to write all buffer",
                ))
            }
            Ok(n) => {
                buf = &buf[n..];
                cmsg = vec![];
            }
            Err(nix::Error::Sys(Errno::EINPROGRESS)) => {}
            Err(nix::Error::Sys(e)) => return Err(e.into()),
            _ => return Err(Error::new(ErrorKind::Other, "unhandled nix error")),
        }
    }

    Ok(())
}
