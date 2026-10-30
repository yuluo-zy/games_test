
void __cdecl
system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo(void)

{
  longlong in_RCX;
  longlong in_RDX;
  longlong in_R8;
  longlong in_R9;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined8 local_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined8 local_180;
  undefined8 local_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 *local_e8;
  undefined8 *local_d8;
  undefined8 local_d0;
  longlong local_98;
  longlong local_88;
  undefined8 local_80;
  longlong local_60;
  undefined8 local_58;
  longlong local_40;
  undefined1 local_38;
  
  local_1e0 = 2;
  local_1d8 = 3;
  local_1d0 = 6;
  local_1c8 = 3;
  local_1c0 = 7;
  local_1b8 = 6;
  local_1b0 = 0;
  local_1a8 = 4;
  local_1a0 = 1;
  local_198 = 1;
  local_190 = 4;
  local_188 = 5;
  local_180 = 2;
  local_178 = 3;
  local_170 = 6;
  local_168 = 3;
  local_160 = 7;
  local_158 = 6;
  local_150 = 0;
  local_148 = 2;
  local_140 = 4;
  local_138 = 2;
  local_130 = 6;
  local_128 = 4;
  local_120 = 1;
  local_118 = 5;
  local_110 = 3;
  local_108 = 3;
  local_100 = 5;
  local_f8 = 7;
  if (in_stack_00000038 != 0) {
    local_60 = in_stack_00000028 + in_stack_00000030 * 4;
    local_88 = in_R8 + in_R9 * 8;
    local_98 = in_RCX + in_RDX * 0xc;
    local_40 = in_stack_00000038 + -1;
    local_e8 = &local_1b0;
    local_d8 = &local_1e0;
    local_d0 = 0;
    local_80 = 0;
    local_58 = 0;
    local_38 = 1;
    alloc::vec::spec_extend::impl$0::
    spec_extend<country_core::systems::collision::world::RaycastMeshData,core::iter::adapters::filter_map::FilterMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_0>,alloc::alloc::Global>
              ();
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

