
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a01620(undefined8 param_1,longlong param_2,undefined8 param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  uint *puVar4;
  code *pcVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  float fVar9;
  float fVar10;
  undefined8 *puVar11;
  undefined8 uVar12;
  undefined4 uVar13;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined4 uStack_388;
  undefined4 uStack_384;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  longlong lStack_310;
  undefined8 uStack_308;
  undefined1 auStack_2d8 [38];
  char cStack_2b2;
  longlong lStack_2b0;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined1 auStack_148 [16];
  undefined1 auStack_138 [22];
  char cStack_122;
  longlong lStack_120;
  undefined8 uStack_d8;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lVar7 = param_2 + 0x38;
  uVar8 = 0x4cab10763b3fdb80;
  lVar6 = FUN_1400dd6f0(lVar7,0x4cab10763b3fdb80,0x1a422ef7534b54d);
  if (((lVar6 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar8)) ||
     (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 == 0)) {
code_r0x000140a01aa0:
    FUN_1428d9310(&UNK_142ac5aa0);
  }
  else {
    lVar6 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar6) == 0)
    goto code_r0x000140a01aa0;
    puVar3 = *(undefined4 **)(*(longlong *)(param_2 + 0x1e0) + lVar6 + 0x10);
    uVar1 = *puVar3;
    uVar2 = puVar3[1];
    uVar8 = 0x3d7b56e787d47c40;
    lVar6 = FUN_1400dd6f0(lVar7,0x3d7b56e787d47c40,0x67b0b10a8d43bfc7);
    if (((lVar6 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
      lVar6 = ~uVar8 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar6) != 0) {
        fVar10 = *(float *)(*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar6 + 0x10) + 0x110);
        puVar11 = &uStack_3e8;
        FUN_1416032a0(puVar11,&UNK_142ac5ad0,0x19,&UNK_142ac5ae9,0x1f,param_2,param_1);
        FUN_141609770(&uStack_258,puVar11,0xa868b69644bd36ba);
        uStack_3e8 = CONCAT44(uStack_3e8._4_4_,2);
        uStack_c8 = param_3;
        FUN_1408ee240(&uStack_c8,&uStack_258,0x6fce509d259daacf,0x19,puVar11);
        uVar13 = (undefined4)((ulonglong)puVar11 >> 0x20);
        func_0x000142923c00(&uStack_3e8,&uStack_258,400);
        if (cStack_2b2 != '\b') {
          uVar12 = CONCAT44(uVar13,uVar2);
          FUN_14096b940(auStack_2d8,lStack_2b0 + 0xe8,0x3b8fce716c29ac83,uVar1,uVar12);
          uVar13 = (undefined4)((ulonglong)uVar12 >> 0x20);
        }
        func_0x000142923c00(&uStack_258,&uStack_3e8,400);
        uStack_71 = 1;
        uVar8 = 0x864ddca98c2312a6;
        lVar7 = FUN_1400dd6f0(lVar7,0x864ddca98c2312a6,0x27b0e9369e9a484e);
        if (((lVar7 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
          lVar7 = ~uVar8 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
            if (cStack_122 != '\b') {
              puVar4 = *(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
              FUN_14096b940(auStack_148,lStack_120 + 0xe8,0x69fdab9b991c4787,(float)*puVar4,
                            CONCAT44(uVar13,(float)puVar4[1]));
            }
            func_0x000142923c00(&uStack_3e8,&uStack_258,400);
            if (cStack_2b2 != '\b') {
              fVar9 = 0.0;
              if (0.0 <= fVar10) {
                fVar9 = fVar10;
              }
              fVar10 = _UNK_142925904;
              if (fVar9 <= _UNK_142925904) {
                fVar10 = fVar9;
              }
              FUN_14096ba30(auStack_2d8,lStack_2b0 + 0xe8,0xc10bfe30e41ed046,fVar10);
            }
            func_0x000142923c00(&uStack_258,&uStack_3e8,400);
            uStack_88 = uStack_1b8;
            uStack_80 = uStack_1b0;
            uStack_98 = uStack_1c8;
            uStack_90 = uStack_1c0;
            uStack_a8 = uStack_1d8;
            uStack_a0 = uStack_1d0;
            uStack_b8 = uStack_1e8;
            uStack_b0 = uStack_1e0;
            uStack_c8 = uStack_1f8;
            uStack_c0 = uStack_1f0;
            uStack_3d8 = uStack_248;
            uStack_3e8 = uStack_258;
            uStack_3e0 = uStack_250;
            uStack_3b8 = uStack_240;
            uStack_3b0 = uStack_238;
            uStack_3a8 = uStack_230;
            uStack_3d0 = uStack_228;
            uStack_3c8 = uStack_220;
            uStack_3c0 = uStack_218;
            uStack_370 = uStack_1a8;
            uStack_368 = uStack_1a0;
            uStack_360 = uStack_198;
            uStack_35c = uStack_194;
            uStack_358 = uStack_190;
            uStack_354 = uStack_18c;
            uStack_350 = uStack_188;
            uStack_34c = uStack_184;
            uStack_348 = uStack_180;
            uStack_344 = uStack_17c;
            uStack_340 = uStack_178;
            uStack_33c = uStack_174;
            uStack_338 = uStack_170;
            uStack_334 = uStack_16c;
            uStack_330 = uStack_168;
            uStack_328 = uStack_160;
            uStack_320 = uStack_158;
            uStack_318 = uStack_150;
            uStack_390 = uStack_200;
            uStack_3a0 = uStack_210;
            uStack_398 = uStack_208;
            uStack_308 = uStack_d8;
            lStack_310 = lStack_120;
            uStack_388 = _UNK_142958fe0;
            uStack_384 = _UNK_142958fe4;
            uStack_380 = _UNK_142958fe8;
            uStack_37c = _UNK_142958fec;
            FUN_1409dd4d0(&uStack_c8,&uStack_3e8);
            if (cStack_122 != '\b') {
              FUN_140be24b0(auStack_138);
            }
            return;
          }
        }
        goto code_r0x000140a01ab8;
      }
    }
  }
  FUN_1428d9310(&UNK_142ac5ab8);
code_r0x000140a01ab8:
  uStack_71 = 1;
  FUN_1428d9310(&UNK_142ac5b08);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

