
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140153bc0(longlong param_1,float param_2,float param_3)

{
  longlong lVar1;
  undefined8 *puVar2;
  ulonglong uVar3;
  uint uVar4;
  longlong lVar5;
  float fVar6;
  float fVar7;
  undefined8 uVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  
  lVar1 = *(longlong *)(param_1 + 0x10);
  uVar3 = 0;
  if ((lVar1 != 0) && (lVar1 != 1)) {
    puVar2 = *(undefined8 **)(param_1 + 8);
    uVar8 = puVar2[1];
    fVar9 = (float)*puVar2;
    fVar11 = (float)((ulonglong)*puVar2 >> 0x20);
    fVar13 = (float)((ulonglong)uVar8 >> 0x20);
    fVar6 = (float)uVar8 - fVar9;
    fVar7 = fVar13 - fVar11;
    fVar10 = fVar7 * fVar7 + fVar6 * fVar6;
    if (fVar10 <= _DAT_14295a0d8) {
      fVar10 = _DAT_14295a0d8;
    }
    fVar10 = ((param_3 - fVar11) * fVar7 + (param_2 - fVar9) * fVar6) / fVar10;
    fVar6 = 0.0;
    if (0.0 <= fVar10) {
      fVar6 = fVar10;
    }
    fVar10 = _DAT_142925904;
    if (fVar6 <= _DAT_142925904) {
      fVar10 = fVar6;
    }
    fVar6 = param_2 - (fVar10 * (float)uVar8 + (_DAT_142925904 - fVar10) * fVar9);
    fVar7 = param_3 - (fVar10 * fVar13 + (_DAT_142925904 - fVar10) * fVar11);
    fVar7 = SQRT(fVar7 * fVar7 + fVar6 * fVar6);
    fVar6 = _DAT_142925980;
    if ((_DAT_142925980 <= fVar7) || ((0.0 <= fVar10 && (fVar6 = fVar7, fVar10 <= _DAT_142925904))))
    {
      if (puVar2 + 2 != puVar2 + lVar1) {
        uVar4 = 1;
        uVar3 = 0;
        lVar5 = 0x10;
        do {
          fVar7 = (float)uVar8;
          fVar13 = (float)((ulonglong)uVar8 >> 0x20);
          uVar8 = *(undefined8 *)((longlong)puVar2 + lVar5);
          fVar9 = (float)((ulonglong)uVar8 >> 0x20);
          fVar11 = (float)uVar8 - fVar7;
          fVar12 = fVar9 - fVar13;
          fVar10 = fVar12 * fVar12 + fVar11 * fVar11;
          if (fVar10 <= _DAT_14295a0d8) {
            fVar10 = _DAT_14295a0d8;
          }
          fVar10 = ((param_3 - fVar13) * fVar12 + (param_2 - fVar7) * fVar11) / fVar10;
          fVar11 = 0.0;
          if (0.0 <= fVar10) {
            fVar11 = fVar10;
          }
          fVar10 = _DAT_142925904;
          if (fVar11 <= _DAT_142925904) {
            fVar10 = fVar11;
          }
          fVar7 = param_2 - (fVar10 * (float)uVar8 + (_DAT_142925904 - fVar10) * fVar7);
          fVar13 = param_3 - (fVar10 * fVar9 + (_DAT_142925904 - fVar10) * fVar13);
          fVar7 = SQRT(fVar13 * fVar13 + fVar7 * fVar7);
          if (fVar7 < fVar6) {
            if (fVar10 < 0.0) goto LAB_140153d9c;
            uVar3 = (ulonglong)uVar4;
            fVar6 = fVar7;
            if (_DAT_142925904 < fVar10) goto LAB_140153d9c;
          }
          lVar5 = lVar5 + 8;
          uVar4 = uVar4 + 1;
        } while (lVar1 << 3 != lVar5);
      }
    }
    else {
LAB_140153d9c:
      uVar3 = FUN_1428d9430(&UNK_14295a628,0x2a,&UNK_14295a658);
    }
  }
  return uVar3;
}

