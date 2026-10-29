//! 已恢复的渲染图、资源与 Vulkan 调用链；原作等价性范围见 README。
//!
//! # Public surface
//! - [`device`] — device creation, capabilities, queues, submission, budgets;
//! - [`resource`] — buffers, textures, samplers, managed handles, setup uploads;
//! - [`command`] — command buffers, barriers, CPU arenas;
//! - [`pipeline`] — shader modules, pipeline layouts/caches, async compilation;
//! - [`bindless`] — bindless descriptor heap;
//! - [`rg`] — render graph declaration, planning, execution and sessions;
//! - [`present`] — surfaces and swapchains;
//! - [`transfer`] — asynchronous uploads;
//! - [`sync`] — queue ids, sync points and completion values;
//! - [`Error`], [`ErrorKind`], [`Result`] and the log ring helpers.
//!
//! Everything else is implementation detail. The `test-support` feature makes
//! the implementation modules reachable (hidden from docs) for this crate's
//! own integration tests; it is not part of the API.

/// Declare implementation modules: crate-private, or `pub` but hidden from
/// docs when the `test-support` feature is enabled. Without that feature,
/// items reached only by the integration tests are not reported as dead code
/// (the same as when these modules were public).
macro_rules! internal_mod {
    ($($name:ident),+ $(,)?) => {$(
        #[cfg(feature = "test-support")]
        #[doc(hidden)]
        pub mod $name;
        #[cfg(not(feature = "test-support"))]
        #[allow(dead_code, unused_imports)]
        pub(crate) mod $name;
    )+};
}

pub mod accounting;
pub mod bindless;
pub mod collections;
pub mod command;
pub mod device;
pub mod format;
pub mod native;
pub mod pipeline;
pub mod profile;
pub mod present;
pub mod resource;
pub mod rg;
pub use prism_gfx::sync;
pub mod tracking;
pub mod transfer;
internal_mod!(retire);
// Moved to prism-gfx. Same paths and the same visibility as before: crate-private unless `test-support`.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use prism_gfx::{log_ring, trace};
#[cfg(not(feature = "test-support"))]
#[allow(unused_imports)]
pub(crate) use prism_gfx::{log_ring, trace};

pub(crate) use prism_gfx::error;
pub use prism_gfx::{Error, ErrorKind, Result, SharedError};
pub use log_ring::{dump_log_vec, init_logging, record_log};
