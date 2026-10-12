
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140be2b40(longlong *param_1)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined *puVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong lVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  longlong alStack_350 [3];
  longlong alStack_338 [3];
  longlong alStack_320 [3];
  longlong alStack_308 [3];
  longlong alStack_2f0 [3];
  longlong alStack_2d8 [3];
  longlong alStack_2c0 [3];
  longlong alStack_2a8 [3];
  longlong alStack_290 [3];
  longlong alStack_278 [3];
  longlong alStack_260 [3];
  longlong alStack_248 [3];
  longlong alStack_230 [3];
  longlong alStack_218 [3];
  longlong alStack_200 [3];
  longlong alStack_1e8 [3];
  longlong alStack_1d0 [3];
  longlong alStack_1b8 [3];
  longlong alStack_1a0 [3];
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  longlong lStack_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  longlong lStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  longlong lStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  longlong lStack_130;
  longlong lStack_128;
  undefined8 *puStack_120;
  undefined8 uStack_118;
  longlong *plStack_110;
  undefined *puStack_108;
  longlong lStack_100;
  undefined *puStack_f8;
  byte bStack_e9;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined *puStack_d8;
  longlong lStack_d0;
  byte **ppbStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  byte *pbStack_a0;
  code *pcStack_98;
  undefined8 *puStack_90;
  code *pcStack_88;
  undefined8 *puStack_80;
  longlong lStack_78;
  undefined1 uStack_69;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar9 = param_1[2];
  puStack_108 = (undefined *)0x0;
  lStack_100 = 8;
  puStack_f8 = (undefined *)0x0;
  lVar1 = param_1[1];
  uVar11 = uVar9;
  plStack_110 = param_1;
  while (0 < (longlong)uVar11) {
    uVar12 = uVar11 - 1;
    if (uVar9 <= uVar12) {
      uStack_69 = 1;
      FUN_1428d9518(uVar12,uVar9,&UNK_142b12f70);
code_r0x000140be55c4:
                    /* WARNING: Does not return */
      pcVar2 = (code *)invalidInstructionException();
      (*pcVar2)();
    }
    bStack_e9 = *(byte *)(lVar1 + uVar12);
    switch(bStack_e9) {
    case 0:
      puVar6 = (undefined8 *)func_0x000140613c10(0x23,1);
      uVar7 = _UNK_142b12fa0;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x23,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      puVar6[2] = _UNK_142b12f98;
      puVar6[3] = uVar7;
      uVar16 = _UNK_142b12f94;
      uVar15 = _UNK_142b12f90;
      uVar14 = _UNK_142b12f8c;
      *(undefined4 *)puVar6 = _UNK_142b12f88;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0x1f) = 0x706d6174;
      uStack_68 = 0x23;
      lStack_58 = 0x23;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x23) = 0x3b;
      lStack_58 = 0x24;
      uVar12 = uVar11 - 0xd;
      if (0xc < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b12fc0;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(&uStack_140,&puStack_d8);
        if (uStack_68 != 0) {
          func_0x000140613c20(uStack_60,uStack_68,1);
        }
        lStack_58 = lStack_130;
        uStack_68 = CONCAT44(uStack_13c,uStack_140);
        uStack_60 = (undefined8 *)CONCAT44(uStack_134,uStack_138);
        uStack_e8 = *(undefined8 *)(lVar1 + 4 + uVar12);
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_1400c84c0;
        puStack_d8 = &UNK_142b12ff0;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_1a0;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(plVar10,&puStack_d8);
code_r0x000140be4b94:
        if (uStack_68 != 0) {
          func_0x000140613c20(uStack_60,uStack_68,1);
        }
        lStack_58 = plVar10[2];
        uStack_68 = *plVar10;
        uStack_60 = (undefined8 *)plVar10[1];
      }
      break;
    case 1:
      puVar6 = (undefined8 *)func_0x000140613c10(0x28,1);
      uVar7 = _UNK_142b13028;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x28,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      puVar6[2] = _UNK_142b13020;
      puVar6[3] = uVar7;
      uVar16 = _UNK_142b1301c;
      uVar15 = _UNK_142b13018;
      uVar14 = _UNK_142b13014;
      *(undefined4 *)puVar6 = _UNK_142b13010;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      puVar6[4] = 0x6c6173726576696e;
      uStack_68 = 0x28;
      lStack_58 = 0x28;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)(uStack_60 + 5) = 0x3b;
      lStack_58 = 0x29;
      uVar12 = uVar11 - 9;
      if (8 < uVar11) {
        uStack_e8 = *(undefined8 *)(lVar1 + uVar12);
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_1400c84c0;
        puStack_d8 = &UNK_142b13058;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_1b8;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 2:
      puVar6 = (undefined8 *)func_0x000140613c10(0x26,1);
      uVar7 = _UNK_142b13090;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x26,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      puVar6[2] = _UNK_142b13088;
      puVar6[3] = uVar7;
      uVar16 = _UNK_142b13084;
      uVar15 = _UNK_142b13080;
      uVar14 = _UNK_142b1307c;
      *(undefined4 *)puVar6 = _UNK_142b13078;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0x1e) = 0x657475706d6f635f;
      uStack_68 = 0x26;
      lStack_58 = 0x26;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x26) = 0x3b;
      lStack_58 = 0x27;
      uVar12 = uVar11 - 10;
      if (9 < uVar11) {
        uStack_e8 = CONCAT71(uStack_e8._1_7_,*(undefined1 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_140156ee0;
        puStack_d8 = &UNK_142b130b0;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(&uStack_158,&puStack_d8);
        if (uStack_68 != 0) {
          func_0x000140613c20(uStack_60,uStack_68,1);
        }
        lStack_58 = lStack_148;
        uStack_68 = CONCAT44(uStack_154,uStack_158);
        uStack_60 = (undefined8 *)CONCAT44(uStack_14c,uStack_150);
        uStack_e8 = *(undefined8 *)(lVar1 + 1 + uVar12);
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_1400c84c0;
        puStack_d8 = &UNK_142b13058;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_1d0;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 3:
      puVar6 = (undefined8 *)func_0x000140613c10(0x1d,1);
      uVar7 = _UNK_142b130e5;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x1d,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(ulonglong *)((longlong)puVar6 + 0xd) = CONCAT53(_UNK_142b130e0,_UNK_142b130dd);
      *(undefined8 *)((longlong)puVar6 + 0x15) = uVar7;
      uVar16 = _UNK_142b130dc;
      uVar15 = _UNK_142b130d8;
      uVar14 = _UNK_142b130d4;
      *(undefined4 *)puVar6 = _UNK_142b130d0;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uStack_68 = 0x1d;
      lStack_58 = 0x1d;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x1d) = 0x3b;
      lStack_58 = 0x1e;
      uVar12 = uVar11 - 0x19;
      if (0x18 < uVar11) {
        puVar6 = (undefined8 *)(lVar1 + uVar12);
        uStack_e8 = *puVar6;
        uStack_e0 = *(undefined4 *)(puVar6 + 1);
        uStack_dc = *(undefined4 *)((longlong)puVar6 + 0xc);
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = FUN_140be2710;
        puStack_d8 = &UNK_142b130f8;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(&uStack_170,&puStack_d8);
        if (uStack_68 != 0) {
          func_0x000140613c20(uStack_60,uStack_68,1);
        }
        lStack_58 = lStack_160;
        uStack_68 = CONCAT44(uStack_16c,uStack_170);
        uStack_60 = (undefined8 *)CONCAT44(uStack_164,uStack_168);
        uStack_e8 = *(undefined8 *)(lVar1 + uVar12 + 0x10);
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_1400c84c0;
        puStack_d8 = &UNK_142b13120;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_1e8;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 4:
      puVar6 = (undefined8 *)func_0x000140613c10(0x1a,1);
      uVar7 = _UNK_142b13152;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x1a,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(ulonglong *)((longlong)puVar6 + 10) =
           CONCAT26(_UNK_142b13150,CONCAT42(_UNK_142b1314c,_UNK_142b1314a));
      *(undefined8 *)((longlong)puVar6 + 0x12) = uVar7;
      uVar16 = _UNK_142b1314c;
      uVar15 = _UNK_142b13144;
      uVar14 = CONCAT22(_UNK_142b1314a,_UNK_142b13148);
      *(undefined4 *)puVar6 = _UNK_142b13140;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar15;
      *(undefined4 *)(puVar6 + 1) = uVar14;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uStack_68 = 0x1a;
      lStack_58 = 0x1a;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x1a) = 0x3b;
      lStack_58 = 0x1b;
      uVar12 = uVar11 - 0x11;
      if (0x10 < uVar11) {
        puVar6 = (undefined8 *)(lVar1 + uVar12);
        uStack_e8 = *puVar6;
        uStack_e0 = *(undefined4 *)(puVar6 + 1);
        uStack_dc = *(undefined4 *)((longlong)puVar6 + 0xc);
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = FUN_140be2710;
        puStack_d8 = &UNK_142b130f8;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_200;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 5:
      lVar13 = 9;
      puVar6 = (undefined8 *)func_0x000140613c10(9,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,9,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *puVar6 = 0x6c64695f74696177;
      *(undefined1 *)(puVar6 + 1) = 0x65;
      uStack_68 = 9;
      lStack_58 = 9;
      lStack_78 = 9;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 6:
      puVar6 = (undefined8 *)func_0x000140613c10(0x13,1);
      uVar16 = _UNK_142b13166;
      uVar15 = _UNK_142b13162;
      uVar14 = _UNK_142b1315e;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x13,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b1315a;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xf) = 0x73776569;
      uStack_68 = 0x13;
      lStack_58 = 0x13;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x13) = 0x3b;
      lStack_58 = 0x14;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_218;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 7:
      puVar6 = (undefined8 *)func_0x000140613c10(0xe,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xe,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 6) = 0x736567616d695f79;
      *puVar6 = 0x5f796f7274736564;
      uStack_68 = 0xe;
      lStack_58 = 0xe;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0xe) = 0x3b;
      lStack_58 = 0xf;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_230;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 8:
      puVar6 = (undefined8 *)func_0x000140613c10(0xf,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xf,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 7) = 0x737265666675625f;
      *puVar6 = 0x5f796f7274736564;
      uStack_68 = 0xf;
      lStack_58 = 0xf;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0xf) = 0x3b;
      lStack_58 = 0x10;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_248;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 9:
      puVar6 = (undefined8 *)func_0x000140613c10(0x12,1);
      uVar16 = _UNK_142b131a4;
      uVar15 = _UNK_142b131a0;
      uVar14 = _UNK_142b1319c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x12,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13198;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined2 *)(puVar6 + 2) = 0x7365;
      uStack_68 = 0x12;
      lStack_58 = 0x12;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x12) = 0x3b;
      lStack_58 = 0x13;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_260;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 10:
      puVar6 = (undefined8 *)func_0x000140613c10(0x14,1);
      uVar16 = _UNK_142b131b6;
      uVar15 = _UNK_142b131b2;
      uVar14 = _UNK_142b131ae;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x14,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b131aa;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined4 *)(puVar6 + 2) = 0x73726566;
      uStack_68 = 0x14;
      lStack_58 = 0x14;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x14) = 0x3b;
      lStack_58 = 0x15;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_278;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0xb:
      puVar6 = (undefined8 *)func_0x000140613c10(0x15,1);
      uVar16 = _UNK_142b131ca;
      uVar15 = _UNK_142b131c6;
      uVar14 = _UNK_142b131c2;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x15,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b131be;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xd) = 0x7365737361705f72;
      uStack_68 = 0x15;
      lStack_58 = 0x15;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x15) = 0x3b;
      lStack_58 = 0x16;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_290;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0xc:
      puVar6 = (undefined8 *)func_0x000140613c10(0x10,1);
      uVar16 = _UNK_142b131df;
      uVar15 = _UNK_142b131db;
      uVar14 = _UNK_142b131d7;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x10,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b131d3;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uStack_68 = 0x10;
      lStack_58 = 0x10;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)(uStack_60 + 2) = 0x3b;
      lStack_58 = 0x11;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_2a8;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0xd:
      puVar6 = (undefined8 *)func_0x000140613c10(0x18,1);
      uVar16 = _UNK_142b131ef;
      uVar15 = _UNK_142b131eb;
      uVar14 = _UNK_142b131e7;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x18,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b131e3;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      puVar6[2] = 0x736c6f6f705f726f;
      uStack_68 = 0x18;
      lStack_58 = 0x18;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)(uStack_60 + 3) = 0x3b;
      lStack_58 = 0x19;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13178;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_2c0;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0xe:
      puVar6 = (undefined8 *)func_0x000140613c10(0x11,1);
      uVar16 = _UNK_142b13207;
      uVar15 = _UNK_142b13203;
      uVar14 = _UNK_142b131ff;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x11,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b131fb;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined1 *)(puVar6 + 2) = 0x6e;
      goto code_r0x000140be31e8;
    case 0xf:
      puVar6 = (undefined8 *)func_0x000140613c10(0x16,1);
      uVar16 = _UNK_142b13218;
      uVar15 = _UNK_142b13214;
      uVar14 = _UNK_142b13210;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x16,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b1320c;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xe) = 0x6c6f6f705f726f74;
      uStack_68 = 0x16;
      lStack_58 = 0x16;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x16) = 0x3b;
      lStack_58 = 0x17;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = FUN_140be2990;
        puStack_d8 = &UNK_142b13228;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_2d8;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x10:
      puVar6 = (undefined8 *)func_0x000140613c10(0x15,1);
      uVar16 = _UNK_142b13254;
      uVar15 = _UNK_142b13250;
      uVar14 = _UNK_142b1324c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x15,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13248;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xd) = 0x6c6f6f705f726f74;
      uStack_68 = 0x15;
      lStack_58 = 0x15;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x15) = 0x3b;
      lStack_58 = 0x16;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = FUN_140be2990;
        puStack_d8 = &UNK_142b13228;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_2f0;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x11:
      puVar6 = (undefined8 *)func_0x000140613c10(0x17,1);
      uVar16 = _UNK_142b13269;
      uVar15 = _UNK_142b13265;
      uVar14 = _UNK_142b13261;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x17,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b1325d;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xf) = 0x7465735f726f7470;
      uStack_68 = 0x17;
      lStack_58 = 0x17;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x17) = 0x3b;
      lStack_58 = 0x18;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = FUN_140be2990;
        puStack_d8 = &UNK_142b13228;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_308;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x12:
      puVar6 = (undefined8 *)func_0x000140613c10(0x20,1);
      uVar7 = _UNK_142b1328c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x20,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      puVar6[2] = _UNK_142b13284;
      puVar6[3] = uVar7;
      uVar14 = _UNK_142b13274;
      uVar15 = _UNK_142b13278;
      uVar16 = _UNK_142b1327c;
      uVar17 = _UNK_142b13280;
      goto code_r0x000140be4487;
    case 0x13:
      lVar13 = 0x17;
      puVar6 = (undefined8 *)func_0x000140613c10(0x17,1);
      uVar16 = _UNK_142b132a0;
      uVar15 = _UNK_142b1329c;
      uVar14 = _UNK_142b13298;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x17,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13294;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xf) = 0x656e696c65706970;
      uStack_68 = 0x17;
      lStack_58 = 0x17;
      lStack_78 = 0x17;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x14:
      lVar13 = 0x16;
      puVar6 = (undefined8 *)func_0x000140613c10(0x16,1);
      uVar16 = _UNK_142b132b7;
      uVar15 = _UNK_142b132b3;
      uVar14 = _UNK_142b132af;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x16,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b132ab;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xe) = 0x656e696c65706970;
      uStack_68 = 0x16;
      lStack_58 = 0x16;
      lStack_78 = 0x16;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x15:
      puVar6 = (undefined8 *)func_0x000140613c10(0x15,1);
      uVar16 = _UNK_142b132cd;
      uVar15 = _UNK_142b132c9;
      uVar14 = _UNK_142b132c5;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x15,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b132c1;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uVar7 = 0x7265666675625f64;
      goto code_r0x000140be41dd;
    case 0x16:
      puVar6 = (undefined8 *)func_0x000140613c10(0x14,1);
      uVar14 = _UNK_142b132d6;
      uVar15 = _UNK_142b132da;
      uVar16 = _UNK_142b132de;
      uVar17 = _UNK_142b132e2;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x14,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      goto code_r0x000140be4c2b;
    case 0x17:
      puVar6 = (undefined8 *)func_0x000140613c10(0x14,1);
      uVar14 = _UNK_142b132ea;
      uVar15 = _UNK_142b132ee;
      uVar16 = _UNK_142b132f2;
      uVar17 = _UNK_142b132f6;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x14,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
