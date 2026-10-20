
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409d49a0(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                  undefined8 *param_5,undefined8 *param_6,undefined8 param_7,longlong *param_8,
                  longlong *param_9)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  undefined8 uVar5;
  float fVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  longlong *plVar15;
  undefined4 *puVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  ulonglong uVar21;
  longlong *plVar22;
  longlong *plVar23;
  longlong lVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  longlong *plStack_298;
  undefined4 uStack_290;
  longlong *plStack_288;
  undefined8 uStack_280;
  longlong *plStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined4 uStack_260;
  undefined1 uStack_25c;
  undefined2 uStack_25b;
  undefined1 uStack_259;
  undefined8 uStack_258;
  undefined4 uStack_250;
  undefined8 *puStack_248;
  undefined8 *puStack_240;
  longlong lStack_238;
  undefined8 *puStack_230;
  longlong *plStack_228;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  longlong *plStack_218;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined1 uStack_1fc;
  undefined2 uStack_1fb;
  undefined1 uStack_1f9;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  longlong *plStack_1e8;
  longlong *plStack_1e0;
  float fStack_1d8;
  undefined4 uStack_1d4;
  float fStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  float fStack_1a4;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined4 uStack_190;
  undefined8 uStack_188;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  longlong *plStack_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  longlong *plStack_138;
  longlong *plStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  fVar11 = _UNK_1429cd034;
  fVar10 = _UNK_1429a0788;
  fVar9 = _UNK_14295a0e8;
  fVar6 = _UNK_142925904;
  uStack_f0 = 0xfffffffffffffffe;
  lVar24 = *(longlong *)*param_1 + 0x10;
  puStack_248 = param_3;
  puStack_240 = param_4;
  lStack_238 = lVar24;
  puStack_230 = param_2;
  if (*(char *)(*param_8 + 0x1f) == '\x01') {
    uStack_2b0 = 0x2a729;
    plVar23 = (longlong *)*param_6;
    lVar24 = 0x18;
    do {
      if (*plVar23 == 0) {
        puVar16 = (undefined4 *)func_0x000140613c10(0x1b,1);
        uVar20 = _UNK_142ac22fc;
        if (puVar16 == (undefined4 *)0x0) {
          FUN_1428d8fe3(1,0x1b,&UNK_142ad07b0);
          goto code_r0x0001409d5741;
        }
        *(ulonglong *)((longlong)puVar16 + 0xb) =
             CONCAT35(_UNK_142ac22f9,CONCAT41(_UNK_142ac22f5,UNK_142ac22f4));
        *(undefined8 *)((longlong)puVar16 + 0x13) = uVar20;
        uVar14 = _UNK_142ac22f5;
        uVar13 = _UNK_142ac22f1;
        uVar12 = _UNK_142ac22ed;
        *puVar16 = _UNK_142ac22e9;
        puVar16[1] = uVar12;
        puVar16[2] = uVar13;
        puVar16[3] = uVar14;
        plStack_228 = (longlong *)0x0;
        plStack_218 = (longlong *)0x1b;
        uStack_210 = SUB84(puVar16,0);
        uStack_20c = (undefined4)((ulonglong)puVar16 >> 0x20);
        uStack_208 = 0x1b;
        uStack_204 = 0;
        uStack_200 = CONCAT31(uStack_200._1_3_,2);
      }
      else {
        FUN_140a54440(&plStack_228,*plVar23 + 0x10,&UNK_142ac22e9,0x1b);
        if (plStack_228 == (longlong *)0x2) {
          uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
          FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1e58,&UNK_142ac2308);
          goto code_r0x0001409d5868;
        }
      }
      uStack_280 = CONCAT44(uStack_21c,uStack_220);
      uStack_270 = CONCAT44(uStack_20c,uStack_210);
      uStack_268 = CONCAT44(uStack_204,uStack_208);
      uStack_260 = uStack_200;
      uStack_25c = uStack_1fc;
      uStack_25b = uStack_1fb;
      uStack_259 = uStack_1f9;
      plStack_278 = plStack_218;
      plStack_288 = plStack_228;
      uStack_f1 = 1;
      fVar25 = (float)func_0x000140caf380(&uStack_2b0);
      uStack_f1 = 1;
      fVar26 = (float)func_0x000140caf380(&uStack_2b0);
      uStack_f1 = 1;
      func_0x000140c9d180(&uStack_178);
      plVar15 = uStack_170;
      plVar22 = uStack_178;
      uStack_258 = CONCAT44(uStack_158,uStack_15c);
      uStack_250 = uStack_154;
      uStack_f1 = 1;
      fVar27 = (float)func_0x000140caf380(&uStack_2b0);
      fStack_1d8 = (fVar25 + fVar9) * fVar10;
      fStack_1d0 = (fVar26 + fVar9) * fVar10;
      fStack_1a4 = fVar27 * fVar11 + fVar6;
      uStack_208 = (undefined4)uStack_268;
      uStack_204 = uStack_268._4_4_;
      uStack_200 = uStack_260;
      uStack_1fc = uStack_25c;
      uStack_1fb = uStack_25b;
      uStack_1f9 = uStack_259;
      plStack_218 = plStack_278;
      uStack_210 = (undefined4)uStack_270;
      uStack_20c = uStack_270._4_4_;
      plStack_228 = plStack_288;
      uStack_220 = (undefined4)uStack_280;
      uStack_21c = uStack_280._4_4_;
      plStack_1e8 = plVar22;
      plStack_1e0 = plVar15;
      uStack_1d4 = 0x3f800000;
      uStack_1c4 = uStack_250;
      uStack_1cc = (undefined4)uStack_258;
      uStack_1c8 = (undefined4)((ulonglong)uStack_258 >> 0x20);
      uStack_1a8 = 2;
      uStack_1a0 = 0;
      uStack_198 = 0;
      uStack_190 = 3;
      uStack_1f8 = 0;
      uStack_1b0 = 0;
      uStack_1b8 = 0;
      uStack_1ac = 0x3d4ccccd;
      uStack_180 = 0;
      uStack_188 = 0;
      uStack_17c = 0x3f800000;
      uStack_f1 = 0;
      FUN_1412e1f20(&uStack_178,param_7,&plStack_228,&UNK_142ac2320);
      lVar24 = lVar24 + -1;
    } while (lVar24 != 0);
    FUN_140b0eeb0(&plStack_228,&UNK_142ac20a5,0x1c);
    if (SBORROW8(0,(longlong)plStack_228)) {
      uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1e58,&UNK_142ac20f8);
      goto code_r0x0001409d5868;
    }
    uStack_108 = uStack_1b8;
    uStack_118 = CONCAT44(uStack_1c4,uStack_1c8);
    uStack_110 = uStack_1c0;
    uStack_128 = CONCAT44(uStack_1d4,fStack_1d8);
    uStack_120 = CONCAT44(uStack_1cc,fStack_1d0);
    plStack_138 = plStack_1e8;
    plStack_130 = plStack_1e0;
    uStack_14c = CONCAT13(uStack_1f9,CONCAT21(uStack_1fb,uStack_1fc));
    uStack_148 = uStack_1f8;
    uStack_144 = uStack_1f4;
    uStack_140 = uStack_1f0;
    uStack_13c = uStack_1ec;
    uStack_158 = uStack_208;
    uStack_154 = uStack_204;
    uStack_150 = uStack_200;
    plStack_168 = plStack_218;
    uStack_160 = uStack_210;
    uStack_15c = uStack_20c;
    uStack_178 = plStack_228;
    uStack_170._0_4_ = uStack_220;
    uStack_170._4_4_ = uStack_21c;
    uVar17 = FUN_140b1aca0(&uStack_178,&UNK_142ac2110,9,*param_5,*puStack_240);
    param_2 = puStack_230;
    lVar24 = lStack_238;
    uVar18 = FUN_140b3fe40(lStack_238,&UNK_142ac2119,0x1f,&UNK_142ac2138,0x10,*puStack_248,
                           *puStack_230);
    func_0x000140c9d180(&plStack_228);
    plVar23 = plStack_228;
    uStack_170 = (longlong *)CONCAT44(uStack_170._4_4_,uStack_210);
    uStack_178 = plStack_218;
    uVar20 = CONCAT44(uStack_21c,uStack_220);
    FUN_140d146a0(&plStack_228,param_7);
    plVar22 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    if ((longlong *)CONCAT44(uStack_21c,uStack_220) == (longlong *)0x0) {
      plVar22 = plStack_218;
    }
    uVar19 = CONCAT44(uStack_204,uStack_208);
    lVar2 = plVar22[2];
    if ((ulonglong)(*plVar22 - lVar2) < 0x78) {
      FUN_1428a9a60(plVar22,lVar2,0x78,1,1);
    }
    lVar3 = plVar22[1];
    *(code **)(lVar3 + lVar2) = FUN_1412dd460;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = plVar23;
    puVar1[1] = uVar20;
    *(longlong **)(lVar3 + 0x18 + lVar2) = uStack_178;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = (undefined4)uStack_170;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = _UNK_142a8c620;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f19999a;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = uVar17;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = uVar18;
    *(undefined1 *)(lVar3 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x58 + lVar2) = uVar19;
    *(code **)(lVar3 + 0x68 + lVar2) = FUN_141564d30;
    plVar22[2] = lVar2 + 0x78;
  }
  fVar25 = _UNK_142ac1930;
  fVar11 = _UNK_142ac192c;
  fVar10 = _UNK_142ac1928;
  fVar9 = _UNK_142a83814;
  fVar6 = _UNK_142925904;
  if (*(char *)(*param_9 + 0x58) != '\x04') {
    uStack_258 = 0x29a;
    uVar21 = 0;
    do {
      fVar27 = (float)FUN_140c909a0(uVar21 & 0xffffffff,2);
      fVar28 = (float)func_0x000140caf380(&uStack_258);
      fVar26 = fVar6;
      fVar27 = (float)func_0x000142923f10(fVar28 * _UNK_142925950 + fVar27);
      fVar28 = (float)func_0x000140caf380(&uStack_258);
      fVar27 = (float)FUN_140c90700(CONCAT44((fVar28 + (float)(longlong)uVar21) / fVar10,
                                             fVar27 * fVar11 + fVar25));
      fVar28 = (float)func_0x000140caf380(&uStack_258);
      func_0x000140c9d180(&plStack_228);
      plVar23 = plStack_228;
      uVar20 = CONCAT44(uStack_21c,uStack_220);
      plStack_288 = (longlong *)CONCAT44(uStack_208,uStack_20c);
      uStack_280 = CONCAT44(uStack_280._4_4_,uStack_204);
      plStack_168 = (longlong *)((ulonglong)plStack_168 & 0xffffffff00000000);
      uStack_170 = (longlong *)0x0;
      FUN_140d146a0(&plStack_228,param_7);
      plVar22 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      if ((longlong *)CONCAT44(uStack_21c,uStack_220) == (longlong *)0x0) {
        plVar22 = plStack_218;
      }
      uVar17 = CONCAT44(uStack_204,uStack_208);
      lVar24 = plVar22[2];
      if ((ulonglong)(*plVar22 - lVar24) < 0xa8) {
        FUN_1428a9a60(plVar22,lVar24,0xa8,1,1);
      }
      uVar21 = uVar21 + 1;
      lVar2 = plVar22[1];
      *(code **)(lVar2 + lVar24) = FUN_1412d7e60;
      *(undefined4 *)(lVar2 + 8 + lVar24) = 0;
      puVar1 = (undefined8 *)(lVar2 + 0x18 + lVar24);
      *puVar1 = plVar23;
      puVar1[1] = uVar20;
      *(float *)(lVar2 + 0x28 + lVar24) = fVar27 * fVar9;
      *(float *)(lVar2 + 0x2c + lVar24) = fVar28 + fVar6;
      *(float *)(lVar2 + 0x30 + lVar24) = fVar26 * fVar9;
      *(longlong **)(lVar2 + 0x34 + lVar24) = plStack_288;
      *(undefined4 *)(lVar2 + 0x3c + lVar24) = (undefined4)uStack_280;
      puVar16 = (undefined4 *)(lVar2 + 0x40 + lVar24);
      *puVar16 = (undefined4)uStack_178;
      puVar16[1] = uStack_178._4_4_;
      puVar16[2] = (undefined4)uStack_170;
      puVar16[3] = uStack_170._4_4_;
      *(undefined4 *)(lVar2 + 0x50 + lVar24) = plStack_168._0_4_;
      *(undefined8 *)(lVar2 + 0x54 + lVar24) = 0x33d4ccccd;
      puVar1 = (undefined8 *)(lVar2 + 0x60 + lVar24);
      *puVar1 = 0;
      puVar1[1] = 0;
      *(undefined4 *)(lVar2 + 0x70 + lVar24) = 0x3f800000;
      *(undefined1 *)(lVar2 + 0x78 + lVar24) = 0;
      *(undefined8 *)(lVar2 + 0x88 + lVar24) = uVar17;
      *(code **)(lVar2 + 0x98 + lVar24) = FUN_141564d30;
      plVar22[2] = lVar24 + 0xa8;
    } while (uVar21 != 0x60);
    FUN_140a06b00(&plStack_228,0,_UNK_142925904);
    if (SBORROW8(0,(longlong)plStack_228)) {
      plStack_168 = (longlong *)CONCAT44(uStack_20c,uStack_210);
      uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      uStack_170 = plStack_218;
      FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1ea8,&UNK_142ac2148);
      goto code_r0x0001409d5868;
    }
    uStack_108 = uStack_1b8;
    uStack_118 = CONCAT44(uStack_1c4,uStack_1c8);
    uStack_110 = uStack_1c0;
    uStack_128 = CONCAT44(uStack_1d4,fStack_1d8);
    uStack_120 = CONCAT44(uStack_1cc,fStack_1d0);
    plStack_138 = plStack_1e8;
    plStack_130 = plStack_1e0;
    uStack_170 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    uStack_14c = CONCAT13(uStack_1f9,CONCAT21(uStack_1fb,uStack_1fc));
    uStack_148 = uStack_1f8;
    uStack_144 = uStack_1f4;
    uStack_140 = uStack_1f0;
    uStack_13c = uStack_1ec;
    uStack_158 = uStack_208;
    uStack_154 = uStack_204;
    uStack_150 = uStack_200;
    plStack_168 = plStack_218;
    uStack_160 = uStack_210;
    uStack_15c = uStack_20c;
    uStack_178 = plStack_228;
    plStack_298 = (longlong *)(ulonglong)(uint)_UNK_142925904;
    uStack_290 = 0x3f800000;
    uStack_280 = CONCAT44(uStack_280._4_4_,0x3f800000);
    plStack_288 = plStack_298;
    FUN_140a07680(&plStack_228,&uStack_178,&plStack_288);
    uStack_2a8 = *param_5;
    uVar20 = *puStack_240;
    uStack_2a0 = FUN_140b1aca0(&plStack_228,&UNK_142ac2160,7,uStack_2a8,uVar20);
    uVar17 = *puStack_248;
    uVar18 = *puStack_230;
    uVar19 = FUN_140b3fe40(lStack_238,&UNK_142ac2167,0x1d,&UNK_142ac2184,0xe,uVar17,uVar18);
    FUN_140d146a0(&plStack_228,param_7);
    plVar23 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    if ((longlong *)CONCAT44(uStack_21c,uStack_220) == (longlong *)0x0) {
      plVar23 = plStack_218;
    }
    uVar5 = CONCAT44(uStack_204,uStack_208);
    lVar24 = plVar23[2];
    if ((ulonglong)(*plVar23 - lVar24) < 0x78) {
      FUN_1428a9a60(plVar23,lVar24,0x78,1,1);
    }
    lVar2 = plVar23[1];
    *(code **)(lVar2 + lVar24) = FUN_1412d9de0;
    uVar8 = _UNK_14293e0c8;
    uVar7 = _UNK_14293e0c0;
    puVar1 = (undefined8 *)(lVar2 + 8 + lVar24);
    *puVar1 = _UNK_14293e0c0;
    puVar1[1] = uVar8;
    *(undefined8 *)(lVar2 + 0x18 + lVar24) = 0;
    *(undefined4 *)(lVar2 + 0x20 + lVar24) = 0;
    *(undefined8 *)(lVar2 + 0x24 + lVar24) = 0x3f8000003f800000;
    *(undefined4 *)(lVar2 + 0x2c + lVar24) = 0x3f800000;
    *(undefined8 *)(lVar2 + 0x38 + lVar24) = uStack_2a0;
    *(undefined8 *)(lVar2 + 0x40 + lVar24) = uVar19;
    *(undefined1 *)(lVar2 + 0x48 + lVar24) = 0;
    *(undefined8 *)(lVar2 + 0x58 + lVar24) = uVar5;
    *(code **)(lVar2 + 0x68 + lVar24) = FUN_141564d30;
    plVar23[2] = lVar24 + 0x78;
    FUN_140a06370(&plStack_228,_UNK_142925904);
    FUN_140a07680(&uStack_178,&plStack_228,&plStack_298);
    uVar20 = FUN_140b1aca0(&uStack_178,&UNK_142ac2192,0xc,uStack_2a8,uVar20);
    lVar24 = lStack_238;
    uVar17 = FUN_140b3fe40(lStack_238,&UNK_142ac219e,0x22,&UNK_142ac21c0,0x13,uVar17,uVar18);
    FUN_140d146a0(&plStack_228,param_7);
    plVar23 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    if ((longlong *)CONCAT44(uStack_21c,uStack_220) == (longlong *)0x0) {
      plVar23 = plStack_218;
    }
    uVar18 = CONCAT44(uStack_204,uStack_208);
    lVar2 = plVar23[2];
    if ((ulonglong)(*plVar23 - lVar2) < 0x88) {
      FUN_1428a9a60(plVar23,lVar2,0x88,1,1);
    }
    lVar3 = plVar23[1];
    *(code **)(lVar3 + lVar2) = FUN_1412db500;
    puVar1 = (undefined8 *)(lVar3 + 8 + lVar2);
    *puVar1 = uVar7;
    puVar1[1] = uVar8;
    *(undefined8 *)(lVar3 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar2) = uVar20;
    *(undefined8 *)(lVar3 + 0x40 + lVar2) = uVar17;
    *(undefined4 *)(lVar3 + 0x48 + lVar2) = 0x11;
    *(undefined1 *)(lVar3 + 0x58 + lVar2) = 0;
    *(undefined8 *)(lVar3 + 0x68 + lVar2) = uVar18;
    *(code **)(lVar3 + 0x78 + lVar2) = FUN_141564d30;
    plVar23[2] = lVar2 + 0x88;
    param_2 = puStack_230;
  }
  FUN_140b0eeb0(&plStack_228,&UNK_142ac21d3,0x1e);
  if (SBORROW8(0,(longlong)plStack_228)) {
code_r0x0001409d5741:
    uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1e58,&UNK_142ac21f8);
  }
  else {
    uStack_108 = uStack_1b8;
    uStack_118 = CONCAT44(uStack_1c4,uStack_1c8);
    uStack_110 = uStack_1c0;
    uStack_128 = CONCAT44(uStack_1d4,fStack_1d8);
    uStack_120 = CONCAT44(uStack_1cc,fStack_1d0);
    plStack_138 = plStack_1e8;
    plStack_130 = plStack_1e0;
    uStack_14c = CONCAT13(uStack_1f9,CONCAT21(uStack_1fb,uStack_1fc));
    uStack_148 = uStack_1f8;
    uStack_144 = uStack_1f4;
    uStack_140 = uStack_1f0;
    uStack_13c = uStack_1ec;
    uStack_158 = uStack_208;
    uStack_154 = uStack_204;
    uStack_150 = uStack_200;
    plStack_168 = plStack_218;
    uStack_160 = uStack_210;
    uStack_15c = uStack_20c;
    uStack_178 = plStack_228;
    uStack_170 = (longlong *)CONCAT44(uStack_21c,uStack_220);
    uVar20 = *param_5;
    uVar17 = *puStack_240;
    FUN_140b1aca0(&uStack_178,&UNK_142ac2210,4,uVar20,uVar17);
    uVar18 = *puStack_248;
    uVar19 = *param_2;
    FUN_140b3fe40(lVar24,&UNK_142ac2214,0x23,&UNK_142ac2237,0xb,uVar18,uVar19);
    FUN_140b0eeb0(&plStack_228,&UNK_142ac2242,0x1d);
    if (SBORROW8(0,(longlong)plStack_228)) {
      uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1e58,&UNK_142ac2260);
    }
    else {
      uStack_108 = uStack_1b8;
      uStack_118 = CONCAT44(uStack_1c4,uStack_1c8);
      uStack_110 = uStack_1c0;
      uStack_128 = CONCAT44(uStack_1d4,fStack_1d8);
      uStack_120 = CONCAT44(uStack_1cc,fStack_1d0);
      plStack_138 = plStack_1e8;
      plStack_130 = plStack_1e0;
      uStack_14c = CONCAT13(uStack_1f9,CONCAT21(uStack_1fb,uStack_1fc));
      uStack_148 = uStack_1f8;
      uStack_144 = uStack_1f4;
      uStack_140 = uStack_1f0;
      uStack_13c = uStack_1ec;
      uStack_158 = uStack_208;
      uStack_154 = uStack_204;
      uStack_150 = uStack_200;
      plStack_168 = plStack_218;
      uStack_160 = uStack_210;
      uStack_15c = uStack_20c;
      uStack_178 = plStack_228;
      uStack_170 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      FUN_140b1aca0(&uStack_178,&UNK_142ac2278,10,uVar20,uVar17);
      FUN_140b0eeb0(&plStack_228,&UNK_142ac2282,0x19);
      if (!SBORROW8(0,(longlong)plStack_228)) {
        uStack_108 = uStack_1b8;
        uStack_118 = CONCAT44(uStack_1c4,uStack_1c8);
        uStack_110 = uStack_1c0;
        uStack_128 = CONCAT44(uStack_1d4,fStack_1d8);
        uStack_120 = CONCAT44(uStack_1cc,fStack_1d0);
        plStack_138 = plStack_1e8;
        plStack_130 = plStack_1e0;
        uStack_14c = CONCAT13(uStack_1f9,CONCAT21(uStack_1fb,uStack_1fc));
        uStack_148 = uStack_1f8;
        uStack_144 = uStack_1f4;
        uStack_140 = uStack_1f0;
        uStack_13c = uStack_1ec;
        uStack_158 = uStack_208;
        uStack_154 = uStack_204;
        uStack_150 = uStack_200;
        plStack_168 = plStack_218;
        uStack_160 = uStack_210;
        uStack_15c = uStack_20c;
        uStack_178 = plStack_228;
        uStack_170 = (longlong *)CONCAT44(uStack_21c,uStack_220);
        FUN_140b1aca0(&uStack_178,&UNK_142ac22b8,6,uVar20,uVar17);
        FUN_140b3fe40(lVar24,&UNK_142ac22be,0x1c,&UNK_142ac22da,0xf,uVar18,uVar19);
        return;
      }
      uStack_178 = (longlong *)CONCAT44(uStack_21c,uStack_220);
      FUN_1428d9760(&UNK_142ac1e78,0x2b,&uStack_178,&UNK_142ac1e58,&UNK_142ac22a0);
    }
  }
code_r0x0001409d5868:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

