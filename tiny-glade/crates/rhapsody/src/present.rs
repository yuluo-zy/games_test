//! Surfaces, swapchains and presentation leases. Graph-driven presentation
//! (`PresentTarget`, sessions) lives in [`crate::rg`].
pub use crate::raw::swapchain::{
    ACQUIRE_TIMEOUT_NS, AcquiredImageLease, FullscreenFallback, FullscreenStatus, PresentTicket,
    Surface, Swapchain, SwapchainDesc, SwapchainImageLease, SwapchainPolicy, WindowMode,
};