code_r0x000140be4c2b:
      lVar13 = 0x14;
      *(undefined4 *)puVar6 = uVar14;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar15;
      *(undefined4 *)(puVar6 + 1) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar17;
      *(undefined4 *)(puVar6 + 2) = 0x72656666;
      uStack_68 = 0x14;
      lStack_58 = 0x14;
      lStack_78 = 0x14;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x18:
      puVar6 = (undefined8 *)func_0x000140613c10(0x12,1);
      uVar16 = _UNK_142b1330a;
      uVar15 = _UNK_142b13306;
      uVar14 = _UNK_142b13302;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x12,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b132fe;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined2 *)(puVar6 + 2) = 0x7265;
      goto code_r0x000140be3f61;
    case 0x19:
      lVar13 = 0xc;
      puVar6 = (undefined8 *)func_0x000140613c10(0xc,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xc,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *puVar6 = 0x75735f6575657571;
      *(undefined4 *)(puVar6 + 1) = 0x74696d62;
      uStack_68 = 0xc;
      lStack_58 = 0xc;
      lStack_78 = 0xc;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x1a:
      puVar6 = (undefined8 *)func_0x000140613c10(0xf,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xf,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 7) = 0x74696d6275735f65;
      uVar7 = 0x65756575715f6b76;
      goto code_r0x000140be4d84;
    case 0x1b:
      puVar6 = (undefined8 *)func_0x000140613c10(0x11,1);
      uVar16 = _UNK_142b1331c;
      uVar15 = _UNK_142b13318;
      uVar14 = _UNK_142b13314;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x11,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13310;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined1 *)(puVar6 + 2) = 0x65;
