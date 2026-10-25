//! `app_loading_state.rs` — `AppLoadingState` (a `bevy_state::State` enum used
//! by the app-shell boot).
//!
//! Source: B skeleton `crates/country-core/src/app_loading_state.rs`.
//! PDB named entity: `country_core::app_loading_state::AppLoadingState` (only
//! `hash`/`eq`/`fmt` impls are visible — a bevy-state enum; its *variants* are
//! **U**, not exposed by the PDB).
//!
//! The original is registered through `bevy_state::state` transitions
//! (reused dependency). This crate keeps the identity type; the integration
//! owner wires it to the real `bevy_state` `State<AppLoadingState>`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `AppLoadingState` (O named entity; variants U — typed gap).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AppLoadingState(pub Gap);
