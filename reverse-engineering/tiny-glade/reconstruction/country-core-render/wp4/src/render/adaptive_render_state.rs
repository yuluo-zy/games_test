//! `adaptive_render_state` — adaptive render-quality state (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/adaptive_render_state.rs`
//! (source_sha256 `39a05efa…`, 0 procedures, modules 194/240 — shared compilands
//! with `render_config`). No standalone procedures; likely drives adaptive
//! internal-resolution scaling. Retained as a typed gap.

/// Adaptive render state (U — typed gap; inferred to hold current/max internal
/// resolution scales).
pub struct AdaptiveRenderState {
    _gap: crate::typed_gap::Gap,
}
