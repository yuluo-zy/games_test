
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1409e7770(undefined8 *param_1,undefined8 param_2,longlong param_3)

{
  code *pcVar1;
  int iVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 *puVar5;
  ulonglong *puVar6;
  undefined4 *puVar7;
  ulonglong uVar8;
  undefined4 uStack_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined4 uStack_408;
  undefined4 uStack_404;
  undefined4 uStack_3f8;
  undefined4 uStack_3f4;
  undefined4 uStack_3f0;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined *puStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined4 uStack_378;
  int iStack_374;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined4 uStack_328;
  undefined4 uStack_324;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined1 auStack_2c8 [16];
  undefined1 auStack_2b8 [22];
  char cStack_2a2;
  longlong lStack_2a0;
  undefined8 uStack_258;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined *puStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined4 uStack_1c8;
  int iStack_1c4;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
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
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined1 auStack_118 [16];
  undefined1 auStack_108 [22];
  char cStack_f2;
  longlong lStack_f0;
  undefined8 uStack_a8;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar8 = 0x8226cc786e679355;
  lVar4 = FUN_1400dd6f0(param_3 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
  if (((lVar4 == 0) || (*(ulonglong *)(param_3 + 0x218) <= uVar8)) ||
     (uVar8 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar8 * 8), uVar8 == 0)) {
code_r0x0001409e842d:
    FUN_1428d9310(&UNK_142ac4930);
  }
  else {
    lVar4 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar4) == 0)
    goto code_r0x0001409e842d;
    puVar5 = (undefined8 *)
             FUN_140969720(*(undefined8 *)(*(longlong *)(param_3 + 0x1e0) + lVar4 + 0x10),
                           &UNK_142c2e230);
    if (puVar5 != (undefined8 *)0x0) {
      puStack_3d8 = &UNK_142a9d2fe;
      uStack_3d0 = 0x35;
      if ((puVar5[3] != 0x35) ||
         (iVar2 = func_0x000142923bf0(&UNK_142a9d2fe,puVar5[2],0x35), iVar2 != 0)) {
        puStack_228 = (undefined *)0x0;
        FUN_1428cab41(0,&puStack_3d8,puVar5 + 2,&puStack_228,&UNK_142c2e180);
        goto code_r0x0001409e842d;
      }
      puVar6 = (ulonglong *)(**(code **)(puVar5[1] + 0x18))(*puVar5);
      if (puVar6 != (ulonglong *)0x0) {
        uVar8 = *puVar6;
        iVar2 = (int)uVar8;
        if (iVar2 == 0) {
          FUN_140bd4e20(&uStack_98,4,0x20);
          func_0x000140bd6d00(&uStack_218,&puStack_3d8);
          puStack_228 = &UNK_142ac4960;
          uStack_220 = 0x22;
          uVar3 = FUN_140be94d0(param_2,&puStack_228);
          *(undefined4 *)param_1 = (undefined4)uStack_98;
          *(undefined4 *)((longlong)param_1 + 4) = uStack_98._4_4_;
          *(undefined4 *)(param_1 + 1) = (undefined4)uStack_90;
          *(undefined4 *)((longlong)param_1 + 0xc) = uStack_90._4_4_;
          *(undefined4 *)(param_1 + 2) = uVar3;
          FUN_140bd4e20(&uStack_98,4,0x20);
          func_0x000140bd6d00(&uStack_218,&puStack_3d8);
          puStack_228 = &UNK_142ac4982;
          uStack_220 = 0x19;
          uVar3 = FUN_140be94d0(param_2,&puStack_228);
          param_1[3] = uStack_98;
          param_1[4] = uStack_90;
          *(undefined4 *)(param_1 + 5) = uVar3;
          return param_1;
        }
        FUN_140bd4e20(&uStack_98,0x10000,0x20);
        puStack_3d8 = uStack_98;
        uStack_3d0 = uStack_90;
        func_0x000140bd6d00(&uStack_218,&puStack_3d8);
        puStack_228 = &UNK_142ac499b;
        uStack_220 = 0x16;
        uStack_238 = FUN_140be94d0(param_2,&puStack_228);
        uStack_248 = (undefined4)uStack_98;
        uStack_244 = uStack_98._4_4_;
        uStack_240 = (undefined4)uStack_90;
        uStack_23c = uStack_90._4_4_;
        FUN_1409ddbb0(param_2,param_3,&uStack_248);
        FUN_1415fe130(&puStack_228,&UNK_142ac49b1,0x11,0,param_3,param_2);
        FUN_14160d680(&puStack_3d8,&puStack_228,0xc6b33eb20992aad);
        FUN_14096dd60(&puStack_228,&puStack_3d8,0x63e587f19a869d94,&uStack_248);
        if (cStack_f2 != '\b') {
          FUN_14096b570(auStack_118,lStack_f0 + 0xe8,0x9b88329e408ff69f,uVar8 & 0xffffffff);
        }
        func_0x000142923c00(&puStack_3d8,&puStack_228,400);
        uStack_78 = uStack_358;
        uStack_88 = uStack_368;
        uStack_80 = uStack_360;
        uStack_98 = (undefined *)CONCAT44(iStack_374,uStack_378);
        uStack_90 = uStack_370;
        uStack_218 = uStack_3c8;
        puStack_228 = puStack_3d8;
        uStack_220 = uStack_3d0;
        uStack_1f8 = uStack_3c0;
        uStack_1f0 = uStack_3b8;
        uStack_1e8 = uStack_3b0;
        uStack_210 = uStack_3a8;
        uStack_208 = uStack_3a0;
        uStack_200 = uStack_398;
        uStack_1b0 = CONCAT44(uStack_324,uStack_328);
        uStack_1a8 = uStack_320;
        uStack_1a0 = (undefined4)uStack_318;
        uStack_19c = uStack_318._4_4_;
        uStack_198 = (undefined4)uStack_310;
        uStack_194 = uStack_310._4_4_;
        uStack_190 = (undefined4)uStack_308;
        uStack_18c = uStack_308._4_4_;
        uStack_188 = (undefined4)uStack_300;
        uStack_184 = uStack_300._4_4_;
        uStack_180 = (undefined4)uStack_2f8;
        uStack_17c = uStack_2f8._4_4_;
        uStack_178 = uStack_2f0;
        uStack_174 = uStack_2ec;
        uStack_170 = uStack_2e8;
        uStack_168 = uStack_2e0;
        uStack_160 = uStack_2d8;
        uStack_158 = uStack_2d0;
        uStack_1d0 = uStack_380;
        uStack_1e0 = (undefined4)uStack_390;
        uStack_1dc = uStack_390._4_4_;
        uStack_1d8 = (undefined4)uStack_388;
        uStack_1d4 = uStack_388._4_4_;
        uStack_148 = uStack_258;
        uStack_150 = lStack_2a0;
        uStack_1c8 = 1;
        uStack_1c0 = 0x100000001;
        iStack_1c4 = iVar2;
        FUN_1409dd4d0(&uStack_98,&puStack_228);
        if (cStack_2a2 != '\b') {
          FUN_140be24b0(auStack_2b8);
        }
        FUN_140bd4e20(&uStack_98,0x44,0x20);
        uStack_3d0 = uStack_90;
        func_0x000140bd6d00(&uStack_218,&puStack_3d8);
        puStack_228 = &UNK_142ac49c2;
        uStack_220 = 0x1b;
        uStack_3e8 = FUN_140be94d0(param_2,&puStack_228);
        uStack_3f8 = (undefined4)uStack_98;
        uStack_3f4 = uStack_98._4_4_;
        uStack_3f0 = (undefined4)uStack_90;
        uStack_3ec = uStack_90._4_4_;
        FUN_1416032a0(&puStack_228,&UNK_142ac49dd,0x12,&UNK_142ac49ef,0x26,param_3,param_2);
        FUN_14096dd60(&puStack_3d8,&puStack_228,0xaab861d318ad7248,&uStack_248);
        FUN_14096dd60(&puStack_228,&puStack_3d8,0xc3674440971f6990,&uStack_3f8);
        uStack_58 = CONCAT44(uStack_184,uStack_188);
        uStack_50 = CONCAT44(uStack_17c,uStack_180);
        uStack_68 = CONCAT44(uStack_194,uStack_198);
        uStack_60 = CONCAT44(uStack_18c,uStack_190);
        uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
        uStack_78 = uStack_1a8;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = (undefined *)CONCAT44(iStack_1c4,uStack_1c8);
        uStack_90 = uStack_1c0;
        uStack_3c8 = uStack_218;
        puStack_3d8 = puStack_228;
        uStack_3d0 = uStack_220;
        uStack_3a8 = uStack_210;
        uStack_3a0 = uStack_208;
        uStack_398 = uStack_200;
        uStack_3c0 = uStack_1f8;
        uStack_3b8 = uStack_1f0;
        uStack_3b0 = uStack_1e8;
        uStack_360 = CONCAT44(uStack_174,uStack_178);
        uStack_358 = uStack_170;
        uStack_350 = (undefined4)uStack_168;
        uStack_34c = uStack_168._4_4_;
        uStack_348 = (undefined4)uStack_160;
        uStack_344 = uStack_160._4_4_;
        uStack_340 = (undefined4)uStack_158;
        uStack_33c = uStack_158._4_4_;
        uStack_338 = (undefined4)uStack_150;
        uStack_334 = uStack_150._4_4_;
        uStack_330 = (undefined4)uStack_148;
        uStack_32c = uStack_148._4_4_;
        uStack_328 = uStack_140;
        uStack_324 = uStack_13c;
        uStack_320 = uStack_138;
        uStack_318 = uStack_130;
        uStack_310 = uStack_128;
        uStack_308 = uStack_120;
        uStack_380 = uStack_1d0;
        uStack_390 = CONCAT44(uStack_1dc,uStack_1e0);
        uStack_388 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
        uStack_2f8 = uStack_a8;
        uStack_300 = lStack_f0;
        uStack_378 = _UNK_142ab8420;
        iStack_374 = _UNK_142ab8424;
        uStack_370 = CONCAT44(_UNK_142ab842c,_UNK_142ab8428);
        FUN_1409dd4d0(&uStack_98,&puStack_3d8);
        if (cStack_f2 != '\b') {
          FUN_140be24b0(auStack_108);
        }
        FUN_1416032a0(&puStack_228,&UNK_142ac4a15,0x1b,&UNK_142ac4a30,0x2f,param_3,param_2);
        FUN_14096dd60(&puStack_3d8,&puStack_228,0xaab861d318ad7248,&uStack_3f8);
        puVar7 = (undefined4 *)func_0x000140613c10(4,1);
        if (puVar7 != (undefined4 *)0x0) {
          *puVar7 = 0x10;
          if (uStack_390 != 0) {
            func_0x000140613c20(uStack_388,uStack_390,1);
          }
          uStack_390 = 4;
          uStack_380 = 4;
          uStack_388 = puVar7;
          func_0x000142923c00(&puStack_228,&puStack_3d8,400);
          uStack_58 = CONCAT44(uStack_184,uStack_188);
          uStack_50 = CONCAT44(uStack_17c,uStack_180);
          uStack_68 = CONCAT44(uStack_194,uStack_198);
          uStack_60 = CONCAT44(uStack_18c,uStack_190);
          uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
          uStack_78 = uStack_1a8;
          uStack_88 = uStack_1b8;
          uStack_80 = uStack_1b0;
          uStack_98 = (undefined *)CONCAT44(iStack_1c4,uStack_1c8);
          uStack_90 = uStack_1c0;
          uStack_3c8 = uStack_218;
          puStack_3d8 = puStack_228;
          uStack_3d0 = uStack_220;
          uStack_3a8 = uStack_210;
          uStack_3a0 = uStack_208;
          uStack_398 = uStack_200;
          uStack_3c0 = uStack_1f8;
          uStack_3b8 = uStack_1f0;
          uStack_3b0 = uStack_1e8;
          uStack_360 = CONCAT44(uStack_174,uStack_178);
          uStack_358 = uStack_170;
          uStack_350 = (undefined4)uStack_168;
          uStack_34c = uStack_168._4_4_;
          uStack_348 = (undefined4)uStack_160;
          uStack_344 = uStack_160._4_4_;
          uStack_340 = (undefined4)uStack_158;
          uStack_33c = uStack_158._4_4_;
          uStack_338 = (undefined4)uStack_150;
          uStack_334 = uStack_150._4_4_;
          uStack_330 = (undefined4)uStack_148;
          uStack_32c = uStack_148._4_4_;
          uStack_328 = uStack_140;
          uStack_324 = uStack_13c;
          uStack_320 = uStack_138;
          uStack_318 = uStack_130;
          uStack_310 = uStack_128;
          uStack_308 = uStack_120;
          uStack_380 = uStack_1d0;
          uStack_390 = CONCAT44(uStack_1dc,uStack_1e0);
          uStack_388 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
          uStack_2f8 = uStack_a8;
          uStack_300 = lStack_f0;
          uStack_378 = _UNK_142ac4560;
          iStack_374 = _UNK_142ac4564;
          uStack_370 = CONCAT44(_UNK_142ac456c,_UNK_142ac4568);
          FUN_1409dd4d0(&uStack_98,&puStack_3d8);
          if (cStack_f2 != '\b') {
            FUN_140be24b0(auStack_108);
          }
          FUN_1416032a0(&puStack_228,&UNK_142ac4a5f,0x18,&UNK_142ac4a77,0x28,param_3,param_2);
          FUN_14096dd60(&puStack_3d8,&puStack_228,0xaab861d318ad7248,&uStack_248);
          FUN_14096dac0(&puStack_228,&puStack_3d8,0xc3674440971f6990,&uStack_3f8);
          uStack_58 = CONCAT44(uStack_184,uStack_188);
          uStack_50 = CONCAT44(uStack_17c,uStack_180);
          uStack_68 = CONCAT44(uStack_194,uStack_198);
          uStack_60 = CONCAT44(uStack_18c,uStack_190);
          uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
          uStack_78 = uStack_1a8;
          uStack_88 = uStack_1b8;
          uStack_80 = uStack_1b0;
          uStack_98 = (undefined *)CONCAT44(iStack_1c4,uStack_1c8);
          uStack_90 = uStack_1c0;
          uStack_3c8 = uStack_218;
          puStack_3d8 = puStack_228;
          uStack_3d0 = uStack_220;
          uStack_3a8 = uStack_210;
          uStack_3a0 = uStack_208;
          uStack_398 = uStack_200;
          uStack_3c0 = uStack_1f8;
          uStack_3b8 = uStack_1f0;
          uStack_3b0 = uStack_1e8;
          uStack_360 = CONCAT44(uStack_174,uStack_178);
          uStack_358 = uStack_170;
          uStack_350 = (undefined4)uStack_168;
          uStack_34c = uStack_168._4_4_;
          uStack_348 = (undefined4)uStack_160;
          uStack_344 = uStack_160._4_4_;
          uStack_340 = (undefined4)uStack_158;
          uStack_33c = uStack_158._4_4_;
          uStack_338 = (undefined4)uStack_150;
          uStack_334 = uStack_150._4_4_;
          uStack_330 = (undefined4)uStack_148;
          uStack_32c = uStack_148._4_4_;
          uStack_328 = uStack_140;
          uStack_324 = uStack_13c;
          uStack_320 = uStack_138;
          uStack_318 = uStack_130;
          uStack_310 = uStack_128;
          uStack_308 = uStack_120;
          uStack_380 = uStack_1d0;
          uStack_390 = CONCAT44(uStack_1dc,uStack_1e0);
          uStack_388 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
          uStack_2f8 = uStack_a8;
          uStack_300 = lStack_f0;
          uStack_378 = _UNK_142ac4570;
          iStack_374 = _UNK_142ac4574;
          uStack_370 = CONCAT44(_UNK_142ac457c,_UNK_142ac4578);
          FUN_1409dd4d0(&uStack_98,&puStack_3d8);
          if (cStack_f2 != '\b') {
            FUN_140be24b0(auStack_108);
          }
          FUN_140bd4e20(&uStack_98,0x100000,0x20);
          puStack_3d8 = uStack_98;
          uStack_3d0 = uStack_90;
          func_0x000140bd6d00(&uStack_218,&puStack_3d8);
          puStack_228 = &UNK_142ac4a9f;
          uStack_220 = 0xd;
          uStack_408 = FUN_140be94d0(param_2,&puStack_228);
          uStack_418 = (undefined4)uStack_98;
          uStack_414 = uStack_98._4_4_;
          uStack_410 = (undefined4)uStack_90;
          uStack_40c = uStack_90._4_4_;
          FUN_1415fe130(&puStack_228,&UNK_142ac4aac,0x13,1,param_3,param_2);
          FUN_14160d680(&puStack_3d8,&puStack_228,0xc6b33eb20992aad);
          FUN_14096dd60(&puStack_228,&puStack_3d8,0x63e587f19a869d94,&uStack_248);
          FUN_14096dd60(&puStack_3d8,&puStack_228,0xe3b8c78bd96aa31d,&uStack_418);
          if (cStack_2a2 != '\b') {
            FUN_14096b570(auStack_2c8,lStack_2a0 + 0xe8,0x9b88329e408ff69f,uVar8 & 0xffffffff);
          }
          func_0x000142923c00(&puStack_228,&puStack_3d8,400);
          uStack_58 = CONCAT44(uStack_184,uStack_188);
          uStack_50 = CONCAT44(uStack_17c,uStack_180);
          uStack_68 = CONCAT44(uStack_194,uStack_198);
          uStack_60 = CONCAT44(uStack_18c,uStack_190);
          uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
          uStack_78 = uStack_1a8;
          uStack_88 = uStack_1b8;
          uStack_80 = uStack_1b0;
          uStack_98 = (undefined *)CONCAT44(iStack_1c4,uStack_1c8);
          uStack_90 = uStack_1c0;
          uStack_3c8 = uStack_218;
          puStack_3d8 = puStack_228;
          uStack_3d0 = uStack_220;
          uStack_3a8 = uStack_210;
          uStack_3a0 = uStack_208;
          uStack_398 = uStack_200;
          uStack_3c0 = uStack_1f8;
          uStack_3b8 = uStack_1f0;
          uStack_3b0 = uStack_1e8;
          uStack_360 = CONCAT44(uStack_174,uStack_178);
          uStack_358 = uStack_170;
          uStack_350 = (undefined4)uStack_168;
          uStack_34c = uStack_168._4_4_;
          uStack_348 = (undefined4)uStack_160;
          uStack_344 = uStack_160._4_4_;
          uStack_340 = (undefined4)uStack_158;
          uStack_33c = uStack_158._4_4_;
          uStack_338 = (undefined4)uStack_150;
          uStack_334 = uStack_150._4_4_;
          uStack_330 = (undefined4)uStack_148;
          uStack_32c = uStack_148._4_4_;
          uStack_328 = uStack_140;
          uStack_324 = uStack_13c;
          uStack_320 = uStack_138;
          uStack_318 = uStack_130;
          uStack_310 = uStack_128;
          uStack_308 = uStack_120;
          uStack_380 = uStack_1d0;
          uStack_390 = CONCAT44(uStack_1dc,uStack_1e0);
          uStack_388 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
          uStack_2f8 = uStack_a8;
          uStack_300 = lStack_f0;
          uStack_378 = 1;
          uStack_370 = 0x100000001;
          iStack_374 = iVar2;
          FUN_1409dd4d0(&uStack_98,&puStack_3d8);
          if (cStack_f2 != '\b') {
            FUN_140be24b0(auStack_108);
          }
          param_1[2] = CONCAT44(uStack_234,uStack_238);
          *param_1 = CONCAT44(uStack_244,uStack_248);
          param_1[1] = CONCAT44(uStack_23c,uStack_240);
          param_1[3] = CONCAT44(uStack_414,uStack_418);
          param_1[4] = CONCAT44(uStack_40c,uStack_410);
          param_1[5] = CONCAT44(uStack_404,uStack_408);
          return param_1;
        }
        goto code_r0x0001409e8445;
      }
    }
  }
  FUN_1428d9310(&UNK_142ac4948);
code_r0x0001409e8445:
  FUN_1428d8fe3(1,4,&UNK_142c7d8d0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

