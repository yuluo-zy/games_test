//! `ray_trace_gi.rs` — W38 software-BVH ray-traced GI (0x140a01cf0).
//!
//! URIs (O): `/gi/{trace_gi_candidates, update_gi_radiance}.cs.hlsl`. Software ray
//! marching against `rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta` (I: no RT
//! capability, findings.md §6). `bind_shadowmap` + `bind_terrain_heightmap` +
//! `bind_terrain_lighting` + read_texture + write_texture + dispatch_2d +
//! `Temporal::into_inner` (O). Runs unconditionally after the GI history temporal
//! pair is created (extent per param_6[0xab]).

use crate::typed_gap::Gap;

pub fn ray_trace_gi(world: &mut Gap, _history: &Gap, _bvh: &Gap) {
    let _ = world;
    unimplemented!()
}
