
void __cdecl
utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
          (void)

{
  float fVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  float *pfVar4;
  longlong extraout_RAX;
  longlong *in_RCX;
  float *pfVar5;
  longlong *in_RDX;
  ulonglong uVar6;
  longlong lVar7;
  ulonglong in_R8;
  longlong lVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  undefined8 uVar12;
  float fVar13;
  
  uVar3 = in_RDX[2];
  if (uVar3 < 2) {
    *(undefined1 *)(in_RCX + 1) = 0;
    *(char *)((longlong)in_RCX + 9) = (char)uVar3;
    *in_RCX = -0x8000000000000000;
    if (*in_RDX == 0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  pfVar4 = (float *)in_RDX[1];
  pfVar5 = pfVar4 + 2;
  fVar9 = 0.0;
  uVar6 = 0;
  fVar10 = pfVar4[2];
  uVar12 = *(undefined8 *)pfVar4;
  do {
    fVar1 = *pfVar5;
    fVar10 = fVar10 - fVar1;
    uVar2 = *(undefined8 *)(pfVar5 + -2);
    fVar11 = (float)uVar12 - (float)uVar2;
    fVar13 = (float)((ulonglong)uVar12 >> 0x20) - (float)((ulonglong)uVar2 >> 0x20);
    fVar9 = fVar9 + SQRT(fVar10 * fVar10 + fVar13 * fVar13 + fVar11 * fVar11);
    *(float *)(extraout_RAX + uVar6 * 4) = fVar9;
    uVar6 = uVar6 + 1;
    pfVar5 = pfVar5 + 3;
    fVar10 = fVar1;
    uVar12 = uVar2;
  } while (uVar3 != uVar6);
  if ((fVar9 != 0.0) || (NAN(fVar9))) {
    lVar7 = 0;
    do {
      if (uVar3 * 4 - lVar7 == 0) {
        if ((in_R8 & 1) == 0) goto LAB_1411f9d9c;
        fVar10 = pfVar4[3];
        if ((*pfVar4 == fVar10) && (!NAN(*pfVar4) && !NAN(fVar10))) {
          if ((pfVar4[1] == pfVar4[4]) && (!NAN(pfVar4[1]) && !NAN(pfVar4[4]))) {
            if ((pfVar4[2] == pfVar4[5]) && (!NAN(pfVar4[2]) && !NAN(pfVar4[5]))) {
              lVar7 = 0;
              goto LAB_1411f9e1f;
            }
          }
        }
        if (uVar3 == 2) goto LAB_1411f9d9c;
        lVar7 = 1;
        lVar8 = 0;
        goto LAB_1411f9dec;
      }
      fVar10 = *(float *)(extraout_RAX + lVar7) / fVar9;
      *(float *)(extraout_RAX + lVar7) = fVar10;
      lVar7 = lVar7 + 4;
    } while ((uint)ABS(fVar10) < 0x7f800000);
  }
  *(undefined1 *)(in_RCX + 1) = 1;
LAB_1411f9d20:
  *in_RCX = -0x8000000000000000;
  std::alloc::__default_lib_allocator::__rust_dealloc();
  if (*in_RDX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_1411f9dec:
  fVar1 = *(float *)((longlong)pfVar4 + lVar8 + 0x18);
  if ((fVar10 == fVar1) && (!NAN(fVar10) && !NAN(fVar1))) {
    fVar10 = *(float *)((longlong)pfVar4 + lVar8 + 0x10);
    pfVar5 = (float *)((longlong)pfVar4 + lVar8 + 0x1c);
    if ((fVar10 == *pfVar5) && (!NAN(fVar10) && !NAN(*pfVar5))) {
      fVar10 = *(float *)((longlong)pfVar4 + lVar8 + 0x14);
      pfVar5 = (float *)((longlong)pfVar4 + lVar8 + 0x20);
      if ((fVar10 == *pfVar5) && (!NAN(fVar10) && !NAN(*pfVar5))) goto LAB_1411f9e1f;
    }
  }
  lVar8 = lVar8 + 0xc;
  lVar7 = lVar7 + 1;
  fVar10 = fVar1;
  if (uVar3 * 0xc + -0x18 == lVar8) {
LAB_1411f9d9c:
    in_RCX[2] = in_RDX[2];
    lVar7 = in_RDX[1];
    *in_RCX = *in_RDX;
    in_RCX[1] = lVar7;
    in_RCX[3] = uVar3;
    in_RCX[4] = extraout_RAX;
    in_RCX[5] = uVar3;
    *(float *)(in_RCX + 6) = fVar9;
    return;
  }
  goto LAB_1411f9dec;
LAB_1411f9e1f:
  *(undefined1 *)(in_RCX + 1) = 2;
  in_RCX[2] = lVar7;
  goto LAB_1411f9d20;
}

