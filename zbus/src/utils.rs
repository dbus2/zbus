use std::io::{Error, ErrorKind};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::net::UnixStream;

use nix::cmsg_space;
use nix::errno::Errno;
use nix::sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags};
use nix::sys::uio::IoVec;

use crate::owned_fd::OwnedFd;

const FDS_MAX: usize = 1024; // this is hardcoded in sdbus - nothing in the spec

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

/// Similar to std Read.read_exact, but handle recvmsg() with ancillary Fds.
pub(crate) fn read_exact(socket: &UnixStream, mut buf: &mut [u8]) -> std::io::Result<Vec<OwnedFd>> {
    let mut fds = vec![];

    while !buf.is_empty() {
        let iov = [IoVec::from_mut_slice(buf)];
        let mut cmsgspace = cmsg_space!([RawFd; FDS_MAX]);

        match recvmsg(
            socket.as_raw_fd(),
            &iov,
            Some(&mut cmsgspace),
            MsgFlags::empty(),
        ) {
            Ok(msg) => {
                for cmsg in msg.cmsgs() {
                    if let ControlMessageOwned::ScmRights(fd) = cmsg {
                        for fd in fd.iter() {
                            // assuming the received FD is valid
                            unsafe {
                                fds.push(OwnedFd::from_raw_fd(*fd as RawFd));
                            }
                        }
                    } else {
                        return Err(Error::new(ErrorKind::InvalidData, "unexpected CMSG kind"));
                    }
                }

                let tmp = buf;
                buf = &mut tmp[msg.bytes..];
            }
            Err(nix::Error::Sys(Errno::EINPROGRESS)) => {}
            Err(nix::Error::Sys(e)) => return Err(e.into()),
            _ => return Err(Error::new(ErrorKind::Other, "unhandled nix error")),
        }
    }

    if !buf.is_empty() {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    } else {
        Ok(fds)
    }
}
