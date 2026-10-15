
undefined8
FUN_1405876a0(undefined8 param_1,undefined8 param_2,undefined8 param_3,longlong *param_4,
             undefined8 param_5,longlong param_6)

{
  undefined4 *puVar1;
  char cVar2;
  code *pcVar3;
  undefined8 uVar4;
  longlong *plVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  char *pcVar9;
  undefined8 *puVar10;
  undefined1 uVar11;
  longlong *plVar12;
  bool bVar13;
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
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined4 uStack_318;
  undefined4 uStack_314;
  longlong lStack_310;
  undefined4 uStack_308;
  undefined4 uStack_304;
  longlong *plStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  longlong lStack_280;
  longlong lStack_278;
  longlong lStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  longlong lStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
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
  undefined8 uStack_190;
  undefined6 uStack_188;
  undefined2 uStack_182;
  undefined6 uStack_180;
  undefined1 uStack_17a;
  undefined1 uStack_179;
  longlong *plStack_178;
  undefined8 uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined2 uStack_cc;
  undefined2 uStack_ca;
  undefined6 uStack_c8;
  longlong *plStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined2 uStack_9c;
  undefined2 uStack_9a;
  undefined6 uStack_98;
  undefined1 uStack_92;
  byte bStack_91;
  longlong *plStack_90;
  undefined8 uStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong *plStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  plStack_60 = param_4;
  FUN_1403abd10(&uStack_3b0,param_6 + 0x110);
  plStack_2b0 = (longlong *)plStack_60[1];
  uStack_2a8 = plStack_60[2];
  uStack_2a0 = 1;
  plStack_c0 = plStack_2b0;
  uStack_88 = FUN_1405bdda0(&plStack_2b0);
  FUN_140909310(&uStack_120,param_5);
  lVar6 = FUN_140589450(param_5);
  if (lVar6 == 0) {
    FUN_1428d9310(&UNK_1429fca88);
  }
  else {
    uVar7 = func_0x000140987ca0(lVar6);
    uVar4 = CONCAT44(uStack_11c,uStack_120);
    puVar10 = &uStack_88;
    uVar8 = FUN_1409378e0(uVar4);
    if (puVar10 == (undefined8 *)0x0) {
      FUN_140056d30(&plStack_2b0,plStack_60);
      FUN_140937e10(&uStack_350,param_2,param_3,&plStack_2b0,uVar7);
      uStack_49 = 1;
      lVar6 = func_0x00014067dd80(&uStack_308);
      uStack_b8 = *(longlong **)(lVar6 + 0x18);
      LOCK();
      lVar6 = *uStack_b8;
      *uStack_b8 = *uStack_b8 + 1;
      UNLOCK();
      if (*uStack_b8 != 0 && SCARRY8(lVar6,1) == *uStack_b8 < 0) {
        plStack_58 = uStack_b8;
        func_0x000142923c00(&plStack_2b0,&uStack_350,0xa0);
        uVar8 = FUN_140937560(uVar4,&plStack_2b0,uStack_88);
        plVar12 = plStack_58;
        goto code_r0x00014058782e;
      }
    }
    else {
      lVar6 = func_0x00014067dd80(puVar10 + 9);
      plVar12 = *(longlong **)(lVar6 + 0x18);
      LOCK();
      lVar6 = *plVar12;
      *plVar12 = *plVar12 + 1;
      UNLOCK();
      if (*plVar12 != 0 && SCARRY8(lVar6,1) == *plVar12 < 0) {
code_r0x00014058782e:
        plStack_90 = plVar12;
        plStack_68 = plVar12;
        uStack_318 = FUN_140bd4b40(*(undefined8 *)(param_6 + 0xa0));
        lStack_310 = param_6;
        uStack_350 = 0;
        uStack_348 = 4;
        uStack_340 = 0;
        uStack_328 = 0;
        lStack_80 = 0;
        lStack_78 = 8;
        lStack_70 = 0;
        uStack_338 = param_2;
        uStack_330 = param_3;
        uStack_308 = uStack_318;
        pcVar9 = (char *)func_0x00014067dd80(CONCAT44(uStack_f4,uStack_f8));
        LOCK();
        cVar2 = *pcVar9;
        if (cVar2 == '\0') {
          *pcVar9 = '\x01';
        }
        UNLOCK();
        if (cVar2 != '\0') {
          FUN_1428cd210(pcVar9);
        }
        FUN_140c81c70(pcVar9);
        plStack_58 = (longlong *)pcVar9;
        FUN_14096b420(&plStack_2b0,uVar7,plVar12 + 2,pcVar9 + 8);
        plVar12 = plStack_58;
        lVar6 = lStack_70;
        if ((int)lStack_280 == 10) {
          FUN_140c81e10();
          LOCK();
          bVar13 = (char)*plVar12 == '\x01';
          if (bVar13) {
            *(char *)plVar12 = '\0';
          }
          UNLOCK();
          if (!bVar13) {
            FUN_1428cdb90(plVar12,0);
          }
          uVar11 = 8;
        }
        else {
          uStack_98 = (undefined6)uStack_290;
          uStack_92 = (undefined1)((ulonglong)uStack_290 >> 0x30);
          bStack_91 = (byte)((ulonglong)uStack_290 >> 0x38);
          uStack_a8 = (undefined4)uStack_2a0;
          uStack_a4 = uStack_2a0._4_4_;
          uStack_a0 = (undefined4)uStack_298;
          uStack_9c = (undefined2)((ulonglong)uStack_298 >> 0x20);
          uStack_9a = (undefined2)((ulonglong)uStack_298 >> 0x30);
          uStack_b8 = plStack_2b0;
          uStack_b0 = (undefined4)uStack_2a8;
          uStack_ac = uStack_2a8._4_4_;
          if (lStack_70 == lStack_80) {
            FUN_140b5f9f0(&lStack_80,&UNK_1429fcaa0);
          }
          plVar5 = plStack_58;
          uVar11 = uStack_92;
          *(undefined8 *)(lStack_78 + 0x40 + lVar6 * 0x48) = uStack_248;
          puVar1 = (undefined4 *)(lStack_78 + 0x30 + lVar6 * 0x48);
          *puVar1 = (undefined4)uStack_258;
          puVar1[1] = uStack_258._4_4_;
          puVar1[2] = (undefined4)uStack_250;
          puVar1[3] = uStack_250._4_4_;
          puVar1 = (undefined4 *)(lStack_78 + 0x20 + lVar6 * 0x48);
          *puVar1 = (undefined4)uStack_268;
          puVar1[1] = uStack_268._4_4_;
          puVar1[2] = (undefined4)uStack_260;
          puVar1[3] = uStack_260._4_4_;
          puVar10 = (undefined8 *)(lStack_78 + 0x10 + lVar6 * 0x48);
          *puVar10 = lStack_278;
          puVar10[1] = lStack_270;
          puVar10 = (undefined8 *)(lStack_78 + lVar6 * 0x48);
          *puVar10 = uStack_288;
          puVar10[1] = lStack_280;
          lStack_70 = lVar6 + 1;
          uStack_e8 = (undefined4)uStack_b8;
          uStack_e4 = uStack_b8._4_4_;
          uStack_e0 = uStack_b0;
          uStack_dc = uStack_ac;
          uStack_d8 = uStack_a8;
          uStack_d4 = uStack_a4;
          uStack_d0 = uStack_a0;
          uStack_cc = uStack_9c;
          uStack_ca = uStack_9a;
          uStack_c8 = uStack_98;
          plVar12 = (longlong *)(ulonglong)bStack_91;
          FUN_140c81e10();
          LOCK();
          bVar13 = (char)*plVar5 == '\x01';
          if (bVar13) {
            *(char *)plVar5 = '\0';
          }
          UNLOCK();
          if (!bVar13) {
            FUN_1428cdb90(plVar5,0);
          }
        }
        uStack_208 = CONCAT44(uStack_304,uStack_308);
        lStack_210 = lStack_310;
        uStack_218 = CONCAT44(uStack_314,uStack_318);
        uStack_220 = uStack_320;
        uStack_230 = uStack_330;
        uStack_228 = uStack_328;
        uStack_240 = uStack_340;
        uStack_238 = uStack_338;
        uStack_250 = uStack_350;
        uStack_248 = uStack_348;
        uStack_1a0 = CONCAT44(uStack_e4,uStack_e8);
        uStack_198 = CONCAT44(uStack_dc,uStack_e0);
        uStack_190 = CONCAT44(uStack_d4,uStack_d8);
        uStack_188 = CONCAT24(uStack_cc,uStack_d0);
        uStack_182 = uStack_ca;
        uStack_180 = uStack_c8;
        lStack_270 = lStack_70;
        lStack_280 = lStack_80;
        lStack_278 = lStack_78;
        uStack_1b0 = uStack_360;
        uStack_1a8 = uStack_358;
        uStack_1c0 = uStack_370;
        uStack_1b8 = uStack_368;
        uStack_1d0 = uStack_380;
        uStack_1cc = uStack_37c;
        uStack_1c8 = uStack_378;
        uStack_1c4 = uStack_374;
        uStack_1e0 = uStack_390;
        uStack_1d8 = uStack_388;
        uStack_1f0 = uStack_3a0;
        uStack_1e8 = uStack_398;
        uStack_200 = uStack_3b0;
        uStack_1f8 = uStack_3a8;
        plStack_178 = plStack_68;
        uStack_179 = SUB81(plVar12,0);
        plStack_2b0 = (longlong *)0x0;
        uStack_2a8 = 4;
        uStack_2a0 = 0;
        uStack_298 = 0;
        uStack_290 = 4;
        uStack_288 = 0;
        uStack_268 = 0;
        uStack_260 = 1;
        uStack_258 = 0;
        uStack_128 = param_5;
        uStack_168 = uStack_120;
        uStack_164 = uStack_11c;
        uStack_160 = uStack_118;
        uStack_15c = uStack_114;
        uStack_158 = uStack_110;
        uStack_154 = uStack_10c;
        uStack_150 = uStack_108;
        uStack_14c = uStack_104;
        uStack_148 = uStack_100;
        uStack_144 = uStack_fc;
        uStack_140 = uStack_f8;
        uStack_13c = uStack_f4;
        uStack_138 = uStack_f0;
        uStack_17a = uVar11;
        uStack_170 = uVar7;
        uStack_130 = uVar8;
        if (*plStack_60 != 0) {
          func_0x000140613c20(plStack_c0,*plStack_60,1);
        }
        FUN_1405896e0(param_1,&plStack_2b0);
        return param_1;
      }
    }
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

