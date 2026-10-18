//! B startup branch of AdaptiveRenderState, native size 0x30 (resource
//! insertion @0x14013d5f0). Startup writes tag 0 and enabled byte +0x28;
//! the inactive payload is uninitialized and must not be interpreted as data.
//! This models only the proven startup variant, not the later adaptive GPU
//! budget controller. The handoff clones this owned value unchanged.

#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdaptiveRenderState {
    pub enabled: bool,
    phase: InitialAdaptivePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InitialAdaptivePhase {
    Initial,
}

impl AdaptiveRenderState {
    /// B @0x1405a272e..2795: +0x28 = !CliArgs.ignore_vram_budget,
    /// +0x00 = 0. No values are invented for the inactive enum payload.
    pub fn at_startup(ignore_vram_budget: bool) -> Self {
        Self {
            enabled: !ignore_vram_budget,
            phase: InitialAdaptivePhase::Initial,
        }
    }

    pub fn native_tag(&self) -> u64 {
        0
    }
}
