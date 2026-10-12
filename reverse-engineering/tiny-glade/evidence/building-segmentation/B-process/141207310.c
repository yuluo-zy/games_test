
/* WARNING: Removing unreachable block (ram,0x00014120758e) */

void __cdecl
utils::curve::curve_slice::CurveSlice::
extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>
          (void)

{
  uint uVar1;
  undefined8 *puVar2;
  int iVar3;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  float fVar4;
  float in_XMM1_Da;
  float fVar5;
  int local_170;
  longlong local_168;
  float local_160;
  float fStack_15c;
  float fStack_158;
  float local_148;
  float fStack_144;
  float fStack_140;
  undefined1 local_138 [16];
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  longlong *local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  longlong local_78;
  undefined1 *local_70;
  undefined8 local_68;
  longlong local_60;
  undefined1 local_58;
  undefined1 local_50;
  
  if (in_XMM1_Da < 0.0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  puVar2 = (undefined8 *)*in_RCX;
  loop_range::LoopRange::len();
  if (extraout_RAX != 0) {
    fVar5 = *(float *)((longlong)in_RCX + 0x14);
    uVar1 = *(uint *)(in_RCX + 2);
    local_110 = *puVar2;
    uStack_108 = puVar2[1];
    local_100 = puVar2[2];
    uStack_f8 = puVar2[3];
    loop_range::LoopRange::len();
    local_78 = (extraout_RAX_00 - (ulonglong)uVar1) + -2;
    local_128 = 0;
    uStack_120 = 0;
    local_118 = 0;
    local_b8 = in_RCX + 1;
    local_f0 = 0;
    local_b0 = 0;
    uStack_a8 = 0;
    local_a0 = 0;
    local_98 = local_110;
    local_90 = uStack_108;
    uStack_88 = local_100;
    local_80 = uStack_f8;
    local_70 = local_138;
    local_68 = 0;
    local_58 = 0;
    local_50 = 0;
    local_60 = (ulonglong)uVar1 + 1;
    itertools::tuple_impl::impl$7::
    next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,tuple_<tuple_<usize,glam::f32::vec3::Vec3>,tuple_<usize,glam::f32::vec3::Vec3>_>_>
              ();
    if (local_170 == 1) {
      iVar3 = (int)local_168;
      if (local_168 != 0) {
        fVar4 = SQRT((fStack_140 - fStack_158) * (fStack_140 - fStack_158) +
                     (fStack_144 - fStack_15c) * (fStack_144 - fStack_15c) +
                     (local_148 - local_160) * (local_148 - local_160));
        fVar5 = (fVar5 + __real_bf800000 + __real_3f800000) * fVar4 - in_XMM1_Da;
        if (0.0 <= fVar5) {
LAB_141207590:
          *(int *)(in_RCX + 2) = iVar3 + -1;
          *(float *)((longlong)in_RCX + 0x14) = fVar5 / fVar4;
          return;
        }
        *(int *)(in_RCX + 2) = iVar3 + -1;
        *(undefined4 *)((longlong)in_RCX + 0x14) = 0;
        itertools::tuple_impl::impl$7::
        next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,tuple_<tuple_<usize,glam::f32::vec3::Vec3>,tuple_<usize,glam::f32::vec3::Vec3>_>_>
                  ();
        while (local_168 != 0) {
          fVar4 = SQRT((fStack_140 - fStack_158) * (fStack_140 - fStack_158) +
                       (fStack_144 - fStack_15c) * (fStack_144 - fStack_15c) +
                       (local_148 - local_160) * (local_148 - local_160));
          fVar5 = fVar5 + fVar4;
          if (0.0 <= fVar5) goto LAB_141207590;
          *(int *)(in_RCX + 2) = iVar3 + -1;
          *(undefined4 *)((longlong)in_RCX + 0x14) = 0;
          itertools::tuple_impl::impl$7::
          next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,tuple_<tuple_<usize,glam::f32::vec3::Vec3>,tuple_<usize,glam::f32::vec3::Vec3>_>_>
                    ();
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
  return;
}

