//! `draw_lists.rs` — CPU draw-list construction + sort (R2).
//!
//! Addresses (O): `DrawLists::new` 0x1409ba340, `build_draw_lists` 0x1409ba540,
//! `DrawLists::sort` 0x1409ba4b0.
//! `build_draw_lists` (O): puffin scope + `World::query` + per-entity dispatch to
//! `DrawData::new_main_pass` (line 189, ~71 mono), `new_main_with_velocity` (212, 5),
//! `new` (181, 3), `new_velocity_only` (227, 1), `new_late_forward` (204, 1).
//! Each `DrawData::new*` is a monomorphized capture of the per-kind draw closures
//! (`closure$0..closure$60`, lines 297–1313) that bind shader + customizations and
//! issue `DrawBuilder::draw_instanced / draw_indexed_indirect / draw_multi_instanced`.
//! `DrawLists::sort` = `sort_by_key` (O). Field layouts U (typed gap).

use crate::consume::pass_builder::{DrawBuilder, DrawPacket};
use crate::typed_gap::Gap;

/// Per-pass draw data (O name; layout U).
pub struct DrawData(pub Vec<DrawPacket>);

impl DrawData {
    /// Generic main-pass item (O, line 181).
    pub fn new(_world: &Gap, _entity: &Gap) -> Option<Self> {
        unimplemented!()
    }
    /// Main-pass item with velocity (O, line 212).
    pub fn new_main_with_velocity(_world: &Gap, _entity: &Gap) -> Option<Self> {
        unimplemented!()
    }
    /// Main-pass item (O, line 189).
    pub fn new_main_pass(_world: &Gap, _entity: &Gap) -> Option<Self> {
        unimplemented!()
    }
    /// Late-forward item (O, line 204).
    pub fn new_late_forward(_world: &Gap, _entity: &Gap) -> Option<Self> {
        unimplemented!()
    }
    /// Velocity-only item (O, line 227).
    pub fn new_velocity_only(_world: &Gap, _entity: &Gap) -> Option<Self> {
        unimplemented!()
    }
}

impl Default for DrawData {
    fn default() -> Self {
        DrawData(Vec::new())
    }
}

/// The CPU draw-list container built at R2 and consumed by the frame passes
/// (`main_pass`, `shadow_pass`, `transparency_pass`, …). Layout U.
#[derive(Default)]
pub struct DrawLists(pub Vec<DrawData>);

impl DrawLists {
    pub fn new(_world: &Gap) -> Self {
        DrawLists(Vec::new())
    }

    /// Sort every list by its sort key (O, `sort_by_key`).
    pub fn sort(&mut self) {
        for list in &mut self.0 {
            let _ = list;
        }
        // O: sort_by_key over the per-item sort key (impl$190::next).
    }
}

/// Build the frame's draw lists from the ECS world (O, 0x1409ba540).
/// Puffin-scoped; queries entities and dispatches to `DrawData::new*`.
pub fn build_draw_lists(world: &mut Gap, lists: &mut DrawLists) {
    let _ = (world, lists);
    unimplemented!()
}

/// Set pipeline topology from a draw item (O, line 1452).
fn apply_topology(_draw: &mut DrawBuilder, _topology: Gap) {
    unimplemented!()
}
