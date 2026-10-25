//! `spawn_boats.rs` — boat spawning.
//!
//! Skeleton bundle holds only `BoatSsbo` (O name; layout U); no compiled
//! procedure. The `/water/spawn_boats.cs.hlsl` URI is absent from the static
//! URI list (255) — spawn_boats is either inlined into
//! `compute_water::water_spawn_vegetation` or uses a runtime-synthesized URI (U).

use crate::typed_gap::Gap;

pub struct BoatSsbo(pub Gap);
