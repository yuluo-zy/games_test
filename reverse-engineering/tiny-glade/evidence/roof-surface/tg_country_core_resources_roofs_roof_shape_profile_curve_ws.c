
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong *
FUN_1408e3d40(longlong *param_1,longlong param_2,float param_3,float param_4,float param_5)

{
  float *pfVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  code *pcVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  longlong *plVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong lVar19;
  float fVar20;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  longlong lStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  
  uVar7 = *(ulonglong *)(param_2 + 0x10);
  if (uVar7 == 0) {
    uStack_b0 = 4;
    fVar3 = _DAT_142925904;
  }
  else {
    uVar8 = *(ulonglong *)(param_2 + 8);
    lVar19 = uVar7 * 8;
    uStack_b0 = func_0x000140613c10(lVar19,4);
    fVar6 = _UNK_14292593c;
    fVar5 = _UNK_142925938;
    fVar4 = _UNK_142925934;
    fVar3 = _DAT_142925930;
    if (uStack_b0 == 0) goto LAB_1408e4021;
    if (uVar7 < 0xc) {
LAB_1408e3dc0:
      uVar17 = 0;
    }
    else {
      uVar17 = 0;
      if (((uStack_b0 <= (uStack_b0 + uVar7 * 8) - 8) &&
          (uStack_b0 + 4 <= (uStack_b0 + uVar7 * 8) - 4)) && (uVar7 - 1 >> 0x3d == 0)) {
        if (uStack_b0 < uVar8 + uVar7 * 8 && uVar8 < uStack_b0 + uVar7 * 8) goto LAB_1408e3dc0;
        uVar17 = uVar7 & 0x3ffffffffffffffc;
        uVar18 = 0;
        do {
          pfVar1 = (float *)(uVar8 + uVar18 * 8);
          fVar20 = *pfVar1;
          fVar10 = pfVar1[1];
          fVar11 = pfVar1[2];
          fVar12 = pfVar1[3];
          pfVar1 = (float *)(uVar8 + 0x10 + uVar18 * 8);
          fVar13 = pfVar1[1];
          fVar14 = pfVar1[2];
          fVar15 = pfVar1[3];
          pfVar2 = (float *)(uStack_b0 + 0x10 + uVar18 * 8);
          *pfVar2 = *pfVar1 * param_4 + (fVar5 - *pfVar1) * param_3;
          pfVar2[1] = fVar13 * param_5;
          pfVar2[2] = fVar14 * param_4 + (fVar6 - fVar14) * param_3;
          pfVar2[3] = fVar15 * param_5;
          pfVar1 = (float *)(uStack_b0 + uVar18 * 8);
          *pfVar1 = fVar20 * param_4 + (fVar3 - fVar20) * param_3;
          pfVar1[1] = fVar10 * param_5;
          pfVar1[2] = fVar11 * param_4 + (fVar4 - fVar11) * param_3;
          pfVar1[3] = fVar12 * param_5;
          uVar18 = uVar18 + 4;
        } while (uVar17 != uVar18);
        fVar3 = _DAT_142925904;
        if (uVar7 == uVar17) goto LAB_1408e3e90;
      }
    }
    uVar18 = uVar17 | 1;
    fVar3 = _DAT_142925904;
    if ((uVar7 & 1) != 0) {
      fVar3 = *(float *)(uVar8 + uVar17 * 8);
      fVar4 = *(float *)(uVar8 + 4 + uVar17 * 8);
      *(float *)(uStack_b0 + uVar17 * 8) = fVar3 * param_4 + (_DAT_142925904 - fVar3) * param_3;
      *(float *)(uStack_b0 + 4 + uVar17 * 8) = fVar4 * param_5;
      uVar17 = uVar18;
      fVar3 = _DAT_142925904;
    }
    while (fVar4 = _DAT_142925904, uVar7 != uVar18) {
      fVar5 = *(float *)(uVar8 + uVar17 * 8);
      fVar20 = _DAT_142925904 - fVar5;
      fVar6 = *(float *)(uVar8 + 4 + uVar17 * 8);
      _DAT_142925904 = fVar3;
      *(float *)(uStack_b0 + uVar17 * 8) = fVar5 * param_4 + fVar20 * param_3;
      *(float *)(uStack_b0 + 4 + uVar17 * 8) = fVar6 * param_5;
      fVar3 = *(float *)(uVar8 + 8 + uVar17 * 8);
      fVar5 = *(float *)(uVar8 + 0xc + uVar17 * 8);
      *(float *)(uStack_b0 + 8 + uVar17 * 8) = fVar3 * param_4 + (fVar4 - fVar3) * param_3;
      *(float *)(uStack_b0 + 0xc + uVar17 * 8) = fVar5 * param_5;
      uVar18 = uVar17 + 2;
      uVar17 = uVar18;
      fVar3 = _DAT_142925904;
      _DAT_142925904 = fVar4;
    }
  }
LAB_1408e3e90:
  _DAT_142925904 = fVar3;
  lVar19 = 0;
  uStack_b8 = uVar7;
  uStack_a8 = uVar7;
  FUN_141469430(&lStack_a0,&uStack_b8,0);
  if (!SBORROW8(0,lStack_a0)) {
    param_1[6] = lStack_70;
    param_1[4] = lStack_80;
    param_1[5] = lStack_78;
    param_1[2] = CONCAT44(uStack_8c,uStack_90);
    param_1[3] = lStack_88;
    *param_1 = lStack_a0;
    param_1[1] = CONCAT44(uStack_94,uStack_98);
    return param_1;
  }
  FUN_1428d9760(&UNK_142c63340,0x2b,&uStack_b8,&UNK_142c63320,&UNK_142aa5940);
LAB_1408e4021:
  FUN_1428d8fe3(4,lVar19,&UNK_142aa5200);
  pcVar9 = (code *)swi(3);
  plVar16 = (longlong *)(*pcVar9)();
  return plVar16;
}

