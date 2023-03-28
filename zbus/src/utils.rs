#[cfg(unix)]
use nix::unistd::{Uid, User};
#[cfg(unix)]
use std::io;
use std::path::PathBuf;

#[cfg(unix)]
pub(crate) const FDS_MAX: usize = 1024; // this is hardcoded in sdbus - nothing in the spec

pub(crate) fn padding_for_8_bytes(value: usize) -> usize {
    padding_for_n_bytes(value, 8)
}

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

/// Helper trait for macro-generated code.
///
/// This trait allows macros to refer to the `Ok` and `Err` types of a [Result] that is behind a
/// type alias.  This is currently required because the macros for properties expect a Result
/// return value, but the macro-generated `receive_` functions need to refer to the actual
/// type without the associated error.
pub trait ResultAdapter {
    type Ok;
    type Err;
}

impl<T, E> ResultAdapter for Result<T, E> {
    type Ok = T;
    type Err = E;
}

#[cfg(not(feature = "tokio"))]
#[doc(hidden)]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    async_io::block_on(future)
}

#[cfg(feature = "tokio")]
lazy_static::lazy_static! {
    static ref TOKIO_RT: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("launch of single-threaded tokio runtime")
    };
}

#[cfg(feature = "tokio")]
#[doc(hidden)]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    TOKIO_RT.block_on(future)
}

#[cfg(not(feature = "tokio"))]
pub(crate) async fn run_in_thread<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let event = event_listener::Event::new();
    let listener = event.listen();
    let value = std::sync::Arc::new(std::sync::Mutex::new(None));
    let value_clone = value.clone();

    std::thread::spawn(move || {
        let res = f();

        *value_clone.lock().expect("Failed to set result") = Some(res);
        event.notify(1);
    });
    listener.await;

    let value = value
        .lock()
        .expect("Failed to receive result")
        .take()
        .expect("Failed to receive result");

    value
}

// We implement this ourselves because:
//
// 1. It helps us avoid a dep on `dirs` (which we don't need for anything else).
// 2. `dirs::home_dir` doesn't do the full job for us anyway:
//    https://github.com/dirs-dev/dirs-rs/issues/45
pub(crate) fn home_dir() -> crate::Result<PathBuf> {
    match std::env::var("HOME") {
        Ok(home) => Ok(home.into()),
        Err(_) => {
            #[cfg(unix)]
            {
                let uid = Uid::effective();
                let user = User::from_uid(uid)
                    .map_err(Into::into)
                    .and_then(|user| {
                        user.ok_or_else(|| {
                            crate::Error::InputOutput(
                                io::Error::new(
                                    io::ErrorKind::NotFound,
                                    format!("No user found for UID {}", uid),
                                )
                                .into(),
                            )
                        })
                    })
                    .map_err(|e| {
                        crate::Error::Handshake(format!(
                            "Failed to get user information for UID {}: {}",
                            uid, e
                        ))
                    })?;

                Ok(user.dir)
            }

            #[cfg(windows)]
            {
                crate::win32::home_dir().map_err(Into::into)
            }
        }
    }
}
