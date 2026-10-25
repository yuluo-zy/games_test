// 0x140995040 backward

void __cdecl country_core::systems::history::replay_terrain::TerrainReplayCmd::backward(void)

{
  longlong *in_RCX;
  longlong in_RDX;
  float extraout_XMM0_Da;
  float fVar1;
  float fVar2;
  
  if (SBORROW8(0,*(longlong *)(in_RDX + 0x10))) {
    core::iter::adapters::map::impl$2::
    fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,country_core::resources::history::terrain_edit::impl_1::duration::closure_env_0,f32,core::iter::traits::accum::impl_100::sum::closure_env_0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,country_core::resources::history::terrain_edit::impl_1::duration::closure_env_0>_>_>
              ();
    if (0x7f7fffff < (uint)ABS(extraout_XMM0_Da)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar2 = 0.4;
    if (0.4 <= extraout_XMM0_Da * 0.01) {
      fVar2 = extraout_XMM0_Da * 0.01;
    }
    fVar1 = 3.0;
    if (fVar2 <= 3.0) {
      fVar1 = fVar2;
    }
  }
  else {
    fVar1 = 0.3;
  }
  *in_RCX = in_RDX;
  *(undefined1 *)((longlong)in_RCX + 0x14) = 1;
  *(undefined4 *)(in_RCX + 1) = 0;
  *(float *)((longlong)in_RCX + 0xc) = fVar1;
  *(undefined4 *)(in_RCX + 2) = 2;
  return;
}

