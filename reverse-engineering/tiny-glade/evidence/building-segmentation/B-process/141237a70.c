
void __cdecl
itertools::tuple_impl::impl$7::
next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D>_>,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl_6::iter_coords::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D>_>_>,utils::curve::curve_slice::impl_6::measure_length::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>,tuple_<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>_>
          (void)

{
  uint uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined4 *in_RCX;
  ulonglong *in_RDX;
  ulonglong local_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  ulonglong local_2c;
  undefined4 local_24;
  
  if (*in_RDX == 2) {
LAB_141237ad2:
    uVar1 = (uint)in_RDX[5];
    if (uVar1 != 2) {
      local_78 = CONCAT44(local_78._4_4_,uVar1);
      *(undefined4 *)(in_RDX + 5) = 0;
      if ((uVar1 & 1) != 0) goto LAB_141237b13;
    }
LAB_141237c04:
    *in_RCX = 0;
  }
  else {
    uVar1 = (uint)in_RDX[3];
    if ((uVar1 == 2) || (*(uint *)(in_RDX + 3) = (uint)(uVar1 == 0) * 2, (uVar1 & 1) == 0)) {
      if (((*in_RDX & 1) == 0) || ((longlong)in_RDX[2] <= (longlong)in_RDX[1])) goto LAB_141237acb;
      in_RDX[1] = in_RDX[1] + 1;
    }
    else if (uVar1 == 0) {
LAB_141237acb:
      *in_RDX = 2;
      goto LAB_141237ad2;
    }
LAB_141237b13:
    core::ops::function::impls::impl$4::
    call_once<tuple_<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D>_>,utils::curve::curve_slice::impl_6::measure_length::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
              ();
    if ((in_RDX[8] & 1) == 0) {
      tuple_impl::impl$31::
      collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D>_>,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl_6::iter_coords::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D>_>_>,utils::curve::curve_slice::impl_6::measure_length::closure_env_0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>_>_>_>
                ();
      *(ulonglong *)((longlong)in_RDX + 0x4c) = CONCAT44(uStack_68,uStack_6c);
      *(ulonglong *)((longlong)in_RDX + 0x54) = CONCAT44(uStack_60,uStack_64);
      in_RDX[8] = local_78;
      in_RDX[9] = CONCAT44(uStack_6c,uStack_70);
      if ((in_RDX[8] & 1) == 0) goto LAB_141237c04;
      uStack_70 = *(undefined4 *)((longlong)in_RDX + 0x4c);
      local_78 = *(undefined8 *)((longlong)in_RDX + 0x44);
      uStack_6c = (undefined4)in_RDX[10];
      uStack_68 = (undefined4)(in_RDX[10] >> 0x20);
      local_24 = (undefined4)in_RDX[0xb];
    }
    else {
      uVar3 = in_RDX[0xb];
      uVar2 = in_RDX[10];
      in_RDX[10] = local_2c;
      *(undefined4 *)(in_RDX + 0xb) = local_24;
      *(ulonglong *)((longlong)in_RDX + 0x44) = uVar2;
      *(int *)((longlong)in_RDX + 0x4c) = (int)uVar3;
      uStack_70 = *(undefined4 *)((longlong)in_RDX + 0x4c);
      local_78 = *(undefined8 *)((longlong)in_RDX + 0x44);
      uStack_6c = (undefined4)local_2c;
      uStack_68 = (undefined4)(local_2c >> 0x20);
    }
    *(ulonglong *)(in_RCX + 1) = local_78;
    in_RCX[3] = uStack_70;
    in_RCX[4] = uStack_6c;
    *(ulonglong *)(in_RCX + 5) = CONCAT44(local_24,uStack_68);
    *in_RCX = 1;
  }
  return;
}

