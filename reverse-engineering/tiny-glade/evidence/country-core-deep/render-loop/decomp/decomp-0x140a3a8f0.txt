
void FUN_140a3a8f0(undefined8 param_1,undefined8 param_2,longlong param_3,undefined1 param_4,
                  undefined4 param_5,undefined4 param_6)

{
  byte bVar1;
  undefined8 uVar2;
  code *pcVar3;
  undefined1 uVar4;
  longlong lVar5;
  ulonglong uVar6;
  uint *puVar7;
  uint uVar8;
  int iVar9;
  longlong unaff_RSI;
  uint uVar10;
  int iVar11;
  undefined4 auStack_448 [2];
  undefined4 uStack_440;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined4 uStack_430;
  undefined4 uStack_42c;
  undefined8 *puStack_428;
  undefined8 uStack_418;
  undefined *puStack_410;
  undefined4 uStack_408;
  undefined4 uStack_404;
  undefined4 uStack_400;
  undefined4 uStack_3fc;
  undefined4 uStack_3f8;
  undefined4 uStack_3f4;
  undefined4 uStack_3f0;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
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
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined1 auStack_2f8 [22];
  char cStack_2e2;
  undefined8 uStack_2e0;
  undefined8 uStack_298;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined8 uStack_280;
  undefined8 uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  longlong lStack_240;
  uint *puStack_238;
  undefined8 uStack_230;
  undefined4 uStack_228;
  int iStack_224;
  int iStack_220;
  undefined4 uStack_21c;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined *puStack_b0;
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
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  auStack_448[0] = param_5;
  uStack_440 = param_6;
  bVar1 = *(byte *)(param_3 + 0x1c);
  uStack_4c = param_4;
  if (bVar1 < 0xe) {
    unaff_RSI = param_3;
    if (*(int *)(param_3 + 8) != 0x7a) goto code_r0x000140a3af6b;
    uStack_418 = auStack_448;
    puStack_410 = &UNK_140109e90;
    uStack_288 = 0x42acd3b0;
    uStack_284 = 1;
    uStack_280 = (undefined *)0x1;
    uStack_278 = &uStack_418;
    uStack_270 = 1;
    uStack_268 = 0;
    FUN_140d79f90(&uStack_a8,&uStack_288);
    puStack_b0 = uStack_a0;
    lVar5 = FUN_140c0f3f0(uStack_a0,uStack_98);
    uVar2 = *(undefined8 *)(lVar5 + -8);
    FUN_140bd4e20(&uStack_418,0x18,0x20);
    FUN_14096c310(&uStack_288,param_1,lVar5,uVar2,&uStack_418);
    if ((char)uStack_270 != '\x02') {
      uStack_408 = (undefined4)uStack_278;
      uStack_404 = uStack_278._4_4_;
      uStack_400 = (undefined4)uStack_270;
      uStack_3fc = uStack_270._4_4_;
      uStack_418._0_4_ = uStack_288;
      uStack_418._4_4_ = uStack_284;
      puStack_410 = uStack_280;
      uStack_438 = uStack_288;
      uStack_434 = uStack_284;
      uStack_430 = (undefined4)uStack_280;
      uStack_42c = uStack_280._4_4_;
      puStack_428 = uStack_278;
      if (uStack_a8 != 0) {
        func_0x000140613c20(puStack_b0,uStack_a8,1);
      }
      uVar6 = func_0x000140bd5790(param_3);
      iVar11 = (((uint)(uVar6 >> 6) & 0x3ffffff) + 1) - (uint)((uVar6 & 0x3f) == 0);
      iVar9 = ((uint)(uVar6 >> 0x26) + 1) - (uint)((uVar6 & 0x3f00000000) == 0);
      FUN_1416032a0(&uStack_288,&UNK_142acd410,0xd,&UNK_142acd41d,0x16,param_2,param_1);
      uStack_4b = 1;
      func_0x000140bd57a0(&uStack_418);
      uStack_4b = 1;
      func_0x000140bd58e0(&uStack_a8,&uStack_418,0);
      uStack_4b = 1;
      func_0x000140bd5920(&uStack_418,&uStack_a8,1,1);
      uVar4 = uStack_4c;
      uStack_a8 = CONCAT44(uStack_418._4_4_,(undefined4)uStack_418);
      uStack_a0 = puStack_410;
      uStack_98 = CONCAT44(uStack_404,uStack_408);
      uStack_90 = CONCAT44(uStack_3fc,uStack_400);
      uStack_88._0_4_ = uStack_3f8;
      uStack_88._4_4_ = uStack_3f4;
      uStack_80._0_4_ = uStack_3f0;
      uStack_80._4_4_ = uStack_3ec;
      uStack_78._0_4_ = uStack_3e8;
      uStack_78._4_4_ = uStack_3e4;
      uStack_70 = CONCAT44(uStack_70._4_4_,uStack_3e0);
      uStack_f8 = CONCAT31(uStack_f8._1_3_,uStack_4c);
      FUN_140905260(&uStack_f8,&uStack_288,0xb466b30ac6fc0292,0x37,&uStack_a8);
      func_0x000142923c00(&uStack_418,&uStack_288,400);
      uStack_49 = 1;
      func_0x000140bd57a0(&uStack_288);
      uStack_49 = 1;
      func_0x000140bd59f0(&uStack_a8,&uStack_288,1);
      uStack_49 = 1;
      func_0x000140bd58e0(&uStack_288,&uStack_a8,1);
      uStack_49 = 1;
      func_0x000140bd57e0(&uStack_a8,&uStack_288,5);
      uStack_f8 = (undefined4)uStack_a8;
      uStack_f4 = uStack_a8._4_4_;
      uStack_f0 = (undefined4)uStack_a0;
      uStack_ec = uStack_a0._4_4_;
      uStack_e8 = (undefined4)uStack_98;
      uStack_e4 = uStack_98._4_4_;
      uStack_e0 = (undefined4)uStack_90;
      uStack_dc = uStack_90._4_4_;
      uStack_d8 = (undefined4)uStack_88;
      uStack_d4 = uStack_88._4_4_;
      uStack_d0 = (undefined4)uStack_80;
      uStack_cc = uStack_80._4_4_;
      uStack_c8 = (undefined4)uStack_78;
      uStack_c4 = uStack_78._4_4_;
      uStack_c0 = (undefined4)uStack_70;
      uStack_a8 = CONCAT71(uStack_a8._1_7_,uVar4);
      FUN_140905260(&uStack_a8,&uStack_418,0xed9d1f0a5683f687,0x37,&uStack_f8);
      func_0x000142923c00(&uStack_288,&uStack_418,400);
      uStack_4a = 1;
      func_0x000140bd57a0(&uStack_418);
      uStack_4a = 1;
      func_0x000140bd57e0(&uStack_a8,&uStack_418,5);
      uVar8 = bVar1 - 1;
      uVar10 = 6;
      if (uVar8 < 6) {
        uVar10 = uVar8;
      }
      uStack_4a = 1;
      func_0x000140bd58e0(&uStack_418,&uStack_a8,uVar10);
      uStack_4a = 1;
      func_0x000140bd5920(&uStack_a8,&uStack_418,1,1);
      uStack_f8 = (undefined4)uStack_a8;
      uStack_f4 = uStack_a8._4_4_;
      uStack_f0 = (undefined4)uStack_a0;
      uStack_ec = uStack_a0._4_4_;
      uStack_e8 = (undefined4)uStack_98;
      uStack_e4 = uStack_98._4_4_;
      uStack_e0 = (undefined4)uStack_90;
      uStack_dc = uStack_90._4_4_;
      uStack_d8 = (undefined4)uStack_88;
      uStack_d4 = uStack_88._4_4_;
      uStack_d0 = (undefined4)uStack_80;
      uStack_cc = uStack_80._4_4_;
      uStack_c8 = (undefined4)uStack_78;
      uStack_c4 = uStack_78._4_4_;
      uStack_c0 = (undefined4)uStack_70;
      uStack_a8 = CONCAT71(uStack_a8._1_7_,uVar4);
      FUN_140905260(&uStack_a8,&uStack_288,0x45f26f91023c2ec3,0x37,&uStack_f8);
      func_0x000142923c00(&uStack_418,&uStack_288,400);
      FUN_14096dd60(&uStack_288,&uStack_418,0x68d4413b4de4f24,&uStack_438);
      puVar7 = (uint *)func_0x000140613c10(0x10,1);
      if (puVar7 != (uint *)0x0) {
        *puVar7 = uVar8;
        puVar7[1] = iVar9 * iVar11;
        puVar7[2] = 0;
        puVar7[3] = 0;
        if (lStack_240 != 0) {
          func_0x000140613c20(puStack_238,lStack_240,1);
        }
        lStack_240 = 0x10;
        uStack_230 = 0x10;
        puStack_238 = puVar7;
        func_0x000142923c00(&uStack_418,&uStack_288,400);
        uStack_68 = uStack_378;
        uStack_60 = uStack_370;
        uStack_78 = uStack_388;
        uStack_70 = uStack_380;
        uStack_88 = uStack_398;
        uStack_80 = uStack_390;
        uStack_98 = uStack_3a8;
        uStack_90 = uStack_3a0;
        uStack_a8 = uStack_3b8;
        uStack_a0 = (undefined *)uStack_3b0;
        uStack_278 = (undefined8 *)CONCAT44(uStack_404,uStack_408);
        uStack_288 = (undefined4)uStack_418;
        uStack_284 = uStack_418._4_4_;
        uStack_280 = puStack_410;
        uStack_258 = CONCAT44(uStack_3fc,uStack_400);
        uStack_260 = uStack_3d8;
        uStack_210 = uStack_368;
        uStack_208 = uStack_360;
        uStack_200 = uStack_358;
        uStack_1fc = uStack_354;
        uStack_1f8 = uStack_350;
        uStack_1f4 = uStack_34c;
        uStack_1f0 = uStack_348;
        uStack_1ec = uStack_344;
        uStack_1e8 = uStack_340;
        uStack_1e4 = uStack_33c;
        uStack_1e0 = uStack_338;
        uStack_1dc = uStack_334;
        uStack_1d8 = uStack_330;
        uStack_1d4 = uStack_32c;
        uStack_1d0 = uStack_328;
        uStack_1c8 = uStack_320;
        uStack_1c0 = uStack_318;
        uStack_1b8 = uStack_310;
        uStack_230 = uStack_3c0;
        uStack_1a8 = uStack_298;
        uStack_1b0 = uStack_2e0;
        uStack_228 = 0;
        uStack_21c = 1;
        iStack_224 = iVar11;
        iStack_220 = iVar9;
        FUN_1409dd4d0(&uStack_a8,&uStack_288);
        if (cStack_2e2 != '\b') {
          FUN_140be24b0(auStack_2f8);
        }
        return;
      }
      FUN_1428d8fe3(1,0x10,&UNK_142c7d8d0);
      goto code_r0x000140a3afe9;
    }
  }
  else {
    FUN_1428d9430(&UNK_142acd2ea,0x2f,&UNK_142acd368);
code_r0x000140a3af6b:
    uStack_288 = 0;
    uStack_284 = 0;
    FUN_1428a9914(0,unaff_RSI + 8,&UNK_142acd380,&uStack_288,&UNK_142acd388);
  }
  uStack_418._0_4_ = uStack_288;
  uStack_418._4_4_ = uStack_284;
  FUN_1428d9760(&UNK_142acd3c0,0x35,&uStack_418,&UNK_142acc6e8,&UNK_142acd3f8);
code_r0x000140a3afe9:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

