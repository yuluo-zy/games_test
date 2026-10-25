// FUN_1408ff590 0x1408ff590

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408ff590(longlong param_1,byte *param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong *plVar5;
  byte *pbVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined8 *puVar10;
  longlong lVar11;
  longlong lVar12;
  undefined1 auStack_240 [32];
  longlong lStack_220;
  undefined8 uStack_218;
  longlong lStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  longlong lStack_188;
  undefined8 *puStack_180;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
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
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  byte *pbStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  pbStack_58 = param_2;
  uVar3 = FUN_1408ff430();
  if ((*pbStack_58 & 1) == 0) {
    uVar4 = FUN_14145b330();
    pbStack_58[0] = 1;
    pbStack_58[1] = 0;
    pbStack_58[2] = 0;
    pbStack_58[3] = 0;
    pbStack_58[4] = 0;
    pbStack_58[5] = 0;
    pbStack_58[6] = 0;
    pbStack_58[7] = 0;
    *(undefined8 *)(pbStack_58 + 8) = uVar4;
  }
  else {
    uVar4 = *(undefined8 *)(pbStack_58 + 8);
  }
  plVar5 = (longlong *)FUN_1400d9b20(0);
  if (plVar5 == (longlong *)0x0) {
    FUN_1428d80d0(&UNK_142c3aaa0);
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  lVar9 = param_1 + 0x28;
  uStack_d8 = *plVar5;
  uStack_d0 = (undefined8 *)plVar5[1];
  *plVar5 = *plVar5 + 1;
  uStack_f8 = _UNK_142c6bf00;
  uStack_f0 = _UNK_142c6bf08;
  uStack_e8._0_4_ = _UNK_142c6bf10;
  uStack_e8._4_4_ = _UNK_142c6bf14;
  uStack_e0._0_4_ = _UNK_142c6bf18;
  uStack_e0._4_4_ = _UNK_142c6bf1c;
  puVar10 = &uStack_f8;
  FUN_14157d090(lVar9);
  uStack_108 = (undefined4)uStack_d8;
  uStack_104 = uStack_d8._4_4_;
  uStack_100 = (undefined4)uStack_d0;
  uStack_fc = uStack_d0._4_4_;
  uStack_118 = (undefined4)uStack_e8;
  uStack_114 = uStack_e8._4_4_;
  uStack_110 = (undefined4)uStack_e0;
  uStack_10c = uStack_e0._4_4_;
  uStack_128 = (undefined4)uStack_f8;
  uStack_124 = uStack_f8._4_4_;
  uStack_120 = (undefined4)uStack_f0;
  uStack_11c = uStack_f0._4_4_;
  lVar1 = *(longlong *)(pbStack_58 + 0x18);
  lVar11 = *(longlong *)(pbStack_58 + 0x20);
  pbVar6 = (byte *)FUN_140d4d740();
  if ((*pbVar6 & 1) == 0) {
    uStack_49 = 1;
    uStack_d8 = FUN_140d64e00();
    pbVar6[0] = 1;
    pbVar6[1] = 0;
    pbVar6[2] = 0;
    pbVar6[3] = 0;
    pbVar6[4] = 0;
    pbVar6[5] = 0;
    pbVar6[6] = 0;
    pbVar6[7] = 0;
    *(longlong *)(pbVar6 + 8) = uStack_d8;
    *(undefined8 **)(pbVar6 + 0x10) = puVar10;
  }
  else {
    uStack_d8 = *(longlong *)(pbVar6 + 8);
    puVar10 = *(undefined8 **)(pbVar6 + 0x10);
  }
  *(longlong *)(pbVar6 + 8) = uStack_d8 + 1;
  uStack_e8 = _UNK_142aa9140;
  uStack_e0 = _UNK_142aa9148;
  uStack_f8 = CONCAT44(_UNK_142aa9134,_UNK_142aa9130);
  uStack_f0 = CONCAT44(_UNK_142aa913c,_UNK_142aa9138);
  uStack_d0 = puVar10;
  if (lVar11 != 0) {
    FUN_1428bbae0(&uStack_f8,lVar11,&uStack_d8,1);
    lVar12 = 0;
    do {
      FUN_1402c8640(&uStack_f8,*(undefined4 *)(lVar1 + lVar12 * 8),
                    *(undefined4 *)(lVar1 + 4 + lVar12 * 8));
      lVar12 = lVar12 + 1;
    } while (lVar12 != lVar11);
  }
  lStack_188 = uStack_d8;
  puStack_180 = uStack_d0;
  uStack_198 = uStack_e8;
  uStack_190 = uStack_e0;
  uStack_1a8 = uStack_f8;
  uStack_1a0 = uStack_f0;
  uStack_d8 = CONCAT44(uStack_104,uStack_108);
  uStack_d0 = (undefined8 *)CONCAT44(uStack_fc,uStack_100);
  uStack_e8 = CONCAT44(uStack_114,uStack_118);
  uStack_e0 = CONCAT44(uStack_10c,uStack_110);
  uStack_f8 = CONCAT44(uStack_124,uStack_128);
  uStack_f0 = CONCAT44(uStack_11c,uStack_120);
  uStack_49 = 0;
  FUN_1409007b0(auStack_240,&uStack_f8,uVar3,&uStack_1a8,uVar4);
  FUN_140900ba0(param_1,auStack_240,0);
  lVar1 = param_1 + 0xb8;
  FUN_14157cec0(lVar1,lVar9,param_1 + 0x148);
  if (*(longlong *)(param_1 + 0xe8) != 0) {
    uVar8 = *(ulonglong *)(param_1 + 0xd8);
    if (uVar8 == 0) {
      uVar7 = 0;
    }
    else {
      func_0x000142923c20(*(undefined8 *)(param_1 + 0xd0),0xff,uVar8 + 0x11);
      uVar7 = (uVar8 + 1 & 0xfffffffffffffff8) - (uVar8 + 1 >> 3);
      if (uVar8 < 8) {
        uVar7 = uVar8;
      }
    }
    *(undefined8 *)(param_1 + 0xe8) = 0;
    *(ulonglong *)(param_1 + 0xe0) = uVar7;
  }
  lVar11 = *(longlong *)(param_1 + 200);
  *(undefined8 *)(param_1 + 200) = 0;
  if (lVar11 != 0) {
    puVar10 = (undefined8 *)(*(longlong *)(param_1 + 0xc0) + 8);
    do {
      if (puVar10[-1] != 0) {
        func_0x000140613c20(*puVar10,puVar10[-1] << 3,4);
      }
      puVar10 = puVar10 + 5;
      lVar11 = lVar11 + -1;
    } while (lVar11 != 0);
  }
  if (*(longlong *)(param_1 + 0x130) != 0) {
    uVar8 = *(ulonglong *)(param_1 + 0x120);
    if (uVar8 == 0) {
      uVar7 = 0;
    }
    else {
      func_0x000142923c20(*(undefined8 *)(param_1 + 0x118),0xff,uVar8 + 0x11);
      uVar7 = (uVar8 + 1 & 0xfffffffffffffff8) - (uVar8 + 1 >> 3);
      if (uVar8 < 8) {
        uVar7 = uVar8;
      }
    }
    *(undefined8 *)(param_1 + 0x130) = 0;
    *(ulonglong *)(param_1 + 0x128) = uVar7;
  }
  *(undefined8 *)(param_1 + 0x110) = 0;
  *(undefined1 *)(param_1 + 0x178) = 0;
  FUN_14157cec0(lVar1,lVar9,param_1 + 0x148);
  FUN_140b4d570(&uStack_170,lVar9);
  uStack_138 = *(undefined4 *)(param_1 + 0x60);
  uStack_134 = *(undefined4 *)(param_1 + 100);
  uStack_130 = *(undefined4 *)(param_1 + 0x68);
  uStack_12c = *(undefined4 *)(param_1 + 0x6c);
  FUN_140b4d320(&uStack_1f0,param_1 + 0x70);
  uVar3 = *(undefined8 *)(param_1 + 0xa8);
  uStack_1b0 = *(undefined8 *)(param_1 + 0xb0);
  uStack_f8 = uStack_170;
  uStack_f0 = uStack_168;
  uStack_e8 = uStack_160;
  uStack_e0 = uStack_158;
  uStack_d8 = uStack_150;
  uStack_d0 = (undefined8 *)uStack_148;
  uStack_c8 = uStack_140;
  uStack_c4 = uStack_13c;
  uStack_c0 = uStack_138;
  uStack_bc = uStack_134;
  uStack_b8 = CONCAT44(uStack_12c,uStack_130);
  uStack_b0 = uStack_1f0;
  uStack_a8 = uStack_1e8;
  uStack_a0 = uStack_1e0;
  uStack_9c = uStack_1dc;
  uStack_98 = uStack_1d8;
  uStack_94 = uStack_1d4;
  uStack_90 = uStack_1d0;
  uStack_8c = uStack_1cc;
  uStack_88 = uStack_1c8;
  uStack_84 = uStack_1c4;
  uStack_1b8._0_4_ = (undefined4)uVar3;
  uStack_1b8._4_4_ = (undefined4)((ulonglong)uVar3 >> 0x20);
  uStack_80 = uStack_1c0;
  uStack_7c = uStack_1bc;
  uStack_78 = (undefined4)uStack_1b8;
  uStack_74 = uStack_1b8._4_4_;
  uStack_1b8 = uVar3;
  uStack_70 = uStack_1b0;
  lStack_68 = lVar1;
  lStack_60 = param_1;
  FUN_1405e3f30(lVar1);
  lVar1 = lStack_60;
  lVar9 = *(longlong *)(lStack_60 + 0x120);
  if (lVar9 != 0) {
    uVar8 = lVar9 * 8 + 0x17U & 0xfffffffffffffff0;
    lVar9 = lVar9 + uVar8 + 0x11;
    if (lVar9 != 0) {
      func_0x000140613c20(*(longlong *)(lStack_60 + 0x118) - uVar8,lVar9,0x10);
    }
  }
  pbVar6 = pbStack_58;
  lVar9 = lStack_68;
  if (*(longlong *)(lVar1 + 0x100) != 0) {
    func_0x000140613c20(*(undefined8 *)(lVar1 + 0x108),*(longlong *)(lVar1 + 0x100) << 4,8);
  }
  func_0x000142923c00(lVar9,&uStack_f8,0x90);
  *(undefined1 *)(lVar1 + 0x178) = 1;
  if (lStack_220 != 0) {
    func_0x000140613c20(uStack_218,lStack_220 << 3,4);
  }
  if (lStack_208 != 0) {
    func_0x000140613c20(uStack_200,lStack_208 << 3,4);
  }
  lVar9 = *(longlong *)(pbVar6 + 0x10);
  if (lVar9 != 0) {
    func_0x000140613c20(*(undefined8 *)(pbVar6 + 0x18),lVar9 << 3,4);
  }
  return;
}

