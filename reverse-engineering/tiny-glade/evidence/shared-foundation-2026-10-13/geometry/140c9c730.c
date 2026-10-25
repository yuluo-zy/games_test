utils::geometry::rectangle::Rectangle2d::lerp

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c9c730(float *param_1,undefined8 *param_2,undefined8 *param_3,float param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  code *pcVar5;
  uint uVar6;
  float *pfVar7;
  uint uVar8;
  uint uVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  
  uVar6 = _DAT_1429258f0;
  fVar15 = (float)*param_2;
  fVar16 = (float)((ulonglong)*param_2 >> 0x20);
  uVar1 = param_2[1];
  uVar2 = param_2[2];
  uVar3 = param_3[1];
  uVar4 = param_3[2];
  fVar14 = (float)*param_3;
  fVar17 = (float)((ulonglong)*param_3 >> 0x20);
  fVar11 = (fVar16 * fVar17 + fVar15 * fVar14) /
           SQRT((fVar17 * fVar17 + fVar14 * fVar14) * (fVar16 * fVar16 + fVar15 * fVar15));
  fVar13 = (float)(_DAT_14292e980 & (uint)fVar11);
  fVar10 = 0.0;
  if (0.0 <= _DAT_142925904 - fVar13) {
    fVar10 = _DAT_142925904 - fVar13;
  }
  fVar13 = (((((((_DAT_142ae0d44 * fVar13 + _DAT_142ae0d48) * fVar13 + _DAT_142ae0d4c) * fVar13 +
               _DAT_142ae0d50) * fVar13 + _DAT_142ae0d54) * fVar13 + _DAT_142ae0d58) * fVar13 +
            _DAT_142ae0d5c) * fVar13 + _DAT_142ae0d60) * SQRT(fVar10);
  uVar8 = -(uint)(fVar11 < 0.0);
  fVar10 = fVar17 * fVar15 - fVar16 * fVar14;
  uVar9 = -(uint)NAN(fVar10);
  fVar17 = _DAT_142925904 - param_4;
  fVar11 = (float)(_DAT_142a8d5c4 & uVar9 |
                  ~uVar9 & ((uint)fVar10 & _DAT_1429258f0 | (uint)_DAT_142925930)) *
           (float)(uVar8 & (uint)(_DAT_1429edba4 - fVar13) | ~uVar8 & (uint)fVar13) * param_4 +
           fVar17 * 0.0;
  fVar10 = (float)func_0x000142923fd0(fVar11);
  fVar11 = (float)func_0x000142923e80(fVar11);
  fVar14 = (float)uVar4 * param_4 + (float)uVar2 * fVar17;
  fVar13 = (float)((ulonglong)uVar4 >> 0x20) * param_4 + (float)((ulonglong)uVar2 >> 0x20) * fVar17;
  if ((uint)ABS(fVar14) < 0x7f800000) {
    if ((uint)ABS(fVar13) < 0x7f800000) {
      fVar12 = (float)(uVar6 ^ (uint)fVar10) * fVar16 + fVar11 * fVar15;
      fVar10 = fVar11 * fVar16 + fVar10 * fVar15;
      if ((float)((uint)(fVar10 * fVar10 + fVar12 * fVar12 + _DAT_14295a0e4) & _DAT_14292e980) <=
          _DAT_142b2e538) {
        *param_1 = fVar12;
        param_1[1] = fVar10;
        param_1[2] = fVar17 * (float)uVar1 + (float)uVar3 * param_4;
        param_1[3] = fVar17 * (float)((ulonglong)uVar1 >> 0x20) +
                     (float)((ulonglong)uVar3 >> 0x20) * param_4;
        *(ulonglong *)(param_1 + 4) = CONCAT44(fVar13,fVar14);
        return param_1;
      }
      goto LAB_140c9ca35;
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e5f8,0x23,&UNK_142b2ea30);
  }
  FUN_1428d9430(&UNK_142b2e61b,0x24,&UNK_142b2ea30);
LAB_140c9ca35:
  FUN_1428d9430(&UNK_142b2e63f,0x2d,&UNK_142b2ea30);
  pcVar5 = (code *)swi(3);
  pfVar7 = (float *)(*pcVar5)();
  return pfVar7;
}

