//! Swapchains and presentation leases. Graph-driven presentation (`PresentTarget`, sessions)
//! lives in [`crate::rg`]; the native surface is [`crate::native::Surface`].
internal_mod!(swapchain);
pub use swapchain::{
    ACQUIRE_TIMEOUT_NS, AcquiredImageLease, FullscreenFallback, FullscreenStatus, PresentTicket,
    PresentPolicy, Swapchain, SwapchainDesc, SwapchainImageLease, SwapchainPolicy, WindowMode,
};
