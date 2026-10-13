//! `mouse_depth_value.rs` — W33 mouse depth compute (0x140a01620).
//!
//! URI `/read_mouse_depth_value.cs.hlsl`. `WindowSize::as_vec2` + clamp +
//! read_texture + uniform + dispatch. Writes `MouseDepthValueSsbo` via
//! `write_to_cpu_mapped_storage_buffer` (O, wp1 §1.1).

use crate::typed_gap::Gap;

pub fn mouse_depth_compute(world: &mut crate::typed_gap::Gap, _window: &Gap, _cursor: &Gap) {
    let _ = world;
    unimplemented!()
}
