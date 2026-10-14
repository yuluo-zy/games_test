
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140a5a570(undefined8 *param_1,undefined8 param_2,longlong param_3,longlong param_4,
             undefined8 param_5,longlong param_6)

{
  undefined4 uVar1;
  ulonglong uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  uint uVar7;
  undefined8 uStack_6c8;
  undefined8 uStack_6c0;
  undefined8 uStack_6b8;
  undefined8 uStack_6b0;
  undefined8 uStack_6a8;
  undefined8 uStack_6a0;
  undefined8 uStack_698;
  undefined8 uStack_690;
  undefined8 uStack_688;
  undefined8 uStack_680;
  undefined4 uStack_4e8;
  undefined4 uStack_4e4;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined4 uStack_4d8;
  undefined4 uStack_4d4;
  undefined4 uStack_4d0;
  undefined4 uStack_4cc;
  undefined4 uStack_4c8;
  undefined8 uStack_408;
  undefined8 uStack_400;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined4 uStack_3c0;
  undefined4 uStack_3bc;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
  undefined8 uStack_3b0;
  undefined4 uStack_3a8;
  undefined4 uStack_3a4;
  undefined4 uStack_3a0;
  undefined4 uStack_39c;
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
  undefined1 auStack_2f8 [38];
  char cStack_2d2;
  longlong lStack_2d0;
  int iStack_260;
  int iStack_25c;
  undefined *puStack_248;
  undefined8 uStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
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
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined1 auStack_128 [22];
  char cStack_112;
  undefined8 uStack_110;
  undefined8 uStack_c8;
  undefined8 *puStack_b0;
  longlong lStack_78;
  undefined8 uStack_70;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  puStack_b0 = param_1;
  if (cRam000000014362c080 != '\0') {
    lStack_78 = param_3;
    func_0x000140bd5790(param_4 + 4);
    puStack_248 = *(undefined **)(param_4 + 4);
    uStack_240 = *(undefined8 *)(param_4 + 0xc);
    uStack_238 = *(undefined4 *)(param_4 + 0x14);
    uStack_234 = *(undefined4 *)(param_4 + 0x18);
    uStack_230 = *(undefined4 *)(param_4 + 0x1c);
    uStack_22c = *(undefined4 *)(param_4 + 0x20);
    func_0x000140bd5730(&uStack_408,&puStack_248);
    func_0x000140bd5790(&uStack_408);
    puStack_248 = *(undefined **)(param_6 + 0x28);
    uStack_240 = *(undefined8 *)(param_6 + 0x30);
    uStack_238 = *(undefined4 *)(param_6 + 0x38);
    uStack_234 = *(undefined4 *)(param_6 + 0x3c);
    uStack_230 = *(undefined4 *)(param_6 + 0x40);
    uStack_22c = *(undefined4 *)(param_6 + 0x44);
    func_0x000140bd5430(&uStack_6c8,&puStack_248,0x4c);
    func_0x000140bd6ce0(&uStack_238,&uStack_408);
    puStack_248 = &UNK_142ad3218;
    uStack_240 = 0xb;
    uStack_70 = param_2;
    FUN_140be94d0(param_2,&puStack_248);
    uVar2 = func_0x000140bd5790(param_6 + 0x28);
    uVar7 = (uint)(uVar2 >> 0x20);
    iStack_260 = (uVar7 - (uVar7 >> 1)) * ((int)uVar2 - ((uint)(uVar2 >> 1) & 0x7fffffff));
    iStack_25c = uVar7 * (int)uVar2;
                    /* WARNING: Could not recover jumptable at 0x000140a5a70a. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar3 = (undefined8 *)
             (*(code *)(&UNK_142ad3614 + _UNK_142ad3614))
                       (CONCAT71((uint7)(uint3)((uint)iStack_25c >> 8),2),
                        &UNK_142ad3614 + _UNK_142ad3614);
    return puVar3;
  }
  uVar4 = func_0x000140bd5790(param_4 + 4);
  uVar5 = func_0x000140bd5790(param_4 + 4);
  func_0x000140bd52a0(&uStack_6c8,uVar5,0x7a);
  uStack_408._0_4_ = SUB84(uStack_6c8,0);
  uStack_408._4_4_ = uStack_6c8._4_4_;
  func_0x000140bd6ce0(&uStack_238,&uStack_408);
  puStack_248 = &UNK_142ad3193;
  uStack_240 = 10;
  uStack_4e8 = FUN_140be94d0(param_2,&puStack_248);
  uStack_4e4 = (undefined4)uStack_6c8;
  uStack_4e0 = uStack_6c8._4_4_;
  uStack_4dc = (undefined4)uStack_6c0;
  uStack_4d8 = uStack_6c0._4_4_;
  uStack_4d4 = (undefined4)uStack_6b8;
  uStack_4d0 = uStack_6b8._4_4_;
  uStack_4cc = (undefined4)uStack_6b0;
  uStack_4c8 = uStack_6b0._4_4_;
  uVar2 = 0x4a142e09642375b9;
  lVar6 = FUN_1400dd6f0(param_3 + 0x38,0x4a142e09642375b9,0x43f6154cfa8c847f);
  if (((lVar6 != 0) && (uVar2 < *(ulonglong *)(param_3 + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar6 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar6) != 0) {
      lVar6 = *(longlong *)(*(longlong *)(param_3 + 0x1e0) + lVar6 + 0x10);
      uVar1 = *(undefined4 *)(lVar6 + 0x30 + (ulonglong)*(byte *)(lVar6 + 0xc0) * 0x40);
      FUN_1416032a0(&puStack_248,&UNK_142ad31f8,0xe,&UNK_142ad3206,0x12,param_3,param_2);
      uStack_408._0_4_ = 2;
      uStack_6c8 = (undefined4 *)param_4;
      FUN_1408ee240(&uStack_6c8,&puStack_248,0xd9978e541ce9a95e,0x19,&uStack_408);
      func_0x000142923c00(&uStack_408,&puStack_248,400);
      uStack_6c8 = (undefined4 *)param_6;
      puStack_248._0_4_ = 2;
      FUN_1408ee240(&uStack_6c8,&uStack_408,0xb90c1f7f93500b5a,0x19,&puStack_248);
      func_0x000142923c00(&puStack_248,&uStack_408,400);
      uStack_6c8 = (undefined4 *)param_5;
      uStack_408._0_4_ = 2;
      FUN_1408ee240(&uStack_6c8,&puStack_248,0x4861579da049fbfc,0x19,&uStack_408);
      func_0x000142923c00(&uStack_408,&puStack_248,400);
      uStack_6c8 = (undefined4 *)0x579f39ab91a01865;
      puStack_248 = (undefined *)CONCAT44(puStack_248._4_4_,2);
      FUN_1409056d0(&uStack_6c8,&uStack_408,0x815bf5036966978c,0x19,&puStack_248);
      func_0x000142923c00(&puStack_248,&uStack_408,400);
      uStack_6c8 = &uStack_4e8;
      uStack_408 = (undefined *)CONCAT44(uStack_408._4_4_,2);
      FUN_1408edd00(&uStack_6c8,&puStack_248,0xb981790202418bc8,0x32,&uStack_408);
      func_0x000142923c00(&uStack_408,&puStack_248,400);
      if (cStack_2d2 != '\b') {
        FUN_14096ba30(auStack_2f8,lStack_2d0 + 0xe8,0xb34ee0b4478708ab,uVar1);
      }
      func_0x000142923c00(&puStack_248,&uStack_408,400);
      uStack_688 = uStack_1a8;
      uStack_680 = uStack_1a0;
      uStack_698 = uStack_1b8;
      uStack_690 = uStack_1b0;
      uStack_6a8 = uStack_1c8;
      uStack_6a0 = uStack_1c0;
      uStack_6b8 = uStack_1d8;
      uStack_6b0 = uStack_1d0;
      uStack_6c8 = (undefined4 *)uStack_1e8;
      uStack_6c0 = uStack_1e0;
      uStack_408 = puStack_248;
      uStack_400 = uStack_240;
      uStack_3d0 = uStack_228;
      uStack_3c8 = uStack_220;
      uStack_3f0 = uStack_218;
      uStack_3e8 = uStack_210;
      uStack_3e0 = uStack_208;
      uStack_390 = uStack_198;
      uStack_388 = uStack_190;
      uStack_380 = uStack_188;
      uStack_37c = uStack_184;
      uStack_378 = uStack_180;
      uStack_374 = uStack_17c;
      uStack_370 = uStack_178;
      uStack_36c = uStack_174;
      uStack_368 = uStack_170;
      uStack_364 = uStack_16c;
      uStack_360 = uStack_168;
      uStack_35c = uStack_164;
      uStack_358 = uStack_160;
      uStack_354 = uStack_15c;
      uStack_350 = uStack_158;
      uStack_348 = uStack_150;
      uStack_340 = uStack_148;
      uStack_338 = uStack_140;
      uStack_3b0 = uStack_1f0;
      uStack_3c0 = uStack_200;
      uStack_3bc = uStack_1fc;
      uStack_3b8 = uStack_1f8;
      uStack_3b4 = uStack_1f4;
      uStack_328 = uStack_c8;
      uStack_330 = uStack_110;
      uStack_3a8 = 1;
      uStack_3a4 = (undefined4)uVar4;
      uStack_3a0 = (undefined4)((ulonglong)uVar4 >> 0x20);
      uStack_39c = 1;
      FUN_1409dd4d0(&uStack_6c8,&uStack_408);
      if (cStack_112 != '\b') {
        FUN_140be24b0(auStack_128);
      }
      *(undefined4 *)(puStack_b0 + 4) = uStack_4c8;
      puStack_b0[2] = CONCAT44(uStack_4d4,uStack_4d8);
      puStack_b0[3] = CONCAT44(uStack_4cc,uStack_4d0);
      *puStack_b0 = CONCAT44(uStack_4e4,uStack_4e8);
      puStack_b0[1] = CONCAT44(uStack_4dc,uStack_4e0);
      return puStack_b0;
    }
  }
  puVar3 = (undefined8 *)FUN_1428d9310(&UNK_142ad31e0);
  return puVar3;
}

