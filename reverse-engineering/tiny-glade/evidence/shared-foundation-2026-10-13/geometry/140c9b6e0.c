utils::geometry::rectangle::Rectangle2d::intersects_circle

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9b6e0(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  code *pcVar2;
  uint uVar3;
  undefined8 uVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  uint auStack_10c [5];
  undefined8 uStack_f8;
  float fStack_f0;
  float fStack_ec;
  float fStack_e8;
  float fStack_e4;
  float fStack_e0;
  float fStack_dc;
  float fStack_d8;
  float fStack_d4;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  float fStack_c0;
  float fStack_bc;
  float fStack_b8;
  float fStack_b4;
  float fStack_b0;
  float fStack_ac;
  float fStack_a8;
  float fStack_a4;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  
  fVar5 = *(float *)(param_1 + 2);
  fVar9 = *(float *)((longlong)param_1 + 0x14);
  if (fVar5 * fVar9 <= 0.0) {
    FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6d0);
    pcVar2 = (code *)swi(3);
    uVar4 = (*pcVar2)();
    return uVar4;
  }
  fVar11 = (float)((ulonglong)*param_1 >> 0x20);
  fVar6 = (float)((uint)fVar11 ^ _DAT_1429258f0);
  fVar10 = (float)*param_1;
  fVar7 = fVar5 * _DAT_142925870;
  fVar12 = _DAT_142925870 * fVar9;
  fVar5 = fVar5 * _DAT_14295a0e8;
  fVar9 = fVar9 * _DAT_14295a0e8;
  uStack_a0 = 0;
  uStack_98 = 4;
  uVar4 = param_1[1];
  uVar1 = param_1[1];
  fStack_e0 = (float)uVar4;
  fStack_f0 = fVar5 * fVar10 + fVar9 * fVar6 + fStack_e0;
  fStack_dc = (float)((ulonglong)uVar4 >> 0x20);
  fStack_ec = fVar5 * fVar11 + fVar9 * fVar10 + fStack_dc;
  fStack_d8 = (float)uVar1;
  fStack_e8 = fVar5 * fVar10 + fVar12 * fVar6 + fStack_d8;
  fStack_d4 = (float)((ulonglong)uVar1 >> 0x20);
  fStack_e4 = fVar5 * fVar11 + fVar12 * fVar10 + fStack_d4;
  fStack_e0 = fVar7 * fVar10 + fVar12 * fVar6 + fStack_e0;
  fStack_dc = fVar7 * fVar11 + fVar12 * fVar10 + fStack_dc;
  fStack_d8 = fVar7 * fVar10 + fVar9 * fVar6 + fStack_d8;
  fStack_d4 = fVar7 * fVar11 + fVar9 * fVar10 + fStack_d4;
  uStack_d0 = 0;
  uStack_c8 = 4;
  uStack_90 = 0;
  uStack_f8 = 4;
  fStack_c0 = fStack_f0;
  fStack_bc = fStack_ec;
  fStack_b8 = fStack_e8;
  fStack_b4 = fStack_e4;
  fStack_b0 = fStack_e0;
  fStack_ac = fStack_dc;
  fStack_a8 = fStack_d8;
  fStack_a4 = fStack_d4;
  FUN_141942600(auStack_10c,&uStack_f8);
  fVar9 = _DAT_142a8cc68;
  fVar5 = _DAT_142925904;
  uVar3 = _DAT_1429258f0;
  if (auStack_10c[0] == 1) {
    uVar4 = *param_2;
    fVar6 = *(float *)(param_2 + 1);
    do {
      fVar10 = (float)auStack_10c._12_8_ - (float)auStack_10c._4_8_;
      fVar11 = SUB84(auStack_10c._12_8_,4) - SUB84(auStack_10c._4_8_,4);
      fVar7 = fVar11 * fVar11 + fVar10 * fVar10;
      if ((fVar7 != 0.0) || (NAN(fVar7))) {
        fVar12 = (float)auStack_10c._4_8_ - (float)uVar4;
        fVar8 = SUB84(auStack_10c._4_8_,4) - (float)((ulonglong)uVar4 >> 0x20);
        fVar10 = fVar11 * fVar8 + fVar10 * fVar12;
        fVar10 = fVar10 + fVar10;
        fVar11 = fVar10 * fVar10 +
                 fVar7 * fVar9 * ((fVar8 * fVar8 + fVar12 * fVar12) - fVar6 * fVar6);
        if (0.0 <= fVar11) {
          fVar11 = SQRT(fVar11);
          fVar12 = ((float)((uint)fVar10 ^ uVar3) - fVar11) / (fVar7 + fVar7);
          if ((0.0 <= fVar12) && (fVar12 <= fVar5)) {
            return 1;
          }
          fVar7 = (fVar11 - fVar10) / (fVar7 + fVar7);
          if ((0.0 <= fVar7) && (fVar7 <= fVar5)) {
            return 1;
          }
        }
      }
      FUN_141942600(auStack_10c,&uStack_f8);
    } while ((auStack_10c[0] & 1) != 0);
  }
  return 0;
}

