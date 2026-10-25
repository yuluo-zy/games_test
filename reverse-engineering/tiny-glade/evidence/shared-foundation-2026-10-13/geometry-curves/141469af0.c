
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141469af0(longlong param_1,undefined8 param_2,float param_3,undefined8 param_4)

{
  float *pfVar1;
  undefined8 uVar2;
  code *pcVar3;
  float fVar4;
  bool bVar5;
  ulonglong uVar6;
  ulonglong uVar7;
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
  float fVar18;
  float *pfStack_f8;
  float *pfStack_f0;
  int iStack_e8;
  undefined8 uStack_e4;
  float fStack_dc;
  float fStack_d8;
  ulonglong uStack_d0;
  float **ppfStack_c8;
  undefined4 uStack_c0;
  float fStack_bc;
  float fStack_b8;
  int iStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  float fStack_a0;
  float fStack_9c;
  
  fVar4 = _DAT_14295a0d8;
  fVar9 = _DAT_142925904;
  if (*(ulonglong *)(param_1 + 0x10) < 2) {
LAB_141469da2:
    FUN_1428d9430(&UNK_142c637f0,0x29,param_4);
LAB_141469db6:
    FUN_1428d9430(&UNK_142aa53d0,0x1d,param_4);
LAB_141469dca:
    FUN_1428d9430(&UNK_142aa5498,0x22,param_4);
  }
  else {
    fVar11 = (float)param_2;
    pfStack_f8 = *(float **)(param_1 + 8);
    pfStack_f0 = pfStack_f8 + *(ulonglong *)(param_1 + 0x10) * 3;
    iStack_e8 = 0;
    uStack_d0 = 0;
    bVar5 = false;
    uVar7 = 0;
    fVar17 = 0.0;
    fVar18 = _DAT_142925980;
    do {
      pfVar1 = pfStack_f8 + 3;
      fVar8 = *pfStack_f8;
      fVar10 = pfStack_f8[2];
      pfStack_f8 = pfVar1;
      uVar2 = param_2;
      if (!bVar5) {
        uStack_c0 = 1;
        ppfStack_c8 = &pfStack_f8;
        fStack_bc = fVar8;
        fStack_b8 = fVar10;
        func_0x0001412afbb0(&iStack_ac,&ppfStack_c8);
        fStack_d8 = fStack_9c;
        iStack_e8 = iStack_ac;
        uStack_e4 = CONCAT44(uStack_a4,uStack_a8);
        fStack_dc = fStack_a0;
        if (iStack_ac != 1) break;
        bVar5 = true;
        fVar8 = fStack_a0;
        fVar10 = fStack_9c;
        uVar2 = uStack_e4;
      }
      uStack_e4 = uVar2;
      uVar6 = uStack_d0 + 1;
      fVar14 = (float)uStack_e4;
      fVar16 = (float)((ulonglong)uStack_e4 >> 0x20);
      fVar13 = fVar8 - fVar14;
      fVar15 = fVar10 - fVar16;
      fVar12 = fVar15 * fVar15 + fVar13 * fVar13;
      if (fVar12 <= fVar4) {
        fVar12 = fVar4;
      }
      fVar12 = ((param_3 - fVar16) * fVar15 + (fVar11 - fVar14) * fVar13) / fVar12;
      fVar13 = 0.0;
      if (0.0 <= fVar12) {
        fVar13 = fVar12;
      }
      fVar12 = fVar9;
      if (fVar13 <= fVar9) {
        fVar12 = fVar13;
      }
      fVar14 = fVar11 - (fVar12 * fVar8 + (fVar9 - fVar12) * fVar14);
      fVar16 = param_3 - (fVar12 * fVar10 + (fVar9 - fVar12) * fVar16);
      fVar14 = SQRT(fVar16 * fVar16 + fVar14 * fVar14);
      fStack_dc = fVar8;
      fStack_d8 = fVar10;
      if (fVar14 < fVar18) {
        if ((fVar12 < 0.0) || (fVar9 < fVar12)) {
          uStack_d0 = uVar6;
          FUN_1428d9430(&UNK_142c63b78,0x2a,&UNK_142c63ba8);
          goto LAB_141469da2;
        }
        uVar7 = uStack_d0 & 0xffffffff;
        fVar17 = fVar12;
        fVar18 = fVar14;
      }
      param_2 = CONCAT44(fVar10,fVar8);
      uStack_d0 = uVar6;
    } while (pfStack_f8 != pfStack_f0);
    if (uVar7 < *(ulonglong *)(param_1 + 0x28)) {
      uVar6 = uVar7 + 1;
      if (*(ulonglong *)(param_1 + 0x28) <= uVar6) goto LAB_141469de6;
      uVar2 = *(undefined8 *)(*(longlong *)(param_1 + 0x20) + uVar7 * 4);
      fVar9 = fVar17 * (float)((ulonglong)uVar2 >> 0x20) + (fVar9 - fVar17) * (float)uVar2;
      if (NAN(fVar9)) goto LAB_141469db6;
      if (ABS(fVar9) != INFINITY) {
        return;
      }
      goto LAB_141469dca;
    }
  }
  uVar6 = FUN_1428d9518();
LAB_141469de6:
  FUN_1428d9518(uVar6);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

