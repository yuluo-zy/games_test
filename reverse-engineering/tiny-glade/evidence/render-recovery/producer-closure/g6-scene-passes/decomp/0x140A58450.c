
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a58450(undefined8 param_1,longlong param_2)

{
  uint uVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  int iVar10;
  longlong lVar11;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined4 uStack_3d4;
  undefined4 uStack_3d0;
  undefined4 uStack_3cc;
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
  undefined *puStack_368;
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
  undefined1 auStack_2b8 [16];
  undefined1 auStack_2a8 [22];
  char cStack_292;
  longlong lStack_290;
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
  undefined8 uStack_1d8;
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
  undefined1 auStack_128 [16];
  undefined1 auStack_118 [22];
  char cStack_102;
  longlong lStack_100;
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
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  lVar8 = param_2 + 0x38;
  uVar9 = 0xc2b8e941c91a6428;
  lVar7 = FUN_1400dd6f0(lVar8,0xc2b8e941c91a6428,0x59f3c6d1d3da43a0);
  if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar9)) ||
     (uVar9 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar9 * 8), uVar9 == 0)) {
code_r0x000140a58e43:
    FUN_1428d9310(&UNK_142ad2df8);
  }
  else {
    lVar7 = ~uVar9 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x000140a58e43;
    uVar1 = **(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
    lVar7 = 0;
    if (uVar1 - 6 < 0xf) {
      lVar7 = (ulonglong)uVar1 - 5;
    }
    iVar10 = -1;
    if ((lVar7 == 5) || ((lVar7 == 0 && (uVar1 == 5)))) {
      uVar9 = 0xc26ebbeefbfc3261;
      lVar7 = FUN_1400dd6f0(lVar8,0xc26ebbeefbfc3261,0x7a483ee28328d5ec);
      if ((lVar7 == 0) ||
         ((*(ulonglong *)(param_2 + 0x218) <= uVar9 ||
          (uVar9 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar9 * 8), uVar9 == 0))))
      goto code_r0x000140a58e5b;
      lVar7 = ~uVar9 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
      goto code_r0x000140a58e5b;
      lVar7 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
      lVar2 = *(longlong *)(lVar7 + 0x10);
      lVar3 = *(longlong *)(lVar7 + 0x30);
      if (lVar3 != 0 || lVar2 != 0) {
        lVar11 = *(longlong *)(lVar7 + 8) + -0x10 + lVar2 * 0x10;
        if (lVar2 == 0) {
          lVar11 = 0;
        }
        lVar7 = *(longlong *)(lVar7 + 0x28) + -0x10 + lVar3 * 0x10;
        if (lVar3 == 0) {
          lVar7 = lVar11;
        }
        iVar10 = *(int *)(lVar7 + 8);
      }
    }
    uVar9 = 0x48b321ef779dc301;
    lVar8 = FUN_1400dd6f0(lVar8,0x48b321ef779dc301,0xbd65767194d9ddc8);
    if (((lVar8 != 0) && (uVar9 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar9 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar9 * 8), uVar9 != 0)) {
      lVar8 = ~uVar9 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
        if (*(char *)(*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10) + 0xf9) != '\0')
        {
          iVar10 = -1;
        }
        FUN_1416032a0(&puStack_238,&UNK_142ad2e40,0x28,&UNK_142ad2e68,0x31,param_2,param_1);
        if (cStack_102 != '\b') {
          FUN_14096b570(auStack_128,lStack_100 + 0xe8,0xb02feb017e06f117,iVar10);
        }
        func_0x000142923c00(&puStack_3c8,&puStack_238,400);
        uStack_a8._0_1_ = 0xb;
        puStack_238 = (undefined *)CONCAT44(puStack_238._4_4_,2);
        FUN_140905260(&uStack_a8,&puStack_3c8,0xe3659c0fbde7bf03,0x19,&puStack_238);
        func_0x000142923c00(&puStack_238,&puStack_3c8,400);
        uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0xc);
        puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
        FUN_140905260(&uStack_a8,&puStack_238,0x1ca51a73373208db,0x32,&puStack_3c8);
        func_0x000142923c00(&puStack_3c8,&puStack_238,400);
        uVar6 = _UNK_142ad0488;
        uVar5 = _UNK_142ad0480;
        uStack_88 = uStack_348;
        uStack_98 = uStack_358;
        uStack_90 = uStack_350;
        uStack_a8 = (undefined4 *)puStack_368;
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
        uStack_160 = lStack_290;
        uStack_1d8 = _UNK_142ad0480;
        uStack_1d0 = _UNK_142ad0488;
        FUN_1409dd4d0(&uStack_a8,&puStack_238);
        if (cStack_292 != '\b') {
          FUN_140be24b0(auStack_2a8);
        }
        if (iVar10 != -1) {
          func_0x000140bd52a0(&uStack_a8,0x10000000100,0x4c);
          uStack_3b8 = uStack_98;
          uStack_3b0 = uStack_90;
          puStack_3c8 = (undefined *)uStack_a8;
          uStack_3c0 = uStack_a0;
          func_0x000140bd6ce0(&uStack_228,&puStack_3c8);
          puStack_238 = &UNK_142ad2e99;
          uStack_230 = 9;
          uStack_3ec = FUN_140be94d0(param_1,&puStack_238);
          uStack_3e8 = (undefined4)uStack_a8;
          uStack_3e4 = uStack_a8._4_4_;
          uStack_3e0 = (undefined4)uStack_a0;
          uStack_3dc = uStack_a0._4_4_;
          uStack_3d8 = (undefined4)uStack_98;
          uStack_3d4 = uStack_98._4_4_;
          uStack_3d0 = (undefined4)uStack_90;
          uStack_3cc = uStack_90._4_4_;
          FUN_1416032a0(&puStack_238,&UNK_142ad2ea2,0x1b,&UNK_142ad2ebd,0x18,param_2,param_1);
          if (cStack_102 != '\b') {
            FUN_14096b850(auStack_128,lStack_100 + 0xe8,0xd91ff8372094bc22,1);
          }
          func_0x000142923c00(&puStack_3c8,&puStack_238,400);
          if (cStack_292 != '\b') {
            FUN_14096ba30(auStack_2b8,lStack_290 + 0xe8,0xdc022c4f3092a794,_UNK_142925904);
          }
          func_0x000142923c00(&puStack_238,&puStack_3c8,400);
          uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0xc);
          puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
          FUN_140905260(&uStack_a8,&puStack_238,0xd82d7a0adb9164c2,0x19,&puStack_3c8);
          func_0x000142923c00(&puStack_3c8,&puStack_238,400);
          puStack_238 = (undefined *)CONCAT44(puStack_238._4_4_,2);
          uStack_a8 = &uStack_3ec;
          FUN_1408edd00(&uStack_a8,&puStack_3c8,0x1ca51a73373208db,0x32,&puStack_238);
          func_0x000142923c00(&puStack_238,&puStack_3c8,400);
          uStack_68 = CONCAT44(uStack_194,uStack_198);
          uStack_60 = CONCAT44(uStack_18c,uStack_190);
          uStack_78 = CONCAT44(uStack_1a4,uStack_1a8);
          uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
          uStack_80 = CONCAT44(uStack_1ac,uStack_1b0);
          uStack_88 = uStack_1b8;
          uStack_98 = uStack_1c8;
          uStack_90 = uStack_1c0;
          uStack_a8 = (undefined4 *)uStack_1d8;
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
          uStack_2f0 = lStack_100;
          puStack_368 = (undefined *)uVar5;
          uStack_360 = uVar6;
          FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
          if (cStack_102 != '\b') {
            FUN_140be24b0(auStack_118);
          }
          FUN_1416032a0(&puStack_238,&UNK_142ad2ea2,0x1b,&UNK_142ad2ebd,0x18,param_2,param_1);
          if (cStack_102 != '\b') {
            FUN_14096b850(auStack_128,lStack_100 + 0xe8,0xd91ff8372094bc22,0);
          }
          func_0x000142923c00(&puStack_3c8,&puStack_238,400);
          if (cStack_292 != '\b') {
            FUN_14096ba30(auStack_2b8,lStack_290 + 0xe8,0xdc022c4f3092a794,_UNK_142925904);
          }
          func_0x000142923c00(&puStack_238,&puStack_3c8,400);
          puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
          uStack_a8 = &uStack_3ec;
          FUN_1408ee240(&uStack_a8,&puStack_238,0xd82d7a0adb9164c2,0x19,&puStack_3c8);
          func_0x000142923c00(&puStack_3c8,&puStack_238,400);
          uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0xc);
          puStack_238 = (undefined *)CONCAT44(puStack_238._4_4_,2);
          FUN_140905260(&uStack_a8,&puStack_3c8,0x1ca51a73373208db,0x32,&puStack_238);
          func_0x000142923c00(&puStack_238,&puStack_3c8,400);
          uStack_68 = CONCAT44(uStack_194,uStack_198);
          uStack_60 = CONCAT44(uStack_18c,uStack_190);
          uStack_78 = CONCAT44(uStack_1a4,uStack_1a8);
          uStack_70 = CONCAT44(uStack_19c,uStack_1a0);
          uStack_80 = CONCAT44(uStack_1ac,uStack_1b0);
          uStack_88 = uStack_1b8;
          uStack_98 = uStack_1c8;
          uStack_90 = uStack_1c0;
          uStack_a8 = (undefined4 *)uStack_1d8;
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
          uStack_2f0 = lStack_100;
          puStack_368 = (undefined *)uVar5;
          uStack_360 = uVar6;
          FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
          if (cStack_102 != '\b') {
            FUN_140be24b0(auStack_118);
          }
        }
        return;
      }
    }
  }
  FUN_1428d9310(&UNK_142ad2e28);
code_r0x000140a58e5b:
  FUN_1428d9310(&UNK_142ad2e10);
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

