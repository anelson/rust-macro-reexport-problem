/// Re-export everything in the `tracing` crate
pub use ::tracing::*;

/// A version of the `trace!` macro that sets a special target
#[macro_export]
macro_rules! print_trace {
    ($($arg:tt)*) => (
        // Invoke the `trace!` macro which is defined in the `tracing` crate but which has been
        // re-exported from this crate, so that downstream callers don't have to take an explicit
        // dependency on `tracing`
        $crate::trace!(target: "console", $($arg)*)
    )
}
