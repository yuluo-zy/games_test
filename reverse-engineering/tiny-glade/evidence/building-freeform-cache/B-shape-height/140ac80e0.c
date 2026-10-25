
void __cdecl country_core::systems::wall::shapes::calc_shape_height::calc_shape_height(void)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  char *pcVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  undefined8 *in_RCX;
  longlong lVar7;
  uint uVar8;
  float fVar9;
  float extraout_XMM0_Da;
  float fVar10;
  float fVar11;
  float fVar12;
  uint uVar13;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_68;
  longlong local_50;
  
  piVar3 = (int *)*in_RCX;
  if ((*piVar3 != 6) || (1 < (byte)((char)piVar3[1] - 1U))) {
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    *(undefined4 *)*in_stack_00000030 = 0;
    return;
  }
  if ((char)piVar3[1] == '\x02') {
    ::utils::geometry::rectangle::Rectangle2d::as_curve2();
  }
  else {
    ::utils::geometry::circle::Circle2d::as_curve2();
  }
  ::utils::curve::Curve::
  crashy_resample_remap_per_segment<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::shapes::calc_shape_height::calc_shape_height::closure_env_0>
            ();
  if (local_90 == 0) {
    fVar10 = 0.0;
    fVar9 = 0.0;
  }
  else {
    lVar7 = 0;
    fVar10 = 0.0;
    fVar9 = 0.0;
    do {
      fVar11 = *(float *)(local_98 + 4 + lVar7);
      fVar10 = fVar10 + fVar11;
      fVar12 = fVar11;
      if (fVar11 <= fVar9) {
        fVar12 = fVar9;
      }
      fVar9 = (float)(~-(uint)NAN(fVar9) & (uint)fVar12 | -(uint)NAN(fVar9) & (uint)fVar11);
      lVar7 = lVar7 + 0xc;
    } while (local_90 * 0xc != lVar7);
  }
  if (*piVar3 == 6 && (char)piVar3[1] == '\x02') {
    pcVar4 = (char *)*in_stack_00000028;
    iVar1 = *(int *)(pcVar4 + 8);
    fVar11 = __real_40c00000;
  }
  else {
    if ((*piVar3 != 6) || ((char)piVar3[1] != '\x01')) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    pcVar4 = (char *)*in_stack_00000028;
    iVar1 = *(int *)(pcVar4 + 8);
    fVar11 = __real_41000000;
  }
  if (iVar1 == 2) {
    if (*pcVar4 == '\0') {
      fVar10 = 0.0;
    }
    else {
      fVar10 = *(float *)(pcVar4 + 4);
    }
    fVar11 = fVar11 + fVar10;
  }
  else {
    fVar11 = fVar10 / (float)local_90 + fVar11 + __real_beeb851f;
  }
  fVar11 = fVar11 - (fVar9 + __real_beeb851f);
  fVar9 = __real_3f333333;
  if (__real_3f333333 <= fVar11) {
    fVar9 = fVar11;
  }
  fVar10 = __real_41600000;
  if (fVar9 <= __real_41600000) {
    fVar10 = fVar9;
  }
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  *(float *)(pcVar4 + 0x40) = fVar10;
  fVar9 = *(float *)(*in_stack_00000028 + 0x3c);
  uVar8 = -(uint)(fVar9 == 0.0);
  uVar13 = __real_c0200000 & uVar8;
  uVar2 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  puVar5 = (undefined4 *)in_stack_00000030[2];
  *puVar5 = uVar2;
  puVar6 = (undefined4 *)*in_stack_00000030;
  puVar6[2] = ~uVar8 & (uint)fVar9 | uVar13;
  ::utils::time_continuous_lerp();
  *puVar5 = uVar2;
  *puVar6 = 1;
  puVar6[1] = extraout_XMM0_Da;
  fVar9 = (float)puVar6[2];
  *puVar5 = uVar2;
  puVar6[3] = extraout_XMM0_Da - fVar9;
  if (local_a0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_88 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_68 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_50 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

