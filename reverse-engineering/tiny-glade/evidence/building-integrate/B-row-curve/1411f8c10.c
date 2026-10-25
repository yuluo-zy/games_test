
void __cdecl
utils::curve::Curve::
get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong in_RCX;
  longlong lVar4;
  ulonglong uVar5;
  float in_XMM1_Da;
  undefined1 local_90 [24];
  undefined **local_78;
  undefined8 local_70;
  undefined1 **local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined1 *local_48;
  code *local_40;
  undefined1 *local_38;
  code *local_30;
  float local_24;
  float local_20;
  undefined1 local_1c [4];
  undefined8 local_18;
  
  local_18 = 0xfffffffffffffffe;
  if (0x7f7fffff < (uint)ABS(in_XMM1_Da)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((in_XMM1_Da < __real_3f800000) && (0.0 < in_XMM1_Da)) {
    lVar2 = *(longlong *)(in_RCX + 0x20);
    uVar3 = *(ulonglong *)(in_RCX + 0x28);
    if (uVar3 == 1) {
      lVar4 = 0;
    }
    else {
      if (uVar3 == 0) goto LAB_1411f8d55;
      lVar4 = 0;
      uVar5 = uVar3;
      do {
        lVar1 = (uVar5 >> 1) + lVar4;
        if (*(float *)(lVar2 + lVar1 * 4) <= in_XMM1_Da) {
          lVar4 = lVar1;
        }
        uVar5 = uVar5 - (uVar5 >> 1);
      } while (1 < uVar5);
    }
    uVar5 = (lVar4 + 1) - (ulonglong)(in_XMM1_Da < *(float *)(lVar2 + lVar4 * 4));
    if (uVar3 <= uVar5) {
LAB_1411f8d55:
      itertools::Itertools::
      collect_vec<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>,utils::curve::impl_4::get_coord_at_u::closure_env_2<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>_>_>
                ();
      local_48 = local_1c;
      local_40 = core::fmt::float::impl$5::fmt;
      local_30 = alloc::vec::impl$27::fmt<f32,alloc::alloc::Global>;
      local_78 = &PTR_s_invalid_i1__u____points_u__142c18d60;
      local_70 = 2;
      local_58 = 0;
      local_68 = &local_48;
      local_60 = 2;
      local_38 = local_90;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    if (uVar5 == 0) {
      itertools::Itertools::
      collect_vec<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>,utils::curve::impl_4::get_coord_at_u::closure_env_2<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>_>_>
                ();
      local_48 = local_1c;
      local_40 = core::fmt::float::impl$5::fmt;
      local_30 = alloc::vec::impl$27::fmt<f32,alloc::alloc::Global>;
      local_78 = &PTR_s_i1_must_be_>_0__u__142c18de0;
      local_70 = 2;
      local_58 = 0;
      local_68 = &local_48;
      local_60 = 2;
      local_38 = local_90;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    if (uVar3 <= uVar5 - 1) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    local_20 = *(float *)(lVar2 + (uVar5 - 1) * 4);
    local_24 = *(float *)(lVar2 + uVar5 * 4);
    if ((local_20 == local_24) && (!NAN(local_20) && !NAN(local_24))) {
      local_78 = (undefined **)0x0;
                    /* WARNING: Subroutine does not return */
      core::panicking::assert_failed<f32,f32>();
    }
    inverse_lerp();
  }
  return;
}

