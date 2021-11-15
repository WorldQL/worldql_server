#[macro_export]
macro_rules! trace_packet {
    ($target: expr, $($field:tt)*) => {
        #[cfg(feature = "trace_packets")]
        tracing::trace!($target, $($field)*);
    };
}