code_r0x000140be31e8:
      lVar13 = 0x11;
      uStack_68 = 0x11;
      lStack_58 = 0x11;
      lStack_78 = 0x11;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x1c:
      puVar6 = (undefined8 *)func_0x000140613c10(0x10,1);
      uVar14 = _UNK_142b13321;
      uVar15 = _UNK_142b13325;
      uVar16 = _UNK_142b13329;
      uVar17 = _UNK_142b1332d;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x10,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      goto code_r0x000140be4c88;
    case 0x1d:
      puVar6 = (undefined8 *)func_0x000140613c10(0x13,1);
      uVar16 = _UNK_142b1333d;
      uVar15 = _UNK_142b13339;
      uVar14 = _UNK_142b13335;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x13,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13331;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xf) = 0x6d697274;
      goto code_r0x000140be4ce8;
    case 0x1e:
      puVar6 = (undefined8 *)func_0x000140613c10(0x17,1);
      uVar16 = _UNK_142b13350;
      uVar15 = _UNK_142b1334c;
      uVar14 = _UNK_142b13348;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x17,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13344;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined8 *)((longlong)puVar6 + 0xf) = 0x726564726f5f6e69;
      uStack_68 = 0x17;
      lStack_58 = 0x17;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0x17) = 0x3b;
      lStack_58 = 0x18;
      uVar12 = uVar11 - 9;
      if (8 < uVar11) {
        uStack_e8 = *(undefined8 *)(lVar1 + uVar12);
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_1400c84c0;
        puStack_d8 = &UNK_142b13368;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_320;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x1f:
      puVar6 = (undefined8 *)func_0x000140613c10(0x12,1);
      uVar16 = _UNK_142b13394;
      uVar15 = _UNK_142b13390;
      uVar14 = _UNK_142b1338c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x12,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13388;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined2 *)(puVar6 + 2) = 0x6567;
      goto code_r0x000140be3f61;
    case 0x20:
      puVar6 = (undefined8 *)func_0x000140613c10(0xd,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xd,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 5) = 0x6567616d695f746e;
      uVar7 = 0x5f746e6573657270;
      goto code_r0x000140be4863;
    case 0x21:
      puVar6 = (undefined8 *)func_0x000140613c10(0xd,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xd,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 5) = 0x746e65736572705f;
      uVar7 = 0x72705f6575657571;
