//! `half_res.rs` — W35 extract_half_res_depth_normals (0x1408f6180).
//!
//! URI `/extract_half_res_depth_normals.cs.hlsl`. `RenderGraphRecorder::create` +
//! read_texture (`gbuffer_input_tex`) + `read_texture_view` (`stencil_tex`) +
//! write_texture + dispatch_2d (O).

pub fn extract_half_res_depth_normals(world: &mut crate::typed_gap::Gap, _recorder: &mut crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
