// 0x141e4d1e0 undefined FUN_141e4d1e0(void) body_bytes=1220

/* WARNING: Possible PIC construction at 0x000141e4d615: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x000140d65824) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141e4d1e0(longlong param_1,longlong param_2,char param_3,longlong *param_4,
                  undefined8 param_5,undefined *param_6)

{
  longlong lVar1;
  char *pcVar2;
  undefined8 *puVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  code *pcVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  undefined8 *unaff_RBP;
  undefined8 *unaff_RSI;
  longlong lVar10;
  undefined8 auStack_2c0 [5];
  undefined *puStack_298;
  undefined8 uStack_290;
  undefined1 auStack_288 [16];
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined8 *puStack_268;
  undefined8 *puStack_260;
  longlong lStack_258;
  undefined8 *puStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
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
  undefined1 auStack_138 [16];
  undefined8 uStack_128;
  undefined2 uStack_120;
  undefined2 uStack_11e;
  undefined1 uStack_11c;
  undefined4 uStack_11b;
  undefined2 uStack_117;
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
  char cStack_d0;
  undefined4 uStack_cf;
  undefined3 uStack_cb;
  int iStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong *plStack_a8;
  undefined8 *puStack_a0;
  longlong lStack_98;
  undefined8 *puStack_90;
  undefined *puStack_88;
  undefined8 *puStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  puVar3 = &uStack_238;
  uStack_70 = 0xfffffffffffffffe;
  lVar4 = param_4[2];
  if (lVar4 == 0) {
    lVar4 = *param_4;
    if (lVar4 == 0) {
      return;
    }
    puVar9 = (undefined8 *)param_4[1];
  }
  else {
    puStack_88 = param_6;
    lVar5 = *(longlong *)(param_1 + 8);
    lVar10 = 0;
    do {
      plStack_a8 = param_4;
      if (*(longlong *)(param_1 + 0x10) << 5 == lVar10) {
        uStack_71 = 1;
        auStack_2c0[0] = 0x141e4d659;
        FUN_1428d9310(&UNK_142f8f668);
        goto LAB_141e4d6a2;
      }
      lVar1 = lVar10 + 0x20;
      pcVar2 = (char *)(lVar5 + 0x18 + lVar10);
      lVar10 = lVar1;
    } while (*pcVar2 != param_3);
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
    puStack_268 = _DAT_142bc2680;
    puStack_260 = _UNK_142bc2688;
    lStack_258 = _DAT_142bc2690;
    uStack_240 = _DAT_142bc26a8;
    puStack_250 = _DAT_142bc2698;
    uStack_248 = _UNK_142bc26a0;
    uStack_228 = _DAT_142bc26c0;
    uStack_238 = _DAT_142bc26b0;
    uStack_230 = _UNK_142bc26b8;
    uStack_210 = _DAT_142bc26d8;
    uStack_220 = _DAT_142bc26c8;
    uStack_218 = _UNK_142bc26d0;
    uStack_11c = 0;
    auStack_138 = (undefined1  [16])0x0;
    uStack_71 = 1;
    auStack_2c0[0] = 0x141e4d3cf;
    FUN_140fe7600(&iStack_c8,param_2,&puStack_268);
    if (iStack_c8 != 4) {
      uStack_278 = uStack_b8;
      uStack_274 = uStack_b4;
      uStack_270 = uStack_b0;
      uStack_26c = uStack_ac;
      uStack_71 = 1;
      puStack_298 = &UNK_142f8f4e8;
      auStack_2c0[0] = 0x141e4d6a2;
      FUN_1428d9760(&UNK_142f8f358,0x2b,auStack_288,&UNK_142f8f338);
LAB_141e4d6a2:
                    /* WARNING: Does not return */
      pcVar7 = (code *)invalidInstructionException();
      (*pcVar7)();
    }
    uVar8 = *(undefined8 *)(lVar5 + -0x20 + lVar1);
    lVar10 = *(longlong *)(param_2 + 0xa8);
    if (lVar10 == *(longlong *)(param_2 + 0x98)) {
      uStack_71 = 1;
      auStack_2c0[0] = 0x141e4d415;
      FUN_140562ab0(param_2 + 0x98,&UNK_142f8f5a0);
    }
    lVar6 = *(longlong *)(param_2 + 0xa0);
    *(undefined8 *)(lVar6 + lVar10 * 0x28) = 1;
    *(undefined8 *)(lVar6 + 8 + lVar10 * 0x28) = 0;
    puVar9 = (undefined8 *)(lVar6 + 0x10 + lVar10 * 0x28);
    *puVar9 = CONCAT44(uStack_c0,uStack_c4);
    puVar9[1] = uVar8;
    *(undefined4 *)(lVar6 + 0x20 + lVar10 * 0x28) = 0x41200000;
    *(undefined1 *)(lVar6 + 0x24 + lVar10 * 0x28) = 0;
    *(longlong *)(param_2 + 0xa8) = lVar10 + 1;
    lStack_258 = *plStack_a8;
    puStack_90 = (undefined8 *)plStack_a8[1];
    lVar10 = lVar4 * 0x40;
    puStack_250 = puStack_90 + lVar4 * 8;
    puStack_268 = puStack_90;
    puStack_a0 = puStack_90;
    lStack_98 = lStack_258;
    puStack_80 = puStack_250;
    do {
      puVar9 = puStack_90 + 8;
      if (*(char *)(puStack_90 + 7) == '\x02') break;
      uStack_d8 = puStack_90[6];
      uStack_108 = *puStack_90;
      uStack_100 = puStack_90[1];
      uStack_f8 = *(undefined4 *)(puStack_90 + 2);
      uStack_f4 = *(undefined4 *)((longlong)puStack_90 + 0x14);
      uStack_f0 = *(undefined4 *)(puStack_90 + 3);
      uStack_ec = *(undefined4 *)((longlong)puStack_90 + 0x1c);
      uStack_e8 = *(undefined4 *)(puStack_90 + 4);
      uStack_e4 = *(undefined4 *)((longlong)puStack_90 + 0x24);
      uStack_e0 = *(undefined4 *)(puStack_90 + 5);
      uStack_dc = *(undefined4 *)((longlong)puStack_90 + 0x2c);
      uStack_cf = *(undefined4 *)((longlong)puStack_90 + 0x39);
      uStack_cb = (undefined3)((uint)*(undefined4 *)((longlong)puStack_90 + 0x3c) >> 8);
      uStack_290 = *(undefined8 *)(lVar5 + -0x18 + lVar1);
      puStack_298 = puStack_88;
      auStack_2c0[0] = 0x141e4d533;
      cStack_d0 = *(char *)(puStack_90 + 7);
      puStack_90 = puStack_90 + 8;
      FUN_141e4dc40(param_2,uStack_c4,uStack_c0,&uStack_108);
      lVar10 = lVar10 + -0x40;
      puVar9 = puStack_80;
    } while (lVar10 != 0);
    puStack_80 = (undefined8 *)((ulonglong)((longlong)puStack_80 - (longlong)puVar9) >> 6);
    puStack_88 = (undefined *)0x0;
    unaff_RSI = (undefined8 *)&uStack_cf;
    puStack_260 = puVar9;
    puStack_90 = puVar9;
    while (puStack_80 != (undefined8 *)puStack_88) {
      puStack_88 = (undefined *)((longlong)puStack_88 + 1);
      puVar9 = puVar9 + 8;
      auStack_2c0[0] = 0x141e4d5f3;
      FUN_140faa6e0();
      unaff_RSI = puVar9;
    }
    if (lStack_98 == 0) {
      *(undefined4 *)(lVar5 + -0x10 + lVar1) = uStack_c4;
      *(undefined4 *)(lVar5 + -0xc + lVar1) = uStack_c0;
      return;
    }
    register0x00000020 = (BADSPACEBASE *)auStack_2c0;
    auStack_2c0[0] = 0x141e4d61a;
    puVar9 = puStack_a0;
    unaff_RBP = puVar3;
    lVar4 = lStack_98;
  }
  *(undefined8 **)((longlong)register0x00000020 + -8) = unaff_RBP;
  *(undefined8 **)((longlong)register0x00000020 + -0x10) = unaff_RSI;
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar8 = GetProcessHeap(puVar9,lVar4 << 6);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar8,0,puVar9);
  return;
}

