
void __cdecl country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge(void)

{
  undefined4 uVar1;
  undefined4 *in_RCX;
  longlong in_RDX;
  float *in_R8;
  float fVar2;
  float fVar3;
  float fVar4;
  float in_XMM3_Da;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 local_c8;
  float local_c0;
  float local_b8;
  float fStack_b4;
  float fStack_b0;
  undefined8 local_98;
  float local_90;
  
  if ((*(byte *)(in_RDX + 8) & 1) == 0) {
    fVar3 = *(float *)(in_RDX + 0x30);
    if (fVar3 <= 0.1) {
      fVar3 = in_R8[1];
      ::utils::swizzlers::impl$1::xvy();
    }
    else {
      fVar4 = ((*(float *)(in_RDX + 0x14) + -0.90000004) / 7.1) / 0.4;
      fVar2 = 0.0;
      if (0.0 <= fVar4) {
        fVar2 = fVar4;
      }
      fVar4 = 1.0;
      if (fVar2 <= 1.0) {
        fVar4 = fVar2;
      }
      fVar2 = (3.0 - (fVar4 + fVar4)) * fVar4 * fVar4;
      fVar2 = fVar2 * 12.0 + (1.0 - fVar2) * 8.0;
      if (fVar2 < 0.0) goto LAB_140981377;
      fVar4 = (1.0 - fVar3) * 0.4 + fVar3 * 12.0;
      fVar3 = 0.0;
      if (0.0 <= fVar4) {
        fVar3 = fVar4;
      }
      if (fVar3 <= fVar2) {
        fVar2 = fVar3;
      }
      local_b8 = fVar2 * 0.0 + *(float *)(in_RDX + 0xc);
      fStack_b4 = fVar2 + *(float *)(in_RDX + 0x4c);
      fStack_b0 = fVar2 * 0.0 + *(float *)(in_RDX + 0x10);
      fVar3 = in_R8[1];
    }
    uVar1 = 0;
    if (SQRT((in_R8[2] - fStack_b0) * (in_R8[2] - fStack_b0) +
             (*in_R8 - local_b8) * (*in_R8 - local_b8) + (fVar3 - fStack_b4) * (fVar3 - fStack_b4))
        < in_XMM3_Da) {
      in_RCX[3] = fStack_b0;
      *(ulonglong *)(in_RCX + 1) = CONCAT44(fStack_b4,local_b8);
      uVar1 = 1;
    }
  }
  else {
    fVar3 = *(float *)(in_RDX + 0x34);
    fVar2 = (1.0 - fVar3) * 0.1;
    if (*(char *)(in_RDX + 0x3c) == '\0') {
      fVar3 = fVar2 + fVar3 * *(float *)(in_RDX + 0x1c);
      fVar2 = 0.1;
    }
    else {
      fVar2 = fVar2 + fVar3 * *(float *)(in_RDX + 0x20);
      fVar3 = 0.1;
    }
    if (0.1 < *(float *)(in_RDX + 0x30)) {
      fVar4 = *(float *)(in_RDX + 0x1c);
      fVar7 = *(float *)(in_RDX + 0x20);
      fVar10 = fVar7;
      if (fVar7 <= fVar4) {
        fVar10 = fVar4;
      }
      fVar7 = (((float)(~-(uint)NAN(fVar4) & (uint)fVar10 | -(uint)NAN(fVar4) & (uint)fVar7) + -2.25
               ) / 12.75) / 0.4;
      fVar4 = 0.0;
      if (0.0 <= fVar7) {
        fVar4 = fVar7;
      }
      fVar7 = 1.0;
      if (fVar4 <= 1.0) {
        fVar7 = fVar4;
      }
      fVar4 = (3.0 - (fVar7 + fVar7)) * fVar7 * fVar7;
      if (fVar4 * 12.0 + (1.0 - fVar4) * 9.0 < 0.0) {
LAB_140981377:
                    // WARNING: Subroutine does not return
        core::f32::impl$0::clamp::do_panic::runtime();
      }
    }
    fVar4 = 0.01;
    if (fVar3 <= fVar2) {
      fVar4 = fVar2;
    }
    fVar3 = (float)(~-(uint)(fVar2 < fVar3) & 0x3c23d70a | (uint)fVar3 & -(uint)(fVar2 < fVar3));
    fVar2 = (float)((ulonglong)local_98 >> 0x20);
    fVar7 = (float)((ulonglong)local_c8 >> 0x20);
    if (*(float *)(in_RDX + 0x30) <= 0.1) {
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      fVar10 = in_R8[1];
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      ::utils::swizzlers::impl$1::xvy();
    }
    else {
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      ::utils::swizzlers::impl$1::xvy();
      fVar10 = in_R8[1];
    }
    fVar9 = (float)local_98;
    fVar5 = local_c0 - local_90;
    fVar11 = (float)local_c8 - fVar9;
    fVar12 = fVar7 - fVar2;
    fVar6 = fVar5 * fVar5 + fVar12 * fVar12 + fVar11 * fVar11;
    fVar8 = fVar6;
    if (fVar6 <= 1e-20) {
      fVar8 = 1e-20;
    }
    fVar8 = ((in_R8[2] - local_90) * fVar5 + (fVar10 - fVar2) * fVar12 + (*in_R8 - fVar9) * fVar11)
            / fVar8;
    fVar5 = 0.0;
    if (0.0 <= fVar8) {
      fVar5 = fVar8;
    }
    fVar8 = 1.0;
    if (fVar5 <= 1.0) {
      fVar8 = fVar5;
    }
    fVar5 = 1.0 - fVar8;
    fVar11 = in_R8[2] - (local_c0 * fVar8 + local_90 * fVar5);
    fVar9 = *in_R8 - (fVar8 * (float)local_c8 + fVar5 * fVar9);
    fVar10 = fVar10 - (fVar8 * fVar7 + fVar5 * fVar2);
    uVar1 = 0;
    if (SQRT(fVar11 * fVar11 + fVar10 * fVar10 + fVar9 * fVar9) < in_XMM3_Da) {
      fVar2 = fVar4;
      if (fVar4 <= fVar3) {
        fVar2 = fVar3;
      }
      fVar7 = 0.5;
      if (0.5 <= (float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fVar4)) {
        fVar7 = 0.0;
        if ((0.3 / SQRT(fVar6) <= fVar8) && (fVar7 = fVar8, 1.0 - 0.3 / SQRT(fVar6) < fVar8)) {
          fVar7 = 1.0;
        }
      }
      *(undefined8 *)(in_RCX + 2) = local_98;
      in_RCX[4] = local_90;
      *(undefined8 *)(in_RCX + 5) = local_c8;
      in_RCX[7] = local_c0;
      in_RCX[1] = fVar7;
      uVar1 = 2;
    }
  }
  *in_RCX = uVar1;
  return;
}

