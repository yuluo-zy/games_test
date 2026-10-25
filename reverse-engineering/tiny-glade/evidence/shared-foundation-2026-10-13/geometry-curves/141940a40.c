
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141940a40(longlong param_1,undefined8 param_2,float param_3,undefined8 param_4)

{
  undefined8 uVar1;
  code *pcVar2;
  float fVar3;
  bool bVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  undefined8 *puStack_f8;
  undefined8 *puStack_f0;
  int iStack_e8;
  undefined1 auStack_e4 [12];
  float fStack_d8;
  ulonglong uStack_d0;
  undefined8 **ppuStack_c8;
  undefined4 uStack_c0;
  undefined8 uStack_bc;
  int iStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  float fStack_a0;
  float fStack_9c;
  
  fVar3 = _DAT_14295a0d8;
  fVar8 = _DAT_142925904;
  if (*(ulonglong *)(param_1 + 0x10) < 2) {
LAB_141940cdb:
    FUN_1428d9430(&UNK_142e71628,0x29,param_4);
LAB_141940cef:
    FUN_1428d9430(&UNK_142e717c8,0x1d,param_4);
LAB_141940d03:
    FUN_1428d9430(&UNK_142e71848,0x22,param_4);
  }
  else {
    fVar10 = (float)param_2;
    puStack_f8 = *(undefined8 **)(param_1 + 8);
    puStack_f0 = puStack_f8 + *(ulonglong *)(param_1 + 0x10);
    iStack_e8 = 0;
    uStack_d0 = 0;
    bVar4 = false;
    uVar6 = 0;
    fVar16 = 0.0;
    fVar17 = _DAT_142925980;
    do {
      uVar1 = *puStack_f8;
      fVar9 = (float)((ulonglong)uVar1 >> 0x20);
      if (bVar4) {
        auStack_e4._8_4_ = (float)uVar1;
        auStack_e4._0_8_ = param_2;
        fStack_d8 = fVar9;
        fVar7 = (float)uVar1;
        puStack_f8 = puStack_f8 + 1;
      }
      else {
        uStack_c0 = 1;
        puStack_f8 = puStack_f8 + 1;
        ppuStack_c8 = &puStack_f8;
        uStack_bc = uVar1;
        func_0x0001412aff00(&iStack_ac,&ppuStack_c8);
        iStack_e8 = iStack_ac;
        auStack_e4._4_4_ = uStack_a4;
        auStack_e4._0_4_ = uStack_a8;
        auStack_e4._8_4_ = fStack_a0;
        fStack_d8 = fStack_9c;
        if (iStack_ac != 1) break;
        bVar4 = true;
        fVar7 = fStack_a0;
        fVar9 = fStack_9c;
        param_2 = auStack_e4._0_8_;
      }
      uVar5 = uStack_d0 + 1;
      fVar13 = (float)param_2;
      fVar15 = (float)((ulonglong)param_2 >> 0x20);
      fVar12 = fVar7 - fVar13;
      fVar14 = fVar9 - fVar15;
      fVar11 = fVar14 * fVar14 + fVar12 * fVar12;
      if (fVar11 <= fVar3) {
        fVar11 = fVar3;
      }
      fVar11 = ((param_3 - fVar15) * fVar14 + (fVar10 - fVar13) * fVar12) / fVar11;
      fVar12 = 0.0;
      if (0.0 <= fVar11) {
        fVar12 = fVar11;
      }
      fVar11 = fVar8;
      if (fVar12 <= fVar8) {
        fVar11 = fVar12;
      }
      fVar13 = fVar10 - (fVar11 * fVar7 + (fVar8 - fVar11) * fVar13);
      fVar15 = param_3 - (fVar11 * fVar9 + (fVar8 - fVar11) * fVar15);
      fVar13 = SQRT(fVar15 * fVar15 + fVar13 * fVar13);
      if (fVar13 < fVar17) {
        if ((fVar11 < 0.0) || (fVar8 < fVar11)) {
          uStack_d0 = uVar5;
          FUN_1428d9430(&UNK_142e716d0,0x2a,&UNK_142e71700);
          goto LAB_141940cdb;
        }
        uVar6 = uStack_d0 & 0xffffffff;
        fVar16 = fVar11;
        fVar17 = fVar13;
      }
      param_2 = CONCAT44(fVar9,fVar7);
      uStack_d0 = uVar5;
    } while (puStack_f8 != puStack_f0);
    if (uVar6 < *(ulonglong *)(param_1 + 0x28)) {
      uVar5 = uVar6 + 1;
      if (*(ulonglong *)(param_1 + 0x28) <= uVar5) goto LAB_141940d1f;
      uVar1 = *(undefined8 *)(*(longlong *)(param_1 + 0x20) + uVar6 * 4);
      fVar8 = fVar16 * (float)((ulonglong)uVar1 >> 0x20) + (fVar8 - fVar16) * (float)uVar1;
      if (NAN(fVar8)) goto LAB_141940cef;
      if (ABS(fVar8) != INFINITY) {
        return;
      }
      goto LAB_141940d03;
    }
  }
  uVar5 = FUN_1428d9518();
LAB_141940d1f:
  FUN_1428d9518(uVar5);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

