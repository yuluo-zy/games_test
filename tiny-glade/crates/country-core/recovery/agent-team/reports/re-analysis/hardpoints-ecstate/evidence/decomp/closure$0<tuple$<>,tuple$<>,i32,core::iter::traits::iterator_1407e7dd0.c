
void __cdecl
core::ops::try_trait::impl$0::wrap_mut_2::
closure_0<tuple_<>,tuple_<>,i32,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<i32,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_env_0>_>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *in_RCX;
  longlong lVar9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 local_1a4;
  float local_19c;
  longlong local_198;
  longlong lStack_190;
  int local_128;
  float fStack_124;
  float fStack_118;
  float fStack_114;
  float fStack_110;
  undefined8 local_108;
  float fStack_100;
  float fStack_fc;
  float local_f8;
  float fStack_f4;
  float fStack_f0;
  undefined1 local_c4 [4];
  longlong local_c0;
  longlong lStack_b8;
  longlong local_b0;
  undefined1 local_a1;
  undefined8 local_a0;
  
  local_a0 = 0xfffffffffffffffe;
  local_c0 = 0;
  lStack_b8 = 4;
  local_b0 = 0;
  lVar3 = in_RCX[1];
  lStack_190 = *(longlong *)(*in_RCX + 8);
  lVar1 = lStack_190 + *(longlong *)(*in_RCX + 0x10) * 8;
  local_198 = 0;
  lVar7 = in_RCX[2];
  lVar8 = in_RCX[3];
  local_128 = 0;
  do {
    fVar12 = fStack_114;
    if (lStack_190 == lVar1) {
LAB_1407e80e4:
      local_a1 = 0;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (SBORROW8(0,local_198)) {
        local_a1 = 0;
                    // WARNING: Subroutine does not return
        result::unwrap_failed();
      }
      plVar4 = (longlong *)in_RCX[10];
      lVar5 = plVar4[2];
      if (lVar5 == *plVar4) {
        ::alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      lVar6 = plVar4[1];
      lVar9 = lVar5 * 0x38;
      *(undefined1 **)(lVar6 + 0x30 + lVar9) = local_c4;
      plVar2 = (longlong *)(lVar6 + 0x20 + lVar9);
      *plVar2 = lVar7;
      plVar2[1] = lVar8;
      plVar2 = (longlong *)(lVar6 + 0x10 + lVar9);
      *plVar2 = lVar1;
      plVar2[1] = lVar3;
      *(longlong *)(lVar6 + lVar9) = local_198;
      ((longlong *)(lVar6 + lVar9))[1] = lStack_190;
      plVar4[2] = lVar5 + 1;
      return;
    }
    lStack_190 = lStack_190 + 8;
    local_a1 = 1;
    function::impls::impl$4::
    call_once<tuple_<ref_<glam::f32::vec2::Vec2>_>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_0::closure_env_0>
              ();
    if (local_128 == 1) {
      fStack_124 = fStack_118;
      fStack_118 = (float)local_1a4;
      fStack_114 = (float)((ulonglong)local_1a4 >> 0x20);
      fVar11 = local_19c;
    }
    else {
      local_a1 = 1;
      itertools::tuple_impl::impl$31::
      collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut_<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_0::closure_env_0>_>_>_>
                ();
      fStack_118 = local_f8;
      fStack_114 = fStack_f4;
      local_128 = (int)local_108;
      fStack_124 = (float)((ulonglong)local_108 >> 0x20);
      fVar11 = fStack_f0;
      fStack_110 = fStack_fc;
      fVar12 = fStack_100;
      if (local_128 != 1) goto LAB_1407e80e4;
    }
    lVar5 = local_b0;
    if (local_198 == 0) {
      if (local_b0 == local_c0) {
        local_a1 = 1;
        ::alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      *(ulonglong *)(lStack_b8 + lVar5 * 0xc) = CONCAT44(fVar12,fStack_124);
      *(float *)(lStack_b8 + 8 + lVar5 * 0xc) = fStack_110;
      local_b0 = lVar5 + 1;
    }
    local_a1 = 1;
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    lVar5 = local_b0;
    if (fStack_114 < extraout_XMM0_Da) {
      local_a1 = 1;
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      lVar5 = local_b0;
      if (local_b0 == local_c0) {
        local_a1 = 1;
        ::alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      fVar10 = (extraout_XMM0_Da_00 - fVar12) / (fStack_114 - fVar12);
      *(ulonglong *)(lStack_b8 + lVar5 * 0xc) =
           CONCAT44(fVar10 * (fStack_114 - fVar12) + fVar12,
                    fVar10 * (fStack_118 - fStack_124) + fStack_124);
      *(float *)(lStack_b8 + 8 + lVar5 * 0xc) = fVar10 * (fVar11 - fStack_110) + fStack_110;
      local_b0 = lVar5 + 1;
      local_198 = local_198 + 1;
      goto LAB_1407e80e4;
    }
    if (local_b0 == local_c0) {
      local_a1 = 1;
      ::alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
    *(ulonglong *)(lStack_b8 + lVar5 * 0xc) = CONCAT44(fStack_114,fStack_118);
    *(float *)(lStack_b8 + 8 + lVar5 * 0xc) = fVar11;
    local_b0 = lVar5 + 1;
    local_198 = local_198 + 1;
    fStack_110 = fVar11;
  } while( true );
}

