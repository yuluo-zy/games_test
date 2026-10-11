
void __cdecl
system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate(void)

{
  uint uVar1;
  uint uVar2;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong in_RCX;
  longlong in_RDX;
  int iVar3;
  longlong in_R8;
  ulonglong in_R9;
  
  if ((in_RCX == 0) || (::utils::loop_range::LoopRange::len(), extraout_RAX <= in_RCX + 1)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_R9 <= in_RCX) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if ((*(byte *)(in_R8 + in_RCX) & 0xf) == 0) {
    ::utils::loop_range::LoopRange::len();
    if (extraout_RAX_00 <= in_RCX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    ::utils::loop_range::LoopRange::len();
    ::utils::loop_range::LoopRange::len();
    ::utils::curve::curve_slice::CurveSlice::
    extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_0>
              ();
    ::utils::curve::curve_slice::CurveSlice::
    extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env_1>
              ();
    ::utils::curve::impl$12::
    sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>
              ();
    ::utils::curve::impl$12::
    sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>
              ();
    uVar1 = *(uint *)(in_RDX + 0x18);
    if (uVar1 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_const::panic_const_rem_by_zero();
    }
    iVar3 = (int)in_RCX + *(int *)(in_RDX + 0x10);
    if (iVar3 == -0x80000000 && uVar1 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_const::panic_const_rem_overflow();
    }
    iVar3 = iVar3 % (int)uVar1;
    uVar2 = -uVar1;
    if ((int)-uVar1 < 0) {
      uVar2 = uVar1;
    }
    if (*(ulonglong *)(in_RDX + 8) <= (ulonglong)(longlong)(int)((iVar3 >> 0x1f & uVar2) + iVar3)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
  }
  return;
}

