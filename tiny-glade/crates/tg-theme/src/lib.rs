//! Seasonal themes: validated schedules, bounded downloads and transactional caching.
//!
//! [`ThemeClient`] owns the synchronous state machine. [`ThemeSubscription`] runs it
//! on a worker and publishes only state changes; dropping the subscription cancels
//! subsequent work. A request already in progress is bounded by its HTTP timeout.
#![doc = include_str!("../README.md")]
pub mod cache;
pub mod http;
pub mod model;
pub mod protocol;
mod service;

pub use model::{ActiveEvent, ContentId, ThemeState};
pub use service::{Config, ThemeClient, ThemeSubscription};
