//! `startup/glade_themes/mod.rs` — theme startup dispatchers.
//!
//! Source: B skeleton `crates/country-core/src/startup/glade_themes/mod.rs`.
//! Themes are selected by the `ActiveSession` glade-type byte @+0x58 (O:
//! `init_clearing`): olden = 1, flowery = 2.

#![allow(dead_code, unused_imports)]

pub mod flowery;
pub mod olden;
