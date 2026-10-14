
void FUN_140a32770(undefined8 param_1,longlong param_2)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  ulonglong *puVar7;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uStack_46c;
  undefined4 *puStack_468;
  undefined8 uStack_460;
  undefined4 uStack_458;
  undefined4 uStack_454;
  undefined4 uStack_450;
  undefined4 uStack_44c;
  undefined1 (*pauStack_448) [16];
  undefined *puStack_440;
  undefined8 uStack_438;
  undefined8 uStack_430;
  undefined8 uStack_428;
  undefined8 uStack_420;
  undefined8 uStack_418;
  undefined8 uStack_410;
  undefined8 uStack_408;
  undefined4 *puStack_3f8;
  undefined8 uStack_3f0;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined1 (*pauStack_3d8) [16];
  undefined1 (*pauStack_3d0) [16];
  undefined *puStack_3c8;
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
  undefined4 uStack_374;
  undefined4 uStack_370;
  undefined4 uStack_36c;
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
  longlong lStack_240;
  undefined8 uStack_238;
  ulonglong uStack_230;
  undefined1 (*pauStack_228) [16];
  undefined1 (*pauStack_220) [16];
  undefined *puStack_218;
  undefined1 auStack_210 [16];
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
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
  undefined4 *puStack_98;
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
  FUN_140a59890(&pauStack_228,0x15);
  uVar4 = func_0x000140bd5790(&pauStack_228);
  lVar8 = param_2 + 0x38;
  uVar9 = 0x9b126e795fb22adf;
  lVar5 = FUN_1400dd6f0(lVar8,0x9b126e795fb22adf,0xcb7c6a0645e229de);
  if (((lVar5 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar9)) ||
     (uVar9 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar9 * 8), uVar9 == 0)) {
code_r0x000140a334b4:
    FUN_1428d9310(&UNK_142acc358);
  }
  else {
    lVar5 = ~uVar9 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar5) == 0)
    goto code_r0x000140a334b4;
    puVar6 = *(undefined8 **)(*(longlong *)(param_2 + 0x1e0) + lVar5 + 0x10);
    pauStack_228 = (undefined1 (*) [16])*puVar6;
    puStack_218 = *pauStack_228 + puVar6[1] + 1;
    auVar1 = *pauStack_228;
    auStack_210._0_2_ =
         ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3
           | (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 | (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6
           | (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9
           | (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
    pauStack_220 = pauStack_228 + 1;
    auStack_210._8_8_ = puVar6[3];
    FUN_1412fc510(&lStack_240,&pauStack_228,&UNK_142acb8d8);
    uVar9 = 0x8226cc786e679355;
    lVar5 = FUN_1400dd6f0(lVar8,0x8226cc786e679355,0x65cfda6a55a841ac);
    if (((lVar5 != 0) && (uVar9 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar9 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar9 * 8), uVar9 != 0)) {
      lVar5 = ~uVar9 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar5) != 0) {
        puVar6 = (undefined8 *)
                 FUN_140969720(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar5 + 0x10),
                               &UNK_142c2e400);
        if (puVar6 == (undefined8 *)0x0) goto code_r0x000140a32a58;
        pauStack_3d8 = (undefined1 (*) [16])&UNK_142a9defd;
        pauStack_3d0 = (undefined1 (*) [16])0x5e;
        if (puVar6[3] == 0x5e) {
          iVar3 = func_0x000142923bf0(&UNK_142a9defd,puVar6[2],0x5e);
          if (iVar3 == 0) {
            puVar7 = (ulonglong *)(**(code **)(puVar6[1] + 0x18))(*puVar6);
            uVar9 = uStack_230;
            if (puVar7 == (ulonglong *)0x0) goto code_r0x000140a32a58;
            if (4999 < uStack_230) {
              pauStack_228 = (undefined1 (*) [16])&UNK_142acc3b8;
              pauStack_220 = (undefined1 (*) [16])0x1;
              puStack_218 = (undefined *)0x8;
              auStack_210 = (undefined1  [16])0x0;
              FUN_1428d9390(&pauStack_228,&UNK_142acc3c8);
              goto code_r0x000140a335d5;
            }
            uVar10 = 0x2ed189581b92decf;
            lVar8 = FUN_1400dd6f0(lVar8,0x2ed189581b92decf,0x15d3d99396065f38);
            if (((lVar8 != 0) && (uVar10 < *(ulonglong *)(param_2 + 0x218))) &&
               (uVar10 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar10 * 8), uVar10 != 0))
            {
              lVar8 = ~uVar10 * 0x60;
              if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
                if (uVar9 == 0) {
code_r0x000140a32a55:
                  *puVar7 = uVar9;
code_r0x000140a32a58:
                  FUN_1416032a0(&pauStack_228,&UNK_142acc410,0x13,&UNK_142acc423,0x1c,param_2,
                                param_1);
                  FUN_14160cba0(&pauStack_3d8,&pauStack_228,0x4a99c8969e0b36cd);
                  if (cStack_2a2 != '\b') {
                    FUN_14096b570(auStack_2c8,lStack_2a0 + 0xe8,0xce4cfd9dd902baae,uStack_230);
                  }
                  func_0x000142923c00(&pauStack_228,&pauStack_3d8,400);
                  puStack_98._0_1_ = 0x15;
                  pauStack_3d8 = (undefined1 (*) [16])CONCAT44(pauStack_3d8._4_4_,2);
                  FUN_140905260(&puStack_98,&pauStack_228,0xbdf6ed93e1e613b8,0x32,&pauStack_3d8);
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  puStack_98 = (undefined4 *)CONCAT71(puStack_98._1_7_,0x16);
                  pauStack_228 = (undefined1 (*) [16])CONCAT44(pauStack_228._4_4_,2);
                  FUN_140905260(&puStack_98,&pauStack_3d8,0xb0a93c48d9f3337e,0x32,&pauStack_228);
                  func_0x000142923c00(&pauStack_228,&pauStack_3d8,400);
                  uStack_78 = uStack_1a8;
                  uStack_88 = uStack_1b8;
                  uStack_80 = uStack_1b0;
                  puStack_3c8 = puStack_218;
                  pauStack_3d8 = pauStack_228;
                  pauStack_3d0 = pauStack_220;
                  uStack_3a8 = auStack_210._0_8_;
                  uStack_3a0 = auStack_210._8_8_;
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
                  uStack_390 = uStack_1e0;
                  uStack_388 = uStack_1d8;
                  uStack_2f8 = uStack_a8;
                  uStack_300 = lStack_f0;
                  uStack_378 = 1;
                  uVar11 = (undefined4)uVar4;
                  uVar12 = (undefined4)((ulonglong)uVar4 >> 0x20);
                  uStack_36c = 1;
                  uStack_374 = uVar11;
                  uStack_370 = uVar12;
                  FUN_1409dd4d0(&puStack_98,&pauStack_3d8);
                  if (cStack_f2 != '\b') {
                    FUN_140be24b0(auStack_108);
                  }
                  FUN_140a59890(&pauStack_3d8,0x15);
                  func_0x000140bd52a0(&puStack_3f8,0x20000000200,
                                      (ulonglong)pauStack_3d0 & 0xffffffff);
                  puStack_98 = puStack_3f8;
                  uStack_90 = uStack_3f0;
                  func_0x000140bd6ce0(&puStack_218,&puStack_98);
                  pauStack_228 = (undefined1 (*) [16])&UNK_142acc43f;
                  pauStack_220 = (undefined1 (*) [16])0x13;
                  uStack_46c = FUN_140be94d0(param_1,&pauStack_228);
                  puStack_468 = puStack_3f8;
                  uStack_460 = uStack_3f0;
                  uStack_458 = uStack_3e8;
                  uStack_454 = uStack_3e4;
                  uStack_450 = uStack_3e0;
                  uStack_44c = uStack_3dc;
                  FUN_1416032a0(&pauStack_228,&UNK_142acc452,0x13,&UNK_142acc465,0x21,param_2,
                                param_1);
                  if (cStack_f2 != '\b') {
                    FUN_14096b850(auStack_118,lStack_f0 + 0xe8,0xd91ff8372094bc22,1);
                  }
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  puStack_98 = (undefined4 *)CONCAT71(puStack_98._1_7_,0x15);
                  pauStack_228 = (undefined1 (*) [16])CONCAT44(pauStack_228._4_4_,2);
                  FUN_140905260(&puStack_98,&pauStack_3d8,0x2de87fdb66740ece,0x19,&pauStack_228);
                  func_0x000142923c00(&pauStack_228,&pauStack_3d8,400);
                  pauStack_3d8 = (undefined1 (*) [16])CONCAT44(pauStack_3d8._4_4_,2);
                  puStack_98 = &uStack_46c;
                  FUN_1408edd00(&puStack_98,&pauStack_228,0x960d27b7b6f81d59,0x32,&pauStack_3d8);
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  uStack_58 = CONCAT44(uStack_334,uStack_338);
                  uStack_50 = CONCAT44(uStack_32c,uStack_330);
                  uStack_68 = CONCAT44(uStack_344,uStack_348);
                  uStack_60 = CONCAT44(uStack_33c,uStack_340);
                  uStack_70 = CONCAT44(uStack_34c,uStack_350);
                  uStack_78 = uStack_358;
                  uStack_88 = uStack_368;
                  uStack_80 = uStack_360;
                  puStack_98 = (undefined4 *)CONCAT44(uStack_374,uStack_378);
                  uStack_90 = CONCAT44(uStack_36c,uStack_370);
                  puStack_218 = puStack_3c8;
                  pauStack_228 = pauStack_3d8;
                  pauStack_220 = pauStack_3d0;
                  uStack_1f8 = uStack_3c0;
                  uStack_1f0 = uStack_3b8;
                  uStack_1e8 = uStack_3b0;
                  auStack_210._8_8_ = uStack_3a0;
                  auStack_210._0_8_ = uStack_3a8;
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
                  uStack_1e0 = uStack_390;
                  uStack_1d8 = uStack_388;
                  uStack_148 = uStack_258;
                  uStack_150 = lStack_2a0;
                  uStack_1c8 = 1;
                  uStack_1bc = 1;
                  uStack_1c4 = uVar11;
                  uStack_1c0 = uVar12;
                  FUN_1409dd4d0(&puStack_98,&pauStack_228);
                  if (cStack_2a2 != '\b') {
                    FUN_140be24b0(auStack_2b8);
                  }
                  FUN_1416032a0(&pauStack_228,&UNK_142acc452,0x13,&UNK_142acc465,0x21,param_2,
                                param_1);
                  if (cStack_f2 != '\b') {
                    FUN_14096b850(auStack_118,lStack_f0 + 0xe8,0xd91ff8372094bc22,0);
                  }
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  pauStack_228 = (undefined1 (*) [16])CONCAT44(pauStack_228._4_4_,2);
                  puStack_98 = &uStack_46c;
                  FUN_1408ee240(&puStack_98,&pauStack_3d8,0x2de87fdb66740ece,0x19,&pauStack_228);
                  func_0x000142923c00(&pauStack_228,&pauStack_3d8,400);
                  puStack_98 = (undefined4 *)CONCAT71(puStack_98._1_7_,0x15);
                  pauStack_3d8 = (undefined1 (*) [16])CONCAT44(pauStack_3d8._4_4_,2);
                  FUN_140905260(&puStack_98,&pauStack_228,0x960d27b7b6f81d59,0x32,&pauStack_3d8);
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  uStack_58 = CONCAT44(uStack_334,uStack_338);
                  uStack_50 = CONCAT44(uStack_32c,uStack_330);
                  uStack_68 = CONCAT44(uStack_344,uStack_348);
                  uStack_60 = CONCAT44(uStack_33c,uStack_340);
                  uStack_70 = CONCAT44(uStack_34c,uStack_350);
                  uStack_78 = uStack_358;
                  uStack_88 = uStack_368;
                  uStack_80 = uStack_360;
                  puStack_98 = (undefined4 *)CONCAT44(uStack_374,uStack_378);
                  uStack_90 = CONCAT44(uStack_36c,uStack_370);
                  puStack_218 = puStack_3c8;
                  pauStack_228 = pauStack_3d8;
                  pauStack_220 = pauStack_3d0;
                  uStack_1f8 = uStack_3c0;
                  uStack_1f0 = uStack_3b8;
                  uStack_1e8 = uStack_3b0;
                  auStack_210._8_8_ = uStack_3a0;
                  auStack_210._0_8_ = uStack_3a8;
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
                  uStack_1e0 = uStack_390;
                  uStack_1d8 = uStack_388;
                  uStack_148 = uStack_258;
                  uStack_150 = lStack_2a0;
                  uStack_1c8 = 1;
                  uStack_1bc = 1;
                  uStack_1c4 = uVar11;
                  uStack_1c0 = uVar12;
                  FUN_1409dd4d0(&puStack_98,&pauStack_228);
                  if (cStack_2a2 != '\b') {
                    FUN_140be24b0(auStack_2b8);
                  }
                  FUN_1416032a0(&pauStack_228,&UNK_142acc486,0x16,&UNK_142acc49c,0x24,param_2,
                                param_1);
                  puStack_98 = (undefined4 *)CONCAT71(puStack_98._1_7_,0x15);
                  pauStack_3d8 = (undefined1 (*) [16])CONCAT44(pauStack_3d8._4_4_,2);
                  FUN_140905260(&puStack_98,&pauStack_228,0xbdf6ed93e1e613b8,0x32,&pauStack_3d8);
                  func_0x000142923c00(&pauStack_3d8,&pauStack_228,400);
                  uStack_58 = CONCAT44(uStack_334,uStack_338);
                  uStack_50 = CONCAT44(uStack_32c,uStack_330);
                  uStack_68 = CONCAT44(uStack_344,uStack_348);
                  uStack_60 = CONCAT44(uStack_33c,uStack_340);
                  uStack_70 = CONCAT44(uStack_34c,uStack_350);
                  uStack_78 = uStack_358;
                  uStack_88 = uStack_368;
                  uStack_80 = uStack_360;
                  puStack_98 = (undefined4 *)CONCAT44(uStack_374,uStack_378);
                  uStack_90 = CONCAT44(uStack_36c,uStack_370);
                  puStack_218 = puStack_3c8;
                  pauStack_228 = pauStack_3d8;
                  pauStack_220 = pauStack_3d0;
                  uStack_1f8 = uStack_3c0;
                  uStack_1f0 = uStack_3b8;
                  uStack_1e8 = uStack_3b0;
                  auStack_210._8_8_ = uStack_3a0;
                  auStack_210._0_8_ = uStack_3a8;
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
                  uStack_1e0 = uStack_390;
                  uStack_1d8 = uStack_388;
                  uStack_148 = uStack_258;
                  uStack_150 = lStack_2a0;
                  uStack_1c8 = 1;
                  uStack_1bc = 1;
                  uStack_1c4 = uVar11;
                  uStack_1c0 = uVar12;
                  FUN_1409dd4d0(&puStack_98,&pauStack_228);
                  if (cStack_2a2 != '\b') {
                    FUN_140be24b0(auStack_2b8);
                  }
                  if (lStack_240 != 0) {
                    func_0x000140613c20(uStack_238,lStack_240 << 5,4);
                  }
                  return;
                }
                uVar10 = uVar9 << 5;
                if (puVar7[4] < uVar10) {
                  pauStack_228 = (undefined1 (*) [16])0xb;
                }
                else {
                  pauStack_3d8 = (undefined1 (*) [16])0x0;
                  pauStack_3d0 = (undefined1 (*) [16])uVar10;
                  FUN_140beefa0(&pauStack_228,
                                **(longlong **)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10) +
                                0x10,puVar7 + 1,uStack_238,uVar10,&pauStack_3d8,1,0);
                  if (pauStack_228 == (undefined1 (*) [16])0xb) goto code_r0x000140a32a55;
                  uStack_408 = uStack_1e0;
                  uStack_418 = uStack_1f0;
                  uStack_410 = uStack_1e8;
                  uStack_428 = uStack_200;
                  uStack_420 = uStack_1f8;
                  uStack_438 = auStack_210._0_8_;
                  uStack_430 = auStack_210._8_8_;
                  pauStack_448 = pauStack_220;
                  puStack_440 = puStack_218;
                }
                pauStack_220 = pauStack_448;
                puStack_218 = puStack_440;
                auStack_210._8_4_ = (undefined4)uStack_430;
                auStack_210._0_8_ = uStack_438;
                auStack_210._12_4_ = uStack_430._4_4_;
                uStack_200 = uStack_428;
                uStack_1f8 = uStack_420;
                uStack_1f0 = uStack_418;
                uStack_1e8 = uStack_410;
                uStack_1e0 = uStack_408;
                FUN_1428d9760(&UNK_142acb938,0x2b,&pauStack_228,&UNK_142acb918,&UNK_142acc3f8);
                goto code_r0x000140a335d5;
              }
            }
            FUN_1428d9310(&UNK_142acc3e0);
            goto code_r0x000140a335d5;
          }
        }
        pauStack_228 = (undefined1 (*) [16])0x0;
        FUN_1428cab41(0,&pauStack_3d8,puVar6 + 2,&pauStack_228,&UNK_142c2e180);
        goto code_r0x000140a335d5;
      }
    }
  }
  FUN_1428d9310(&UNK_142acc370);
code_r0x000140a335d5:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

