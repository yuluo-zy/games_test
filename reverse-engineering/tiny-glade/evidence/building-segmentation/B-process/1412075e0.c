
/* WARNING: Removing unreachable block (ram,0x0001412076f3) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xfffffffffffffe78 : 0x0001412077ef */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void __cdecl
utils::curve::curve_slice::CurveSlice::
extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>
          (void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  float fVar3;
  undefined4 uVar4;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong *in_RCX;
  float *pfVar5;
  longlong in_RDX;
  float *pfVar6;
  float *pfVar7;
  float fVar8;
  float in_XMM1_Da;
  float fVar9;
  ulonglong local_1b8;
  undefined8 uStack_1b0;
  float fStack_1a8;
  float fStack_1a4;
  float fStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  longlong *local_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  ulonglong local_158;
  ulonglong local_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined1 *local_130;
  undefined1 local_128;
  undefined8 local_110;
  float fStack_108;
  float fStack_104;
  undefined8 local_100;
  undefined8 local_f8 [3];
  undefined8 local_e0;
  longlong **local_d8;
  undefined1 local_d0 [16];
  ulonglong local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  
  if (in_XMM1_Da < 0.0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar9 = *(float *)((longlong)in_RCX + 0x1c);
  local_158 = (ulonglong)*(uint *)(in_RCX + 3);
  puVar1 = (undefined8 *)*in_RCX;
  local_178 = *puVar1;
  uStack_170 = puVar1[1];
  local_168 = puVar1[2];
  uStack_160 = puVar1[3];
  local_148 = 0;
  uStack_140 = 0;
  local_138 = 0;
  local_180 = in_RCX + 1;
  local_130 = local_d0;
  local_128 = 0;
  local_150 = local_158;
  take_until::take_until::impl$2::
  next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>
            ();
  if (in_RDX != 0) {
    (*(code *)*local_180)(&fStack_108,extraout_RAX,in_RDX);
    local_110 = extraout_RAX;
    local_e0 = local_100;
    local_f8[0] = 1;
    pfVar6 = (float *)local_f8;
    local_d8 = &local_180;
    itertools::tuple_impl::impl$31::
    collect_from_iter_no_buf<tuple_<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec3::Vec3>_>,ref_mut_<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>_>_>_>
              ();
    fVar3 = __real_3f800000;
    local_188 = local_90;
    uStack_190 = uStack_98;
    fStack_1a8 = (float)local_b0;
    fStack_1a4 = (float)((ulonglong)local_b0 >> 0x20);
    fStack_1a0 = (float)uStack_a8;
    local_1b8._0_4_ = (int)local_c0;
    if ((int)local_1b8 == 1) {
      uStack_190._0_4_ = (float)uStack_98;
      uStack_190._4_4_ = (float)((ulonglong)uStack_98 >> 0x20);
      uVar4 = (undefined4)uStack_b8;
      local_188._0_4_ = (float)local_90;
      fVar8 = SQRT(((float)local_188 - fStack_1a0) * ((float)local_188 - fStack_1a0) +
                   ((float)uStack_190 - fStack_1a8) * ((float)uStack_190 - fStack_1a8) +
                   (uStack_190._4_4_ - fStack_1a4) * (uStack_190._4_4_ - fStack_1a4));
      uVar2 = uStack_b8;
      local_1b8 = local_c0;
      uStack_1b0 = local_a0;
      for (fVar9 = (__real_3f800000 - fVar9) * fVar8 - in_XMM1_Da; fVar9 < 0.0;
          fVar9 = fVar9 + fVar8) {
        *(int *)(in_RCX + 3) = (int)uVar2;
        *(undefined4 *)((longlong)in_RCX + 0x1c) = 0x3f800000;
        take_until::take_until::impl$2::
        next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>
                  ();
        if (pfVar6 == (float *)0x0) {
          return;
        }
        (*(code *)*local_180)(&fStack_108,extraout_RAX_00,pfVar6);
        local_110 = extraout_RAX_00;
        if ((int)local_1b8 == 1) {
          fStack_1a0 = (float)local_188;
          fStack_1a8 = (float)uStack_190;
          fStack_1a4 = (float)((ulonglong)uStack_190 >> 0x20);
          uStack_190 = CONCAT44(fStack_104,fStack_108);
          local_188 = local_100;
          pfVar5 = &fStack_104;
          pfVar7 = (float *)&local_100;
          pfVar6 = &fStack_108;
          local_198 = extraout_RAX_00;
        }
        else {
          local_e0 = local_100;
          local_f8[0] = 1;
          local_d8 = &local_180;
          itertools::tuple_impl::impl$31::
          collect_from_iter_no_buf<tuple_<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec3::Vec3>_>,ref_mut_<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>,utils::curve::curve_slice::impl_6::extend_right_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>_>_>_>_>
                    ();
          local_188 = local_90;
          local_1b8 = local_c0;
          uStack_1b0 = uStack_b8;
          local_198 = local_a0;
          uStack_190 = uStack_98;
          fStack_1a8 = (float)local_b0;
          fStack_1a4 = (float)((ulonglong)local_b0 >> 0x20);
          fStack_1a0 = (float)uStack_a8;
          pfVar5 = (float *)((longlong)&uStack_190 + 4);
          pfVar7 = (float *)&local_188;
          pfVar6 = (float *)&uStack_190;
          if ((local_c0 & 1) == 0) {
            return;
          }
        }
        uVar4 = (undefined4)uStack_1b0;
        fVar8 = SQRT((*pfVar7 - fStack_1a0) * (*pfVar7 - fStack_1a0) +
                     (*pfVar6 - fStack_1a8) * (*pfVar6 - fStack_1a8) +
                     (*pfVar5 - fStack_1a4) * (*pfVar5 - fStack_1a4));
        uVar2 = uStack_1b0;
        uStack_1b0 = local_198;
      }
      *(undefined4 *)(in_RCX + 3) = uVar4;
      *(float *)((longlong)in_RCX + 0x1c) = fVar3 - fVar9 / fVar8;
    }
  }
  return;
}

