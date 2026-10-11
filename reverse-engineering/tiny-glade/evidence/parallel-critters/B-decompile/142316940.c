
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
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  uint uVar17;
  longlong lVar18;
  longlong lVar19;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  uint *puVar23;
  ulonglong uVar24;
  longlong *in_R8;
  longlong lVar25;
  longlong *in_R9;
  uint *puVar26;
  float fVar27;
  float extraout_XMM0_Da;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
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
  fVar16 = __real_42820000;
  fVar15 = __real_3d4ccccd;
  fVar14 = __real_3e19999a;
  fVar13 = __real_3f800000;
  fVar12 = __real_3f000000;
  fVar5 = *(float *)(*in_R9 + 0x28);
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar8 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar26 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar26 + *(longlong *)(lVar9 + 0x100);
  cVar7 = *(char *)(lVar9 + 0x108);
  local_118 = 8;
  local_158 = 0;
  local_140 = 0;
  local_148 = 0;
  uVar24 = 0;
  uVar21 = 0;
  do {
    if (cVar7 == '\0') {
      if (uVar21 == uVar24) {
        do {
          if (puVar26 == puVar1) {
            return;
          }
          uVar17 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar17 * 0xa0);
        } while (uVar24 == 0);
        lVar18 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar17 * 0xa0;
        uVar21 = (ulonglong)*(uint *)(lVar18 + 0x94);
        lVar19 = *(longlong *)(lVar10 + 0x1a8) + uVar21 * 0x48;
        uVar21 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar21 * 0x48);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar21) &&
           (uVar22 = *(ulonglong *)
                      (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar22 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
          local_158 = 0;
        }
        if (*(ulonglong *)(lVar9 + 0x118) < uVar21) {
          lVar20 = *(longlong *)(lVar19 + 0x18);
        }
        else {
          lVar20 = *(longlong *)(lVar19 + 0x18);
        }
        lVar25 = ~*(ulonglong *)(*(longlong *)(lVar19 + 0x38) + *(longlong *)(lVar9 + 0x120) * 8) *
                 0x30;
        local_140 = *(longlong *)(lVar20 + 0x10 + lVar25);
        local_130 = *(longlong *)(lVar20 + 0x28 + lVar25);
        lVar19 = ~*(ulonglong *)(*(longlong *)(lVar19 + 0x38) + *(longlong *)(lVar9 + 0x128) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar20 + 0x10 + lVar19);
        local_138 = *(longlong *)(lVar20 + 0x28 + lVar19);
        local_118 = *(longlong *)(lVar18 + 0x50);
        uVar21 = 0;
      }
      uVar22 = (ulonglong)*(uint *)(local_118 + 8 + uVar21 * 0x10);
    }
    else {
      if (uVar21 == uVar24) {
        do {
          if (puVar26 == puVar1) {
            return;
          }
          uVar17 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar17 * 0x48);
        } while (uVar24 == 0);
        lVar19 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar17 * 0x48;
        if ((*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar19 + 0x40)) &&
           (uVar21 = *(ulonglong *)
                      (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8),
           uVar21 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar21 * 0x30);
        }
        else {
          local_158 = 0;
        }
        if (*(ulonglong *)(lVar9 + 0x118) < *(ulonglong *)(lVar19 + 0x40)) {
          lVar18 = *(longlong *)(lVar19 + 0x18);
        }
        else {
          lVar18 = *(longlong *)(lVar19 + 0x18);
        }
        lVar20 = ~*(ulonglong *)(*(longlong *)(lVar19 + 0x38) + *(longlong *)(lVar9 + 0x120) * 8) *
                 0x30;
        local_140 = *(longlong *)(lVar18 + 0x10 + lVar20);
        local_130 = *(longlong *)(lVar18 + 0x28 + lVar20);
        lVar19 = ~*(ulonglong *)(*(longlong *)(lVar19 + 0x38) + *(longlong *)(lVar9 + 0x128) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar18 + 0x10 + lVar19);
        local_138 = *(longlong *)(lVar18 + 0x28 + lVar19);
        uVar21 = 0;
      }
      uVar22 = uVar21 & 0xffffffff;
    }
    lVar19 = uVar22 * 0x30 + local_158;
    pfVar2 = (float *)(local_140 + uVar22 * 4);
    puVar3 = (undefined4 *)(local_130 + uVar22 * 4);
    puVar4 = (undefined4 *)(local_138 + uVar22 * 4);
    puVar23 = (uint *)(uVar22 * 0x10 + local_148);
    uVar21 = uVar21 + 1;
    *puVar3 = uVar8;
    fVar27 = *pfVar2 - fVar5;
    *pfVar2 = fVar27;
    if (((fVar27 <= 0.0) || ((*puVar23 & 1) == 0)) ||
       (fVar27 = (float)puVar23[2] - *(float *)(lVar19 + 0x14),
       fVar29 = (float)puVar23[1] - *(float *)(lVar19 + 0x10),
       fVar30 = (float)puVar23[3] - *(float *)(lVar19 + 0x18),
       fVar30 * fVar30 + fVar27 * fVar27 + fVar29 * fVar29 < __real_3c23d70b)) {
      fVar27 = *(float *)(&__real_bf0000003f000000 + (ulonglong)(*puVar23 == 0) * 4);
      perchance::PerchanceContext::uniform_f32();
      *puVar3 = uVar8;
      *pfVar2 = extraout_XMM0_Da * fVar14 + fVar15;
      perchance::PerchanceContext::uniform_f32();
      perchance::PerchanceContext::uniform_f32();
      ::utils::uniform_sample_sphere();
      ::utils::swizzlers::impl$3::x0z();
      fVar31 = fVar27 * local_120 + local_120;
      fVar29 = local_128 * fVar27 + local_128;
      fVar27 = fStack_124 * fVar27 + fStack_124 * fVar12;
      fVar30 = fVar13 / SQRT(fVar27 * fVar27 + fVar29 * fVar29 + fVar31 * fVar31);
      bVar11 = (int)fVar30 - 1U < 0x7fffff ||
               (int)ABS(fVar30) - 0x800000U < 0x7f000000 && -1 < (int)fVar30;
      if (bVar11) {
        fVar31 = fVar31 * fVar30;
      }
      else {
        fVar31 = 0.0;
      }
      uVar17 = (uint)bVar11;
      uVar22 = CONCAT44((int)(uVar17 << 0x1f) >> 0x1f,(int)(uVar17 << 0x1f) >> 0x1f) &
               CONCAT44(fVar27 * fVar30,fVar29 * fVar30);
      fVar27 = *pfVar2;
      uVar6 = *(undefined8 *)(lVar19 + 0x10);
      fVar29 = *(float *)(lVar19 + 0x18);
      fVar28 = (float)uVar6;
      fVar30 = fVar27 * (float)uVar22 + fVar28;
      fVar32 = fVar31 * fVar27 + fVar29;
      fVar31 = SQRT(fVar32 * fVar32 + fVar30 * fVar30);
      if ((fVar31 < fVar16) || (fVar31 < SQRT(fVar29 * fVar29 + fVar28 * fVar28))) {
        *puVar4 = uVar8;
        *puVar23 = 1;
        *(ulonglong *)(puVar23 + 1) =
             CONCAT44(fVar27 * (float)(uVar22 >> 0x20) + (float)((ulonglong)uVar6 >> 0x20),fVar30);
        puVar23[3] = (uint)fVar32;
      }
      else {
        *puVar4 = uVar8;
        *puVar23 = 0;
      }
    }
  } while( true );
}

