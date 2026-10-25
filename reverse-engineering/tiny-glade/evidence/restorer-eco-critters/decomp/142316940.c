// 0x142316940 update_firefly_goals
// SIG: void __cdecl update_firefly_goals(void)

void __cdecl system_critters::firefly::update_firefly_goals(void)

{
  uint *puVar1;
  float *pfVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  float fVar5;
  undefined8 uVar6;
  char cVar7;
  undefined4 uVar8;
  longlong lVar9;
  longlong lVar10;
  bool bVar11;
  uint uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  uint *puVar18;
  ulonglong uVar19;
  longlong *in_R8;
  longlong lVar20;
  longlong *in_R9;
  uint *puVar21;
  float fVar22;
  float extraout_XMM0_Da;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  longlong local_158;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  longlong local_130;
  float local_128;
  float fStack_124;
  float local_120;
  longlong local_118;
  
  if (*(char *)(*in_R8 + 4) != '\x01') {
    return;
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  fVar5 = *(float *)(*in_R9 + 0x28);
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar8 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar21 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar21 + *(longlong *)(lVar9 + 0x100);
  cVar7 = *(char *)(lVar9 + 0x108);
  local_118 = 8;
  local_158 = 0;
  local_140 = 0;
  local_148 = 0;
  uVar19 = 0;
  uVar16 = 0;
  do {
    if (cVar7 == '\0') {
      if (uVar16 == uVar19) {
        do {
          if (puVar21 == puVar1) {
            return;
          }
          uVar12 = *puVar21;
          puVar21 = puVar21 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar12 * 0xa0);
        } while (uVar19 == 0);
        lVar13 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar12 * 0xa0;
        uVar16 = (ulonglong)*(uint *)(lVar13 + 0x94);
        lVar14 = *(longlong *)(lVar10 + 0x1a8) + uVar16 * 0x48;
        uVar16 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar16 * 0x48);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar16) &&
           (uVar17 = *(ulonglong *)
                      (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar17 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar17 * 0x30);
        }
        else {
          local_158 = 0;
        }
        if (*(ulonglong *)(lVar9 + 0x118) < uVar16) {
          lVar15 = *(longlong *)(lVar14 + 0x18);
        }
        else {
          lVar15 = *(longlong *)(lVar14 + 0x18);
        }
        lVar20 = ~*(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(longlong *)(lVar9 + 0x120) * 8) *
                 0x30;
        local_140 = *(longlong *)(lVar15 + 0x10 + lVar20);
        local_130 = *(longlong *)(lVar15 + 0x28 + lVar20);
        lVar14 = ~*(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(longlong *)(lVar9 + 0x128) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar15 + 0x10 + lVar14);
        local_138 = *(longlong *)(lVar15 + 0x28 + lVar14);
        local_118 = *(longlong *)(lVar13 + 0x50);
        uVar16 = 0;
      }
      uVar17 = (ulonglong)*(uint *)(local_118 + 8 + uVar16 * 0x10);
    }
    else {
      if (uVar16 == uVar19) {
        do {
          if (puVar21 == puVar1) {
            return;
          }
          uVar12 = *puVar21;
          puVar21 = puVar21 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar12 * 0x48);
        } while (uVar19 == 0);
        lVar14 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar12 * 0x48;
        if ((*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar14 + 0x40)) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar16 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar16 * 0x30);
        }
        else {
          local_158 = 0;
        }
        if (*(ulonglong *)(lVar9 + 0x118) < *(ulonglong *)(lVar14 + 0x40)) {
          lVar13 = *(longlong *)(lVar14 + 0x18);
        }
        else {
          lVar13 = *(longlong *)(lVar14 + 0x18);
        }
        lVar15 = ~*(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(longlong *)(lVar9 + 0x120) * 8) *
                 0x30;
        local_140 = *(longlong *)(lVar13 + 0x10 + lVar15);
        local_130 = *(longlong *)(lVar13 + 0x28 + lVar15);
        lVar14 = ~*(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(longlong *)(lVar9 + 0x128) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar13 + 0x10 + lVar14);
        local_138 = *(longlong *)(lVar13 + 0x28 + lVar14);
        uVar16 = 0;
      }
      uVar17 = uVar16 & 0xffffffff;
    }
    lVar14 = uVar17 * 0x30 + local_158;
    pfVar2 = (float *)(local_140 + uVar17 * 4);
    puVar3 = (undefined4 *)(local_130 + uVar17 * 4);
    puVar4 = (undefined4 *)(local_138 + uVar17 * 4);
    puVar18 = (uint *)(uVar17 * 0x10 + local_148);
    uVar16 = uVar16 + 1;
    *puVar3 = uVar8;
    fVar22 = *pfVar2 - fVar5;
    *pfVar2 = fVar22;
    if (((fVar22 <= 0.0) || ((*puVar18 & 1) == 0)) ||
       (fVar22 = (float)puVar18[2] - *(float *)(lVar14 + 0x14),
       fVar24 = (float)puVar18[1] - *(float *)(lVar14 + 0x10),
       fVar25 = (float)puVar18[3] - *(float *)(lVar14 + 0x18),
       fVar25 * fVar25 + fVar22 * fVar22 + fVar24 * fVar24 < 0.010000001)) {
      fVar22 = *(float *)(&__real_bf0000003f000000 + (ulonglong)(*puVar18 == 0) * 4);
      perchance::PerchanceContext::uniform_f32();
      *puVar3 = uVar8;
      *pfVar2 = extraout_XMM0_Da * 0.15 + 0.05;
      perchance::PerchanceContext::uniform_f32();
      perchance::PerchanceContext::uniform_f32();
      ::utils::uniform_sample_sphere();
      ::utils::swizzlers::impl$3::x0z();
      fVar26 = fVar22 * local_120 + local_120;
      fVar24 = local_128 * fVar22 + local_128;
      fVar22 = fStack_124 * fVar22 + fStack_124 * 0.5;
      fVar25 = 1.0 / SQRT(fVar22 * fVar22 + fVar24 * fVar24 + fVar26 * fVar26);
      bVar11 = (int)fVar25 - 1U < 0x7fffff ||
               (int)ABS(fVar25) - 0x800000U < 0x7f000000 && -1 < (int)fVar25;
      if (bVar11) {
        fVar26 = fVar26 * fVar25;
      }
      else {
        fVar26 = 0.0;
      }
      uVar12 = (uint)bVar11;
      uVar17 = CONCAT44((int)(uVar12 << 0x1f) >> 0x1f,(int)(uVar12 << 0x1f) >> 0x1f) &
               CONCAT44(fVar22 * fVar25,fVar24 * fVar25);
      fVar22 = *pfVar2;
      uVar6 = *(undefined8 *)(lVar14 + 0x10);
      fVar24 = *(float *)(lVar14 + 0x18);
      fVar23 = (float)uVar6;
      fVar25 = fVar22 * (float)uVar17 + fVar23;
      fVar27 = fVar26 * fVar22 + fVar24;
      fVar26 = SQRT(fVar27 * fVar27 + fVar25 * fVar25);
      if ((fVar26 < 65.0) || (fVar26 < SQRT(fVar24 * fVar24 + fVar23 * fVar23))) {
        *puVar4 = uVar8;
        *puVar18 = 1;
        *(ulonglong *)(puVar18 + 1) =
             CONCAT44(fVar22 * (float)(uVar17 >> 0x20) + (float)((ulonglong)uVar6 >> 0x20),fVar25);
        puVar18[3] = (uint)fVar27;
      }
      else {
        *puVar4 = uVar8;
        *puVar18 = 0;
      }
    }
  } while( true );
}

