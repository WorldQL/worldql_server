#[macro_export]
macro_rules! packet_trace {
    ($target: expr, $($field:tt)*) => {
        #[cfg(feature = "packet_trace")]
        tracing::trace!($target, $($field)*);
    };
}
