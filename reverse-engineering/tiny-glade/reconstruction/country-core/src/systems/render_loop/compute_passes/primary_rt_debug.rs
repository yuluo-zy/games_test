//! `primary_rt_debug.rs` — debug primary-ray tracing view.
//!
//! debug family. write_texture + `bind_terrain_heightmap` + `bind_terrain_lighting`
//! + `bind_shadowmap` + uniform + dispatch (O).

pub fn primary_rt_debug(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
    let _ = world;
    unimplemented!()
}
