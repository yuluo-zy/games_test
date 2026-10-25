
void __cdecl
alloc::vec::spec_extend::impl$0::
spec_extend<country_core::systems::collision::world::RaycastMeshData,core::iter::adapters::filter_map::FilterMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_0>,alloc::alloc::Global>
          (void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *in_RCX;
  longlong lVar4;
  longlong local_1a8;
  longlong local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  longlong local_a8;
  
  itertools::tuple_impl::impl$7::
  next<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>
            ();
  while (local_a8 != 0) {
    core::ops::function::impls::impl$3::
    call_mut<tuple_<tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_0>
              ();
    if (local_1a8 != 0) {
      lVar3 = in_RCX[2];
      if (lVar3 == *in_RCX) {
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar2 = in_RCX[1];
      lVar4 = lVar3 * 0x80;
      puVar1 = (undefined8 *)(lVar2 + 0x70 + lVar4);
      *puVar1 = uStack_138;
      puVar1[1] = local_130;
      puVar1 = (undefined8 *)(lVar2 + 0x60 + lVar4);
      *puVar1 = uStack_148;
      puVar1[1] = local_140;
      puVar1 = (undefined8 *)(lVar2 + 0x50 + lVar4);
      *puVar1 = uStack_158;
      puVar1[1] = local_150;
      puVar1 = (undefined8 *)(lVar2 + 0x40 + lVar4);
      *puVar1 = uStack_168;
      puVar1[1] = local_160;
      puVar1 = (undefined8 *)(lVar2 + 0x30 + lVar4);
      *puVar1 = uStack_178;
      puVar1[1] = local_170;
      puVar1 = (undefined8 *)(lVar2 + 0x20 + lVar4);
      *puVar1 = uStack_188;
      puVar1[1] = local_180;
      puVar1 = (undefined8 *)(lVar2 + 0x10 + lVar4);
      *puVar1 = uStack_198;
      puVar1[1] = local_190;
      *(longlong *)(lVar2 + lVar4) = local_1a8;
      ((longlong *)(lVar2 + lVar4))[1] = local_1a0;
      in_RCX[2] = lVar3 + 1;
    }
    itertools::tuple_impl::impl$7::
    next<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>
              ();
  }
  return;
}

