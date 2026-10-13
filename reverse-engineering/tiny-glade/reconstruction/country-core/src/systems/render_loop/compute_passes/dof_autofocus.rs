//! `dof_autofocus.rs` — DoF autofocus (W32, address 0x140a17880).
//!
//! URI `/dof/autofocus.cs.hlsl`. Focus parameters from `MainCamera::focus` +
//! `WindowSize::aspect_ratio/as_vec2` (O: div/mul/add), uniform + read_texture +
//! dispatch_2d. Writes `DofAutofocusSsbo` (consumed by dof_prepare), reads
//! `DofFocusReadbackSsbo`.

use crate::typed_gap::Gap;

/// Autofocus readback for the CPU (O name; layout U).
pub struct DofAutofocusSsbo(pub Gap);
/// Focus readback (O name; layout U).
pub struct DofFocusReadbackSsbo(pub Gap);

pub fn calculate_dof_autofocus(world: &mut Gap, _focus: &Gap, _window: &Gap) {
    let _ = world;
    unimplemented!()
}