code_r0x000140be4863:
      lVar13 = 0xd;
      *puVar6 = uVar7;
      uStack_68 = 0xd;
      lStack_58 = 0xd;
      lStack_78 = 0xd;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x22:
      puVar6 = (undefined8 *)func_0x000140613c10(0x13,1);
      uVar16 = _UNK_142b133a6;
      uVar15 = _UNK_142b133a2;
      uVar14 = _UNK_142b1339e;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x13,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b1339a;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xf) = 0x65746164;
code_r0x000140be4ce8:
      lVar13 = 0x13;
      uStack_68 = 0x13;
      lStack_58 = 0x13;
      lStack_78 = 0x13;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x23:
      puVar6 = (undefined8 *)func_0x000140613c10(0x12,1);
      uVar16 = _UNK_142b133b9;
      uVar15 = _UNK_142b133b5;
      uVar14 = _UNK_142b133b1;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x12,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b133ad;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      *(undefined2 *)(puVar6 + 2) = 0x6c61;
code_r0x000140be3f61:
      lVar13 = 0x12;
      uStack_68 = 0x12;
      lStack_58 = 0x12;
      lStack_78 = 0x12;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x24:
      puVar6 = (undefined8 *)func_0x000140613c10(0xf,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xf,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 7) = 0x74736f6c5f6e6565;
      uVar7 = 0x657263736c6c7566;
      goto code_r0x000140be4d84;
    case 0x25:
      puVar6 = (undefined8 *)func_0x000140613c10(0xb,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xb,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *puVar6 = 0x5f746e6573657270;
      *(undefined4 *)((longlong)puVar6 + 7) = 0x7272655f;
      uStack_68 = 0xb;
      lStack_58 = 0xb;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0xb) = 0x3b;
      lStack_58 = 0xc;
      uVar12 = uVar11 - 5;
      if (4 < uVar11) {
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12));
        pcStack_98 = (code *)&UNK_14003bad0;
        puStack_90 = &uStack_e8;
        pcStack_88 = (code *)&UNK_140c8f550;
        puStack_d8 = &UNK_142b133c8;
        lStack_d0 = 2;
        ppbStack_c8 = &pbStack_a0;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_338;
        pbStack_a0 = (byte *)&uStack_68;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x26:
      puVar6 = (undefined8 *)func_0x000140613c10(0xf,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xf,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 7) = 0x646572697465725f;
      uVar7 = 0x5f746e6573657270;
