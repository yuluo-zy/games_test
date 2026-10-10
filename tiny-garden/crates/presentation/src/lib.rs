//! Theme validation is available without the renderer; desktop rendering is opt-in.
#[cfg(feature = "desktop")]
pub mod app;
pub mod building_kit;
#[cfg(feature = "desktop")]
pub mod camera;
pub mod catalog;
#[cfg(feature = "desktop")]
pub mod context_tools;
#[cfg(feature = "desktop")]
pub mod controls;
#[cfg(feature = "desktop")]
pub mod editor;
#[cfg(feature = "desktop")]
mod handoff;
pub mod manifest;
#[cfg(feature = "desktop")]
pub mod picking;
#[cfg(feature = "desktop")]
pub mod pointer;
#[cfg(feature = "desktop")]
pub mod preview;
#[cfg(feature = "desktop")]
pub mod renderer;
#[cfg(feature = "desktop")]
pub mod strokes;
#[cfg(feature = "desktop")]
pub mod ui;
#[cfg(feature = "desktop")]
mod upload;
