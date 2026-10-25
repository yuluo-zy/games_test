
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::geometry::rectangle::Rectangle2d::lerp(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  float *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  uint uVar6;
  uint uVar7;
  float fVar8;
  float fVar9;
  float in_XMM3_Da;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  
  uVar5 = ___xmm_80000000800000008000000080000000;
  fVar13 = (float)*in_RDX;
  fVar14 = (float)((ulonglong)*in_RDX >> 0x20);
  uVar1 = in_RDX[1];
  uVar2 = in_RDX[2];
  uVar3 = in_R8[1];
  uVar4 = in_R8[2];
  fVar12 = (float)*in_R8;
  fVar15 = (float)((ulonglong)*in_R8 >> 0x20);
  fVar9 = (fVar14 * fVar15 + fVar13 * fVar12) /
          SQRT((fVar15 * fVar15 + fVar12 * fVar12) * (fVar14 * fVar14 + fVar13 * fVar13));
  fVar11 = (float)(___xmm_7fffffff7fffffff7fffffff7fffffff & (uint)fVar9);
  fVar8 = 0.0;
  if (0.0 <= __real_3f800000 - fVar11) {
    fVar8 = __real_3f800000 - fVar11;
  }
  fVar11 = (((((((__real_baa57a2c * fVar11 + ___real_3bda90c5) * fVar11 + ___real_bc8bfc66) * fVar11
               + ___real_3cfd10f8) * fVar11 + ___real_bd4d8392) * fVar11 + ___real_3db63a9e) *
             fVar11 + ___real_be5bbfca) * fVar11 + ___real_3fc90fda) * SQRT(fVar8);
  uVar6 = -(uint)(fVar9 < 0.0);
  fVar8 = fVar15 * fVar13 - fVar14 * fVar12;
  uVar7 = -(uint)NAN(fVar8);
  fVar15 = __real_3f800000 - in_XMM3_Da;
  fVar9 = (float)(__real_7fc00000 & uVar7 |
                 ~uVar7 & ((uint)fVar8 & ___xmm_80000000800000008000000080000000 |
                          (uint)__xmm_3f8000003f8000003f8000003f800000)) *
          (float)(uVar6 & (uint)(__real_40490fdb - fVar11) | ~uVar6 & (uint)fVar11) * in_XMM3_Da +
          fVar15 * 0.0;
  fVar8 = sinf(fVar9);
  fVar9 = cosf(fVar9);
  fVar12 = (float)uVar4 * in_XMM3_Da + (float)uVar2 * fVar15;
  fVar11 = (float)((ulonglong)uVar4 >> 0x20) * in_XMM3_Da +
           (float)((ulonglong)uVar2 >> 0x20) * fVar15;
  if (0x7f7fffff < (uint)ABS(fVar12)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (0x7f7fffff < (uint)ABS(fVar11)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar10 = (float)(uVar5 ^ (uint)fVar8) * fVar14 + fVar9 * fVar13;
  fVar8 = fVar9 * fVar14 + fVar8 * fVar13;
  if ((float)((uint)(fVar8 * fVar8 + fVar10 * fVar10 + __real_bf800000) &
             ___xmm_7fffffff7fffffff7fffffff7fffffff) <= __real_3951b717) {
    *in_RCX = fVar10;
    in_RCX[1] = fVar8;
    in_RCX[2] = fVar15 * (float)uVar1 + (float)uVar3 * in_XMM3_Da;
    in_RCX[3] = fVar15 * (float)((ulonglong)uVar1 >> 0x20) +
                (float)((ulonglong)uVar3 >> 0x20) * in_XMM3_Da;
    *(ulonglong *)(in_RCX + 4) = CONCAT44(fVar11,fVar12);
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

