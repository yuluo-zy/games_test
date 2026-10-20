
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141476ba0(longlong *param_1,undefined8 *param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  code *pcVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  uint uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  float extraout_XMM0_Da;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  undefined *puStack_148;
  undefined8 uStack_140;
  float **ppfStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  float *pfStack_118;
  code *pcStack_110;
  longlong *plStack_108;
  float fStack_fc;
  longlong lStack_f8;
  undefined8 uStack_f0;
  
  fVar16 = _DAT_1429cd2d4;
  fVar15 = _DAT_1429cd078;
  fVar14 = _DAT_142934604;
  fVar13 = _DAT_142925984;
  fVar12 = _DAT_142925970;
  fVar11 = _DAT_142925950;
  fVar10 = _DAT_142925904;
  uStack_f0 = 0xfffffffffffffffe;
  uVar19 = param_1[4];
  uVar1 = param_1[5];
  plStack_108 = (longlong *)*param_2;
  lStack_f8 = param_2[1];
  if (uVar19 < uVar1) {
    lVar2 = *param_1;
    puVar3 = (undefined8 *)param_1[1];
    puVar4 = (undefined8 *)param_1[2];
    lVar5 = param_2[2];
    lVar6 = param_1[3];
    do {
      fStack_fc = (float)uVar19 * fVar11;
      if (fVar10 < fStack_fc) {
        pfStack_118 = &fStack_fc;
        pcStack_110 = FUN_140d8c910;
        puStack_148 = &UNK_142aa53c0;
        uStack_140 = 1;
        uStack_128 = 0;
        ppfStack_138 = &pfStack_118;
        uStack_130 = 1;
        FUN_1428d9390(&puStack_148,&UNK_142c659b8);
LAB_141476f34:
                    /* WARNING: Does not return */
        pcVar9 = (code *)invalidInstructionException();
        (*pcVar9)();
      }
      uVar17 = FUN_141467ce0(lVar2);
      uVar18 = (ulonglong)uVar17;
      uVar7 = *(ulonglong *)(lVar2 + 0x10);
      if (uVar7 <= uVar18) {
        FUN_1428d9518(uVar18,uVar7,&UNK_142c63658);
        goto LAB_141476f34;
      }
      if (uVar7 <= uVar18 + 1) {
        FUN_1428d9518(uVar18 + 1,uVar7,&UNK_142c63670);
        goto LAB_141476f34;
      }
      lVar8 = *(longlong *)(lVar2 + 8);
      fVar22 = *(float *)(lVar8 + 8 + uVar18 * 8) * extraout_XMM0_Da +
               *(float *)(lVar8 + uVar18 * 8) * (fVar10 - extraout_XMM0_Da);
      pfStack_118 = (float *)CONCAT44(fVar22 * (float)((ulonglong)*puVar4 >> 0x20) +
                                      (fVar10 - fVar22) * (float)((ulonglong)*puVar3 >> 0x20),
                                      fVar22 * (float)*puVar4 + (fVar10 - fVar22) * (float)*puVar3);
      fVar22 = *(float *)(lVar6 + 0x30);
      fVar21 = 0.0;
      if (fVar13 < fVar22) {
        if ((*(byte *)(lVar6 + 8) & 1) == 0) {
          fVar20 = (*(float *)(lVar6 + 0x14) + _DAT_142aa508c) / _DAT_142aa5090;
          fVar21 = fVar14;
        }
        else {
          fVar21 = *(float *)(lVar6 + 0x1c);
          fVar20 = *(float *)(lVar6 + 0x20);
          fVar23 = fVar20;
          if (fVar20 <= fVar21) {
            fVar23 = fVar21;
          }
          fVar20 = ((float)(~-(uint)NAN(fVar21) & (uint)fVar23 | -(uint)NAN(fVar21) & (uint)fVar20)
                   + _DAT_142aa5084) / _DAT_142aa5088;
          fVar21 = _DAT_142a7ba84;
        }
        fVar20 = fVar20 / fVar12;
        fVar23 = 0.0;
        if (0.0 <= fVar20) {
          fVar23 = fVar20;
        }
        fVar20 = fVar10;
        if (fVar23 <= fVar10) {
          fVar20 = fVar23;
        }
        fVar20 = (fVar15 - (fVar20 + fVar20)) * fVar20 * fVar20;
        fVar21 = fVar20 * fVar16 + (fVar10 - fVar20) * fVar21;
        if (fVar21 < 0.0) {
          FUN_140d8b480(0,uVar7,&UNK_142aa5190);
          goto LAB_141476f34;
        }
        fVar20 = fVar22 * fVar16 + (fVar10 - fVar22) * fVar12;
        fVar22 = 0.0;
        if (0.0 <= fVar20) {
          fVar22 = fVar20;
        }
        if (fVar22 <= fVar21) {
          fVar21 = fVar22;
        }
      }
      func_0x000140c9cde0(&puStack_148,&pfStack_118,
                          *(float *)(lVar6 + 0x4c) +
                          ((fVar10 - extraout_XMM0_Da) * *(float *)(lVar8 + 4 + uVar18 * 8) +
                          *(float *)(lVar8 + 0xc + uVar18 * 8) * extraout_XMM0_Da) * fVar21);
      uVar19 = uVar19 + 1;
      *(undefined4 *)(lVar5 + 8 + lStack_f8 * 0xc) = (undefined4)uStack_140;
      *(undefined **)(lVar5 + lStack_f8 * 0xc) = puStack_148;
      lStack_f8 = lStack_f8 + 1;
    } while (uVar19 != uVar1);
  }
  *plStack_108 = lStack_f8;
  return;
}

