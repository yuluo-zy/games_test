
undefined8 *
FUN_14054aa00(undefined8 *param_1,undefined4 *param_2,undefined8 param_3,undefined8 param_4)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined8 uStack_5c8;
  undefined8 uStack_5c0;
  undefined8 uStack_5b8;
  undefined8 uStack_5b0;
  undefined4 uStack_388;
  undefined4 uStack_384;
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
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_100;
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
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined1 uStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uStack_68 = *param_2;
  uStack_64 = param_2[1];
  uStack_60 = param_2[2];
  uStack_5c = param_2[3];
  uStack_58 = param_2[4];
  uStack_54 = param_2[5];
  uStack_50 = param_2[6];
  uStack_4c = param_2[7];
  uStack_338 = CONCAT71(uStack_338._1_7_,1);
  uStack_29 = 1;
  uStack_78 = param_3;
  uStack_70 = param_4;
  uVar2 = FUN_140d76ed0(&uStack_68);
  uStack_29 = 1;
  lVar3 = FUN_140d5a150(&uStack_338,uVar2,param_2);
  if (lVar3 != 0) {
    uStack_29 = 1;
    uVar2 = FUN_1428a4000(lVar3);
    param_1[1] = uVar2;
    *param_1 = 2;
    if (CONCAT44(uStack_64,uStack_68) != 0) {
      func_0x000140613c20(CONCAT44(uStack_5c,uStack_60),CONCAT44(uStack_64,uStack_68),1);
    }
    FUN_1405d9ae0(&uStack_78);
    return param_1;
  }
  uStack_38 = CONCAT44(uStack_64,uStack_68);
  uStack_40 = CONCAT44(uStack_5c,uStack_60);
  uStack_a8 = uStack_58;
  uStack_a4 = uStack_54;
  uStack_a0 = uStack_50;
  uStack_9c = uStack_4c;
  FUN_140718a00(&uStack_338);
  uStack_b8 = 0x3c;
  uStack_b0 = 0;
  func_0x000142923c00(&uStack_5c8,&uStack_338,0x290);
  uStack_b0 = uStack_340;
  uStack_ac = uStack_33c;
  func_0x000142923c00(&uStack_338,&uStack_5c8,0x238);
  uStack_100 = 0;
  uStack_f8 = uStack_388;
  uStack_f4 = uStack_384;
  uStack_f0 = uStack_380;
  uStack_ec = uStack_37c;
  uStack_e8 = uStack_378;
  uStack_e4 = uStack_374;
  uStack_e0 = uStack_370;
  uStack_dc = uStack_36c;
  uStack_d8 = uStack_368;
  uStack_d4 = uStack_364;
  uStack_d0 = uStack_360;
  uStack_cc = uStack_35c;
  uStack_c8 = uStack_358;
  uStack_c4 = uStack_354;
  uStack_c0 = uStack_350;
  uStack_bc = uStack_34c;
  FUN_140718160(&uStack_5c8,&uStack_338);
  if ((int)uStack_5b8 == 0x3b9aca01) {
    uStack_338 = uStack_5c8;
    FUN_1428d9760(&UNK_1429f8458,0x2b,&uStack_338,&UNK_1429f8438,&UNK_1429f83a0);
  }
  else {
    uStack_88 = uStack_5b8;
    uStack_98 = (undefined4)uStack_5c8;
    uStack_94 = uStack_5c8._4_4_;
    uStack_90 = (undefined4)uStack_5c0;
    uStack_8c = uStack_5c0._4_4_;
    uStack_5c0 = 0;
    uStack_5b8 = 1;
    uStack_5b0 = 0;
    uStack_5c8 = 1;
    uStack_338 = 1;
    uStack_330 = 1;
    uStack_328 = 0;
    uStack_320 = 8;
    uStack_318 = 0;
    puVar4 = (undefined8 *)func_0x000140613c10(0x28,8);
    if (puVar4 != (undefined8 *)0x0) {
      puVar4[4] = uStack_318;
      puVar4[2] = uStack_328;
      puVar4[3] = uStack_320;
      *puVar4 = uStack_338;
      puVar4[1] = uStack_330;
      param_1[6] = uStack_38;
      param_1[7] = uStack_40;
      param_1[8] = CONCAT44(uStack_a4,uStack_a8);
      param_1[9] = CONCAT44(uStack_9c,uStack_a0);
      param_1[0x13] = CONCAT44(uStack_94,uStack_98);
      param_1[0x14] = CONCAT44(uStack_8c,uStack_90);
      param_1[0x15] = uStack_88;
      param_1[10] = 0;
      param_1[0xb] = 1;
      param_1[0xc] = 0;
      *param_1 = uStack_5c8;
      param_1[1] = uStack_5c0;
      param_1[2] = uStack_5b8;
      param_1[3] = uStack_5b0;
      param_1[4] = uStack_78;
      param_1[5] = uStack_70;
      param_1[0xd] = 0x8000000000000000;
      param_1[0xe] = 0x8000000000000000;
      param_1[0x16] = 0;
      param_1[0x17] = puVar4 + 2;
      return param_1;
    }
    FUN_1428d9000(8,0x28);
  }
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

