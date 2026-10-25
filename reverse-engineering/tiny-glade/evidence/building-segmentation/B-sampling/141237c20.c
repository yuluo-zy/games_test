
void __cdecl
itertools::tuple_impl::impl$7::
next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,tuple_<tuple_<usize,glam::f32::vec3::Vec3>,tuple_<usize,glam::f32::vec3::Vec3>_>_>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong extraout_RAX;
  undefined8 uVar5;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  int *in_RDX;
  undefined8 uVar6;
  int local_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_68;
  undefined4 local_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  ulonglong local_50;
  undefined8 uStack_48;
  int *local_38;
  
  if (((char)in_RDX[0x28] == '\0') && ((char)in_RDX[0x26] == '\0')) {
    uVar1 = *(ulonglong *)(in_RDX + 0x24);
    if (*(ulonglong *)(in_RDX + 0x22) <= uVar1) {
      if (*(ulonglong *)(in_RDX + 0x22) < uVar1) {
        *(ulonglong *)(in_RDX + 0x24) = uVar1 - 1;
      }
      else {
        *(undefined1 *)(in_RDX + 0x26) = 1;
      }
      if ((ulonglong)(*(longlong **)(in_RDX + 0x20))[1] <= uVar1) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      if ((*(byte *)(**(longlong **)(in_RDX + 0x20) + uVar1) & 0xf) != 0) {
        *(undefined1 *)(in_RDX + 0x28) = 1;
      }
      if (*(longlong *)(in_RDX + 0x1e) == 0) {
        ::utils::loop_range::impl$3::next_back();
        if (local_b0 == 1) {
          lVar2 = *(longlong *)(in_RDX + 0x16);
          uVar3 = *(ulonglong *)(in_RDX + 0x18);
          ::utils::loop_range::LoopIdx::index();
          if (uVar3 <= extraout_RAX) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          lVar2 = lVar2 + extraout_RAX * 0xc;
          goto joined_r0x000141237e5d;
        }
      }
      else {
        in_RDX[0x1e] = 0;
        in_RDX[0x1f] = 0;
        core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>
                  ();
        lVar2 = extraout_RAX_00;
joined_r0x000141237e5d:
        if (lVar2 != 0) {
          (*(code *)**(undefined8 **)(in_RDX + 0xe))(&uStack_68,uVar1);
          if (*in_RDX == 1) {
            uVar6 = *(undefined8 *)(in_RDX + 0xc);
            uVar5 = *(undefined8 *)(in_RDX + 8);
            uVar4 = *(undefined8 *)(in_RDX + 10);
            *(ulonglong *)(in_RDX + 8) = uVar1;
            *(undefined8 *)(in_RDX + 10) = uStack_68;
            *(ulonglong *)(in_RDX + 0xc) = CONCAT44(uStack_5c,local_60);
            *(undefined8 *)(in_RDX + 2) = uVar5;
            *(undefined8 *)(in_RDX + 4) = uVar4;
            *(undefined8 *)(in_RDX + 6) = uVar6;
            uVar5 = *(undefined8 *)(in_RDX + 2);
            uVar4 = *(undefined8 *)(in_RDX + 4);
            *(int *)(in_RCX + 3) = (int)uVar6;
            in_RCX[2] = uVar4;
            uVar6 = *(undefined8 *)(in_RDX + 8);
            *(undefined4 *)(in_RCX + 6) = local_60;
          }
          else {
            uStack_48 = uStack_68;
            local_58 = 1;
            local_50 = uVar1;
            local_38 = in_RDX + 0xe;
            tuple_impl::impl$31::
            collect_from_iter_no_buf<tuple_<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<usize,glam::f32::vec3::Vec3>_>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,utils::curve::curve_slice::impl_6::extend_left_until::closure_env_1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>_>_>_>_>
                      ();
            *(undefined8 *)(in_RDX + 0xc) = local_80;
            *(undefined8 *)(in_RDX + 8) = local_90;
            *(undefined8 *)(in_RDX + 10) = uStack_88;
            *(undefined8 *)(in_RDX + 4) = local_a0;
            *(undefined8 *)(in_RDX + 6) = uStack_98;
            *(ulonglong *)in_RDX = CONCAT44(uStack_ac,local_b0);
            *(undefined8 *)(in_RDX + 2) = uStack_a8;
            if (*in_RDX != 1) goto LAB_141237e63;
            uVar5 = *(undefined8 *)(in_RDX + 2);
            uVar6 = *(undefined8 *)(in_RDX + 4);
            *(int *)(in_RCX + 3) = in_RDX[6];
            in_RCX[2] = uVar6;
            uVar6 = *(undefined8 *)(in_RDX + 8);
            uStack_68 = *(undefined8 *)(in_RDX + 10);
            *(int *)(in_RCX + 6) = in_RDX[0xc];
          }
          in_RCX[5] = uStack_68;
          in_RCX[1] = uVar5;
          in_RCX[4] = uVar6;
          uVar5 = 1;
          goto LAB_141237c58;
        }
      }
LAB_141237e63:
      uVar5 = 0;
      goto LAB_141237c58;
    }
  }
  uVar5 = 0;
LAB_141237c58:
  *in_RCX = uVar5;
  return;
}

