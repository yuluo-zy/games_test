
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141475090(longlong *param_1,undefined8 *param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  float fVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  code *pcVar10;
  uint uVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  uint uVar16;
  ulonglong uVar17;
  longlong lVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  undefined8 uStack_e0;
  undefined1 auStack_d8 [8];
  float fStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong *plStack_b8;
  undefined8 uStack_b0;
  
  fVar15 = _DAT_142aa3280;
  fVar14 = _UNK_1429df1b4;
  fVar13 = _DAT_1429df1b0;
  fVar12 = _DAT_142925904;
  uVar11 = _DAT_1429258f0;
  uStack_b0 = 0xfffffffffffffffe;
  lVar5 = param_1[4];
  lVar6 = param_1[5];
  plStack_b8 = (longlong *)*param_2;
  lStack_c0 = param_2[1];
  if (lVar6 != lVar5) {
    lVar7 = *param_1;
    lStack_c8 = param_1[2];
    lVar8 = param_2[2];
    lVar9 = param_1[7];
    lVar18 = 0;
    do {
      lVar2 = lVar5 + lVar18;
      uVar4 = *(undefined8 *)(lVar7 + lVar2 * 0xc);
      fVar3 = *(float *)(lVar7 + 8 + lVar2 * 0xc);
      uVar16 = FUN_141467ce0(lVar9 + 0x38,*(undefined4 *)(lStack_c8 + lVar2 * 4));
      uVar17 = (ulonglong)uVar16;
      uVar1 = uVar17 + 1;
      if (*(ulonglong *)(lVar9 + 0x48) <= uVar1) {
        FUN_1428d9518(uVar1,*(ulonglong *)(lVar9 + 0x48),&UNK_142c63688);
                    /* WARNING: Does not return */
        pcVar10 = (code *)invalidInstructionException();
        (*pcVar10)();
      }
      lVar2 = *(longlong *)(lVar9 + 0x40);
      fVar19 = *(float *)(lVar2 + 4 + uVar1 * 0xc) - *(float *)(lVar2 + 4 + uVar17 * 0xc);
      fVar20 = *(float *)(lVar2 + 8 + uVar1 * 0xc) - *(float *)(lVar2 + 8 + uVar17 * 0xc);
      fVar21 = *(float *)(lVar2 + uVar1 * 0xc) - *(float *)(lVar2 + uVar17 * 0xc);
      fVar19 = fVar12 / SQRT(fVar21 * fVar21 + fVar19 * fVar19 + fVar20 * fVar20);
      fVar20 = fVar19 * fVar20;
      fVar19 = fVar19 * fVar21;
      fVar21 = fVar12 / SQRT(fVar19 * fVar19 + fVar20 * fVar20);
      uStack_e0 = CONCAT44(fVar21 * fVar19,(float)((uint)fVar21 ^ uVar11) * fVar20);
      func_0x000140c9cdc0(auStack_d8,&uStack_e0);
      lVar18 = lVar18 + 1;
      *(ulonglong *)(lVar8 + lStack_c0 * 0xc) =
           CONCAT44((float)((ulonglong)uVar4 >> 0x20) + auStack_d8._4_4_ * fVar14,
                    (float)uVar4 + auStack_d8._0_4_ * fVar13);
      *(float *)(lVar8 + 8 + lStack_c0 * 0xc) = fVar3 + fStack_d0 * fVar15;
      lStack_c0 = lStack_c0 + 1;
    } while (lVar18 != lVar6 - lVar5);
  }
  *plStack_b8 = lStack_c0;
  return;
}

