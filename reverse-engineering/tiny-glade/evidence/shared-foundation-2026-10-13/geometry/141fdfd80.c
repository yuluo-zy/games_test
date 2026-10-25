utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint * FUN_141fdfd80(uint *param_1,float *param_2,float *param_3)

{
  float *pfVar1;
  float *pfVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined4 uVar7;
  float fVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  undefined8 uStack_188;
  float fStack_180;
  float fStack_17c;
  undefined8 uStack_178;
  float fStack_170;
  float fStack_16c;
  float fStack_168;
  float fStack_164;
  float fStack_160;
  undefined4 uStack_15c;
  float fStack_158;
  float fStack_154;
  float fStack_150;
  undefined4 uStack_14c;
  undefined1 auStack_148 [16];
  undefined1 auStack_138 [16];
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  float fStack_f8;
  uint uStack_f4;
  undefined8 uStack_f0;
  uint uStack_e8;
  float fStack_e4;
  undefined8 uStack_e0;
  
  fVar8 = _DAT_142abaa00;
  uVar7 = _UNK_14293e0cc;
  fStack_158 = *param_2;
  fStack_154 = param_2[1];
  fVar16 = (float)*(undefined8 *)(param_2 + 1);
  fStack_150 = (float)((ulonglong)*(undefined8 *)(param_2 + 1) >> 0x20);
  auStack_148 = ZEXT416((uint)param_2[3]);
  fVar23 = (float)*(undefined8 *)(param_2 + 4);
  fVar24 = (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20);
  auStack_138 = ZEXT416((uint)param_2[6]);
  uStack_128 = *(undefined8 *)(param_2 + 7);
  fVar14 = fStack_150 - fVar24;
  fVar15 = fStack_150 - (float)((ulonglong)uStack_128 >> 0x20);
  uStack_120 = 0;
  fVar13 = fVar16 - (float)uStack_128;
  fVar16 = fVar16 - fVar23;
  if (_DAT_142abaa00 <= (float)((uint)(fVar15 * fVar16 - fVar14 * fVar13) & _DAT_14292e980)) {
LAB_141fdfeb6:
    uVar5 = *(undefined8 *)param_3;
    uVar3 = *(undefined8 *)param_3;
    uStack_188 = *(undefined8 *)param_3;
    fVar17 = (float)*(undefined8 *)(param_3 + 1);
    fVar22 = (float)((ulonglong)*(undefined8 *)(param_3 + 1) >> 0x20);
    pfVar1 = param_3 + 3;
    uVar6 = *(undefined8 *)pfVar1;
    uVar4 = *(undefined8 *)pfVar1;
    uStack_178 = *(undefined8 *)pfVar1;
    pfVar2 = param_3 + 6;
    fVar14 = *pfVar2;
    fVar15 = param_3[7];
    fVar16 = *pfVar2;
    fVar13 = param_3[7];
    fStack_168 = *pfVar2;
    fStack_164 = param_3[7];
    fVar20 = (float)((ulonglong)*(undefined8 *)(param_3 + 7) >> 0x20);
    fVar21 = (float)((ulonglong)*(undefined8 *)(param_3 + 4) >> 0x20);
    fVar25 = fVar17 - (float)*(undefined8 *)(param_3 + 7);
    fVar17 = fVar17 - (float)*(undefined8 *)(param_3 + 4);
    if ((float)((uint)((fVar22 - fVar20) * fVar17 - (fVar22 - fVar21) * fVar25) & _DAT_14292e980) <
        _DAT_142abaa00) {
      fVar18 = *param_3 - *pfVar2;
      fVar19 = *param_3 - *pfVar1;
      if (((float)((uint)((fVar22 - fVar21) * fVar18 - (fVar22 - fVar20) * fVar19) & _DAT_14292e980)
           < _DAT_142abaa00) &&
         ((float)((uint)(fVar19 * fVar25 - fVar17 * fVar18) & _DAT_14292e980) < _DAT_142abaa00))
      goto LAB_141fdff73;
    }
    _fStack_180 = CONCAT44(_UNK_14293e0cc,fVar22);
    fStack_16c = (float)_UNK_14293e0cc;
    uStack_15c = _UNK_14293e0cc;
    uStack_14c = _UNK_14293e0cc;
    fStack_170 = fVar21;
    fStack_160 = fVar20;
    fVar25 = (float)func_0x0001412b7c50(&uStack_188);
    fVar17 = _DAT_142fcb220;
    uVar11 = (uint)(fVar8 < fVar25);
    if (fVar25 < _DAT_142fcb220) {
      uVar11 = 0xffffffff;
    }
    _fStack_180 = CONCAT44(uVar7,fVar22);
    fStack_16c = (float)uVar7;
    uStack_15c = uVar7;
    fStack_158 = (float)auStack_148._0_4_;
    uStack_14c = uVar7;
    uStack_188 = uVar3;
    uStack_178 = uVar4;
    fStack_170 = fVar21;
    fStack_168 = fVar16;
    fStack_164 = fVar13;
    fStack_160 = fVar20;
    fStack_154 = fVar23;
    fStack_150 = fVar24;
    fVar16 = (float)func_0x0001412b7c50(&uStack_188);
    uVar10 = (uint)(fVar8 < fVar16);
    if (fVar16 < fVar17) {
      uVar10 = 0xffffffff;
    }
    _fStack_180 = CONCAT44(uVar7,fVar22);
    fStack_16c = (float)uVar7;
    uStack_15c = uVar7;
    fStack_158 = (float)auStack_138._0_4_;
    fStack_154 = (float)(undefined4)uStack_128;
    fStack_150 = (float)uStack_128._4_4_;
    uStack_14c = uVar7;
    uStack_188 = uVar5;
    uStack_178 = uVar6;
    fStack_170 = fVar21;
    fStack_168 = fVar14;
    fStack_164 = fVar15;
    fStack_160 = fVar20;
    fVar16 = (float)func_0x0001412b7c50(&uStack_188);
    uVar9 = (uint)(fVar8 < fVar16);
    if (fVar16 < fVar17) {
      uVar9 = 0xffffffff;
    }
    if (uVar11 == uVar9 && uVar11 == uVar10) {
      if (uVar11 == 0) {
        *param_1 = 2;
        return param_1;
      }
    }
    else {
      fStack_f8 = param_2[8];
      uStack_118 = *(undefined8 *)param_2;
      uStack_110 = *(undefined8 *)(param_2 + 2);
      uStack_108 = *(undefined8 *)(param_2 + 4);
      uStack_100 = *(undefined8 *)(param_2 + 6);
      fStack_168 = param_3[8];
      uStack_188 = *(undefined8 *)param_3;
      fStack_180 = param_3[2];
      fStack_17c = param_3[3];
      uStack_178 = *(undefined8 *)(param_3 + 4);
      fStack_170 = param_3[6];
      fStack_16c = param_3[7];
      FUN_1424181a0(&uStack_f4,&uStack_118,&uStack_188,uVar11,uVar10,uVar9);
      if (uStack_f4 != 2) {
        bVar12 = (uStack_f4 & 1) != 0;
        if (bVar12) {
          uStack_178 = uStack_e0;
          fStack_17c = fStack_e4;
        }
        *param_1 = (uint)bVar12;
        *(undefined8 *)(param_1 + 1) = uStack_f0;
        param_1[3] = uStack_e8;
        param_1[4] = (uint)fStack_17c;
        *(undefined8 *)(param_1 + 5) = uStack_178;
        return param_1;
      }
    }
    *param_1 = 3;
  }
  else {
    fVar20 = *param_2 - param_2[6];
    fVar21 = *param_2 - param_2[3];
    if ((_DAT_142abaa00 <= (float)((uint)(fVar14 * fVar20 - fVar15 * fVar21) & _DAT_14292e980)) ||
       (_DAT_142abaa00 <= (float)((uint)(fVar21 * fVar13 - fVar16 * fVar20) & _DAT_14292e980)))
    goto LAB_141fdfeb6;
LAB_141fdff73:
    *param_1 = 3;
  }
  return param_1;
}

