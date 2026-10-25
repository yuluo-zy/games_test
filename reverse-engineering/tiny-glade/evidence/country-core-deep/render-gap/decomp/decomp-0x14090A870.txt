
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14090a870(longlong param_1,longlong param_2)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  undefined8 uVar4;
  code *pcVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  undefined1 auStack_2e8 [16];
  undefined4 uStack_2d8;
  undefined1 auStack_2d0 [16];
  undefined4 uStack_2c0;
  undefined1 auStack_2b8 [16];
  undefined4 uStack_2a8;
  undefined1 auStack_2a0 [16];
  undefined4 uStack_290;
  undefined8 uStack_288;
  undefined8 uStack_280;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined8 *puStack_268;
  undefined8 uStack_260;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined8 *puStack_248;
  undefined8 uStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 *puStack_228;
  undefined8 uStack_220;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined *puStack_1f0;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  longlong lStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined *puStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  longlong lStack_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 *puStack_158;
  longlong lStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 *puStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 *puStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  char *pcStack_60;
  undefined1 uStack_52;
  undefined1 uStack_51;
  undefined1 uStack_50;
  undefined1 uStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar12 = 0x47c94cb78283a72e;
  lVar11 = FUN_1400dd6f0(param_2 + 0x38,0x47c94cb78283a72e,0x8c173ded97323e9d);
  if (((lVar11 != 0) && (uVar12 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 != 0)) {
    lVar11 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar11) != 0) {
      pcVar3 = *(char **)(*(longlong *)(param_2 + 0x1e0) + lVar11 + 0x10);
      uVar12 = 0x8226cc786e679355;
      lVar11 = FUN_1400dd6f0(param_2 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
      if (((lVar11 != 0) && (uVar12 < *(ulonglong *)(param_2 + 0x218))) &&
         (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 != 0)) {
        lVar11 = ~uVar12 * 0x60;
        if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar11) != 0) {
          uVar4 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar11 + 0x10);
          FUN_1412d2170(auStack_2a0,uVar4,param_1);
          FUN_1412d5ad0(auStack_2b8,uVar4,param_1);
          FUN_1412d6130(auStack_2d0,uVar4,param_1);
          FUN_1412cb1e0(auStack_2e8,uVar4,param_1);
          uStack_180 = FUN_140bd4b40(*(undefined8 *)(param_1 + 0xa0));
          uStack_1b8 = 0;
          uStack_1b0 = 4;
          uStack_1a8 = 0;
          puStack_1a0 = &UNK_142aaaf60;
          uStack_198 = 0x13;
          uStack_190 = 0;
          LOCK();
          cVar2 = *pcVar3;
          if (cVar2 == '\0') {
            *pcVar3 = '\x01';
          }
          UNLOCK();
          lStack_178 = param_1;
          uStack_170 = uStack_180;
          if (cVar2 != '\0') {
            uStack_52 = 1;
            FUN_1428cd210(pcVar3);
          }
          uStack_52 = 1;
          FUN_140c81c70(pcVar3);
          pcVar1 = pcVar3 + 8;
          uStack_4a = 1;
          uStack_4b = 1;
          pcStack_60 = pcVar3;
          FUN_140c02740(&puStack_158,pcVar1,*(undefined8 *)(param_1 + 400),0x80,0);
          uVar9 = puStack_118._0_4_;
          uVar7 = uStack_140;
          uVar6 = _UNK_142aaa1e8;
          uVar4 = _UNK_142aaa1e0;
          if (lStack_150 == 0x80) {
            *puStack_158 = _UNK_142aaa1e0;
            puStack_158[1] = uVar6;
            puStack_158[2] = uVar4;
            puStack_158[3] = uVar6;
            puStack_158[4] = uVar4;
            puStack_158[5] = uVar6;
            puStack_158[6] = uVar4;
            puStack_158[7] = uVar6;
            puStack_158[8] = uVar4;
            puStack_158[9] = uVar6;
            puStack_158[10] = uVar4;
            puStack_158[0xb] = uVar6;
            puStack_158[0xc] = uVar4;
            puStack_158[0xd] = uVar6;
            puStack_158[0xe] = uVar4;
            puStack_158[0xf] = uVar6;
            puStack_228 = puStack_138;
            uStack_220 = uStack_130;
            uStack_218 = (undefined4)uStack_128;
            uStack_214 = uStack_128._4_4_;
            uStack_210 = (undefined4)uStack_120;
            uStack_20c = uStack_120._4_4_;
            uStack_4c = 1;
            uStack_4d = 1;
            FUN_140c02740(&puStack_158,pcVar1,*(undefined8 *)(param_1 + 400),0x1e0,0);
            if (lStack_150 == 0x1e0) {
              func_0x000142923c00(puStack_158,&UNK_142aaaa78,0x1e0);
              uVar10 = puStack_118._0_4_;
              uVar6 = uStack_140;
              puStack_248 = puStack_138;
              uStack_240 = uStack_130;
              uStack_238 = (undefined4)uStack_128;
              uStack_234 = uStack_128._4_4_;
              uStack_230 = (undefined4)uStack_120;
              uStack_22c = uStack_120._4_4_;
              uStack_4e = 1;
              uStack_4f = 1;
              FUN_140c02740(&puStack_158,pcVar1,*(undefined8 *)(param_1 + 400),0x10,0);
              uVar8 = uStack_140;
              uVar4 = _UNK_142aaaf7c;
              if (lStack_150 == 0x10) {
                *puStack_158 = _UNK_142aaaf74;
                puStack_158[1] = uVar4;
                puStack_268 = puStack_138;
                uStack_260 = uStack_130;
                uStack_258 = (undefined4)uStack_128;
                uStack_254 = uStack_128._4_4_;
                uStack_250 = (undefined4)uStack_120;
                uStack_24c = uStack_120._4_4_;
                uStack_6c = puStack_118._0_4_;
                uStack_50 = 1;
                uStack_51 = 1;
                FUN_140c02740(&puStack_158,pcVar1,*(undefined8 *)(param_1 + 400),8,0);
                if (lStack_150 == 8) {
                  uStack_168 = uVar8;
                  uStack_68 = uVar10;
                  *puStack_158 = 0x9000000000000;
                  uStack_160 = uStack_140;
                  uStack_288 = puStack_138;
                  uStack_280 = uStack_130;
                  uStack_278 = (undefined4)uStack_128;
                  uStack_274 = uStack_128._4_4_;
                  uStack_270 = (undefined4)uStack_120;
                  uStack_26c = uStack_120._4_4_;
                  uStack_64 = puStack_118._0_4_;
                  uStack_49 = 1;
                  FUN_140c81e10();
                  LOCK();
                  cVar2 = *pcStack_60;
                  if (cVar2 == '\x01') {
                    *pcStack_60 = '\0';
                  }
                  UNLOCK();
                  if (cVar2 != '\x01') {
                    uStack_49 = 1;
                    FUN_1428cdb90(pcStack_60,0);
                  }
                  uStack_49 = 1;
                  func_0x00014003bb80(0x34);
                  uStack_49 = 1;
                  FUN_1409dca90(&uStack_1b8,uStack_290,0x34,0);
                  uStack_49 = 1;
                  func_0x00014003bb80(0x34);
                  uStack_49 = 1;
                  FUN_1409dca90(&uStack_1b8,uStack_2a8,0x34,0);
                  uStack_49 = 1;
                  func_0x00014003bb80(0x34);
                  uStack_49 = 1;
                  FUN_1409dca90(&uStack_1b8,uStack_2c0,0x34,0);
                  uStack_49 = 1;
                  func_0x00014003bb80(0x34);
                  uStack_49 = 1;
                  FUN_1409dca90(&uStack_1b8,uStack_2d8,0x34,0);
                  uStack_1c0 = CONCAT44(uStack_16c,uStack_170);
                  lStack_1c8 = lStack_178;
                  uStack_1d8 = uStack_188;
                  uStack_1d4 = uStack_184;
                  uStack_1d0 = uStack_180;
                  uStack_1cc = uStack_17c;
                  uStack_1e8 = (undefined4)uStack_198;
                  uStack_1e4 = uStack_198._4_4_;
                  uStack_1e0 = (undefined4)uStack_190;
                  uStack_1dc = uStack_190._4_4_;
                  uStack_1f8 = uStack_1a8;
                  puStack_1f0 = puStack_1a0;
                  uStack_208 = uStack_1b8;
                  uStack_200 = uStack_1b0;
                  uStack_b8 = uStack_290;
                  uStack_b4 = uStack_2a8;
                  uStack_b0 = uStack_2c0;
                  uStack_ac = uStack_2d8;
                  uStack_148 = CONCAT44(uStack_214,uStack_218);
                  uStack_140 = CONCAT44(uStack_20c,uStack_210);
                  puStack_158 = puStack_228;
                  lStack_150 = uStack_220;
                  uStack_a8 = uVar9;
                  uStack_d8 = uVar7;
                  uStack_128 = CONCAT44(uStack_234,uStack_238);
                  uStack_120 = CONCAT44(uStack_22c,uStack_230);
                  puStack_138 = puStack_248;
                  uStack_130 = uStack_240;
                  uStack_a4 = uStack_68;
                  uStack_d0 = uVar6;
                  uStack_108 = CONCAT44(uStack_254,uStack_258);
                  uStack_100 = CONCAT44(uStack_24c,uStack_250);
                  puStack_118 = puStack_268;
                  uStack_110 = uStack_260;
                  uStack_a0 = uStack_6c;
                  uStack_c8 = uStack_168;
                  uStack_e8 = uStack_278;
                  uStack_e4 = uStack_274;
                  uStack_e0 = uStack_270;
                  uStack_dc = uStack_26c;
                  uStack_f8 = (undefined4)uStack_288;
                  uStack_f4 = uStack_288._4_4_;
                  uStack_f0 = (undefined4)uStack_280;
                  uStack_ec = uStack_280._4_4_;
                  uStack_9c = uStack_64;
                  uStack_c0 = uStack_160;
                  uStack_49 = 0;
                  FUN_1409dcfd0(&uStack_208,&puStack_158);
                  return;
                }
                FUN_1428d98e0(lStack_150,8,&UNK_142a956c0);
              }
              else {
                FUN_1428d98e0(lStack_150,0x10,&UNK_142a956c0);
              }
            }
            else {
              FUN_1428d98e0(lStack_150,0x1e0,&UNK_142a956c0);
            }
          }
          else {
            FUN_1428d98e0(lStack_150,0x80,&UNK_142a956c0);
          }
                    /* WARNING: Does not return */
          pcVar5 = (code *)invalidInstructionException();
          (*pcVar5)();
        }
      }
      goto code_r0x00014090af14;
    }
  }
  FUN_1428d9310(&UNK_142aaaf30);
code_r0x00014090af14:
  FUN_1428d9310(&UNK_142aaaf48);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