code_r0x000140be4d84:
      lVar13 = 0xf;
      *puVar6 = uVar7;
      uStack_68 = 0xf;
      lStack_58 = 0xf;
      lStack_78 = 0xf;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x27:
      puVar6 = (undefined8 *)func_0x000140613c10(0x10,1);
      uVar14 = _UNK_142b133e8;
      uVar15 = _UNK_142b133ec;
      uVar16 = _UNK_142b133f0;
      uVar17 = _UNK_142b133f4;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x10,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
code_r0x000140be4c88:
      lVar13 = 0x10;
      *(undefined4 *)puVar6 = uVar14;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar15;
      *(undefined4 *)(puVar6 + 1) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar17;
      uStack_68 = 0x10;
      lStack_58 = 0x10;
      lStack_78 = 0x10;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x28:
      lVar13 = 0x1c;
      puVar6 = (undefined8 *)func_0x000140613c10(0x1c,1);
      uVar7 = _UNK_142b1340c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x1c,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(ulonglong *)((longlong)puVar6 + 0xc) = CONCAT44(_UNK_142b13408,_UNK_142b13404);
      *(undefined8 *)((longlong)puVar6 + 0x14) = uVar7;
      uVar16 = _UNK_142b13404;
      uVar15 = _UNK_142b13400;
      uVar14 = _UNK_142b133fc;
      *(undefined4 *)puVar6 = _UNK_142b133f8;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uStack_68 = 0x1c;
      lStack_58 = 0x1c;
      lStack_78 = 0x1c;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x29:
      puVar6 = (undefined8 *)func_0x000140613c10(0x20,1);
      uVar7 = _UNK_142b1342c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x20,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      puVar6[2] = _UNK_142b13424;
      puVar6[3] = uVar7;
      uVar14 = _UNK_142b13414;
      uVar15 = _UNK_142b13418;
      uVar16 = _UNK_142b1341c;
      uVar17 = _UNK_142b13420;
