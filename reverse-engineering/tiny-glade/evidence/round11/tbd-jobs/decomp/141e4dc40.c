// 0x141e4dc40 undefined FUN_141e4dc40(void) body_bytes=3372

/* WARNING: Possible PIC construction at 0x000141e4e7a5: Changing call to branch */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141e4dc40(undefined8 param_1,undefined4 param_2,undefined4 param_3,byte *param_4,
                  undefined4 *param_5,undefined8 param_6)

{
  byte bVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  code *pcVar4;
  undefined8 uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined4 *unaff_RBP;
  undefined4 *unaff_RSI;
  longlong *plVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  longlong lVar11;
  undefined8 auStack_320 [5];
  undefined4 *puStack_2f8;
  undefined8 uStack_2f0;
  undefined4 uStack_2e8;
  undefined4 uStack_2e4;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined4 uStack_2d8;
  undefined4 uStack_2d4;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  undefined4 uStack_2c8;
  undefined4 uStack_2c4;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined8 uStack_2b8;
  char cStack_2b0;
  undefined4 uStack_2af;
  undefined3 uStack_2ab;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined4 uStack_2a0;
  undefined4 uStack_29c;
  undefined4 uStack_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined8 uStack_278;
  char cStack_270;
  undefined4 uStack_26f;
  undefined3 uStack_26b;
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
  ulonglong uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
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
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined2 uStack_120;
  undefined2 uStack_11e;
  undefined1 uStack_11c;
  undefined4 uStack_11b;
  undefined2 uStack_117;
  longlong lStack_108;
  undefined4 *puStack_100;
  longlong lStack_f8;
  int iStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  int iStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined4 *puStack_a0;
  longlong lStack_98;
  undefined4 *puStack_90;
  byte *pbStack_88;
  undefined4 *puStack_80;
  uint uStack_74;
  undefined *puStack_70;
  undefined1 uStack_64;
  undefined1 uStack_63;
  undefined1 uStack_61;
  undefined8 uStack_60;
  byte *pbStack_48;
  longlong lVar12;
  
  puVar6 = &uStack_298;
  uStack_60 = 0xfffffffffffffffe;
  puStack_70 = (undefined *)param_6;
  uStack_74 = (uint)*param_4;
  pbStack_88 = param_4;
  if (uStack_74 == 4) {
    lStack_f8 = *(longlong *)(param_4 + 0x18);
    lStack_108 = *(longlong *)(param_4 + 8);
    puStack_100 = *(undefined4 **)(param_4 + 0x10);
    uStack_120 = 0;
    uStack_1b0 = _DAT_142bc2738;
    uStack_1a8 = _UNK_142bc2740;
    uStack_1a0 = _DAT_142bc2748;
    uStack_198 = _UNK_142bc2750;
    uStack_190 = _DAT_142bc2758;
    uStack_188 = _UNK_142bc2760;
    uStack_180 = _DAT_142bc2768;
    uStack_178 = _UNK_142bc2770;
    uStack_170 = _DAT_142bc2778;
    uStack_168 = _UNK_142bc2780;
    uStack_208 = 0;
    uStack_160 = _DAT_142bc2788;
    uStack_158 = _UNK_142bc2790;
    uStack_150 = _DAT_142bc2798;
    uStack_148 = _UNK_142bc27a0;
    uStack_200 = _DAT_142bc26e8;
    uStack_1f8 = _UNK_142bc26f0;
    uStack_1f0 = _DAT_142bc26f8;
    uStack_1e8 = _UNK_142bc2700;
    uStack_1d0 = _DAT_142bc2718;
    uStack_1c8 = _UNK_142bc2720;
    uStack_1e0 = _DAT_142bc2708;
    uStack_1d8 = _UNK_142bc2710;
    uStack_11b = 0x7070704;
    uStack_117 = 0x409;
    uStack_1c0 = _DAT_142bb1370;
    uStack_1b8 = _UNK_142bb1378;
    uStack_11e = 0;
    uStack_140 = 2;
    uStack_128 = _DAT_1429345b0;
    uStack_258 = _DAT_142bc2690;
    uStack_268 = (undefined4 *)_DAT_142bc2680;
    uStack_260 = (undefined4 *)_UNK_142bc2688;
    uStack_240 = _DAT_142bc26a8;
    uStack_250 = (undefined4 *)_DAT_142bc2698;
    uStack_248 = _UNK_142bc26a0;
    uStack_228 = _DAT_142bc26c0;
    uStack_238 = _DAT_142bc26b0;
    uStack_230 = _UNK_142bc26b8;
    uStack_210 = _DAT_142bc26d8;
    uStack_220 = _DAT_142bc26c8;
    uStack_218 = _UNK_142bc26d0;
    uStack_11c = 0;
    uStack_138 = 0;
    uStack_130 = 0;
    uStack_63 = 1;
    auStack_320[0] = 0x141e4e127;
    FUN_140fe7600(&iStack_e8,param_1,&uStack_268);
    if (iStack_e8 == 4) {
      uStack_63 = 1;
      uStack_2f0 = CONCAT44(uStack_2f0._4_4_,uStack_e0);
      puStack_2f8 = (undefined4 *)CONCAT44(puStack_2f8._4_4_,uStack_e4);
      auStack_320[0] = 0x141e4e168;
      FUN_140fe7790(&iStack_c8,param_1,param_2,param_3);
      if (iStack_c8 == 4) {
        lVar11 = lStack_f8 * 0x40;
        puStack_80 = puStack_100 + lStack_f8 * 0x10;
        uStack_268 = puStack_100;
        uStack_260 = puStack_100;
        lStack_98 = lStack_108;
        uStack_258 = lStack_108;
        puStack_a0 = puStack_100;
        puVar7 = puStack_100;
        uStack_250 = puStack_80;
        if (lStack_f8 != 0) {
          puStack_90 = puStack_100;
          do {
            puVar7 = puStack_90 + 0x10;
            if (*(char *)(puStack_90 + 0xe) == '\x02') break;
            uStack_278 = *(undefined8 *)(puStack_90 + 0xc);
            uStack_2a8 = *puStack_90;
            uStack_2a4 = puStack_90[1];
            uStack_2a0 = puStack_90[2];
            uStack_29c = puStack_90[3];
            uStack_298 = puStack_90[4];
            uStack_294 = puStack_90[5];
            uStack_290 = puStack_90[6];
            uStack_28c = puStack_90[7];
            uStack_288 = puStack_90[8];
            uStack_284 = puStack_90[9];
            uStack_280 = puStack_90[10];
            uStack_27c = puStack_90[0xb];
            uStack_26f = *(undefined4 *)((longlong)puStack_90 + 0x39);
            uStack_26b = (undefined3)((uint)puStack_90[0xf] >> 8);
            uStack_2f0 = puStack_70;
            puStack_2f8 = param_5;
            auStack_320[0] = 0x141e4e24c;
            cStack_270 = *(char *)(puStack_90 + 0xe);
            puStack_90 = puVar7;
            FUN_141e4dc40(param_1,uStack_e4,uStack_e0,&uStack_2a8);
            lVar11 = lVar11 + -0x40;
            puVar7 = puStack_80;
          } while (lVar11 != 0);
        }
        puStack_80 = (undefined4 *)((ulonglong)((longlong)puStack_80 - (longlong)puVar7) >> 6);
        puStack_70 = (undefined *)0x0;
        uStack_260 = puVar7;
        puStack_90 = puVar7;
        while (puStack_80 != (undefined4 *)puStack_70) {
          puStack_70 = puStack_70 + 1;
          puVar7 = puVar7 + 0x10;
          auStack_320[0] = 0x141e4e783;
          FUN_140faa6e0();
          param_5 = puVar7;
        }
LAB_141e4e788:
        if (lStack_98 == 0) {
          if (((byte)uStack_74 & 6) == 4) {
            return;
          }
          bVar1 = *pbStack_88;
          if (bVar1 == 0) {
            puVar7 = *(undefined4 **)(pbStack_88 + 8);
            puVar2 = *(undefined8 **)(pbStack_88 + 0x10);
            if ((code *)*puVar2 != (code *)0x0) {
              puStack_70 = &UNK_141e4cb1a;
              (*(code *)*puVar2)(puVar7);
            }
            lVar11 = puVar2[1];
            if (lVar11 == 0) {
              return;
            }
            uVar9 = puVar2[2];
          }
          else {
            if (bVar1 == 4) {
              puVar7 = *(undefined4 **)(pbStack_88 + 0x10);
              pbStack_48 = pbStack_88;
              lVar11 = *(longlong *)(pbStack_88 + 0x18);
              puVar6 = puVar7;
              for (lVar12 = 0; lVar11 != lVar12; lVar12 = lVar12 + 1) {
                puStack_70 = &UNK_141e4cada;
                FUN_140faa6e0(puVar6);
                puVar6 = puVar6 + 0x10;
              }
            }
            else {
              if (bVar1 != 5) {
                return;
              }
              puVar7 = *(undefined4 **)(pbStack_88 + 0x10);
              pbStack_48 = pbStack_88;
              lVar11 = *(longlong *)(pbStack_88 + 0x18);
              puVar6 = puVar7;
              for (lVar12 = 0; lVar11 != lVar12; lVar12 = lVar12 + 1) {
                puStack_70 = &UNK_141e4ca9a;
                FUN_140faa6e0(puVar6);
                puVar6 = puVar6 + 0x10;
              }
            }
            if (*(longlong *)(pbStack_48 + 8) == 0) {
              return;
            }
            lVar11 = *(longlong *)(pbStack_48 + 8) << 6;
            uVar9 = 8;
          }
        }
        else {
          lVar11 = lStack_98 << 6;
          uVar9 = 8;
          register0x00000020 = (BADSPACEBASE *)auStack_320;
          auStack_320[0] = 0x141e4e7aa;
          puVar7 = puStack_a0;
          unaff_RBP = puVar6;
          unaff_RSI = param_5;
        }
        *(undefined4 **)((longlong)register0x00000020 + -8) = unaff_RBP;
        *(undefined4 **)((longlong)register0x00000020 + -0x10) = unaff_RSI;
        puVar6 = puVar7;
        if (0x10 < uVar9) {
          puVar6 = *(undefined4 **)(puVar7 + -2);
        }
        *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
        uVar5 = GetProcessHeap(puVar7,lVar11);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        HeapFree(uVar5,0,puVar6);
        return;
      }
      uStack_258 = CONCAT44(uStack_b4,uStack_b8);
      uStack_250 = (undefined4 *)CONCAT44(uStack_ac,uStack_b0);
      uStack_268 = (undefined4 *)CONCAT44(uStack_c4,iStack_c8);
      uStack_260 = (undefined4 *)CONCAT44(uStack_bc,uStack_c0);
      uStack_63 = 1;
      puStack_2f8 = (undefined4 *)&UNK_142f8f5b8;
      auStack_320[0] = 0x141e4eeb9;
      FUN_1428d9760(&UNK_142f8f358,0x2b,&uStack_268,&UNK_142f8f338);
    }
    else {
      uStack_b8 = uStack_d8;
      uStack_b4 = uStack_d4;
      uStack_b0 = uStack_d0;
      uStack_ac = uStack_cc;
      iStack_c8 = iStack_e8;
      uStack_c4 = uStack_e4;
      uStack_c0 = uStack_e0;
      uStack_bc = uStack_dc;
      uStack_63 = 1;
      puStack_2f8 = (undefined4 *)&UNK_142f8f4e8;
      auStack_320[0] = 0x141e4edd2;
      FUN_1428d9760(&UNK_142f8f358,0x2b,&iStack_c8,&UNK_142f8f338);
    }
  }
  else if (*param_4 == 5) {
    lStack_f8 = *(longlong *)(param_4 + 0x18);
    lStack_108 = *(longlong *)(param_4 + 8);
    puStack_100 = *(undefined4 **)(param_4 + 0x10);
    uStack_120 = 0;
    uStack_1b0 = _DAT_142bc2738;
    uStack_1a8 = _UNK_142bc2740;
    uStack_1a0 = _DAT_142bc2748;
    uStack_198 = _UNK_142bc2750;
    uStack_190 = _DAT_142bc2758;
    uStack_188 = _UNK_142bc2760;
    uStack_180 = _DAT_142bc2768;
    uStack_178 = _UNK_142bc2770;
    uStack_170 = _DAT_142bc2778;
    uStack_168 = _UNK_142bc2780;
    uStack_208 = 0;
    uStack_160 = _DAT_142bc2788;
    uStack_158 = _UNK_142bc2790;
    uStack_150 = _DAT_142bc2798;
    uStack_148 = _UNK_142bc27a0;
    uStack_200 = _DAT_142bc26e8;
    uStack_1f8 = _UNK_142bc26f0;
    uStack_1f0 = _DAT_142bc26f8;
    uStack_1e8 = _UNK_142bc2700;
    uStack_1d0 = _DAT_142bc2718;
    uStack_1c8 = _UNK_142bc2720;
    uStack_1e0 = _DAT_142bc2708;
    uStack_1d8 = _UNK_142bc2710;
    uStack_11b = 0x7070704;
    uStack_117 = 0x409;
    uStack_1c0 = _DAT_142bb1370;
    uStack_1b8 = _UNK_142bb1378;
    uStack_11e = 0x100;
    uStack_140 = 2;
    uStack_128 = _DAT_1429345b0;
    uStack_258 = _DAT_142bc2690;
    uStack_268 = (undefined4 *)_DAT_142bc2680;
    uStack_260 = (undefined4 *)_UNK_142bc2688;
    uStack_240 = _DAT_142bc26a8;
    uStack_250 = (undefined4 *)_DAT_142bc2698;
    uStack_248 = _UNK_142bc26a0;
    uStack_228 = _DAT_142bc26c0;
    uStack_238 = _DAT_142bc26b0;
    uStack_230 = _UNK_142bc26b8;
    uStack_210 = _DAT_142bc26d8;
    uStack_220 = _DAT_142bc26c8;
    uStack_218 = _UNK_142bc26d0;
    uStack_11c = 0;
    uStack_138 = 0;
    uStack_130 = 0;
    uStack_64 = 1;
    auStack_320[0] = 0x141e4de49;
    FUN_140fe7600(&iStack_e8,param_1,&uStack_268);
    if (iStack_e8 == 4) {
      uStack_64 = 1;
      uStack_2f0 = CONCAT44(uStack_2f0._4_4_,uStack_e0);
      puStack_2f8 = (undefined4 *)CONCAT44(puStack_2f8._4_4_,uStack_e4);
      auStack_320[0] = 0x141e4de8a;
      FUN_140fe7790(&iStack_c8,param_1,param_2,param_3);
      if (iStack_c8 == 4) {
        lVar11 = lStack_f8 * 0x40;
        puStack_80 = puStack_100 + lStack_f8 * 0x10;
        uStack_268 = puStack_100;
        uStack_260 = puStack_100;
        lStack_98 = lStack_108;
        uStack_258 = lStack_108;
        puStack_a0 = puStack_100;
        puVar7 = puStack_100;
        uStack_250 = puStack_80;
        if (lStack_f8 != 0) {
          puStack_90 = puStack_100;
          do {
            puVar7 = puStack_90 + 0x10;
            if (*(char *)(puStack_90 + 0xe) == '\x02') break;
            uStack_2b8 = *(undefined8 *)(puStack_90 + 0xc);
            uStack_2e8 = *puStack_90;
            uStack_2e4 = puStack_90[1];
            uStack_2e0 = puStack_90[2];
            uStack_2dc = puStack_90[3];
            uStack_2d8 = puStack_90[4];
            uStack_2d4 = puStack_90[5];
            uStack_2d0 = puStack_90[6];
            uStack_2cc = puStack_90[7];
            uStack_2c8 = puStack_90[8];
            uStack_2c4 = puStack_90[9];
            uStack_2c0 = puStack_90[10];
            uStack_2bc = puStack_90[0xb];
            uStack_2af = *(undefined4 *)((longlong)puStack_90 + 0x39);
            uStack_2ab = (undefined3)((uint)puStack_90[0xf] >> 8);
            uStack_2f0 = puStack_70;
            puStack_2f8 = param_5;
            auStack_320[0] = 0x141e4df6c;
            cStack_2b0 = *(char *)(puStack_90 + 0xe);
            puStack_90 = puVar7;
            FUN_141e4dc40(param_1,uStack_e4,uStack_e0,&uStack_2e8);
            lVar11 = lVar11 + -0x40;
            puVar7 = puStack_80;
          } while (lVar11 != 0);
        }
        puStack_80 = (undefined4 *)((ulonglong)((longlong)puStack_80 - (longlong)puVar7) >> 6);
        puStack_70 = (undefined *)0x0;
        uStack_260 = puVar7;
        puStack_90 = puVar7;
        while (puStack_80 != (undefined4 *)puStack_70) {
          puStack_70 = puStack_70 + 1;
          puVar7 = puVar7 + 0x10;
          auStack_320[0] = 0x141e4e723;
          FUN_140faa6e0();
          param_5 = puVar7;
        }
        goto LAB_141e4e788;
      }
      uStack_258 = CONCAT44(uStack_b4,uStack_b8);
      uStack_250 = (undefined4 *)CONCAT44(uStack_ac,uStack_b0);
      uStack_268 = (undefined4 *)CONCAT44(uStack_c4,iStack_c8);
      uStack_260 = (undefined4 *)CONCAT44(uStack_bc,uStack_c0);
      uStack_64 = 1;
      puStack_2f8 = (undefined4 *)&UNK_142f8f5b8;
      auStack_320[0] = 0x141e4ee6f;
      FUN_1428d9760(&UNK_142f8f358,0x2b,&uStack_268,&UNK_142f8f338);
    }
    else {
      uStack_b8 = uStack_d8;
      uStack_b4 = uStack_d4;
      uStack_b0 = uStack_d0;
      uStack_ac = uStack_cc;
      iStack_c8 = iStack_e8;
      uStack_c4 = uStack_e4;
      uStack_c0 = uStack_e0;
      uStack_bc = uStack_dc;
      uStack_64 = 1;
      puStack_2f8 = (undefined4 *)&UNK_142f8f4e8;
      auStack_320[0] = 0x141e4ed7f;
      FUN_1428d9760(&UNK_142f8f358,0x2b,&iStack_c8,&UNK_142f8f338);
    }
  }
  else {
    uStack_120 = 0;
    uStack_1b0 = _DAT_142bc2738;
    uStack_1a8 = _UNK_142bc2740;
    uStack_1a0 = _DAT_142bc2748;
    uStack_198 = _UNK_142bc2750;
    uStack_190 = _DAT_142bc2758;
    uStack_188 = _UNK_142bc2760;
    uStack_180 = _DAT_142bc2768;
    uStack_178 = _UNK_142bc2770;
    uStack_170 = _DAT_142bc2778;
    uStack_168 = _UNK_142bc2780;
    uStack_208 = 0;
    uStack_160 = _DAT_142bc2788;
    uStack_158 = _UNK_142bc2790;
    uStack_150 = _DAT_142bc2798;
    uStack_148 = _UNK_142bc27a0;
    uStack_1f0 = _DAT_142bc26f8;
    uStack_1e8 = _UNK_142bc2700;
    uStack_200 = _DAT_142bc26e8;
    uStack_1f8 = _UNK_142bc26f0;
    uStack_1d0 = _DAT_142bc2718;
    uStack_1c8 = _UNK_142bc2720;
    uStack_1e0 = _DAT_142bc2708;
    uStack_1d8 = _UNK_142bc2710;
    uStack_11b = 0x7070704;
    uStack_117 = 0x409;
    uStack_1c0 = _DAT_142bc2728;
    uStack_1b8 = _UNK_142bc2730;
    uStack_11e = 0;
    uStack_140 = 2;
    uStack_128 = _DAT_1429345b0;
    uStack_268 = (undefined4 *)_DAT_142bc2680;
    uStack_260 = (undefined4 *)_UNK_142bc2688;
    uStack_258 = _DAT_142bc2690;
    uStack_240 = _DAT_142bc26a8;
    uStack_250 = (undefined4 *)_DAT_142bc2698;
    uStack_248 = _UNK_142bc26a0;
    uStack_228 = _DAT_142bc26c0;
    uStack_238 = _DAT_142bc26b0;
    uStack_230 = _UNK_142bc26b8;
    uStack_210 = _DAT_142bc26d8;
    uStack_220 = _DAT_142bc26c8;
    uStack_218 = _UNK_142bc26d0;
    uStack_11c = 0;
    uStack_138 = 0;
    uStack_130 = 0;
    uStack_61 = 1;
    auStack_320[0] = 0x141e4e3e5;
    FUN_140fe7600(&iStack_e8,param_1,&uStack_268);
    if (iStack_e8 == 4) {
      uStack_61 = 1;
      uStack_2f0 = CONCAT44(uStack_2f0._4_4_,uStack_e0);
      puStack_2f8 = (undefined4 *)CONCAT44(puStack_2f8._4_4_,uStack_e4);
      auStack_320[0] = 0x141e4e426;
      FUN_140fe7790(&iStack_c8,param_1,param_2,param_3);
      if (iStack_c8 == 4) {
        uVar5 = *(undefined8 *)(pbStack_88 + 0x30);
        uVar3 = *(undefined8 *)(pbStack_88 + 0x20);
        lVar11 = *(longlong *)(pbStack_88 + 0x28);
        uStack_238 = 0;
        uStack_230 = 0;
        if (lVar11 < 0) {
          uVar10 = 0;
        }
        else {
          if (lVar11 == 0) {
            lVar12 = 1;
LAB_141e4e487:
            auStack_320[0] = 0x141e4e495;
            func_0x000142923c00(lVar12,uVar3,lVar11);
            uStack_250 = (undefined4 *)((ulonglong)uStack_250 & 0xffffffff00000000);
            uStack_220 = CONCAT44((float)((ulonglong)uVar5 >> 0x20) * _UNK_142f8f604,
                                  (float)uVar5 * _DAT_142f8f600);
            uStack_218 = _DAT_142f8f610;
            uStack_210 = uStack_210 & 0xffffffffff000000;
            uStack_248 = 0;
            uStack_240 = CONCAT35((int3)((ulonglong)uStack_240 >> 0x28),0x200000000);
            uStack_208 = 0;
            uStack_61 = 1;
            auStack_320[0] = 0x141e4e512;
            uStack_268 = (undefined4 *)lVar11;
            uStack_260 = (undefined4 *)lVar12;
            uStack_258 = lVar11;
            FUN_140fe0470(&iStack_c8,param_5,&uStack_268,&UNK_142f8f6c0);
            plVar8 = (longlong *)CONCAT44(uStack_bc,uStack_c0);
            if ((longlong *)CONCAT44(uStack_bc,uStack_c0) == (longlong *)0x0) {
              plVar8 = (longlong *)CONCAT44(uStack_b4,uStack_b8);
            }
            lVar11 = plVar8[2];
            if ((ulonglong)(*plVar8 - lVar11) < 0x28) {
              uStack_61 = 1;
              puStack_2f8 = (undefined4 *)0x1;
              auStack_320[0] = 0x141e4eb4c;
              FUN_1428a58d0(plVar8,lVar11,0x28,1);
            }
            lVar12 = plVar8[1];
            *(code **)(lVar12 + lVar11) = FUN_140fdf5f0;
            *(code **)(lVar12 + 8 + lVar11) = FUN_141e4f860;
            *(undefined8 *)(lVar12 + 0x10 + lVar11) = uStack_a8;
            *(undefined **)(lVar12 + 0x18 + lVar11) = puStack_70;
            *(undefined1 *)(lVar12 + 0x20 + lVar11) = 0;
            lVar12 = lVar11 + 0x28;
            plVar8[2] = lVar12;
            if (pbStack_88[0x38] != 0) {
              if ((ulonglong)(*plVar8 - lVar12) < 0x20) {
                uStack_61 = 1;
                puStack_2f8 = (undefined4 *)0x1;
                auStack_320[0] = 0x141e4eba3;
                FUN_1428a58d0(plVar8,lVar12,0x20,1);
              }
              lVar12 = plVar8[1];
              *(code **)(lVar12 + 0x28 + lVar11) = FUN_140fde4d0;
              *(code **)(lVar12 + 0x30 + lVar11) = FUN_141e4f860;
              *(undefined8 *)(lVar12 + 0x38 + lVar11) = uStack_a8;
              *(undefined1 *)(lVar12 + 0x40 + lVar11) = 0;
              plVar8[2] = lVar11 + 0x48;
                    /* WARNING: Could not recover jumptable at 0x000141e4e5db. Too many branches */
                    /* WARNING: Treating indirect jump as call */
              (*(code *)(&UNK_142f8f774 + *(int *)(&UNK_142f8f774 + (ulonglong)uStack_74 * 4)))();
              return;
            }
                    /* WARNING: Could not recover jumptable at 0x000141e4e5f1. Too many branches */
                    /* WARNING: Treating indirect jump as call */
            (*(code *)(&UNK_142f8f774 + *(int *)(&UNK_142f8f774 + (ulonglong)uStack_74 * 4)))();
            return;
          }
          uVar10 = 1;
          auStack_320[0] = 0x141e4e473;
          lVar12 = func_0x000140613c10(lVar11,1);
          if (lVar12 != 0) goto LAB_141e4e487;
        }
        uStack_61 = 1;
        auStack_320[0] = 0x141e4ef21;
        FUN_1428d8fe3(uVar10,lVar11,&UNK_142f8c828);
      }
      else {
        uStack_258 = CONCAT44(uStack_b4,uStack_b8);
        uStack_250 = (undefined4 *)CONCAT44(uStack_ac,uStack_b0);
        uStack_268 = (undefined4 *)CONCAT44(uStack_c4,iStack_c8);
        uStack_260 = (undefined4 *)CONCAT44(uStack_bc,uStack_c0);
        uStack_61 = 1;
        puStack_2f8 = (undefined4 *)&UNK_142f8f5b8;
        auStack_320[0] = 0x141e4ef03;
        FUN_1428d9760(&UNK_142f8f358,0x2b,&uStack_268,&UNK_142f8f338);
      }
    }
    else {
      uStack_b8 = uStack_d8;
      uStack_b4 = uStack_d4;
      uStack_b0 = uStack_d0;
      uStack_ac = uStack_cc;
      iStack_c8 = iStack_e8;
      uStack_c4 = uStack_e4;
      uStack_c0 = uStack_e0;
      uStack_bc = uStack_dc;
      uStack_61 = 1;
      puStack_2f8 = (undefined4 *)&UNK_142f8f4e8;
      auStack_320[0] = 0x141e4ee25;
      FUN_1428d9760(&UNK_142f8f358,0x2b,&iStack_c8,&UNK_142f8f338);
    }
  }
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

