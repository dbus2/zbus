#[cfg(feature = "tracing")]
mod log {
    /// A macro for [`tracing::debug`], meant to make it optional in zbus.
    macro_rules! debug {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ::tracing::debug!($first $(, $arg)+)
        };
        ($only:literal) => {
            ::tracing::debug!($only)
        };
    }
    /// A macro for [`tracing::trace`], meant to make it optional in zbus.
    macro_rules! trace {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ::tracing::trace!($first $(, $arg)+)
        };
        ($only:literal) => {
            ::tracing::trace!($only)
        };
    }
    /// A macro for [`tracing::warn`], meant to make it optional in zbus.
    ///
    /// This is named warning because I was getting internal attribute collisions
    macro_rules! warning {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ::tracing::warn!($first $(, $arg)+)
        };
        ($only:literal) => {
            ::tracing::warn!($only)
        };
    }
    /// A macro for [`tracing::info`], meant to make it optional in zbus.
    macro_rules! info {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ::tracing::info!($first $(, $arg)+)
        };
        ($only:literal) => {
            ::tracing::info!($only)
        };
    }

    pub(crate) use {debug, info, trace, warning};
}

#[cfg(not(feature = "tracing"))]
mod log {
    /// A macro for [`tracing::debug`] that does nothing, because the tracing feature is not
    /// enabled.
    macro_rules! debug {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ()
        };
        ($only:literal) => {
            ()
        };
    }
    /// A macro for [`tracing::trace`] that does nothing, because the tracing feature is not
    /// enabled.
    macro_rules! trace {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ()
        };
        ($only:literal) => {
            ()
        };
    }

    /// A macro for [`tracing::warn`] that does nothing, because the tracing feature is not enabled.
    macro_rules! warning {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ()
        };
        ($only:literal) => {
            ()
        };
    }
    /// A macro for [`tracing::info`] that does nothing, because the tracing feature is not enabled.
    macro_rules! info {
        ($first:literal $(, $arg:expr)+$(,)?) => {
            ()
        };
        ($only:literal) => {
            ()
        };
    }

    pub(crate) use {debug, info, trace, warning};
}

pub(crate) use log::*;
