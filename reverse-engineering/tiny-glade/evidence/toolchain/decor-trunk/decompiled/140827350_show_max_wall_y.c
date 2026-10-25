// ADDR 0x140827350
// SIG void __cdecl show_max_wall_y(void)
// ORIG system::show_max_wall_y


void __cdecl system_decorator::blueprint::show_max_wall_y(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  longlong in_RCX;
  longlong local_40;
  undefined8 *puStack_38;
  longlong local_30;
  
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32>_>,system_decorator::blueprint::show_max_wall_y::closure_env_0>_>
            ();
  if (*(int *)(in_RCX + 600) != 2) {
    if (local_30 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar2 = *puStack_38;
    uVar1 = *(undefined4 *)(puStack_38 + 1);
    if (local_30 == local_40) {
      alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
    *(undefined4 *)((longlong)puStack_38 + local_30 * 0xc + 8) = uVar1;
    *(undefined8 *)((longlong)puStack_38 + local_30 * 0xc) = uVar2;
  }
  country_core::utils::debug_viz::DebugLine::new();
  country_core::utils::debug_viz::DebugLine::dashed();
  country_core::utils::debug_viz::DebugLine::line_width();
  country_core::utils::debug_viz::DebugLine::show();
  return;
}


