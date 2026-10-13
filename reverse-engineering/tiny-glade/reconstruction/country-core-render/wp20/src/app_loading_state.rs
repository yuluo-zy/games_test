//! `app_loading_state.rs` — INTEGRATION MARKER (wp15-owned).
//! Real type: `reconstruction/country-core/src/app_loading_state.rs`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `AppLoadingState` (wp15; a `bevy_state::State` enum; variants U).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AppLoadingState(pub Gap);