code_r0x000140be4487:
      lVar13 = 0x20;
      *(undefined4 *)puVar6 = uVar14;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar15;
      *(undefined4 *)(puVar6 + 1) = uVar16;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar17;
      uStack_68 = 0x20;
      lStack_58 = 0x20;
      lStack_78 = 0x20;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x2a:
      puVar6 = (undefined8 *)func_0x000140613c10(0xd,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xd,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined8 *)((longlong)puVar6 + 5) = 0x737361705f67725f;
      *puVar6 = 0x67725f6e69676562;
      uStack_68 = 0xd;
      lStack_58 = 0xd;
      uStack_49 = 1;
      uStack_60 = puVar6;
      FUN_140d79570(&uStack_68,&UNK_142b12ee8);
      *(undefined1 *)((longlong)uStack_60 + 0xd) = 0x3b;
      lStack_58 = 0xe;
      uVar12 = uVar11 - 0x15;
      if (0x14 < uVar11) {
        puVar6 = (undefined8 *)(lVar1 + uVar12);
        uStack_e8 = *puVar6;
        uStack_e0 = *(undefined4 *)(puVar6 + 1);
        uStack_dc = *(undefined4 *)((longlong)puVar6 + 0xc);
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = FUN_140be2710;
        puStack_d8 = &UNK_142b130f8;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(&uStack_188,&puStack_d8);
        if (uStack_68 != 0) {
          func_0x000140613c20(uStack_60,uStack_68,1);
        }
        lStack_58 = lStack_178;
        uStack_68 = CONCAT44(uStack_184,uStack_188);
        uStack_60 = (undefined8 *)CONCAT44(uStack_17c,uStack_180);
        uStack_e8 = CONCAT44(uStack_e8._4_4_,*(undefined4 *)(lVar1 + uVar12 + 0x10));
        pcStack_98 = (code *)&UNK_14003bad0;
        pcStack_88 = (code *)&UNK_140031290;
        puStack_d8 = &UNK_142b13448;
        lStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 0;
        uStack_49 = 1;
        plVar10 = alStack_350;
        ppbStack_c8 = &pbStack_a0;
        pbStack_a0 = (byte *)&uStack_68;
        puStack_90 = &uStack_e8;
        FUN_140d79f90(plVar10,&puStack_d8);
        goto code_r0x000140be4b94;
      }
      break;
    case 0x2b:
      lVar13 = 0xb;
      puVar6 = (undefined8 *)func_0x000140613c10(0xb,1);
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0xb,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *puVar6 = 0x705f67725f646e65;
      *(undefined4 *)((longlong)puVar6 + 7) = 0x73736170;
      uStack_68 = 0xb;
      lStack_58 = 0xb;
      lStack_78 = 0xb;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x2c:
      puVar6 = (undefined8 *)func_0x000140613c10(0x15,1);
      uVar16 = _UNK_142b13474;
      uVar15 = _UNK_142b13470;
      uVar14 = _UNK_142b1346c;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x15,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(undefined4 *)puVar6 = _UNK_142b13468;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar14;
      *(undefined4 *)(puVar6 + 1) = uVar15;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uVar7 = 0x726569727261625f;
