
undefined8 *
FUN_140a6fec0(undefined8 *param_1,undefined8 param_2,longlong param_3,undefined **param_4,
             undefined **param_5,undefined **param_6,longlong param_7)

{
  code *pcVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 *puVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  undefined4 uStack_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  undefined4 uStack_408;
  int iStack_3f8;
  undefined4 uStack_3f4;
  undefined4 uStack_3f0;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined4 uStack_3d0;
  undefined4 uStack_3cc;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
  undefined4 uStack_3b0;
  undefined4 uStack_3ac;
  undefined4 uStack_3a8;
  undefined4 uStack_3a4;
  undefined8 uStack_3a0;
  undefined4 uStack_398;
  undefined4 uStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined4 uStack_370;
  undefined4 uStack_36c;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  longlong lStack_320;
  undefined8 uStack_318;
  undefined1 auStack_2e8 [38];
  char cStack_2c2;
  longlong lStack_2c0;
  undefined *puStack_268;
  undefined8 uStack_260;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined8 uStack_210;
  undefined8 *puStack_208;
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
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined1 auStack_158 [16];
  undefined1 auStack_148 [22];
  char cStack_132;
  longlong lStack_130;
  undefined8 uStack_e8;
  undefined *puStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined8 uStack_b0;
  undefined **ppuStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  byte bStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar7 = 0x8226cc786e679355;
  lVar4 = FUN_1400dd6f0(param_3 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
  if (((lVar4 == 0) || (*(ulonglong *)(param_3 + 0x218) <= uVar7)) ||
     (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar7 * 8), uVar7 == 0)) {
code_r0x000140a705f4:
    FUN_1428d9310(&UNK_142ad5840);
  }
  else {
    lVar4 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar4) == 0)
    goto code_r0x000140a705f4;
    puVar5 = (undefined8 *)
             FUN_140969720(*(undefined8 *)(*(longlong *)(param_3 + 0x1e0) + lVar4 + 0x10),
                           &UNK_142c2e230);
    if (puVar5 != (undefined8 *)0x0) {
      puStack_268 = &UNK_142a9d2fe;
      uStack_260 = 0x35;
      if ((puVar5[3] != 0x35) ||
         (iVar2 = func_0x000142923bf0(&UNK_142a9d2fe,puVar5[2],0x35), iVar2 != 0)) {
        iStack_3f8 = 0;
        uStack_3f4 = 0;
        FUN_1428cab41(0,&puStack_268,puVar5 + 2,&iStack_3f8,&UNK_142c2e180);
        goto code_r0x000140a705f4;
      }
      puVar6 = (ulonglong *)(**(code **)(puVar5[1] + 0x18))(*puVar5);
      if (puVar6 != (ulonglong *)0x0) {
        uVar7 = *puVar6;
        if ((int)uVar7 == 0) {
          func_0x000140bd52a0(&ppuStack_a8,0x100000001,0x65);
          uStack_258 = (undefined4)uStack_98;
          uStack_254 = uStack_98._4_4_;
          uStack_250 = (undefined4)uStack_90;
          uStack_24c = uStack_90._4_4_;
          func_0x000140bd6ce0(&uStack_3e8,&puStack_268);
          iStack_3f8 = 0x42ad5870;
          uStack_3f4 = 1;
          uStack_3f0 = 0x1a;
          uStack_3ec = 0;
          uVar3 = FUN_140be94d0(param_2,&iStack_3f8);
          *(undefined8 *)((longlong)param_1 + 0x14) = uStack_98;
          *(undefined8 *)((longlong)param_1 + 0x1c) = uStack_90;
          *(undefined ***)((longlong)param_1 + 4) = ppuStack_a8;
          *(undefined8 *)((longlong)param_1 + 0xc) = uStack_a0;
          *(undefined4 *)param_1 = uVar3;
          return param_1;
        }
        uStack_b0 = func_0x000140bd5790((longlong)param_4 + 4);
        func_0x000140bd52a0(&iStack_3f8,uStack_b0,0x65);
        func_0x000140bd5370(&puStack_268,&iStack_3f8);
        func_0x000140bd5390(&ppuStack_a8,&puStack_268);
        FUN_14096d490(&iStack_3f8,param_2,&UNK_142ad588a,0xe,&ppuStack_a8);
        if (iStack_3f8 != 1) {
          puStack_268 = (undefined *)CONCAT44(uStack_3f0,uStack_3f4);
          uStack_260 = CONCAT44(uStack_3e8,uStack_3ec);
          uStack_238 = uStack_3c8._4_4_;
          uStack_234 = (undefined4)uStack_3c0;
          uStack_230 = uStack_3c0._4_4_;
          uStack_22c = uStack_3b8;
          uStack_248 = uStack_3d8._4_4_;
          uStack_244 = uStack_3d0;
          uStack_240 = uStack_3cc;
          uStack_23c = (undefined4)uStack_3c8;
          uStack_258 = uStack_3e4;
          uStack_254 = (undefined4)uStack_3e0;
          uStack_250 = uStack_3e0._4_4_;
          uStack_24c = (undefined4)uStack_3d8;
          bStack_49 = (byte)uStack_3ac;
          uStack_b8 = uStack_3d8._4_4_;
          uStack_c8 = uStack_3e4;
          uStack_c4 = (undefined4)uStack_3e0;
          uStack_c0 = uStack_3e0._4_4_;
          uStack_bc = (undefined4)uStack_3d8;
          uStack_428 = uStack_3d0;
          uStack_424 = uStack_3cc;
          uStack_420 = (undefined4)uStack_3c8;
          uStack_41c = uStack_3c8._4_4_;
          uStack_408 = uStack_3b0;
          puStack_d8 = puStack_268;
          uStack_d0 = uStack_260;
          FUN_1416032a0(&iStack_3f8,&UNK_142ad58b0,0xc,&UNK_142ad58bc,0x15,param_3,param_2);
          ppuStack_a8 = (undefined **)0x579f39ab91a01865;
          puStack_268._0_4_ = 2;
          FUN_1409056d0(&ppuStack_a8,&iStack_3f8,0x815bf5036966978c,0x19,&puStack_268);
          func_0x000142923c00(&puStack_268,&iStack_3f8,400);
          ppuStack_a8 = param_6;
          iStack_3f8 = 2;
          FUN_1408ee240(&ppuStack_a8,&puStack_268,0xb8a74ccd835a6be3,0x19,&iStack_3f8);
          func_0x000142923c00(&iStack_3f8,&puStack_268,400);
          puStack_268._0_4_ = 2;
          ppuStack_a8 = param_4;
          FUN_1408ee240(&ppuStack_a8,&iStack_3f8,0x2a333c3a28028230,0x19,&puStack_268);
          func_0x000142923c00(&puStack_268,&iStack_3f8,400);
          ppuStack_a8 = param_5;
          iStack_3f8 = 2;
          FUN_1408ee240(&ppuStack_a8,&puStack_268,0x350f8dc714051d10,0x19,&iStack_3f8);
          func_0x000142923c00(&iStack_3f8,&puStack_268,400);
          ppuStack_a8 = (undefined **)&uStack_428;
          puStack_268._0_4_ = 2;
          FUN_14096f090(&ppuStack_a8,&iStack_3f8,0x4da9887cf3dc26f4,0x19,&puStack_268);
          func_0x000142923c00(&puStack_268,&iStack_3f8,400);
          FUN_14160d680(&iStack_3f8,&puStack_268,0xc6b33eb20992aad);
          FUN_14096dac0(&puStack_268,&iStack_3f8,0x2bd0ac289087c8e1,param_7);
          FUN_14096dac0(&iStack_3f8,&puStack_268,0x129f01ce8d5406a0,param_7 + 0x18);
          ppuStack_a8 = &puStack_d8;
          puStack_268 = (undefined *)CONCAT44(puStack_268._4_4_,2);
          FUN_1408edd00(&ppuStack_a8,&iStack_3f8,0x8daeeefe7463dee0,0x32,&puStack_268);
          func_0x000142923c00(&puStack_268,&iStack_3f8,400);
          if (cStack_132 != '\b') {
            FUN_14096b570(auStack_158,lStack_130 + 0xe8,0x9b88329e408ff69f,uVar7 & 0xffffffff);
          }
          func_0x000142923c00(&iStack_3f8,&puStack_268,400);
          if (cStack_2c2 != '\b') {
            FUN_14096b850(auStack_2e8,lStack_2c0 + 0xe8,0xa9a109b760eed902,bStack_49 & 1);
          }
          func_0x000142923c00(&puStack_268,&iStack_3f8,400);
          uStack_68 = uStack_1c8;
          uStack_60 = uStack_1c0;
          uStack_78 = uStack_1d8;
          uStack_70 = uStack_1d0;
          uStack_88 = uStack_1e8;
          uStack_80 = uStack_1e0;
          uStack_98 = uStack_1f8;
          uStack_90 = uStack_1f0;
          ppuStack_a8 = (undefined **)puStack_208;
          uStack_a0 = uStack_200;
          uStack_3e8 = uStack_258;
          uStack_3e4 = uStack_254;
          iStack_3f8 = (int)puStack_268;
          uStack_3f4 = (undefined4)((ulonglong)puStack_268 >> 0x20);
          uStack_3f0 = (undefined4)uStack_260;
          uStack_3ec = (undefined4)((ulonglong)uStack_260 >> 0x20);
          uStack_3c8 = CONCAT44(uStack_24c,uStack_250);
          uStack_3c0 = CONCAT44(uStack_244,uStack_248);
          uStack_3b8 = uStack_240;
          uStack_3b4 = uStack_23c;
          uStack_3e0 = CONCAT44(uStack_234,uStack_238);
          uStack_3d8 = CONCAT44(uStack_22c,uStack_230);
          uStack_380 = uStack_1b8;
          uStack_378 = uStack_1b0;
          uStack_370 = uStack_1a8;
          uStack_36c = uStack_1a4;
          uStack_368 = uStack_1a0;
          uStack_364 = uStack_19c;
          uStack_360 = uStack_198;
          uStack_35c = uStack_194;
          uStack_358 = uStack_190;
          uStack_354 = uStack_18c;
          uStack_350 = uStack_188;
          uStack_34c = uStack_184;
          uStack_348 = uStack_180;
          uStack_344 = uStack_17c;
          uStack_340 = uStack_178;
          uStack_338 = uStack_170;
          uStack_330 = uStack_168;
          uStack_328 = uStack_160;
          uStack_3a0 = uStack_210;
          uStack_3b0 = uStack_220;
          uStack_3ac = uStack_21c;
          uStack_3a8 = uStack_218;
          uStack_3a4 = uStack_214;
          uStack_318 = uStack_e8;
          lStack_320 = lStack_130;
          uStack_398 = 1;
          uStack_394 = (undefined4)uStack_b0;
          uStack_390 = (undefined4)((ulonglong)uStack_b0 >> 0x20);
          uStack_38c = 1;
          FUN_1409dd4d0(&ppuStack_a8,&iStack_3f8);
          if (cStack_132 != '\b') {
            FUN_140be24b0(auStack_148);
          }
          *(undefined4 *)(param_1 + 4) = uStack_b8;
          param_1[2] = CONCAT44(uStack_c4,uStack_c8);
          param_1[3] = CONCAT44(uStack_bc,uStack_c0);
          *param_1 = puStack_d8;
          param_1[1] = uStack_d0;
          return param_1;
        }
        goto code_r0x000140a7060c;
      }
    }
  }
  FUN_1428d9310(&UNK_142ad5858);
code_r0x000140a7060c:
  FUN_1428d9760(&UNK_142ad57c8,0x2b,&puStack_268,&UNK_142ad57a8,&UNK_142ad5898);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

