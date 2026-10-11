
void __cdecl
utils::curve::curve_slice::CurveSlice::
extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
          (void)

{
  ulonglong uVar1;
  undefined4 uVar2;
  longlong *in_RCX;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float in_XMM1_Da;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 uVar9;
  float fVar10;
  undefined1 local_139;
  undefined8 local_138;
  ulonglong local_130;
  float local_128;
  float local_124;
  longlong **local_120;
  undefined8 local_118;
  ulonglong uStack_110;
  undefined8 local_108;
  ulonglong uStack_100;
  undefined8 local_f8;
  longlong *local_f0;
  longlong local_e8;
  longlong local_e0;
  ulonglong local_d8;
  ulonglong local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined1 *local_b0;
  char local_a8;
  ulonglong local_a0;
  ulonglong uStack_98;
  undefined8 local_90;
  ulonglong uStack_88;
  undefined8 local_80;
  
  if (in_XMM1_Da < 0.0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar10 = *(float *)((longlong)in_RCX + 0x24);
  uVar4 = (ulonglong)*(uint *)(in_RCX + 4);
  local_f0 = in_RCX + 2;
  local_d0 = uVar4 + 1;
  local_e8 = *in_RCX;
  uVar5 = in_RCX[1];
  local_e0 = local_e8 + uVar5 * 0xc;
  local_c8 = 0;
  uStack_c0 = 0;
  local_b8 = 0;
  local_118 = 0;
  local_b0 = &local_139;
  local_a8 = '\0';
  if (uVar4 == 0) {
    if (uVar5 == 0) {
      return;
    }
  }
  else {
    if (uVar5 <= uVar4) {
      return;
    }
    local_e8 = local_e8 + uVar4 * 0xc;
  }
  local_d8 = 0;
  local_e8 = local_e8 + 0xc;
  fVar6 = in_XMM1_Da;
  local_128 = (float)(*(code *)*local_f0)(uVar4);
  local_138 = 1;
  local_130 = uVar4;
  local_124 = fVar6;
  local_120 = &local_f0;
  itertools::tuple_impl::impl$31::
  collect_from_iter_no_buf<tuple_<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec2::Vec2>_>,ref_mut_<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_1<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>_>_>_>
            ();
  fVar6 = __real_3f800000;
  if (local_a0 != 0) {
    uVar2 = (undefined4)uStack_98;
    fVar7 = (float)local_80 - (float)local_90;
    fVar8 = (float)((ulonglong)local_80 >> 0x20) - (float)((ulonglong)local_90 >> 0x20);
    fVar7 = fVar7 * fVar7;
    fVar8 = SQRT(fVar8 * fVar8 + fVar7);
    local_f8 = local_80;
    local_108 = local_90;
    uStack_100 = uStack_88;
    uStack_110 = uStack_98;
    uVar4 = local_d8;
    uVar1 = local_d0;
    uVar5 = local_a0;
    for (fVar10 = (__real_3f800000 - fVar10) * fVar8 - in_XMM1_Da; fVar10 < 0.0;
        fVar10 = fVar10 + fVar8) {
      *(int *)(in_RCX + 4) = (int)uStack_110;
      *(undefined4 *)((longlong)in_RCX + 0x24) = 0x3f800000;
      if (local_a8 != '\0') {
        return;
      }
      local_d0 = uVar1 + 1;
      if (uVar4 == 0) {
        local_d8 = uVar4;
        if (local_e8 == local_e0) {
          return;
        }
      }
      else {
        local_d8 = 0;
        uVar3 = ((ulonglong)(local_e0 - local_e8) >> 2) * -0x5555555555555555;
        if (uVar3 < uVar4 || uVar3 - uVar4 == 0) {
          return;
        }
        local_e8 = local_e8 + uVar4 * 0xc;
      }
      local_e8 = local_e8 + 0xc;
      fVar8 = (float)(*(code *)*local_f0)(uVar1);
      uVar9 = local_f8;
      if ((uVar5 & 1) == 0) {
        local_138 = 1;
        local_130 = uVar1;
        local_128 = fVar8;
        local_124 = fVar7;
        local_120 = &local_f0;
        itertools::tuple_impl::impl$31::
        collect_from_iter_no_buf<tuple_<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec2::Vec2>_>,ref_mut_<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_1<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>_>_>_>
                  ();
        local_f8 = local_80;
        if (local_a0 != 1) {
          return;
        }
        fVar8 = (float)local_80;
        fVar7 = (float)((ulonglong)local_80 >> 0x20);
        uVar5 = 1;
        uStack_110 = uStack_98;
        uVar9 = local_90;
        uStack_100 = uStack_88;
      }
      else {
        local_f8 = CONCAT44(fVar7,fVar8);
        uStack_110 = uStack_100;
        uStack_100 = uVar1;
      }
      uVar2 = (undefined4)uStack_110;
      fVar8 = fVar8 - (float)uVar9;
      fVar7 = fVar7 - (float)((ulonglong)uVar9 >> 0x20);
      fVar7 = fVar7 * fVar7 + fVar8 * fVar8;
      fVar8 = SQRT(fVar7);
      local_108 = uVar9;
      uVar4 = local_d8;
      uVar1 = local_d0;
    }
    *(undefined4 *)(in_RCX + 4) = uVar2;
    *(float *)((longlong)in_RCX + 0x24) = fVar6 - fVar10 / fVar8;
  }
  return;
}

