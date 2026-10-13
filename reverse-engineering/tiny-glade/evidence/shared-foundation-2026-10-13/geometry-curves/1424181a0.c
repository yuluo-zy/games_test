
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 *
FUN_1424181a0(undefined4 *param_1,float *param_2,undefined8 *param_3,int param_4,int param_5,
             int param_6)

{
  float *pfVar1;
  float *pfVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  uint uVar22;
  uint uVar23;
  uint uVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  undefined8 uStack_178;
  float fStack_170;
  float fStack_16c;
  float fStack_168;
  float fStack_164;
  float fStack_160;
  undefined4 uStack_15c;
  float fStack_158;
  float fStack_154;
  undefined4 uStack_150;
  float fStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  float fStack_13c;
  float fStack_138;
  float fStack_134;
  undefined8 uStack_130;
  undefined1 auStack_128 [16];
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined1 auStack_108 [16];
  undefined1 auStack_f8 [16];
  undefined1 auStack_e8 [16];
  
  uVar21 = _UNK_14293e0cc;
  fStack_170 = param_2[2];
  fStack_160 = param_2[5];
  uStack_150 = (undefined4)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20);
  uStack_140 = *(undefined4 *)(param_3 + 1);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[3];
  fStack_164 = param_2[4];
  fStack_158 = param_2[6];
  fStack_154 = param_2[7];
  uStack_148 = *(undefined4 *)param_3;
  uStack_144 = *(undefined4 *)((longlong)param_3 + 4);
  fStack_16c = (float)_UNK_14293e0cc;
  uStack_15c = _UNK_14293e0cc;
  fStack_14c = (float)_UNK_14293e0cc;
  fStack_13c = (float)_UNK_14293e0cc;
  fVar25 = (float)func_0x0001412b7c50(&uStack_178);
  fVar28 = _DAT_142fcb220;
  fVar30 = _DAT_142abaa00;
  uVar23 = (uint)(_DAT_142abaa00 < fVar25);
  if (fVar25 < _DAT_142fcb220) {
    uVar23 = 0xffffffff;
  }
  fStack_170 = param_2[2];
  fStack_160 = param_2[5];
  uStack_150 = (undefined4)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20);
  uStack_140 = *(undefined4 *)((longlong)param_3 + 0x14);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[3];
  fStack_164 = param_2[4];
  fStack_158 = param_2[6];
  fStack_154 = param_2[7];
  uStack_148 = *(undefined4 *)((longlong)param_3 + 0xc);
  uStack_144 = *(undefined4 *)(param_3 + 2);
  fStack_16c = (float)uVar21;
  uStack_15c = uVar21;
  fStack_14c = (float)uVar21;
  fStack_13c = (float)uVar21;
  fVar25 = (float)func_0x0001412b7c50(&uStack_178);
  uVar24 = (uint)(fVar30 < fVar25);
  if (fVar25 < fVar28) {
    uVar24 = 0xffffffff;
  }
  fStack_170 = param_2[2];
  fStack_160 = param_2[5];
  uStack_150 = (undefined4)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20);
  uStack_140 = (undefined4)((ulonglong)*(undefined8 *)((longlong)param_3 + 0x1c) >> 0x20);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[3];
  fStack_164 = param_2[4];
  fStack_158 = param_2[6];
  fStack_154 = param_2[7];
  uStack_148 = *(undefined4 *)(param_3 + 3);
  uStack_144 = *(undefined4 *)((longlong)param_3 + 0x1c);
  fStack_16c = (float)uVar21;
  uStack_15c = uVar21;
  fStack_14c = (float)uVar21;
  fStack_13c = (float)uVar21;
  fVar26 = (float)func_0x0001412b7c50(&uStack_178);
  uVar22 = (uint)(fVar30 < fVar26);
  if (fVar26 < fVar28) {
    uVar22 = 0xffffffff;
  }
  if (uVar23 == uVar22 && uVar23 == uVar24) goto LAB_1424183e8;
  pfVar1 = param_2 + 3;
  pfVar2 = param_2 + 6;
  if (param_4 == param_5) {
LAB_142418470:
    fStack_170 = param_2[8];
    uStack_178 = *(undefined8 *)pfVar2;
    param_2[8] = param_2[5];
    *(undefined8 *)pfVar2 = *(undefined8 *)pfVar1;
    param_2[5] = param_2[2];
    *(undefined8 *)pfVar1 = *(undefined8 *)param_2;
    param_2[2] = fStack_170;
    *(undefined8 *)param_2 = uStack_178;
  }
  else if (param_4 == param_6) {
LAB_1424184c7:
    fStack_170 = param_2[2];
    uStack_178 = *(undefined8 *)param_2;
    param_2[2] = param_2[5];
    *(undefined8 *)param_2 = *(undefined8 *)pfVar1;
    param_2[5] = param_2[8];
    *(undefined8 *)pfVar1 = *(undefined8 *)pfVar2;
    param_2[8] = fStack_170;
    *(undefined8 *)pfVar2 = uStack_178;
  }
  else if (param_5 != param_6) {
    if (0 < param_5) goto LAB_1424184c7;
    if (0 < param_6) goto LAB_142418470;
  }
  puVar3 = (undefined8 *)((longlong)param_3 + 0xc);
  puVar4 = param_3 + 3;
  if (uVar23 == uVar24) {
LAB_142418510:
    fStack_170 = *(float *)(param_3 + 4);
    uStack_178 = *puVar4;
    *(undefined4 *)(param_3 + 4) = *(undefined4 *)((longlong)param_3 + 0x14);
    *puVar4 = *puVar3;
    *(undefined4 *)((longlong)param_3 + 0x14) = *(undefined4 *)(param_3 + 1);
    *puVar3 = *param_3;
    *(float *)(param_3 + 1) = fStack_170;
    *param_3 = uStack_178;
  }
  else if (uVar23 == uVar22) {
LAB_14241854d:
    fStack_170 = *(float *)(param_3 + 1);
    uStack_178 = *param_3;
    *(undefined4 *)(param_3 + 1) = *(undefined4 *)((longlong)param_3 + 0x14);
    *param_3 = *puVar3;
    *(undefined4 *)((longlong)param_3 + 0x14) = *(undefined4 *)(param_3 + 4);
    *puVar3 = *puVar4;
    *(float *)(param_3 + 4) = fStack_170;
    *puVar4 = uStack_178;
  }
  else if (uVar24 != uVar22) {
    if ((fVar28 <= fVar25) && (_DAT_142abaa00 < fVar25)) goto LAB_14241854d;
    if ((fVar28 <= fVar26) && (_DAT_142abaa00 < fVar26)) goto LAB_142418510;
  }
  FUN_142418dc0(param_2,param_3);
  FUN_142418dc0(param_3,param_2);
  fVar28 = _DAT_142925904;
  fStack_170 = param_2[2];
  fStack_160 = param_2[5];
  fVar30 = *(float *)(param_3 + 1);
  uVar5 = *param_3;
  uVar12 = *(undefined4 *)param_3;
  uVar13 = *(undefined4 *)((longlong)param_3 + 4);
  uVar14 = *(undefined4 *)(param_3 + 1);
  uVar9 = *(undefined4 *)param_3;
  uVar10 = *(undefined4 *)((longlong)param_3 + 4);
  uVar11 = *(undefined4 *)(param_3 + 1);
  uVar6 = *(undefined4 *)param_3;
  uVar7 = *(undefined4 *)((longlong)param_3 + 4);
  uVar8 = *(undefined4 *)(param_3 + 1);
  fStack_158 = *(float *)param_3;
  fStack_154 = *(float *)((longlong)param_3 + 4);
  uStack_150 = *(undefined4 *)(param_3 + 1);
  puVar3 = (undefined8 *)((longlong)param_3 + 0xc);
  fVar32 = (float)*puVar3;
  fVar29 = (float)((ulonglong)*puVar3 >> 0x20);
  uVar15 = *(undefined4 *)puVar3;
  uVar16 = *(undefined4 *)(param_3 + 2);
  uVar17 = *(undefined4 *)((longlong)param_3 + 0x14);
  uStack_148 = *(undefined4 *)puVar3;
  uStack_144 = *(undefined4 *)(param_3 + 2);
  uStack_140 = *(undefined4 *)((longlong)param_3 + 0x14);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[3];
  fStack_164 = param_2[4];
  auStack_f8 = ZEXT416(*(uint *)((longlong)param_3 + 0x14));
  fStack_16c = (float)uVar21;
  uStack_15c = uVar21;
  fStack_14c = _DAT_142925904;
  fStack_13c = _DAT_142925904;
  fVar26 = (float)func_0x0001412b7c50(&uStack_178);
  fStack_170 = param_2[2];
  fStack_160 = (float)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20);
  puVar3 = param_3 + 3;
  uStack_118 = *puVar3;
  uVar18 = *(undefined4 *)puVar3;
  uVar19 = *(undefined4 *)((longlong)param_3 + 0x1c);
  uVar20 = *(undefined4 *)(param_3 + 4);
  fStack_158 = *(float *)puVar3;
  fStack_154 = *(float *)((longlong)param_3 + 0x1c);
  uStack_150 = *(undefined4 *)(param_3 + 4);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[6];
  fStack_164 = param_2[7];
  auStack_108 = ZEXT416(*(uint *)(param_3 + 4));
  uStack_110 = 0;
  fStack_16c = (float)uVar21;
  uStack_15c = uVar21;
  fStack_14c = fVar28;
  fStack_13c = fVar28;
  uStack_148 = uVar6;
  uStack_144 = uVar7;
  uStack_140 = uVar8;
  fVar27 = (float)func_0x0001412b7c50(&uStack_178);
  fVar25 = _DAT_142abaa00;
  if ((_DAT_142fcb220 <= fVar26 && _DAT_142abaa00 < fVar26) ||
     (_DAT_142abaa00 < fVar27 && _DAT_142fcb220 <= fVar27)) goto LAB_1424183e8;
  fStack_170 = param_2[2];
  auStack_e8 = ZEXT416((uint)fStack_170);
  fVar26 = (float)*(undefined8 *)(param_2 + 3);
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[6];
  fStack_164 = param_2[7];
  fStack_160 = (float)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20);
  fVar31 = param_2[6] - fVar26;
  fVar26 = fVar26 - *param_2;
  fVar27 = (float)((ulonglong)*(undefined8 *)(param_2 + 3) >> 0x20) - param_2[1];
  fVar35 = (float)*(undefined8 *)(param_2 + 7) - (float)*(undefined8 *)(param_2 + 4);
  fVar36 = fStack_160 - (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20);
  fVar40 = (param_2[5] - fStack_170) * fVar31 - fVar36 * fVar26;
  fStack_138 = fVar26 * fVar35 - fVar31 * fVar27;
  fVar26 = fVar27 * fVar36 - fVar35 * (param_2[5] - fStack_170);
  fStack_134 = _DAT_142925904 / SQRT(fVar26 * fVar26 + fVar40 * fVar40 + fStack_138 * fStack_138);
  auStack_128 = ZEXT416((uint)(fVar40 * fStack_134));
  fStack_138 = fStack_134 * fStack_138;
  fStack_134 = fStack_134 * fVar26;
  uStack_130 = 0;
  fVar26 = fVar32 - (float)uVar5;
  fVar27 = fVar29 - (float)((ulonglong)uVar5 >> 0x20);
  fVar32 = (float)uStack_118 - fVar32;
  fVar31 = ((float)auStack_f8._0_4_ - fVar30) * fVar32 -
           ((float)auStack_108._0_4_ - (float)auStack_f8._0_4_) * fVar26;
  fVar26 = fVar26 * (uStack_118._4_4_ - fVar29) - fVar32 * fVar27;
  fVar30 = fVar27 * ((float)auStack_108._0_4_ - (float)auStack_f8._0_4_) -
           (uStack_118._4_4_ - fVar29) * ((float)auStack_f8._0_4_ - fVar30);
  fVar27 = _DAT_142925904 / SQRT(fVar30 * fVar30 + fVar31 * fVar31 + fVar26 * fVar26);
  fVar31 = fVar31 * fVar27;
  fVar26 = fVar27 * fVar26;
  fVar27 = fVar27 * fVar30;
  fStack_16c = _DAT_142925904;
  uStack_15c = uVar21;
  fStack_14c = fVar28;
  fStack_13c = fVar28;
  fStack_158 = (float)uVar15;
  fStack_154 = (float)uVar16;
  uStack_150 = uVar17;
  uStack_148 = uVar9;
  uStack_144 = uVar10;
  uStack_140 = uVar11;
  fVar30 = (float)func_0x0001412b7c50(&uStack_178);
  fStack_170 = param_2[2];
  fStack_160 = param_2[5];
  uStack_178 = *(undefined8 *)param_2;
  fStack_168 = param_2[3];
  fStack_164 = param_2[4];
  fStack_16c = (float)uVar21;
  uStack_15c = uVar21;
  fStack_14c = fVar28;
  fStack_13c = fVar28;
  fStack_158 = (float)uVar18;
  fStack_154 = (float)uVar19;
  uStack_150 = uVar20;
  uStack_148 = uVar12;
  uStack_144 = uVar13;
  uStack_140 = uVar14;
  fVar28 = (float)func_0x0001412b7c50(&uStack_178);
  if ((fVar30 < _DAT_142fcb220) || (fVar30 <= _DAT_142abaa00)) {
    fVar30 = *(float *)(param_3 + 1);
    fVar33 = (float)*(undefined8 *)param_2;
    fVar34 = (float)((ulonglong)*(undefined8 *)param_2 >> 0x20);
    fVar32 = param_2[2];
    fVar37 = (float)*param_3;
    fVar38 = (float)((ulonglong)*param_3 >> 0x20);
    fVar40 = *(float *)((longlong)param_3 + 0xc) - fVar37;
    fVar35 = (float)((uint)((float)auStack_128._0_4_ * (fVar38 - fVar34) +
                            fStack_134 * (fVar37 - fVar33) + (fVar30 - fVar32) * fStack_138) ^
                    _DAT_1429258f0);
    fVar36 = fVar35 / ((fVar29 - fVar38) * (float)auStack_128._0_4_ + fStack_134 * fVar40 +
                      fStack_138 * ((float)auStack_f8._0_4_ - fVar30));
    fVar40 = fVar36 * fVar40 + fVar37;
    fVar39 = fVar36 * (fVar29 - fVar38) + fVar38;
    fVar29 = fVar36 * ((float)auStack_f8._0_4_ - fVar30) + fVar30;
    if (fVar25 < fVar28 && _DAT_142fcb220 <= fVar28) {
      if ((0x7f7fffff < (uint)ABS(fVar29) || 0x7f7fffff < (uint)ABS(fVar40)) ||
          0x7f7fffff < (uint)ABS(fVar39)) goto LAB_1424183e8;
      fVar32 = (float)auStack_108._0_4_ - fVar30;
      uStack_118._0_4_ = (float)uStack_118 - fVar37;
      uStack_118._4_4_ = uStack_118._4_4_ - fVar38;
      fVar35 = fVar35 / (fStack_134 * (float)uStack_118 +
                         (float)auStack_128._0_4_ * uStack_118._4_4_ + fStack_138 * fVar32);
      goto LAB_142418ce8;
    }
    if ((0x7f7fffff < (uint)ABS(fVar29) || 0x7f7fffff < (uint)ABS(fVar40)) ||
        0x7f7fffff < (uint)ABS(fVar39)) goto LAB_1424183e8;
    fVar28 = (float)*(undefined8 *)(param_2 + 3) - fVar33;
    fVar25 = (float)((ulonglong)*(undefined8 *)(param_2 + 3) >> 0x20) - fVar34;
    fVar30 = (float)((uint)((fVar32 - fVar30) * fVar26 +
                           fVar31 * (fVar34 - fVar38) + fVar27 * (fVar33 - fVar37)) ^ _DAT_1429258f0
                    ) / (fVar28 * fVar27 + fVar25 * fVar31 + (param_2[5] - fVar32) * fVar26);
    fVar28 = fVar30 * fVar28;
    fVar25 = fVar30 * fVar25;
    fVar30 = fVar30 * (param_2[5] - fVar32);
LAB_142418c98:
    fVar28 = fVar28 + fVar33;
    fVar25 = fVar25 + fVar34;
    fVar30 = fVar30 + fVar32;
  }
  else {
    fVar30 = param_2[2];
    fVar33 = (float)*param_3;
    fVar34 = (float)((ulonglong)*param_3 >> 0x20);
    fVar32 = *(float *)(param_3 + 1);
    fVar37 = (float)*(undefined8 *)param_2;
    fVar38 = (float)((ulonglong)*(undefined8 *)param_2 >> 0x20);
    fVar36 = (float)*(undefined8 *)(param_2 + 6) - fVar37;
    fVar39 = (float)((ulonglong)*(undefined8 *)(param_2 + 6) >> 0x20) - fVar38;
    fVar35 = (float)((uint)(fVar31 * (fVar38 - fVar34) + fVar27 * (fVar37 - fVar33) +
                           (fVar30 - fVar32) * fVar26) ^ _DAT_1429258f0);
    fVar29 = fVar35 / (fVar39 * fVar31 + fVar27 * fVar36 + fVar26 * (param_2[8] - fVar30));
    fVar40 = fVar29 * fVar36 + fVar37;
    fVar39 = fVar29 * fVar39 + fVar38;
    fVar29 = fVar29 * (param_2[8] - fVar30) + fVar30;
    if (fVar25 < fVar28 && _DAT_142fcb220 <= fVar28) {
      if ((0x7f7fffff < (uint)ABS(fVar29) || 0x7f7fffff < (uint)ABS(fVar40)) ||
          0x7f7fffff < (uint)ABS(fVar39)) goto LAB_1424183e8;
      fVar28 = (float)param_3[3] - fVar33;
      fVar25 = (float)((ulonglong)param_3[3] >> 0x20) - fVar34;
      fVar30 = (float)((uint)((fVar32 - fVar30) * fStack_138 +
                             (float)auStack_128._0_4_ * (fVar34 - fVar38) +
                             fStack_134 * (fVar33 - fVar37)) ^ _DAT_1429258f0) /
               (fVar25 * (float)auStack_128._0_4_ + fStack_134 * fVar28 +
               fStack_138 * (*(float *)(param_3 + 4) - fVar32));
      fVar28 = fVar30 * fVar28;
      fVar25 = fVar30 * fVar25;
      fVar30 = fVar30 * (*(float *)(param_3 + 4) - fVar32);
      goto LAB_142418c98;
    }
    if ((0x7f7fffff < (uint)ABS(fVar29) || 0x7f7fffff < (uint)ABS(fVar40)) ||
        0x7f7fffff < (uint)ABS(fVar39)) goto LAB_1424183e8;
    fVar32 = param_2[5] - fVar30;
    uStack_118._0_4_ = (float)*(undefined8 *)(param_2 + 3) - fVar37;
    uStack_118._4_4_ = (float)((ulonglong)*(undefined8 *)(param_2 + 3) >> 0x20) - fVar38;
    fVar35 = fVar35 / (fVar31 * uStack_118._4_4_ + fVar27 * (float)uStack_118 + fVar26 * fVar32);
LAB_142418ce8:
    fVar28 = fVar35 * (float)uStack_118 + fVar37;
    fVar25 = fVar35 * uStack_118._4_4_ + fVar38;
    fVar30 = fVar35 * fVar32 + fVar30;
  }
  if ((((uint)ABS(fVar28) < 0x7f800000) && ((uint)ABS(fVar25) < 0x7f800000)) &&
     ((uint)ABS(fVar30) < 0x7f800000)) {
    if ((fVar40 - fVar28) * (fVar40 - fVar28) + (fVar39 - fVar25) * (fVar39 - fVar25) +
        (fVar29 - fVar30) * (fVar29 - fVar30) < _DAT_14306f7e8) {
      *param_1 = 0;
      *(ulonglong *)(param_1 + 1) = CONCAT44(fVar39,fVar40);
      param_1[3] = fVar29;
      return param_1;
    }
    *param_1 = 1;
    *(ulonglong *)(param_1 + 1) = CONCAT44(fVar39,fVar40);
    param_1[3] = fVar29;
    *(ulonglong *)(param_1 + 4) = CONCAT44(fVar25,fVar28);
    param_1[6] = fVar30;
    return param_1;
  }
LAB_1424183e8:
  *param_1 = 2;
  return param_1;
}