code_r0x000140be41dd:
      lVar13 = 0x15;
      *(undefined8 *)((longlong)puVar6 + 0xd) = uVar7;
      uStack_68 = 0x15;
      lStack_58 = 0x15;
      lStack_78 = 0x15;
      uStack_60 = puVar6;
      goto code_r0x000140be4db0;
    case 0x2d:
      lVar13 = 0x1a;
      puVar6 = (undefined8 *)func_0x000140613c10(0x1a,1);
      uVar7 = _UNK_142b1348f;
      if (puVar6 == (undefined8 *)0x0) {
        uStack_49 = 0;
        FUN_1428d8fe3(1,0x1a,&UNK_142b12e80);
        goto code_r0x000140be55c4;
      }
      *(ulonglong *)((longlong)puVar6 + 10) =
           CONCAT26(_UNK_142b1348d,CONCAT42(_UNK_142b13489,_UNK_142b13487));
      *(undefined8 *)((longlong)puVar6 + 0x12) = uVar7;
      uVar16 = _UNK_142b13489;
      uVar15 = _UNK_142b13481;
      uVar14 = CONCAT22(_UNK_142b13487,_UNK_142b13485);
      *(undefined4 *)puVar6 = _UNK_142b1347d;
      *(undefined4 *)((longlong)puVar6 + 4) = uVar15;
      *(undefined4 *)(puVar6 + 1) = uVar14;
      *(undefined4 *)((longlong)puVar6 + 0xc) = uVar16;
      uStack_68 = 0x1a;
      lStack_58 = 0x1a;
      lStack_78 = 0x1a;
      uStack_60 = puVar6;
