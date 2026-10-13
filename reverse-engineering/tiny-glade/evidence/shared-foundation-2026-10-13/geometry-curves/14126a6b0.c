
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_14126a6b0(longlong param_1,undefined8 param_2,float param_3)

{
  undefined8 uVar1;
  float fVar2;
  float fVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 *puStack_e8;
  undefined8 *puStack_e0;
  int iStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  float fStack_cc;
  float fStack_c8;
  ulonglong uStack_c0;
  undefined8 **ppuStack_b8;
  undefined4 uStack_b0;
  undefined8 uStack_ac;
  ulonglong auStack_9c [2];
  float fStack_8c;
  
  fVar3 = _DAT_14295a0d8;
  fVar2 = _DAT_142925904;
  puStack_e8 = *(undefined8 **)(param_1 + 8);
  puStack_e0 = puStack_e8 + *(longlong *)(param_1 + 0x10);
  iStack_d8 = 0;
  uStack_c0 = 0;
  if (*(longlong *)(param_1 + 0x10) == 0) {
LAB_14126a89f:
    uVar5 = 0;
  }
  else {
    fVar6 = (float)param_2;
    uVar4 = 0;
    uVar5 = 0;
    fVar12 = _DAT_142925980;
    do {
      uVar1 = *puStack_e8;
      if ((uVar4 & 1) == 0) {
        uStack_b0 = 1;
        puStack_e8 = puStack_e8 + 1;
        ppuStack_b8 = &puStack_e8;
        uStack_ac = uVar1;
        func_0x0001412aff00(auStack_9c,&ppuStack_b8);
        iStack_d8 = (int)auStack_9c[0];
        uStack_d4 = (undefined4)(auStack_9c[0] >> 0x20);
        uStack_d0 = (undefined4)auStack_9c[1];
        fStack_cc = (float)(auStack_9c[1] >> 0x20);
        uVar4 = auStack_9c[0] & 0xffffffff;
        if (iStack_d8 != 1) {
          return uVar5;
        }
        param_2 = CONCAT44(uStack_d0,uStack_d4);
        fStack_c8 = fStack_8c;
      }
      else {
        uStack_d4 = (undefined4)param_2;
        uStack_d0 = (undefined4)((ulonglong)param_2 >> 0x20);
        fStack_c8 = (float)((ulonglong)uVar1 >> 0x20);
        puStack_e8 = puStack_e8 + 1;
        fStack_cc = (float)uVar1;
      }
      fVar9 = (float)param_2;
      fVar11 = (float)((ulonglong)param_2 >> 0x20);
      fVar8 = fStack_cc - fVar9;
      fVar10 = fStack_c8 - fVar11;
      fVar7 = fVar10 * fVar10 + fVar8 * fVar8;
      if (fVar7 <= fVar3) {
        fVar7 = fVar3;
      }
      fVar7 = ((param_3 - fVar11) * fVar10 + (fVar6 - fVar9) * fVar8) / fVar7;
      fVar8 = 0.0;
      if (0.0 <= fVar7) {
        fVar8 = fVar7;
      }
      fVar7 = fVar2;
      if (fVar8 <= fVar2) {
        fVar7 = fVar8;
      }
      fVar9 = fVar6 - (fVar7 * fStack_cc + (fVar2 - fVar7) * fVar9);
      fVar11 = param_3 - (fVar7 * fStack_c8 + (fVar2 - fVar7) * fVar11);
      fVar9 = SQRT(fVar11 * fVar11 + fVar9 * fVar9);
      if (fVar9 < fVar12) {
        if ((fVar7 < 0.0) || (fVar2 < fVar7)) {
          uStack_c0 = uStack_c0 + 1;
          FUN_1428d9430(&UNK_142c22978,0x2a,&UNK_142c229a8);
          goto LAB_14126a89f;
        }
        uVar5 = uStack_c0 & 0xffffffff;
        fVar12 = fVar9;
      }
      param_2 = CONCAT44(fStack_c8,fStack_cc);
      uStack_c0 = uStack_c0 + 1;
    } while (puStack_e8 != puStack_e0);
  }
  return uVar5;
}

