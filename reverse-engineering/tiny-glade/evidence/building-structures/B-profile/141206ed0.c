
void __cdecl
utils::curve::curve_slice::CurveSlice::
extend_left<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>
          (void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  longlong *in_RCX;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  float fVar8;
  float in_XMM1_Da;
  float fVar9;
  float fVar10;
  undefined1 local_129;
  ulonglong local_128;
  ulonglong uStack_120;
  undefined8 local_118;
  ulonglong uStack_110;
  undefined8 local_108;
  longlong *local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  longlong local_e0;
  longlong local_d8;
  ulonglong local_d0;
  undefined1 *local_c8;
  ulonglong local_c0;
  ulonglong local_b8;
  char local_b0;
  char local_a8;
  undefined8 local_a0;
  ulonglong local_98;
  float local_90;
  float local_8c;
  longlong **local_88;
  ulonglong local_80;
  ulonglong uStack_78;
  undefined8 local_70;
  ulonglong uStack_68;
  undefined8 local_60;
  
  if (in_XMM1_Da < 0.0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  uVar5 = in_RCX[1];
  if (uVar5 != 0) {
    local_e0 = *in_RCX;
    fVar10 = *(float *)((longlong)in_RCX + 0x1c);
    local_b8 = (ulonglong)*(uint *)(in_RCX + 3);
    local_d8 = local_e0 + uVar5 * 0xc;
    lVar7 = uVar5 - local_b8;
    local_100 = in_RCX + 2;
    local_f8 = 0;
    uStack_f0 = 0;
    local_e8 = 0;
    local_128 = 0;
    local_c8 = &local_129;
    local_c0 = 0;
    local_b0 = '\0';
    local_a8 = '\0';
    if ((lVar7 == 2) || (local_d8 = local_d8 + (2 - lVar7) * 0xc, lVar7 - 2U < uVar5)) {
      local_d0 = 0;
      local_d8 = local_d8 + -0xc;
      uVar5 = local_b8 + 1;
      fVar9 = in_XMM1_Da;
      local_90 = (float)(*(code *)*local_100)(uVar5);
      local_a0 = 1;
      local_98 = uVar5;
      local_8c = fVar9;
      local_88 = &local_100;
      itertools::tuple_impl::impl$31::
      collect_from_iter_no_buf<tuple_<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec2::Vec2>_>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_left::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_left::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>_>_>_>
                ();
      local_108 = local_60;
      local_118 = local_70;
      uStack_110 = uStack_68;
      local_128 = local_80;
      uStack_120 = uStack_78;
      if (local_80 != 0) {
        lVar7 = uStack_78 - 1;
        uVar4 = (undefined4)lVar7;
        if (uStack_78 == 0) {
LAB_1412072fc:
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        local_108._0_4_ = (float)local_60;
        local_118._0_4_ = (float)local_70;
        local_108._4_4_ = (float)((ulonglong)local_60 >> 0x20);
        local_118._4_4_ = (float)((ulonglong)local_70 >> 0x20);
        fVar9 = (local_108._4_4_ - local_118._4_4_) * (local_108._4_4_ - local_118._4_4_) +
                ((float)local_108 - (float)local_118) * ((float)local_108 - (float)local_118);
        fVar8 = SQRT(fVar9);
        uVar1 = local_d0;
        uVar2 = local_b8;
        uVar5 = local_80;
        for (fVar10 = (fVar10 + __real_bf800000 + __real_3f800000) * fVar8 - in_XMM1_Da;
            fVar10 < 0.0; fVar10 = fVar10 + fVar8) {
          *(int *)(in_RCX + 3) = (int)lVar7;
          *(undefined4 *)((longlong)in_RCX + 0x1c) = 0;
          if (local_a8 != '\0') {
            return;
          }
          if (local_b0 != '\0') {
            return;
          }
          if (uVar2 < local_c0) {
            return;
          }
          if (local_c0 < uVar2) {
            local_b8 = uVar2 - 1;
          }
          else {
            local_b0 = '\x01';
            local_b8 = uVar2;
          }
          if (uVar1 == 0) {
            local_d0 = uVar1;
            if (local_e0 == local_d8) {
              return;
            }
          }
          else {
            local_d0 = 0;
            uVar6 = ((ulonglong)(local_d8 - local_e0) >> 2) * -0x5555555555555555;
            local_d8 = local_d8 + uVar1 * -0xc;
            if (uVar6 < uVar1 || uVar6 - uVar1 == 0) {
              return;
            }
          }
          local_d8 = local_d8 + -0xc;
          fVar8 = (float)(*(code *)*local_100)(uVar2);
          uVar3 = local_108;
          if ((uVar5 & 1) == 0) {
            local_a0 = 1;
            local_98 = uVar2;
            local_90 = fVar8;
            local_8c = fVar9;
            local_88 = &local_100;
            itertools::tuple_impl::impl$31::
            collect_from_iter_no_buf<tuple_<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec2::Vec2>_>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_left::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_left::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>_>_>_>
                      ();
            local_108 = local_60;
            uStack_110 = uStack_68;
            local_128 = local_80;
            if (local_80 != 1) {
              return;
            }
            fVar8 = (float)local_60;
            fVar9 = (float)((ulonglong)local_60 >> 0x20);
            uVar5 = 1;
            uStack_120 = uStack_78;
            local_118 = local_70;
          }
          else {
            local_108 = CONCAT44(fVar9,fVar8);
            uStack_120 = uStack_110;
            local_118 = uVar3;
            uStack_110 = uVar2;
          }
          if (uStack_120 == 0) goto LAB_1412072fc;
          lVar7 = uStack_120 - 1;
          uVar4 = (undefined4)lVar7;
          fVar8 = fVar8 - (float)local_118;
          fVar9 = fVar9 - (float)((ulonglong)local_118 >> 0x20);
          fVar9 = fVar9 * fVar9 + fVar8 * fVar8;
          fVar8 = SQRT(fVar9);
          uVar1 = local_d0;
          uVar2 = local_b8;
        }
        *(undefined4 *)(in_RCX + 3) = uVar4;
        *(float *)((longlong)in_RCX + 0x1c) = fVar10 / fVar8;
      }
    }
  }
  return;
}

