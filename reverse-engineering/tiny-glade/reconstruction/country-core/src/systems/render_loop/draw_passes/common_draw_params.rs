//! `common_draw_params.rs` — shared per-frame draw binding (bind_common).
//!
//! `CommonDrawParams::new` (line 26, inlined into every draw pass): `utils::get_res`
//! + `Duration::as_secs_f32` (frame time uniform). `bind_common`: `bind_dummy_shadowmap`
//! + `bind_shadowmap` + `Transform::compute_matrix` + uniform (O). Customization
//! binders: `bind_roof_color_customization` (84), `bind_window_color_customization_textures`
//! (89), `bind_wall_color_customization`, `bind_brick_erase_highlight`,
//! `bind_stone_floor_textures`, `bind_wooden_floor_textures` — all read_texture / uniform
//! (O). Fields of `CommonDrawParams` U.

use crate::typed_gap::Gap;

pub struct CommonDrawParams(pub Gap);

impl CommonDrawParams {
    /// Frame-time uniform + resources (O, line 26).
    pub fn new(world: &Gap) -> Self {
        let _ = world;
        CommonDrawParams(Gap)
    }

    /// Shared bindings for all draw passes (O, `bind_common` 0x… module 521).
    pub fn bind_common(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        unimplemented!()
    }

    pub fn bind_roof_color_customization(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        // O: read_texture (roof palette).
        unimplemented!()
    }
    pub fn bind_wall_color_customization(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        unimplemented!()
    }
    pub fn bind_window_color_customization_textures(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        unimplemented!()
    }
    pub fn bind_brick_erase_highlight(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder, _cursor: &Gap) {
        // O: CursorTerrainRaycast::position + uniform.
        unimplemented!()
    }
    pub fn bind_stone_floor_textures(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        unimplemented!()
    }
    pub fn bind_wooden_floor_textures(&self, _draw: &mut crate::consume::pass_builder::DrawBuilder) {
        unimplemented!()
    }
}
