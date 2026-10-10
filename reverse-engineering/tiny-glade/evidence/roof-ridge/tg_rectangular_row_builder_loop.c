
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1419f0790(undefined8 *param_1,undefined8 *param_2)

{
  float *pfVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int iVar6;
  longlong lVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  undefined8 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  code *pcVar13;
  float fVar14;
  float fVar15;
  uint uVar16;
  ulonglong uVar17;
  int iVar18;
  float extraout_XMM0_Da;
  float fVar19;
  float extraout_XMM0_Da_00;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  undefined *puStack_150;
  undefined8 uStack_148;
  undefined4 **ppuStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  float *pfStack_120;
  code *pcStack_118;
  float *pfStack_110;
  undefined8 *puStack_108;
  undefined8 *puStack_100;
  longlong *plStack_f8;
  float fStack_ec;
  longlong lStack_e8;
  undefined8 uStack_e0;
  
  fVar15 = _DAT_14295a0e4;
  fVar14 = _DAT_142925904;
  uStack_e0 = 0xfffffffffffffffe;
  iVar18 = *(int *)(param_1 + 7);
  iVar6 = *(int *)((longlong)param_1 + 0x3c);
  plStack_f8 = (longlong *)*param_2;
  lStack_e8 = param_2[1];
  if (iVar18 < iVar6) {
    pfStack_110 = (float *)*param_1;
    lVar7 = param_1[1];
    puStack_108 = (undefined8 *)param_1[2];
    puStack_100 = (undefined8 *)param_1[3];
    puVar8 = (undefined8 *)param_1[4];
    puVar9 = (undefined8 *)param_1[5];
    uVar10 = param_1[6];
    lVar11 = param_2[2];
    do {
      fVar22 = (float)iVar18 / (*pfStack_110 + fVar15);
      fStack_ec = fVar22;
      if ((fVar22 < 0.0) || (fVar14 < fVar22)) {
        pfStack_120 = &fStack_ec;
        pcStack_118 = FUN_140d8c910;
        puStack_150 = &UNK_142ff4538;
        uStack_148 = 1;
        uStack_130 = 0;
        ppuStack_140 = &pfStack_120;
        uStack_138 = 1;
        FUN_1428d9390(&puStack_150,&UNK_142eaa288);
LAB_1419f0a9c:
                    /* WARNING: Does not return */
        pcVar13 = (code *)invalidInstructionException();
        (*pcVar13)();
      }
      if (NAN(fVar22)) {
        FUN_1428d9430(&UNK_142ff4548,0x1d,&UNK_142eaa288);
        goto LAB_1419f0a9c;
      }
      uVar16 = FUN_142197610(lVar7,fVar22);
      uVar12 = *(ulonglong *)(lVar7 + 0x10);
      uVar17 = (ulonglong)uVar16;
      if (uVar12 <= uVar17) {
        FUN_1428d9518(uVar17,uVar12,&UNK_142ff4438);
        goto LAB_1419f0a9c;
      }
      if (uVar12 <= uVar17 + 1) {
        FUN_1428d9518(uVar17 + 1,uVar12,&UNK_142ff4450);
        goto LAB_1419f0a9c;
      }
      fVar20 = *(float *)(*(longlong *)(lVar7 + 8) + uVar17 * 8);
      fVar21 = *(float *)(*(longlong *)(lVar7 + 8) + 8 + uVar17 * 8);
      uVar2 = *puStack_108;
      uVar3 = *puVar8;
      uVar4 = *puStack_100;
      uVar5 = *puVar9;
      fVar19 = (float)FUN_1408e3260(uVar10);
      uVar16 = FUN_142197610(lVar7,fVar22);
      uVar12 = *(ulonglong *)(lVar7 + 0x10);
      uVar17 = (ulonglong)uVar16;
      if (uVar12 <= uVar17) {
        FUN_1428d9518(uVar17,uVar12,&UNK_142ff4438);
        goto LAB_1419f0a9c;
      }
      if (uVar12 <= uVar17 + 1) {
        FUN_1428d9518(uVar17 + 1,uVar12,&UNK_142ff4450);
        goto LAB_1419f0a9c;
      }
      iVar18 = iVar18 + 1;
      fVar23 = (fVar14 - extraout_XMM0_Da) * fVar20 + fVar21 * extraout_XMM0_Da;
      fVar20 = fVar14 - fVar23;
      fVar21 = *(float *)(*(longlong *)(lVar7 + 8) + 0xc + uVar17 * 8) * extraout_XMM0_Da_00 +
               (fVar14 - extraout_XMM0_Da_00) *
               *(float *)(*(longlong *)(lVar7 + 8) + 4 + uVar17 * 8);
      pfVar1 = (float *)(lVar11 + lStack_e8 * 0x18);
      *pfVar1 = fVar23 * (float)uVar4 + (float)uVar2 * fVar20;
      pfVar1[1] = fVar23 * (float)((ulonglong)uVar4 >> 0x20) +
                  (float)((ulonglong)uVar2 >> 0x20) * fVar20;
      pfVar1[2] = fVar23 * (float)uVar5 + (float)uVar3 * fVar20;
      pfVar1[3] = fVar23 * (float)((ulonglong)uVar5 >> 0x20) +
                  (float)((ulonglong)uVar3 >> 0x20) * fVar20;
      *(float *)(lVar11 + 0x10 + lStack_e8 * 0x18) = fVar22;
      *(float *)(lVar11 + 0x14 + lStack_e8 * 0x18) = fVar19 * fVar21 + (fVar14 - fVar21) * 0.0;
      lStack_e8 = lStack_e8 + 1;
    } while (iVar18 != iVar6);
  }
  *plStack_f8 = lStack_e8;
  return;
}

