
void FUN_14098bf60(undefined8 param_1,undefined8 param_2,ulonglong *param_3)

{
  ulonglong uVar1;
  code *pcVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  int iVar6;
  ulonglong uVar7;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined *puStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined4 uStack_378;
  undefined4 uStack_374;
  undefined8 uStack_370;
  undefined4 uStack_368;
  int iStack_364;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined4 uStack_328;
  undefined4 uStack_324;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined1 auStack_2a8 [22];
  char cStack_292;
  undefined8 uStack_290;
  undefined8 uStack_248;
  undefined *puStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 uStack_1e0;
  undefined4 uStack_1d8;
  int iStack_1d4;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
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
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined1 auStack_118 [22];
  char cStack_102;
  undefined8 uStack_100;
  undefined8 uStack_b8;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  ulonglong *puStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar1 = *param_3;
  puStack_3c8 = (undefined *)(uVar1 >> 2 & 0x3ff);
  if (puStack_3c8 == (undefined *)0x0) {
    FUN_1416032a0(&puStack_238,&UNK_142ab9070,9,&UNK_142ab9079,0x2a,param_2,param_1);
    FUN_14096dd60(&puStack_3c8,&puStack_238,0xaab861d318ad7248,param_3);
    iVar6 = (int)(uVar1 >> 2) - (int)(uVar1 >> 3);
    uStack_88 = uStack_348;
    uStack_98 = uStack_358;
    uStack_90 = uStack_350;
    uStack_a8 = CONCAT44(iStack_364,uStack_368);
    uStack_a0 = uStack_360;
    uStack_228 = uStack_3b8;
    puStack_238 = puStack_3c8;
    uStack_230 = uStack_3c0;
    uStack_208 = uStack_3b0;
    uStack_200 = uStack_3a8;
    uStack_1f8 = uStack_3a0;
    uStack_220 = uStack_398;
    uStack_218 = uStack_390;
    uStack_210 = uStack_388;
    uStack_1c0 = CONCAT44(uStack_314,uStack_318);
    uStack_1b8 = uStack_310;
    uStack_1b0 = (undefined4)uStack_308;
    uStack_1ac = uStack_308._4_4_;
    uStack_1a8 = (undefined4)uStack_300;
    uStack_1a4 = uStack_300._4_4_;
    uStack_1a0 = (undefined4)uStack_2f8;
    uStack_19c = uStack_2f8._4_4_;
    uStack_198 = (undefined4)uStack_2f0;
    uStack_194 = uStack_2f0._4_4_;
    uStack_190 = (undefined4)uStack_2e8;
    uStack_18c = uStack_2e8._4_4_;
    uStack_188 = uStack_2e0;
    uStack_184 = uStack_2dc;
    uStack_180 = uStack_2d8;
    uStack_178 = uStack_2d0;
    uStack_170 = uStack_2c8;
    uStack_168 = uStack_2c0;
    uStack_1e0 = uStack_370;
    uStack_1f0 = uStack_380;
    uStack_1ec = uStack_37c;
    uStack_1e8 = uStack_378;
    uStack_1e4 = uStack_374;
    uStack_158 = uStack_248;
    uStack_160 = uStack_290;
    uStack_1d8 = 1;
    uStack_1d0 = 0x100000001;
    iStack_1d4 = iVar6;
    FUN_1409dd4d0(&uStack_a8,&puStack_238);
    puStack_50 = param_3;
    if (cStack_292 != '\b') {
      FUN_140be24b0(auStack_2a8);
    }
    uVar7 = uVar1 >> 0xc;
    lVar3 = uVar7 + (uVar7 == 0);
    uVar4 = (ulonglong)((uint)lVar3 & 0x3ff);
    lVar5 = 0x400 - uVar4;
    if (uVar4 == 0) {
      lVar5 = 0;
    }
    FUN_140bd4e20(&uStack_a8,(lVar5 + lVar3) * 4,0);
    uStack_3c0 = uStack_a0;
    func_0x000140bd6d00(&uStack_228,&puStack_3c8);
    puStack_238 = &UNK_142ab90a3;
    uStack_230 = 0x10;
    uStack_3d8 = FUN_140be94d0(param_1,&puStack_238);
    uStack_3e8 = (undefined4)uStack_a8;
    uStack_3e4 = uStack_a8._4_4_;
    uStack_3e0 = (undefined4)uStack_a0;
    uStack_3dc = uStack_a0._4_4_;
    FUN_1416032a0(&puStack_238,&UNK_142ab90b3,9,&UNK_142ab90bc,0x34,param_2,param_1);
    FUN_14096dac0(&puStack_3c8,&puStack_238,0xbb51ae979b5e9131,puStack_50);
    FUN_14096dd60(&puStack_238,&puStack_3c8,0xecf59a3a8eeaedc6,&uStack_3e8);
    iStack_364 = (int)uVar7 - (int)(uVar1 >> 0xd);
    uStack_68 = CONCAT44(uStack_194,uStack_198);
    uStack_60 = CONCAT44(uStack_18c,uStack_190);
    uStack_78 = CONCAT44(uStack_1a4,uStack_1a8);
    uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
    uStack_80 = CONCAT44(uStack_1ac,uStack_1b0);
    uStack_88 = uStack_1b8;
    uStack_98 = uStack_1c8;
    uStack_90 = uStack_1c0;
    uStack_a8 = CONCAT44(iStack_1d4,uStack_1d8);
    uStack_a0 = uStack_1d0;
    uStack_3b8 = uStack_228;
    puStack_3c8 = puStack_238;
    uStack_3c0 = uStack_230;
    uStack_398 = uStack_220;
    uStack_390 = uStack_218;
    uStack_388 = uStack_210;
    uStack_3b0 = uStack_208;
    uStack_3a8 = uStack_200;
    uStack_3a0 = uStack_1f8;
    uStack_350 = CONCAT44(uStack_184,uStack_188);
    uStack_348 = uStack_180;
    uStack_340 = (undefined4)uStack_178;
    uStack_33c = uStack_178._4_4_;
    uStack_338 = (undefined4)uStack_170;
    uStack_334 = uStack_170._4_4_;
    uStack_330 = (undefined4)uStack_168;
    uStack_32c = uStack_168._4_4_;
    uStack_328 = (undefined4)uStack_160;
    uStack_324 = uStack_160._4_4_;
    uStack_320 = (undefined4)uStack_158;
    uStack_31c = uStack_158._4_4_;
    uStack_318 = uStack_150;
    uStack_314 = uStack_14c;
    uStack_310 = uStack_148;
    uStack_308 = uStack_140;
    uStack_300 = uStack_138;
    uStack_2f8 = uStack_130;
    uStack_370 = uStack_1e0;
    uStack_380 = uStack_1f0;
    uStack_37c = uStack_1ec;
    uStack_378 = uStack_1e8;
    uStack_374 = uStack_1e4;
    uStack_2e8 = uStack_b8;
    uStack_2f0 = uStack_100;
    uStack_368 = 1;
    uStack_360 = 0x100000001;
    FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
    if (cStack_102 != '\b') {
      FUN_140be24b0(auStack_118);
    }
    FUN_1416032a0(&puStack_238,&UNK_142ab90f0,0xd,&UNK_142ab90fd,0x30,param_2,param_1);
    FUN_14096dd60(&puStack_3c8,&puStack_238,0xaab861d318ad7248,puStack_50);
    FUN_14096dac0(&puStack_238,&puStack_3c8,0x5067cfd1b30b03ab,&uStack_3e8);
    uStack_68 = CONCAT44(uStack_194,uStack_198);
    uStack_60 = CONCAT44(uStack_18c,uStack_190);
    uStack_78 = CONCAT44(uStack_1a4,uStack_1a8);
    uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
    uStack_80 = CONCAT44(uStack_1ac,uStack_1b0);
    uStack_88 = uStack_1b8;
    uStack_98 = uStack_1c8;
    uStack_90 = uStack_1c0;
    uStack_a8 = CONCAT44(iStack_1d4,uStack_1d8);
    uStack_a0 = uStack_1d0;
    uStack_3b8 = uStack_228;
    puStack_3c8 = puStack_238;
    uStack_3c0 = uStack_230;
    uStack_398 = uStack_220;
    uStack_390 = uStack_218;
    uStack_388 = uStack_210;
    uStack_3b0 = uStack_208;
    uStack_3a8 = uStack_200;
    uStack_3a0 = uStack_1f8;
    uStack_350 = CONCAT44(uStack_184,uStack_188);
    uStack_348 = uStack_180;
    uStack_340 = (undefined4)uStack_178;
    uStack_33c = uStack_178._4_4_;
    uStack_338 = (undefined4)uStack_170;
    uStack_334 = uStack_170._4_4_;
    uStack_330 = (undefined4)uStack_168;
    uStack_32c = uStack_168._4_4_;
    uStack_328 = (undefined4)uStack_160;
    uStack_324 = uStack_160._4_4_;
    uStack_320 = (undefined4)uStack_158;
    uStack_31c = uStack_158._4_4_;
    uStack_318 = uStack_150;
    uStack_314 = uStack_14c;
    uStack_310 = uStack_148;
    uStack_308 = uStack_140;
    uStack_300 = uStack_138;
    uStack_2f8 = uStack_130;
    uStack_370 = uStack_1e0;
    uStack_380 = uStack_1f0;
    uStack_37c = uStack_1ec;
    uStack_378 = uStack_1e8;
    uStack_374 = uStack_1e4;
    uStack_2e8 = uStack_b8;
    uStack_2f0 = uStack_100;
    uStack_368 = 1;
    uStack_360 = 0x100000001;
    iStack_364 = iVar6;
    FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
    if (cStack_102 != '\b') {
      FUN_140be24b0(auStack_118);
    }
    return;
  }
  puStack_238 = (undefined *)0x0;
  FUN_1428d9596(0,&puStack_3c8,&UNK_142ab8c80,&puStack_238,&UNK_142ab9058);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

