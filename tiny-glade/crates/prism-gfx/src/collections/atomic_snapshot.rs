//! Declare counters and their value snapshots together. Each call chooses its
//! existing atomic operation/order; snapshots are not multi-counter barriers.
// `#[macro_export]` is what lets another crate (rhapsody) invoke this; the underscore name plus the
// re-export below keeps the invocation path `collections::atomic_snapshot::atomic_snapshot!` unchanged.
#[macro_export]
#[doc(hidden)]
macro_rules! __atomic_snapshot {
    ($(#[$meta:meta])* $vis:vis $counters:ident, $snapshot:ident {
        $($field_vis:vis $field:ident),+ $(,)?
    } $($methods:tt)*) => {
        #[derive(Default)]
        $vis struct $counters { $($field_vis $field: std::sync::atomic::AtomicU64),+ }
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        $vis struct $snapshot { $(pub $field: u64),+ }
        impl $counters {
            $crate::collections::atomic_snapshot::atomic_snapshot!(@methods $snapshot { $($field),+ } $($methods)*);
        }
    };
    (@methods $snapshot:ident { $($field:ident),+ }
        $method:ident => $operation:ident $arguments:tt; $($rest:tt)*) => {
        pub fn $method(&self) -> $snapshot {
            $snapshot { $($field: self.$field.$operation $arguments),+ }
        }
        $crate::collections::atomic_snapshot::atomic_snapshot!(@methods $snapshot { $($field),+ } $($rest)*);
    };
    (@methods $snapshot:ident { $($field:ident),+ }) => {};
}
#[doc(hidden)]
pub use __atomic_snapshot as atomic_snapshot;
