#[macro_export]
macro_rules! trace_packet {
    ($target: expr, $($field:tt)*) => {
        #[cfg(all(debug_assertions, feature = "trace_packets"))]
        tracing::trace!($target, $($field)*);
    };
}