code_r0x000140be4db0:
      puStack_80 = uStack_60;
      if (puStack_f8 == puStack_108) goto code_r0x000140be4dce;
      goto code_r0x000140be2bb0;
    default:
      pbStack_a0 = &bStack_e9;
      pcStack_98 = FUN_140d8d1a0;
      puStack_d8 = &UNK_142b134b0;
      lStack_d0 = 1;
      ppbStack_c8 = &pbStack_a0;
      uStack_c0 = 1;
      uStack_b8 = 0;
      uStack_69 = 1;
      FUN_140d79f90(&lStack_128,&puStack_d8);
      puVar5 = puStack_f8;
      lStack_78 = lStack_128;
      puStack_80 = puStack_120;
      if (puStack_f8 == puStack_108) {
        FUN_140562db0(&puStack_108,&UNK_142b12f70);
      }
      *(longlong *)(lStack_100 + (longlong)puVar5 * 0x18) = lStack_78;
      *(undefined8 **)(lStack_100 + 8 + (longlong)puVar5 * 0x18) = puStack_80;
      *(undefined8 *)(lStack_100 + 0x10 + (longlong)puVar5 * 0x18) = uStack_118;
      puStack_f8 = puVar5 + 1;
      if (puStack_f8 < (undefined *)0x2) goto code_r0x000140be4f39;
      goto code_r0x000140be4eec;
    }
    lStack_78 = uStack_68;
    puStack_80 = uStack_60;
    lVar13 = lStack_58;
    if (puStack_f8 == puStack_108) {
code_r0x000140be4dce:
      puVar5 = puStack_f8;
      puStack_80 = uStack_60;
      FUN_140562db0(&puStack_108,&UNK_142b12f70);
      puStack_f8 = puVar5;
    }
code_r0x000140be2bb0:
    *(longlong *)(lStack_100 + (longlong)puStack_f8 * 0x18) = lStack_78;
    *(undefined8 **)(lStack_100 + 8 + (longlong)puStack_f8 * 0x18) = puStack_80;
    *(longlong *)(lStack_100 + 0x10 + (longlong)puStack_f8 * 0x18) = lVar13;
    puStack_f8 = puStack_f8 + 1;
    uVar11 = uVar12;
  }
  if ((undefined *)0x1 < puStack_f8) {
code_r0x000140be4eec:
    uVar9 = (ulonglong)puStack_f8 >> 1;
    puVar8 = (undefined8 *)(lStack_100 + (longlong)puStack_f8 * 0x18 + -8);
    puVar6 = (undefined8 *)(lStack_100 + 0x10);
    do {
      uVar7 = puVar6[-2];
      uVar3 = puVar6[-1];
      uVar4 = puVar8[-1];
      puVar6[-2] = puVar8[-2];
      puVar6[-1] = uVar4;
      puVar8[-2] = uVar7;
      puVar8[-1] = uVar3;
      uVar7 = *puVar6;
      *puVar6 = *puVar8;
      *puVar8 = uVar7;
      puVar6 = puVar6 + 3;
      puVar8 = puVar8 + -3;
      uVar9 = uVar9 - 1;
    } while (uVar9 != 0);
  }
code_r0x000140be4f39:
  ppbStack_c8 = (byte **)puStack_f8;
  puStack_d8 = puStack_108;
  lStack_d0 = lStack_100;
  uStack_69 = 0;
  FUN_1418c17f0(&puStack_d8);
  if (*plStack_110 != 0) {
    func_0x000140613c20(plStack_110[1],*plStack_110,1);
  }
  return;
}

