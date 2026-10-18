// 0x141d86060 update_butterfly_goals
// SIG: void __cdecl update_butterfly_goals(void)

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
  longlong lVar14;
  longlong lVar15;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  uint *puVar19;
  ulonglong uVar20;
  longlong *in_R8;
  longlong lVar21;
  uint *puVar22;
  float fVar23;
  float extraout_XMM0_Da;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  uint uVar28;
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
  fVar5 = *(float *)(*in_R8 + 0x28);
  lVar11 = *in_RCX;
  lVar12 = in_RCX[1];
  uVar9 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar22 = *(uint **)(lVar11 + 0xf8);
  puVar1 = puVar22 + *(longlong *)(lVar11 + 0x100);
  cVar8 = *(char *)(lVar11 + 0x108);
  local_118 = 8;
  local_160 = 0;
  local_148 = 0;
  local_150 = 0;
  uVar20 = 0;
  uVar17 = 0;
  do {
    if (cVar8 == '\0') {
      if (uVar17 == uVar20) {
        do {
          if (puVar22 == puVar1) {
            return;
          }
          uVar28 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar28 * 0xa0);
        } while (uVar20 == 0);
        lVar14 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar28 * 0xa0;
        uVar17 = (ulonglong)*(uint *)(lVar14 + 0x94);
        lVar15 = *(longlong *)(lVar12 + 0x1a8) + uVar17 * 0x48;
        uVar17 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x40 + uVar17 * 0x48);
        if ((*(ulonglong *)(lVar11 + 0x110) < uVar17) &&
           (uVar18 = *(ulonglong *)
                      (*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
           uVar18 != 0)) {
          local_160 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          local_160 = 0;
        }
        if (*(ulonglong *)(lVar11 + 0x118) < uVar17) {
          lVar16 = *(longlong *)(lVar15 + 0x18);
        }
        else {
          lVar16 = *(longlong *)(lVar15 + 0x18);
        }
        lVar21 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar11 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar16 + 0x10 + lVar21);
        local_128 = *(longlong *)(lVar16 + 0x28 + lVar21);
        lVar15 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar11 + 0x128) * 8) *
                 0x30;
        local_150 = *(longlong *)(lVar16 + 0x10 + lVar15);
        local_130 = *(longlong *)(lVar16 + 0x28 + lVar15);
        local_118 = *(longlong *)(lVar14 + 0x50);
        uVar17 = 0;
      }
      uVar18 = (ulonglong)*(uint *)(local_118 + 8 + uVar17 * 0x10);
    }
    else {
      if (uVar17 == uVar20) {
        do {
          if (puVar22 == puVar1) {
            return;
          }
          uVar28 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)uVar28 * 0x48);
        } while (uVar20 == 0);
        lVar15 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)uVar28 * 0x48;
        if ((*(ulonglong *)(lVar11 + 0x110) < *(ulonglong *)(lVar15 + 0x40)) &&
           (uVar17 = *(ulonglong *)
                      (*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
           uVar17 != 0)) {
          local_160 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar17 * 0x30);
        }
        else {
          local_160 = 0;
        }
        if (*(ulonglong *)(lVar11 + 0x118) < *(ulonglong *)(lVar15 + 0x40)) {
          lVar14 = *(longlong *)(lVar15 + 0x18);
        }
        else {
          lVar14 = *(longlong *)(lVar15 + 0x18);
        }
        lVar16 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar11 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar14 + 0x10 + lVar16);
        local_128 = *(longlong *)(lVar14 + 0x28 + lVar16);
        lVar15 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar11 + 0x128) * 8) *
                 0x30;
        local_150 = *(longlong *)(lVar14 + 0x10 + lVar15);
        local_130 = *(longlong *)(lVar14 + 0x28 + lVar15);
        uVar17 = 0;
      }
      uVar18 = uVar17 & 0xffffffff;
    }
    piVar2 = (int *)(local_148 + uVar18 * 0x18);
    lVar15 = uVar18 * 0x30 + local_160;
    puVar3 = (undefined4 *)(local_128 + uVar18 * 4);
    puVar4 = (undefined4 *)(local_130 + uVar18 * 4);
    puVar19 = (uint *)(uVar18 * 0x10 + local_150);
    uVar17 = uVar17 + 1;
    *puVar3 = uVar9;
    fVar23 = (float)piVar2[2] - fVar5;
    piVar2[2] = (int)fVar23;
    uVar28 = *puVar19;
    if (((fVar23 <= 0.0) || ((uVar28 & 1) == 0)) ||
       (fVar23 = (float)puVar19[2] - *(float *)(lVar15 + 0x14),
       fVar24 = (float)puVar19[1] - *(float *)(lVar15 + 0x10),
       fVar25 = (float)puVar19[3] - *(float *)(lVar15 + 0x18),
       fVar25 * fVar25 + fVar23 * fVar23 + fVar24 * fVar24 < 0.010000001)) {
      perchance::PerchanceContext::uniform_f32();
      *puVar3 = uVar9;
      piVar2[2] = (int)(extraout_XMM0_Da * 0.5 + 0.1);
      iVar10 = *piVar2;
      perchance::PerchanceContext::uniform_f32();
      perchance::PerchanceContext::uniform_f32();
      ::utils::uniform_sample_sphere();
      ::utils::swizzlers::impl$3::x0z();
      uVar28 = -(uint)(uVar28 == 0);
      fVar25 = (float)(-(uint)(iVar10 == 1) & 0x3f000000 | ~-(uint)(iVar10 == 1) & 0x3f800000);
      fVar26 = (float)(uVar28 & 0xbe91745d | ~uVar28 & 0x3f5a2e8c);
      fVar27 = local_138 * fVar25 * 0.5 + local_138 * fVar26;
      fVar23 = (float)piVar2[2] * 1.76;
      fVar24 = fVar26 * local_13c + fVar25 * local_13c;
      fVar25 = fVar26 * local_134 + fVar25 * local_134;
      fVar26 = 1.0 / SQRT(fVar25 * fVar25 + fVar27 * fVar27 + fVar24 * fVar24);
      bVar13 = (int)fVar26 - 1U < 0x7fffff ||
               (int)ABS(fVar26) - 0x800000U < 0x7f000000 && -1 < (int)fVar26;
      if (bVar13) {
        fVar27 = fVar27 * fVar26;
      }
      else {
        fVar27 = 0.0;
      }
      fVar6 = *(float *)(lVar15 + 0x10);
      fVar7 = *(float *)(lVar15 + 0x18);
      uVar28 = (uint)bVar13;
      fVar27 = fVar27 * fVar23 + *(float *)(lVar15 + 0x14);
      fVar24 = fVar6 + fVar23 * (float)((int)(uVar28 << 0x1f) >> 0x1f & (uint)(fVar24 * fVar26));
      fVar23 = fVar7 + fVar23 * (float)((int)(uVar28 << 0x1f) >> 0x1f & (uint)(fVar25 * fVar26));
      if (8.0 <= fVar27) {
        fVar27 = 8.0;
      }
      fVar25 = SQRT(fVar23 * fVar23 + fVar24 * fVar24);
      if ((fVar25 < 65.0) || (fVar25 < SQRT(fVar7 * fVar7 + fVar6 * fVar6))) {
        *puVar4 = uVar9;
        *puVar19 = 1;
        puVar19[1] = (uint)fVar24;
        puVar19[2] = (uint)fVar27;
        puVar19[3] = (uint)fVar23;
      }
      else {
        *puVar4 = uVar9;
        *puVar19 = 0;
      }
    }
  } while( true );
}

