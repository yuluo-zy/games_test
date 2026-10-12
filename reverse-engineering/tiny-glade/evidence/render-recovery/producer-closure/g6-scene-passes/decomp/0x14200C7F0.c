
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14200c7f0(float param_1,float param_2,float param_3,float param_4,float param_5,
                  float param_6,float param_7,undefined8 param_8,float param_9,float param_10,
                  undefined8 *param_11,undefined8 param_12)

{
  longlong lVar1;
  float *pfVar2;
  code *pcVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  longlong lVar10;
  undefined8 uVar11;
  float fVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  bool bVar20;
  char cVar21;
  undefined4 uVar22;
  int iVar23;
  ulonglong uVar24;
  float *pfVar25;
  float *pfVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  float *pfVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fStack_318;
  float fStack_314;
  float fStack_310;
  float fStack_30c;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined4 uStack_2d0;
  undefined4 uStack_2bc;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined1 auStack_288 [16];
  undefined8 uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  ulonglong uStack_228;
  ulonglong uStack_220;
  ulonglong uStack_218;
  float *pfStack_210;
  ulonglong uStack_208;
  longlong lStack_200;
  undefined8 uStack_1f8;
  undefined1 *puStack_1f0;
  float *pfStack_1e8;
  ulonglong uStack_1e0;
  longlong lStack_1d8;
  float *pfStack_1d0;
  float fStack_1c8;
  float fStack_1c4;
  float fStack_1c0;
  float fStack_1bc;
  float fStack_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  float fStack_1a8;
  float fStack_1a4;
  float fStack_1a0;
  float fStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined8 uStack_184;
  int iStack_17c;
  undefined4 uStack_178;
  undefined8 uStack_174;
  undefined4 uStack_16c;
  undefined8 uStack_164;
  undefined4 uStack_15c;
  float fStack_158;
  float fStack_154;
  float fStack_150;
  float fStack_14c;
  ulonglong uStack_148;
  ulonglong uStack_140;
  float *pfStack_138;
  float *pfStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  float fStack_11c;
  float fStack_118;
  float fStack_114;
  float fStack_108;
  float fStack_104;
  float fStack_100;
  float fStack_fc;
  longlong lStack_f8;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  fVar37 = (param_3 - param_4) + param_6;
  fStack_150 = param_2;
  fStack_14c = param_1;
  fVar30 = (float)func_0x000142923e60(fVar37 / _UNK_1429cd068);
  uVar24 = 0;
  if (0.0 <= fVar30) {
    uVar24 = (longlong)(fVar30 - _UNK_14295a0dc) & (longlong)fVar30 >> 0x3f | (longlong)fVar30;
  }
  uVar28 = 0xffffffffffffffff;
  if (fVar30 <= _UNK_14295a0e0) {
    uVar28 = uVar24;
  }
  uVar24 = 2;
  if (2 < uVar28) {
    uVar24 = uVar28;
  }
  FUN_140c90350(&fStack_1c8,uVar24,_UNK_142fce8a0 / fVar37,param_8,&UNK_142fce968);
  fVar30 = fStack_1b8;
  lVar10 = CONCAT44(fStack_1bc,fStack_1c0);
  uVar24 = CONCAT44(fStack_1b4,fStack_1b8);
  lVar1 = uVar24 * 4;
  if (uVar24 == 0) {
    pfVar25 = (float *)0x4;
    fVar37 = _UNK_142925904;
  }
  else {
    pfVar25 = (float *)func_0x000140613c10(lVar1,4);
    fVar12 = _UNK_14292593c;
    fVar34 = _UNK_142925938;
    fVar31 = _UNK_142925934;
    fVar37 = _UNK_142925930;
    if (pfVar25 == (float *)0x0) {
      lStack_f8 = lVar10;
      FUN_1428d8fe3(4,lVar1,&UNK_142fce900);
code_r0x00014200d67a:
                    /* WARNING: Does not return */
      pcVar3 = (code *)invalidInstructionException();
      (*pcVar3)();
    }
    param_6 = param_4 - param_6;
    if (uVar24 < 8 || (ulonglong)((longlong)pfVar25 - lVar10) < 0x20) {
      uVar28 = 0;
    }
    else {
      uVar28 = uVar24 & 0xfffffffffffffff8;
      uVar27 = 0;
      do {
        pfVar26 = (float *)(lVar10 + uVar27 * 4);
        fVar4 = pfVar26[1];
        fVar32 = pfVar26[2];
        fVar33 = pfVar26[3];
        pfVar29 = (float *)(lVar10 + 0x10 + uVar27 * 4);
        fVar35 = *pfVar29;
        fVar36 = pfVar29[1];
        fVar5 = pfVar29[2];
        fVar6 = pfVar29[3];
        pfVar29 = pfVar25 + uVar27;
        *pfVar29 = *pfVar26 * param_6 + (fVar37 - *pfVar26) * param_3;
        pfVar29[1] = fVar4 * param_6 + (fVar31 - fVar4) * param_3;
        pfVar29[2] = fVar32 * param_6 + (fVar34 - fVar32) * param_3;
        pfVar29[3] = fVar33 * param_6 + (fVar12 - fVar33) * param_3;
        pfVar26 = pfVar25 + uVar27 + 4;
        *pfVar26 = fVar35 * param_6 + (fVar37 - fVar35) * param_3;
        pfVar26[1] = fVar36 * param_6 + (fVar31 - fVar36) * param_3;
        pfVar26[2] = fVar5 * param_6 + (fVar34 - fVar5) * param_3;
        pfVar26[3] = fVar6 * param_6 + (fVar12 - fVar6) * param_3;
        uVar27 = uVar27 + 8;
      } while (uVar28 != uVar27);
      fVar37 = _UNK_142925904;
      if (uVar24 == uVar28) goto code_r0x00014200ca8b;
    }
    uVar27 = uVar28 | 1;
    fVar37 = _UNK_142925904;
    if (((uint)fVar30 & 1) != 0) {
      fVar30 = *(float *)(lVar10 + uVar28 * 4);
      pfVar25[uVar28] = fVar30 * param_6 + (_UNK_142925904 - fVar30) * param_3;
      uVar28 = uVar27;
      fVar37 = _UNK_142925904;
    }
    while (fVar30 = _UNK_142925904, uVar24 != uVar27) {
      fVar31 = *(float *)(lVar10 + uVar28 * 4);
      fVar34 = _UNK_142925904 - fVar31;
      _UNK_142925904 = fVar37;
      pfVar25[uVar28] = fVar31 * param_6 + fVar34 * param_3;
      fVar37 = *(float *)(lVar10 + 4 + uVar28 * 4);
      pfVar25[uVar28 + 1] = fVar37 * param_6 + (fVar30 - fVar37) * param_3;
      uVar27 = uVar28 + 2;
      uVar28 = uVar27;
      fVar37 = _UNK_142925904;
      _UNK_142925904 = fVar30;
    }
  }
code_r0x00014200ca8b:
  _UNK_142925904 = fVar37;
  lStack_1d8 = lVar1;
  if (CONCAT44(fStack_1c4,fStack_1c8) != 0) {
    func_0x000140613c20(lVar10,CONCAT44(fStack_1c4,fStack_1c8) << 2,4);
  }
  fVar31 = (float)func_0x000142923e60((param_3 - param_4) / _UNK_1429cd068);
  fVar37 = _UNK_14295a0e0;
  fVar30 = _UNK_14295a0dc;
  uVar28 = (longlong)fVar31 >> 0x3f & (longlong)(fVar31 - _UNK_14295a0dc) | (longlong)fVar31;
  if (fVar31 < 0.0) {
    uVar28 = 0;
  }
  if (_UNK_14295a0e0 < fVar31) {
    uVar28 = 0xffffffffffffffff;
  }
  uVar27 = 5;
  if (uVar28 < 5) {
    uVar27 = uVar28;
  }
  uStack_208 = 2;
  if (2 < uVar27) {
    uStack_208 = uVar27;
  }
  param_7 = param_7 / (float)(longlong)uStack_208;
  fVar32 = (float)func_0x000142923e60(param_5 / _UNK_142a7fca4);
  fVar4 = _UNK_14292593c;
  fVar12 = _UNK_142925938;
  fVar34 = _UNK_142925934;
  fVar31 = _UNK_142925930;
  uVar28 = (longlong)fVar32 >> 0x3f & (longlong)(fVar32 - fVar30) | (longlong)fVar32;
  if (fVar32 < 0.0) {
    uVar28 = 0;
  }
  if (fVar37 < fVar32) {
    uVar28 = 0xffffffffffffffff;
  }
  uStack_220 = 3;
  if (3 < uVar28) {
    uStack_220 = uVar28;
  }
  pfStack_1e8 = pfVar25 + uVar24;
  fStack_104 = _UNK_142925984 + param_7;
  fVar32 = _UNK_142fce8a4 / param_5;
  param_5 = param_5 * _UNK_142925870;
  uStack_1f8 = *param_11;
  puStack_1f0 = (undefined1 *)param_11[1];
  bVar20 = true;
  uVar28 = 0;
  fVar30 = _UNK_142a80a4c;
  fVar37 = _UNK_142925904;
  uStack_1e0 = uVar24;
  pfStack_1d0 = pfVar25;
  pfStack_130 = pfVar25;
  fStack_100 = param_7;
  fStack_fc = fVar32;
code_r0x00014200cc79:
  if (pfStack_130 == pfStack_1e8) {
code_r0x00014200d5bd:
    if (uStack_1e0 != 0) {
      func_0x000140613c20(pfStack_1d0,lStack_1d8,4);
    }
    return;
  }
  pfVar25 = pfStack_130 + 1;
  pfVar26 = pfStack_130;
  if (bVar20) {
    if (pfVar25 == pfStack_1e8) goto code_r0x00014200d5bd;
    pfStack_210 = pfStack_130;
    pfVar26 = pfVar25;
    pfVar25 = pfStack_130 + 2;
  }
  pfStack_130 = pfVar25;
  uStack_218 = uVar28 + 1;
  uStack_228 = uStack_218;
  if (uStack_208 < uStack_218) {
    uStack_228 = uStack_208;
  }
  auStack_288 = ZEXT416((uint)(*pfVar26 - *pfStack_210) & _UNK_14292e980);
  lStack_200 = uVar28 * 2;
  fStack_108 = *pfVar26 * _UNK_142925870 + *pfStack_210 * _UNK_142925870;
  uStack_148 = 0;
  pfStack_210 = pfVar26;
code_r0x00014200cd95:
  do {
    if (uStack_228 <= uStack_148) break;
    FUN_140c90350(&fStack_1c8,uStack_220,fVar32,param_8,&UNK_142fce980);
    fVar33 = fStack_1b8;
    lVar1 = CONCAT44(fStack_1bc,fStack_1c0);
    uVar24 = CONCAT44(fStack_1b4,fStack_1b8);
    if (uVar24 == 0) {
      pfVar25 = (float *)0x4;
    }
    else {
      pfVar25 = (float *)func_0x000140613c10(uVar24 * 4,4);
      if (pfVar25 == (float *)0x0) {
        lStack_f8 = lVar1;
        FUN_1428d8fe3(4,uVar24 * 4,&UNK_142fce900);
        goto code_r0x00014200d67a;
      }
      if (uVar24 < 8 || (ulonglong)((longlong)pfVar25 - lVar1) < 0x20) {
        uVar28 = 0;
      }
      else {
        uVar28 = uVar24 & 0xfffffffffffffff8;
        uVar27 = 0;
        do {
          pfVar26 = (float *)(lVar1 + uVar27 * 4);
          fVar35 = pfVar26[1];
          fVar36 = pfVar26[2];
          fVar5 = pfVar26[3];
          pfVar29 = (float *)(lVar1 + 0x10 + uVar27 * 4);
          fVar6 = *pfVar29;
          fVar7 = pfVar29[1];
          fVar8 = pfVar29[2];
          fVar9 = pfVar29[3];
          pfVar29 = pfVar25 + uVar27;
          *pfVar29 = *pfVar26 * param_5 - (fVar31 - *pfVar26) * param_5;
          pfVar29[1] = fVar35 * param_5 - (fVar34 - fVar35) * param_5;
          pfVar29[2] = fVar36 * param_5 - (fVar12 - fVar36) * param_5;
          pfVar29[3] = fVar5 * param_5 - (fVar4 - fVar5) * param_5;
          pfVar26 = pfVar25 + uVar27 + 4;
          *pfVar26 = fVar6 * param_5 - (fVar31 - fVar6) * param_5;
          pfVar26[1] = fVar7 * param_5 - (fVar34 - fVar7) * param_5;
          pfVar26[2] = fVar8 * param_5 - (fVar12 - fVar8) * param_5;
          pfVar26[3] = fVar9 * param_5 - (fVar4 - fVar9) * param_5;
          uVar27 = uVar27 + 8;
        } while (uVar28 != uVar27);
        if (uVar24 == uVar28) goto code_r0x00014200cf2f;
      }
      uVar27 = uVar28 | 1;
      if (((uint)fVar33 & 1) != 0) {
        fVar33 = *(float *)(lVar1 + uVar28 * 4);
        pfVar25[uVar28] = fVar33 * param_5 - (fVar37 - fVar33) * param_5;
        uVar28 = uVar27;
      }
      while (uVar24 != uVar27) {
        fVar33 = *(float *)(lVar1 + uVar28 * 4);
        pfVar25[uVar28] = fVar33 * param_5 - (fVar37 - fVar33) * param_5;
        fVar33 = *(float *)(lVar1 + 4 + uVar28 * 4);
        pfVar25[uVar28 + 1] = fVar33 * param_5 - (fVar37 - fVar33) * param_5;
        uVar27 = uVar28 + 2;
        uVar28 = uVar27;
      }
    }
code_r0x00014200cf2f:
    if (CONCAT44(fStack_1c4,fStack_1c8) != 0) {
      func_0x000140613c20(lVar1,CONCAT44(fStack_1c4,fStack_1c8) << 2,4);
    }
    fVar33 = (float)(longlong)uStack_148;
    uStack_148 = uStack_148 + 1;
    fVar33 = fVar33 * param_7 + fVar30;
    if (uVar24 != 0) {
      if (uVar24 != 1) {
        fStack_154 = fVar33 * param_10 + fStack_150;
        fStack_158 = fVar33 * param_9 + fStack_14c;
        fVar30 = *pfVar25;
        fVar37 = pfVar25[1];
        uVar11 = *(undefined8 *)pfVar25;
        fVar32 = fVar37 * _UNK_142925870 + fVar30 * _UNK_142925870;
        fStack_1c8 = fStack_158 - fVar32 * param_10;
        fStack_1c4 = fVar32 * param_9 + fStack_154;
        uStack_140 = uVar24;
        pfStack_138 = pfVar25;
        func_0x000140c9cde0(&uStack_164,&fStack_1c8,fStack_108);
        FUN_140c90fd0(&fStack_318,param_9,param_10);
        fStack_11c = (float)((uint)(fVar37 - fVar30) & _UNK_14292e980);
        uStack_128 = uStack_164;
        uVar13 = uStack_128;
        uStack_120 = uStack_15c;
        lStack_f8 = CONCAT44(lStack_f8._4_4_,fVar30);
        fVar33 = fStack_310 + fStack_310;
        fVar32 = fStack_314 + fStack_314;
        fVar30 = fStack_318 * (fStack_318 + fStack_318);
        fVar37 = (fStack_318 + fStack_318) * fStack_30c;
        fStack_1bc = fVar33 * fStack_318 - fVar32 * fStack_30c;
        fStack_1ac = fVar37 + fVar33 * fStack_314;
        fStack_118 = (float)auStack_288._0_4_;
        fStack_114 = fStack_104;
        fStack_19c = _UNK_142925904 - (fVar30 + fStack_314 * fVar32);
        fStack_1c8 = fStack_11c * (_UNK_142925904 - (fStack_314 * fVar32 + fStack_310 * fVar33));
        fStack_1c4 = fStack_11c * (fVar32 * fStack_318 + fStack_30c * fVar33);
        fStack_1c0 = fStack_11c * fStack_1bc;
        fStack_1bc = fStack_11c * fStack_1bc;
        fStack_1b8 = (float)auStack_288._0_4_ * (fVar32 * fStack_318 - fStack_30c * fVar33);
        fStack_1b4 = (float)auStack_288._0_4_ * (_UNK_142925904 - (fStack_310 * fVar33 + fVar30));
        fStack_1b0 = (float)auStack_288._0_4_ * fStack_1ac;
        fStack_1ac = (float)auStack_288._0_4_ * fStack_1ac;
        fStack_1a8 = fStack_104 * (fVar32 * fStack_30c + fVar33 * fStack_318);
        fStack_1a4 = fStack_104 * (fVar33 * fStack_314 - fVar37);
        fStack_1a0 = fStack_104 * fStack_19c;
        fStack_19c = fStack_104 * fStack_19c;
        uStack_128._0_4_ = (undefined4)uStack_164;
        uStack_128._4_4_ = (undefined4)((ulonglong)uStack_164 >> 0x20);
        uStack_198 = (undefined4)uStack_128;
        uStack_194 = uStack_128._4_4_;
        uStack_190 = uStack_15c;
        _uStack_18c = CONCAT44(uStack_188,uStack_15c);
        uStack_128 = uVar13;
        func_0x00014089d560(&uStack_2b8,&fStack_1c8);
        uVar22 = func_0x000140858790(param_12);
        iVar23 = func_0x0001400885d0(&param_12);
        func_0x0001408587c0(&fStack_318);
        iStack_17c = iVar23 + (int)lStack_200;
        uStack_258 = uStack_298;
        uVar17 = uStack_258;
        uStack_250 = uStack_290;
        uVar18 = uStack_250;
        uStack_268 = uStack_2a8;
        uVar15 = uStack_268;
        uStack_260 = uStack_2a0;
        uVar16 = uStack_260;
        uStack_278 = uStack_2b8;
        uVar13 = uStack_278;
        uStack_270 = uStack_2b0;
        uVar14 = uStack_270;
        uStack_248 = uStack_2e8;
        uVar19 = uStack_248;
        uStack_240 = (undefined4)uStack_2e0;
        uStack_23c = (undefined4)((ulonglong)uStack_2e0 >> 0x20);
        uStack_230 = uStack_2d0;
        uStack_238 = (undefined4)uStack_2d8;
        uStack_234 = (undefined4)((ulonglong)uStack_2d8 >> 0x20);
        uStack_184 = CONCAT44(uStack_2d0,uStack_234);
        uStack_278._0_4_ = (float)uStack_2b8;
        uStack_278._4_4_ = (undefined4)((ulonglong)uStack_2b8 >> 0x20);
        uStack_270._0_4_ = (undefined4)uStack_2b0;
        uStack_270._4_4_ = (undefined4)((ulonglong)uStack_2b0 >> 0x20);
        uStack_268._0_4_ = (uint)uStack_2a8;
        uStack_268._4_4_ = (undefined4)((ulonglong)uStack_2a8 >> 0x20);
        uStack_260._0_4_ = (undefined4)uStack_2a0;
        uStack_260._4_4_ = (undefined4)((ulonglong)uStack_2a0 >> 0x20);
        uStack_258._0_4_ = (undefined4)uStack_298;
        uStack_258._4_4_ = (undefined4)((ulonglong)uStack_298 >> 0x20);
        uStack_250._0_4_ = (undefined4)uStack_290;
        uStack_250._4_4_ = (undefined4)((ulonglong)uStack_290 >> 0x20);
        uStack_248._0_4_ = (undefined4)uStack_2e8;
        uStack_248._4_4_ = (undefined4)((ulonglong)uStack_2e8 >> 0x20);
        uStack_198 = (undefined4)uStack_248;
        uStack_194 = uStack_248._4_4_;
        uStack_190 = uStack_240;
        _uStack_18c = CONCAT44(uStack_238,uStack_23c);
        fStack_1a8 = (float)(undefined4)uStack_258;
        fStack_1a4 = (float)uStack_258._4_4_;
        fStack_1a0 = (float)(undefined4)uStack_250;
        fStack_19c = (float)uStack_250._4_4_;
        fStack_1b8 = (float)(uint)uStack_268;
        fStack_1b4 = (float)uStack_268._4_4_;
        fStack_1b0 = (float)(undefined4)uStack_260;
        fStack_1ac = (float)uStack_260._4_4_;
        fStack_1c8 = (float)uStack_278;
        fStack_1c4 = (float)uStack_278._4_4_;
        fStack_1c0 = (float)(undefined4)uStack_270;
        fStack_1bc = (float)uStack_270._4_4_;
        uStack_16c = uStack_2bc;
        uStack_278 = uVar13;
        uStack_270 = uVar14;
        uStack_268 = uVar15;
        uStack_260 = uVar16;
        uStack_258 = uVar17;
        uStack_250 = uVar18;
        uStack_248 = uVar19;
        uStack_178 = uVar22;
        uStack_174 = uVar11;
        cVar21 = FUN_14128cdd0(uStack_1f8,&fStack_1c8);
        if (cVar21 != '\0') {
          *puStack_1f0 = 1;
        }
        uVar24 = uStack_140;
        pfVar25 = pfStack_138;
        fVar30 = _UNK_142a80a4c;
        fVar37 = _UNK_142925904;
        param_7 = fStack_100;
        fVar32 = fStack_fc;
        if (uStack_140 != 2) {
          pfVar25 = pfStack_138 + uStack_140;
          pfVar26 = pfStack_138 + 1;
          pfVar29 = pfStack_138 + 2;
          do {
            fVar30 = *pfVar26;
            fVar37 = *pfVar29;
            fVar32 = fVar37 * _UNK_142925870 + fVar30 * _UNK_142925870;
            fStack_1c8 = fStack_158 - fVar32 * param_10;
            fStack_1c4 = fVar32 * param_9 + fStack_154;
            func_0x000140c9cde0(&uStack_164,&fStack_1c8,fStack_108);
            FUN_140c90fd0(&fStack_318,param_9,param_10);
            fStack_11c = (float)((uint)(fVar37 - fVar30) & _UNK_14292e980);
            uStack_128 = uStack_164;
            uVar11 = uStack_128;
            uStack_120 = uStack_15c;
            lStack_f8 = CONCAT44(lStack_f8._4_4_,fVar30);
            fVar36 = fStack_310 + fStack_310;
            fVar35 = fStack_314 + fStack_314;
            fVar32 = fStack_318 * (fStack_318 + fStack_318);
            fVar33 = (fStack_318 + fStack_318) * fStack_30c;
            fStack_1bc = fVar36 * fStack_318 - fVar35 * fStack_30c;
            fStack_1ac = fVar33 + fVar36 * fStack_314;
            fStack_118 = (float)auStack_288._0_4_;
            fStack_114 = fStack_104;
            fStack_19c = _UNK_142925904 - (fVar32 + fStack_314 * fVar35);
            fStack_1c8 = fStack_11c * (_UNK_142925904 - (fStack_314 * fVar35 + fStack_310 * fVar36))
            ;
            fStack_1c4 = fStack_11c * (fVar35 * fStack_318 + fStack_30c * fVar36);
            fStack_1c0 = fStack_11c * fStack_1bc;
            fStack_1bc = fStack_11c * fStack_1bc;
            fStack_1b8 = (float)auStack_288._0_4_ * (fVar35 * fStack_318 - fStack_30c * fVar36);
            fStack_1b4 = (float)auStack_288._0_4_ *
                         (_UNK_142925904 - (fStack_310 * fVar36 + fVar32));
            fStack_1b0 = (float)auStack_288._0_4_ * fStack_1ac;
            fStack_1ac = (float)auStack_288._0_4_ * fStack_1ac;
            fStack_1a8 = fStack_104 * (fVar35 * fStack_30c + fVar36 * fStack_318);
            fStack_1a4 = fStack_104 * (fVar36 * fStack_314 - fVar33);
            fStack_1a0 = fStack_104 * fStack_19c;
            fStack_19c = fStack_104 * fStack_19c;
            uStack_128._0_4_ = (undefined4)uStack_164;
            uStack_128._4_4_ = (undefined4)((ulonglong)uStack_164 >> 0x20);
            uStack_198 = (undefined4)uStack_128;
            uStack_194 = uStack_128._4_4_;
            uStack_190 = uStack_15c;
            _uStack_18c = CONCAT44(uStack_188,uStack_15c);
            uStack_128 = uVar11;
            func_0x00014089d560(&uStack_2b8,&fStack_1c8);
            uVar22 = func_0x000140858790(param_12);
            iVar23 = func_0x0001400885d0(&param_12);
            func_0x0001408587c0(&fStack_318);
            iStack_17c = iVar23 + (int)lStack_200;
            uStack_258 = uStack_298;
            uVar16 = uStack_258;
            uStack_250 = uStack_290;
            uVar17 = uStack_250;
            uStack_268 = uStack_2a8;
            uVar14 = uStack_268;
            uStack_260 = uStack_2a0;
            uVar15 = uStack_260;
            uStack_278 = uStack_2b8;
            uVar11 = uStack_278;
            uStack_270 = uStack_2b0;
            uVar13 = uStack_270;
            uStack_248 = uStack_2e8;
            uVar18 = uStack_248;
            uStack_240 = (undefined4)uStack_2e0;
            uStack_23c = (undefined4)((ulonglong)uStack_2e0 >> 0x20);
            uStack_230 = uStack_2d0;
            uStack_238 = (undefined4)uStack_2d8;
            uStack_234 = (undefined4)((ulonglong)uStack_2d8 >> 0x20);
            uStack_184 = CONCAT44(uStack_2d0,uStack_234);
            uStack_278._0_4_ = (float)uStack_2b8;
            uStack_278._4_4_ = (undefined4)((ulonglong)uStack_2b8 >> 0x20);
            uStack_270._0_4_ = (undefined4)uStack_2b0;
            uStack_270._4_4_ = (undefined4)((ulonglong)uStack_2b0 >> 0x20);
            uStack_268._0_4_ = (uint)uStack_2a8;
            uStack_268._4_4_ = (undefined4)((ulonglong)uStack_2a8 >> 0x20);
            uStack_260._0_4_ = (undefined4)uStack_2a0;
            uStack_260._4_4_ = (undefined4)((ulonglong)uStack_2a0 >> 0x20);
            uStack_258._0_4_ = (undefined4)uStack_298;
            uStack_258._4_4_ = (undefined4)((ulonglong)uStack_298 >> 0x20);
            uStack_250._0_4_ = (undefined4)uStack_290;
            uStack_250._4_4_ = (undefined4)((ulonglong)uStack_290 >> 0x20);
            uStack_248._0_4_ = (undefined4)uStack_2e8;
            uStack_248._4_4_ = (undefined4)((ulonglong)uStack_2e8 >> 0x20);
            uStack_198 = (undefined4)uStack_248;
            uStack_194 = uStack_248._4_4_;
            uStack_190 = uStack_240;
            _uStack_18c = CONCAT44(uStack_238,uStack_23c);
            fStack_1a8 = (float)(undefined4)uStack_258;
            fStack_1a4 = (float)uStack_258._4_4_;
            fStack_1a0 = (float)(undefined4)uStack_250;
            fStack_19c = (float)uStack_250._4_4_;
            fStack_1b8 = (float)(uint)uStack_268;
            fStack_1b4 = (float)uStack_268._4_4_;
            fStack_1b0 = (float)(undefined4)uStack_260;
            fStack_1ac = (float)uStack_260._4_4_;
            fStack_1c8 = (float)uStack_278;
            fStack_1c4 = (float)uStack_278._4_4_;
            fStack_1c0 = (float)(undefined4)uStack_270;
            fStack_1bc = (float)uStack_270._4_4_;
            uStack_174 = CONCAT44(fVar37,fVar30);
            uStack_16c = uStack_2bc;
            uStack_278 = uVar11;
            uStack_270 = uVar13;
            uStack_268 = uVar14;
            uStack_260 = uVar15;
            uStack_258 = uVar16;
            uStack_250 = uVar17;
            uStack_248 = uVar18;
            uStack_178 = uVar22;
            cVar21 = FUN_14128cdd0(uStack_1f8,&fStack_1c8);
            if (cVar21 != '\0') {
              *puStack_1f0 = 1;
            }
            pfVar2 = pfVar29 + 1;
            pfVar26 = pfVar29;
            pfVar29 = pfVar2;
          } while (pfVar2 != pfVar25);
          uVar24 = uStack_140;
          pfVar25 = pfStack_138;
          fVar30 = _UNK_142a80a4c;
          fVar37 = _UNK_142925904;
          param_7 = fStack_100;
          fVar32 = fStack_fc;
          if (uStack_140 == 0) goto code_r0x00014200cd95;
        }
      }
      func_0x000140613c20(pfVar25,uVar24 << 2,4);
    }
  } while( true );
  bVar20 = false;
  uVar28 = uStack_218;
  goto code_r0x00014200cc79;
}

