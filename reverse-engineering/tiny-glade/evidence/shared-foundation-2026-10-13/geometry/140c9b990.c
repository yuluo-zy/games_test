utils::geometry::rectangle::Rectangle2d::intersects

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9b990(undefined8 *param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined1 auVar10 [16];
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  uint uStack_15c;
  float fStack_158;
  float fStack_154;
  float fStack_150;
  float fStack_14c;
  undefined8 uStack_148;
  float *pfStack_140;
  float fStack_138;
  float fStack_134;
  float fStack_130;
  float fStack_12c;
  float fStack_128;
  float fStack_124;
  float fStack_120;
  float fStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  float *pfStack_108;
  float fStack_100;
  float fStack_fc;
  float fStack_f8;
  float fStack_f4;
  float fStack_f0;
  float fStack_ec;
  float fStack_e8;
  float fStack_e4;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  float fStack_b8;
  float fStack_b4;
  float fStack_b0;
  float fStack_ac;
  undefined8 uStack_a8;
  
  fVar13 = (float)param_1[2];
  fVar15 = (float)((ulonglong)param_1[2] >> 0x20);
  if (SQRT(fVar15 * fVar15 + fVar13 * fVar13) < 0.0) {
    FUN_1428d9430(&UNK_142c1f4f8,0x26,&UNK_142c1f540);
  }
  else {
    fVar3 = (float)*param_2;
    fVar4 = (float)((ulonglong)*param_2 >> 0x20);
    fVar9 = (float)((ulonglong)param_2[1] >> 0x20);
    fVar5 = (float)param_2[1];
    fVar11 = (float)param_1[1];
    fVar12 = (float)((ulonglong)param_1[1] >> 0x20);
    fVar7 = (float)((ulonglong)*param_1 >> 0x20);
    fVar8 = (float)*param_1;
    fVar14 = fVar8 * fVar8 - (float)((uint)fVar7 ^ _DAT_1429258f0) * fVar7;
    auVar10._4_4_ = fVar14;
    auVar10._0_4_ = fVar14;
    auVar10._8_4_ = fVar14;
    auVar10._12_4_ = fVar14;
    auVar10 = divps(_DAT_142a901b0,auVar10);
    fVar14 = fVar8 * auVar10._0_4_;
    fVar6 = fVar7 * auVar10._4_4_;
    fVar7 = (float)((uint)fVar7 ^ _DAT_1429258f0) * auVar10._8_4_;
    fVar8 = fVar8 * auVar10._12_4_;
    fStack_b8 = fVar3 * fVar14 + fVar4 * fVar7;
    fStack_b4 = fVar3 * fVar6 + fVar4 * fVar8;
    fStack_b0 = (float)((uint)fVar4 ^ _UNK_1429258f8) * fVar14 + fVar3 * fVar7;
    fStack_ac = (float)((uint)fVar4 ^ _UNK_1429258f8) * fVar6 + fVar3 * fVar8;
    uStack_a8 = CONCAT44((fVar8 * fVar9 + fVar6 * fVar5) - (fVar12 * fVar8 + fVar11 * fVar6),
                         (fVar7 * fVar9 + fVar14 * fVar5) - (fVar12 * fVar7 + fVar11 * fVar14));
    fStack_128 = (float)param_2[2];
    fStack_12c = (float)((ulonglong)param_2[2] >> 0x20);
    if (0.0 < fStack_12c * fStack_128) {
      fStack_134 = fStack_12c * _DAT_142aa5d60;
      fStack_138 = fStack_128 * _UNK_142aa5d64;
      fStack_12c = fStack_12c * _UNK_142aa5d68;
      fStack_128 = fStack_128 * _UNK_142aa5d6c;
      pfStack_140 = &fStack_b8;
      uStack_e0 = 0;
      uStack_d8 = 4;
      uStack_118 = 0;
      uStack_110 = 4;
      uStack_d0 = 0;
      uStack_148 = 4;
      fStack_130 = fStack_138;
      fStack_124 = fStack_12c;
      fStack_120 = fStack_128;
      fStack_11c = fStack_134;
      pfStack_108 = pfStack_140;
      fStack_100 = fStack_138;
      fStack_fc = fStack_134;
      fStack_f8 = fStack_138;
      fStack_f4 = fStack_12c;
      fStack_f0 = fStack_128;
      fStack_ec = fStack_12c;
      fStack_e8 = fStack_128;
      fStack_e4 = fStack_134;
      FUN_1419423f0(&uStack_15c,&uStack_148);
      fVar3 = _DAT_142925904;
      if (uStack_15c == 1) {
        fVar13 = fVar13 * _DAT_142925870;
        fVar15 = fVar15 * _DAT_142925870;
        do {
          fVar5 = ((fVar13 + 0.0) - fStack_158) / (fStack_150 - fStack_158);
          fVar9 = ((0.0 - fVar13) - fStack_158) / (fStack_150 - fStack_158);
          fVar4 = fVar9;
          if (fVar9 <= fVar5) {
            fVar4 = fVar5;
          }
          fVar6 = ((fVar15 + 0.0) - fStack_154) / (fStack_14c - fStack_154);
          fVar7 = ((0.0 - fVar15) - fStack_154) / (fStack_14c - fStack_154);
          fVar14 = fVar7;
          if (fVar7 <= fVar6) {
            fVar14 = fVar6;
          }
          fVar8 = fVar9;
          if (fVar5 <= fVar9) {
            fVar8 = fVar5;
          }
          fVar9 = (float)(~-(uint)NAN(fVar5) & (uint)fVar8 | -(uint)NAN(fVar5) & (uint)fVar9);
          fVar5 = fVar7;
          if (fVar6 <= fVar7) {
            fVar5 = fVar6;
          }
          fVar5 = (float)(~-(uint)NAN(fVar6) & (uint)fVar5 | -(uint)NAN(fVar6) & (uint)fVar7);
          fVar6 = fVar5;
          if (fVar5 <= fVar9) {
            fVar6 = fVar9;
          }
          fVar9 = (float)(~-(uint)NAN(fVar9) & (uint)fVar6 | -(uint)NAN(fVar9) & (uint)fVar5);
          fVar5 = fVar14;
          if (fVar4 <= fVar14) {
            fVar5 = fVar4;
          }
          fVar4 = (float)(~-(uint)NAN(fVar4) & (uint)fVar5 | -(uint)NAN(fVar4) & (uint)fVar14);
          if (((fVar9 <= fVar4) && (0.0 <= fVar4)) &&
             ((fVar4 <= fVar3 || (fVar9 <= fVar3 && 0.0 <= fVar9)))) {
            return 1;
          }
          FUN_1419423f0(&uStack_15c,&uStack_148);
        } while ((uStack_15c & 1) != 0);
      }
      return 0;
    }
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

