
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140987dd0(longlong param_1,longlong param_2,longlong param_3)

{
  undefined *puVar1;
  undefined *puVar2;
  code *pcVar3;
  undefined8 uVar4;
  undefined *puVar5;
  byte bVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  undefined8 uStack_5f8;
  undefined8 uStack_5f0;
  undefined8 uStack_5e8;
  undefined8 uStack_5e0;
  undefined8 uStack_5d8;
  undefined8 uStack_5d0;
  undefined8 uStack_5c8;
  undefined8 uStack_5c0;
  undefined8 uStack_5b8;
  undefined8 uStack_5b0;
  undefined8 uStack_460;
  undefined4 uStack_458;
  undefined4 uStack_454;
  undefined4 uStack_450;
  undefined4 uStack_44c;
  undefined4 uStack_448;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined4 uStack_430;
  undefined4 uStack_42c;
  undefined4 uStack_428;
  undefined1 auStack_420 [24];
  undefined *puStack_408;
  longlong lStack_400;
  ulonglong uStack_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined4 uStack_3c0;
  undefined4 uStack_3bc;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
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
  undefined4 uStack_370;
  undefined4 uStack_36c;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined1 auStack_2e8 [22];
  char cStack_2d2;
  undefined8 uStack_2d0;
  undefined8 uStack_288;
  undefined *puStack_278;
  undefined8 uStack_270;
  ulonglong uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined1 auStack_158 [22];
  char cStack_142;
  undefined8 uStack_140;
  undefined8 uStack_f8;
  undefined1 auStack_e8 [64];
  longlong lStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  longlong lStack_90;
  undefined *puStack_88;
  uint uStack_80;
  undefined1 uStack_79;
  longlong lStack_78;
  undefined1 uStack_69;
  undefined *puStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uVar8 = 0x662978458699a5ef;
  lStack_a8 = param_3;
  lVar7 = FUN_1400dd6f0(param_2 + 0x38,0x662978458699a5ef,0x3ec057c162419b46);
  if (((lVar7 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar7 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
      lVar7 = *(longlong *)(param_2 + 0x1e0) + lVar7;
      puVar1 = *(undefined **)(lVar7 + 0x10);
      *(undefined4 *)(lVar7 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
      FUN_140bd4e20(&uStack_5f8,0x6c000,0);
      func_0x000140bd6d00(&uStack_268,&puStack_408);
      puStack_278 = &UNK_142ab8bd8;
      uStack_270 = 0x1b;
      uStack_428 = FUN_140be94d0(param_1,&puStack_278);
      uStack_438 = (undefined4)uStack_5f8;
      uStack_434 = uStack_5f8._4_4_;
      uStack_430 = (undefined4)uStack_5f0;
      uStack_42c = uStack_5f0._4_4_;
      puStack_88 = puVar1;
      FUN_140bd4e20(&uStack_5f8,(ulonglong)*(uint *)(*(longlong *)(puVar1 + 0x1450) + 0x60) << 2,0);
      func_0x000140bd6d00(&uStack_268,&puStack_408);
      puStack_278 = &UNK_142ab8bf3;
      uStack_270 = 0x1b;
      uStack_448 = FUN_140be94d0(param_1,&puStack_278);
      uStack_458 = (undefined4)uStack_5f8;
      uStack_454 = uStack_5f8._4_4_;
      uStack_450 = (undefined4)uStack_5f0;
      uStack_44c = uStack_5f0._4_4_;
      FUN_1416032a0(&puStack_278,&UNK_142ab8c47,0x15,&UNK_142ab8c5c,0x1e,param_2,param_1);
      FUN_14160d110(&puStack_408,&puStack_278,0x86fe9dc51990c5e3);
      FUN_141606900(&puStack_278,&puStack_408,0x8661a811321c4474);
      FUN_141604a30(&puStack_408,&puStack_278,0x2d8ee4b8ed072cba);
      FUN_141606730(&puStack_278,&puStack_408,0x686c38b1c905f34d);
      uStack_5d8 = uStack_1f8;
      uStack_5e8 = uStack_208;
      uStack_5e0 = uStack_200;
      uStack_3f8 = uStack_268;
      puStack_408 = puStack_278;
      lStack_400 = uStack_270;
      uStack_3d8 = uStack_260;
      uStack_3d0 = uStack_258;
      uStack_3c8 = uStack_250;
      uStack_3f0 = uStack_248;
      uStack_3e8 = uStack_240;
      uStack_3e0 = uStack_238;
      uStack_390 = CONCAT44(uStack_1c4,uStack_1c8);
      uStack_388 = uStack_1c0;
      uStack_380 = (undefined4)uStack_1b8;
      uStack_37c = uStack_1b8._4_4_;
      uStack_378 = (undefined4)uStack_1b0;
      uStack_374 = uStack_1b0._4_4_;
      uStack_370 = (undefined4)uStack_1a8;
      uStack_36c = uStack_1a8._4_4_;
      uStack_368 = (undefined4)uStack_1a0;
      uStack_364 = uStack_1a0._4_4_;
      uStack_360 = (undefined4)uStack_198;
      uStack_35c = uStack_198._4_4_;
      uStack_358 = uStack_190;
      uStack_354 = uStack_18c;
      uStack_350 = uStack_188;
      uStack_348 = uStack_180;
      uStack_340 = uStack_178;
      uStack_338 = uStack_170;
      uStack_3b0 = uStack_220;
      uStack_3c0 = (undefined4)uStack_230;
      uStack_3bc = uStack_230._4_4_;
      uStack_3b8 = (undefined4)uStack_228;
      uStack_3b4 = uStack_228._4_4_;
      uStack_328 = uStack_f8;
      uStack_330 = uStack_140;
      uStack_3a8 = _UNK_142ab83d0;
      uStack_3a0 = _UNK_142ab83d8;
      FUN_1409dd4d0(&uStack_5f8,&puStack_408);
      if (cStack_142 != '\b') {
        FUN_140be24b0(auStack_158);
      }
      FUN_1416032a0(&puStack_278,&UNK_142ab8c88,0x15,&UNK_142ab8c9d,0x1d,param_2,param_1);
      FUN_14096dd60(&puStack_408,&puStack_278,0xfee3c4c9d72cf101,&uStack_438);
      uStack_5b8 = CONCAT44(uStack_364,uStack_368);
      uStack_5b0 = CONCAT44(uStack_35c,uStack_360);
      uStack_5c8 = CONCAT44(uStack_374,uStack_378);
      uStack_5c0 = CONCAT44(uStack_36c,uStack_370);
      uStack_5d0 = CONCAT44(uStack_37c,uStack_380);
      uStack_5d8 = uStack_388;
      uStack_5e8 = uStack_398;
      uStack_5e0 = uStack_390;
      uStack_5f8 = uStack_3a8;
      uStack_5f0 = uStack_3a0;
      uStack_268 = uStack_3f8;
      puStack_278 = puStack_408;
      uStack_270 = lStack_400;
      uStack_248 = uStack_3f0;
      uStack_240 = uStack_3e8;
      uStack_238 = uStack_3e0;
      uStack_260 = uStack_3d8;
      uStack_258 = uStack_3d0;
      uStack_250 = uStack_3c8;
      uStack_200 = CONCAT44(uStack_354,uStack_358);
      uStack_1f8 = uStack_350;
      uStack_1f0 = (undefined4)uStack_348;
      uStack_1ec = uStack_348._4_4_;
      uStack_1e8 = (undefined4)uStack_340;
      uStack_1e4 = uStack_340._4_4_;
      uStack_1e0 = (undefined4)uStack_338;
      uStack_1dc = uStack_338._4_4_;
      uStack_1d8 = (undefined4)uStack_330;
      uStack_1d4 = uStack_330._4_4_;
      uStack_1d0 = (undefined4)uStack_328;
      uStack_1cc = uStack_328._4_4_;
      uStack_1c8 = uStack_320;
      uStack_1c4 = uStack_31c;
      uStack_1c0 = uStack_318;
      uStack_1b8 = uStack_310;
      uStack_1b0 = uStack_308;
      uStack_1a8 = uStack_300;
      uStack_220 = uStack_3b0;
      uStack_230 = CONCAT44(uStack_3bc,uStack_3c0);
      uStack_228 = CONCAT44(uStack_3b4,uStack_3b8);
      uStack_198 = uStack_288;
      uStack_1a0 = uStack_2d0;
      uStack_218 = _UNK_142ab83e0;
      uStack_214 = _UNK_142ab83e4;
      uStack_210 = _UNK_142ab83e8;
      uStack_20c = _UNK_142ab83ec;
      FUN_1409dd4d0(&uStack_5f8,&puStack_278);
      if (cStack_2d2 != '\b') {
        FUN_140be24b0(auStack_2e8);
      }
      uVar8 = 0x8226cc786e679355;
      lVar7 = FUN_1400dd6f0(param_2 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
      puStack_68 = (undefined *)CONCAT71(puStack_68._1_7_,param_3 != 0);
      if (lStack_a8 == 0) {
        if (((lVar7 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
          lVar7 = ~uVar8 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
            FUN_1412d1180(&puStack_278,
                          *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10),param_1);
            FUN_1409ddbb0(param_1,param_2,&puStack_278);
            uStack_79 = 4;
            goto code_r0x0001409883ed;
          }
        }
      }
      else if (((lVar7 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
              (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
        lVar7 = ~uVar8 * 0x60;
        if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
          FUN_1412ce810(&puStack_278,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10),
                        param_1);
          FUN_1409ddbb0(param_1,param_2,&puStack_278);
          uStack_79 = 1;
code_r0x0001409883ed:
          puVar5 = puStack_88;
          puVar2 = *(undefined **)(puStack_88 + 0x1450);
          lStack_90 = param_2;
          FUN_1416032a0(&puStack_278,&UNK_142acab6a,0x20,&UNK_142acab8a,0x29,param_2,param_1);
          puVar1 = puVar5 + 0x1458;
          FUN_14096e2a0(&puStack_408,&puStack_278,0x2a75aeb2d5bdccbd,puVar1);
          FUN_141608090(&puStack_278,&puStack_408,0xd024032eecbe8bdf);
          lStack_400 = (ulonglong)CONCAT61((int6)((ulonglong)lStack_400 >> 0x10),uStack_79) << 8;
          puStack_408 = puVar2;
          FUN_141609be0(&puStack_278,&puStack_408);
          func_0x000140bd6b00(auStack_420,puVar1);
          func_0x000140bd6b00(auStack_e8,puVar5 + 0x1480);
          lStack_400 = *(undefined8 *)(param_1 + 400);
          uStack_3f8 = uStack_3f8 & 0xffffffffffffff00;
          puStack_408 = puStack_88;
          lStack_78 = param_1;
          FUN_140b2b7a0(&puStack_278,&puStack_408);
          uVar4 = uStack_250;
          uVar8 = uStack_268;
          bVar6 = (byte)puStack_68;
          uVar9 = (ulonglong)puStack_68 & 0xff;
          puStack_68 = puStack_278;
          uStack_98 = uStack_270;
          uStack_a0 = uStack_260;
          uStack_460 = uStack_258;
          uStack_69 = 1;
          uStack_80 = (uint)bVar6;
          FUN_140a28950(&puStack_278,uVar9,lStack_90,lStack_78,&uStack_458,&uStack_438,auStack_420,
                        auStack_e8,uStack_80,lStack_a8);
          puStack_408 = puStack_68;
          lStack_400 = uStack_98;
          uStack_3f8 = uVar8;
          uStack_3f0 = uStack_a0;
          uStack_3e8 = uStack_460;
          uStack_3e0 = uVar4;
          uStack_69 = 0;
          FUN_14160a2e0(&puStack_278,&puStack_408);
          puStack_68 = (undefined *)CONCAT44((int)((ulonglong)puStack_68 >> 0x20),2);
                    /* WARNING: Could not recover jumptable at 0x0001409886c0. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)(&UNK_142ab93a0 + _UNK_142ab93a0))(0,&UNK_142ab93a0 + _UNK_142ab93a0,puStack_88)
          ;
          return;
        }
      }
      goto code_r0x00014098965b;
    }
  }
  FUN_1428d9310(&UNK_142ab8bc0);
code_r0x00014098965b:
  FUN_1428d9310(&UNK_142aab1c8);
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

