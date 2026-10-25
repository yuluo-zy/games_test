
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_critters::butterfly::update_butterfly_goals(void)

{
  uint *puVar1;
  int *piVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  char cVar8;
  undefined4 uVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  bool bVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  longlong lVar19;
  longlong lVar20;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  uint *puVar24;
  ulonglong uVar25;
  longlong *in_R8;
  longlong lVar26;
  uint *puVar27;
  float fVar28;
  float extraout_XMM0_Da;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  uint uVar33;
  longlong local_160;
  longlong local_150;
  longlong local_148;
  float local_13c;
  float local_138;
  float local_134;
  longlong local_130;
  longlong local_128;
  longlong local_118;
  
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  fVar18 = __real_3fe147ae;
  fVar17 = __real_42820000;
  fVar16 = __real_41000000;
  fVar15 = __real_3f800000;
  fVar14 = __real_3f000000;
  fVar5 = *(float *)(*in_R8 + 0x28);
  lVar11 = *in_RCX;
  lVar12 = in_RCX[1];
  uVar9 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar27 = *(uint **)(lVar11 + 0xf8);
  puVar1 = puVar27 + *(longlong *)(lVar11 + 0x100);
  cVar8 = *(char *)(lVar11 + 0x108);
  local_118 = 8;
  local_160 = 0;
  local_148 = 0;
  local_150 = 0;
  uVar25 = 0;
  uVar22 = 0;
  do {
    if (cVar8 == '\0') {
      if (uVar22 == uVar25) {
        do {
          if (puVar27 == puVar1) {
            return;
          }
          uVar33 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar25 = *(ulonglong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar33 * 0xa0);
        } while (uVar25 == 0);
        lVar19 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar33 * 0xa0;
        uVar22 = (ulonglong)*(uint *)(lVar19 + 0x94);
        lVar20 = *(longlong *)(lVar12 + 0x1a8) + uVar22 * 0x48;
        uVar22 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x40 + uVar22 * 0x48);
        if ((*(ulonglong *)(lVar11 + 0x110) < uVar22) &&
           (uVar23 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
           uVar23 != 0)) {
          local_160 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar23 * 0x30);
        }
        else {
          local_160 = 0;
        }
        if (*(ulonglong *)(lVar11 + 0x118) < uVar22) {
          lVar21 = *(longlong *)(lVar20 + 0x18);
        }
        else {
          lVar21 = *(longlong *)(lVar20 + 0x18);
        }
        lVar26 = ~*(ulonglong *)(*(longlong *)(lVar20 + 0x38) + *(longlong *)(lVar11 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar21 + 0x10 + lVar26);
        local_128 = *(longlong *)(lVar21 + 0x28 + lVar26);
        lVar20 = ~*(ulonglong *)(*(longlong *)(lVar20 + 0x38) + *(longlong *)(lVar11 + 0x128) * 8) *
                 0x30;
        local_150 = *(longlong *)(lVar21 + 0x10 + lVar20);
        local_130 = *(longlong *)(lVar21 + 0x28 + lVar20);
        local_118 = *(longlong *)(lVar19 + 0x50);
        uVar22 = 0;
      }
      uVar23 = (ulonglong)*(uint *)(local_118 + 8 + uVar22 * 0x10);
    }
    else {
      if (uVar22 == uVar25) {
        do {
          if (puVar27 == puVar1) {
            return;
          }
          uVar33 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar25 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)uVar33 * 0x48);
        } while (uVar25 == 0);
        lVar20 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)uVar33 * 0x48;
        if ((*(ulonglong *)(lVar11 + 0x110) < *(ulonglong *)(lVar20 + 0x40)) &&
           (uVar22 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
           uVar22 != 0)) {
          local_160 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
          local_160 = 0;
        }
        if (*(ulonglong *)(lVar11 + 0x118) < *(ulonglong *)(lVar20 + 0x40)) {
          lVar19 = *(longlong *)(lVar20 + 0x18);
        }
        else {
          lVar19 = *(longlong *)(lVar20 + 0x18);
        }
        lVar21 = ~*(ulonglong *)(*(longlong *)(lVar20 + 0x38) + *(longlong *)(lVar11 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar19 + 0x10 + lVar21);
        local_128 = *(longlong *)(lVar19 + 0x28 + lVar21);
        lVar20 = ~*(ulonglong *)(*(longlong *)(lVar20 + 0x38) + *(longlong *)(lVar11 + 0x128) * 8) *
                 0x30;
        local_150 = *(longlong *)(lVar19 + 0x10 + lVar20);
        local_130 = *(longlong *)(lVar19 + 0x28 + lVar20);
        uVar22 = 0;
      }
      uVar23 = uVar22 & 0xffffffff;
    }
    piVar2 = (int *)(local_148 + uVar23 * 0x18);
    lVar20 = uVar23 * 0x30 + local_160;
    puVar3 = (undefined4 *)(local_128 + uVar23 * 4);
    puVar4 = (undefined4 *)(local_130 + uVar23 * 4);
    puVar24 = (uint *)(uVar23 * 0x10 + local_150);
    uVar22 = uVar22 + 1;
    *puVar3 = uVar9;
    fVar28 = (float)piVar2[2] - fVar5;
    piVar2[2] = (int)fVar28;
    uVar33 = *puVar24;
    if (((fVar28 <= 0.0) || ((uVar33 & 1) == 0)) ||
       (fVar28 = (float)puVar24[2] - *(float *)(lVar20 + 0x14),
       fVar29 = (float)puVar24[1] - *(float *)(lVar20 + 0x10),
       fVar30 = (float)puVar24[3] - *(float *)(lVar20 + 0x18),
       fVar30 * fVar30 + fVar28 * fVar28 + fVar29 * fVar29 < __real_3c23d70b)) {
      perchance::PerchanceContext::uniform_f32();
      fVar28 = extraout_XMM0_Da * fVar14 + __real_3dcccccd;
      *puVar3 = uVar9;
      piVar2[2] = (int)fVar28;
      iVar10 = *piVar2;
      perchance::PerchanceContext::uniform_f32();
      perchance::PerchanceContext::uniform_f32();
      ::utils::uniform_sample_sphere();
      ::utils::swizzlers::impl$3::x0z();
      uVar33 = -(uint)(uVar33 == _UNK_142aebb84);
      fVar30 = (float)(-(uint)(iVar10 == ___xmm_00000000000000000000000000000001) &
                       ___xmm_0000000000000000be91745d3f000000 |
                      ~-(uint)(iVar10 == ___xmm_00000000000000000000000000000001) &
                      ___xmm_00000000000000003f5a2e8c3f800000);
      fVar31 = (float)(uVar33 & _UNK_142f67d24 | ~uVar33 & _UNK_142f67d14);
      fVar32 = local_138 * fVar30 * fVar14 + local_138 * fVar31;
      fVar28 = (float)piVar2[2] * fVar18;
      fVar29 = fVar31 * local_13c + fVar30 * local_13c;
      fVar30 = fVar31 * local_134 + fVar30 * local_134;
      fVar31 = fVar15 / SQRT(fVar30 * fVar30 + fVar32 * fVar32 + fVar29 * fVar29);
      bVar13 = (int)fVar31 - 1U < 0x7fffff ||
               (int)ABS(fVar31) - 0x800000U < 0x7f000000 && -1 < (int)fVar31;
      if (bVar13) {
        fVar32 = fVar32 * fVar31;
      }
      else {
        fVar32 = 0.0;
      }
      fVar6 = *(float *)(lVar20 + 0x10);
      fVar7 = *(float *)(lVar20 + 0x18);
      uVar33 = (uint)bVar13;
      fVar32 = fVar32 * fVar28 + *(float *)(lVar20 + 0x14);
      fVar29 = fVar6 + fVar28 * (float)((int)(uVar33 << 0x1f) >> 0x1f & (uint)(fVar29 * fVar31));
      fVar28 = fVar7 + fVar28 * (float)((int)(uVar33 << 0x1f) >> 0x1f & (uint)(fVar30 * fVar31));
      if (fVar16 <= fVar32) {
        fVar32 = fVar16;
      }
      fVar30 = SQRT(fVar28 * fVar28 + fVar29 * fVar29);
      if ((fVar30 < fVar17) || (fVar30 < SQRT(fVar7 * fVar7 + fVar6 * fVar6))) {
        *puVar4 = uVar9;
        *puVar24 = 1;
        puVar24[1] = (uint)fVar29;
        puVar24[2] = (uint)fVar32;
        puVar24[3] = (uint)fVar28;
      }
      else {
        *puVar4 = uVar9;
        *puVar24 = 0;
      }
    }
  } while( true );
}

