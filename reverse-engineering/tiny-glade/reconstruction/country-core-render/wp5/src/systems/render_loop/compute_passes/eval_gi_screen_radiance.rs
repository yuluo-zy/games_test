//! `eval_gi_screen_radiance.rs` — W39 eval GI screen radiance (0x140a44cd0).
//!
//! URIs (O): `/gi/{eval_screen_radiance, reproject_screen_radiance,
//! blur_screen_radiance}.cs.hlsl`. `ShaderRefactoringState::disabled` guard +
//! read/write/read_write_texture + dispatch_2d (O).

pub fn eval_gi_screen_radiance(world: &mut crate::typed_gap::Gap, _frame: &crate::consume::core::FrameConstants) {
    let _ = world;
    unimplemented!()
}
