
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1418c46c0(longlong param_1,ulonglong param_2)

{
  uint uVar1;
  code *pcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  code *pcVar9;
  undefined4 *puVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 uStack_1e0;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined8 uStack_198;
  longlong lStack_190;
  undefined8 *puStack_188;
  undefined8 uStack_180;
  ulonglong uStack_178;
  ulonglong uStack_170;
  code *pcStack_168;
  undefined4 uStack_160;
  uint uStack_15c;
  undefined4 *puStack_158;
  code *pcStack_150;
  undefined8 uStack_148;
  undefined4 *puStack_140;
  code *pcStack_138;
  code **ppcStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined *puStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  code **ppcStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  code *pcStack_c0;
  code *pcStack_b8;
  undefined4 **ppuStack_b0;
  code *pcStack_a8;
  undefined4 **ppuStack_a0;
  undefined *puStack_98;
  uint *puStack_90;
  code *pcStack_88;
  undefined8 *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  longlong lStack_60;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  puVar6 = puRam000000014362c3a8;
  uStack_48 = 0xfffffffffffffffe;
  uStack_170 = param_2 >> 2;
  if (2 < *puRam000000014362c3a8) {
    pcStack_c0 = (code *)&uStack_170;
    pcStack_b8 = FUN_140d8d6c0;
    puVar8 = (undefined8 *)func_0x00014067dd80(&UNK_142d06b88);
    uStack_120 = *puVar8;
    uStack_118 = puVar8[1];
    uStack_f4 = *(undefined4 *)(puVar8 + 2);
    uStack_110 = 3;
    puStack_108 = &UNK_142d06ba0;
    uStack_100 = 0x14;
    puStack_f0 = &UNK_142d06b48;
    uStack_e8 = 2;
    ppcStack_e0 = &pcStack_c0;
    uStack_d8 = 1;
    uStack_d0 = 0;
    puStack_140 = (undefined4 *)0x0;
    pcStack_138 = (code *)&UNK_142d06ba0;
    ppcStack_130 = (code **)0x14;
    uStack_128 = 0;
    uStack_f8 = 1;
    func_0x000140d4a330(&uStack_49,&puStack_140);
  }
  uVar7 = uStack_170;
  uVar12 = 0;
  while( true ) {
    if (uVar7 <= uVar12) {
      return;
    }
    uVar11 = uVar12 * 4 + 4;
    uStack_178 = uVar12;
    if ((uVar12 & 0x3fffffffffffffff) == 0x3fffffffffffffff) break;
    if (param_2 < uVar11) goto code_r0x0001418c4f0d;
    uVar1 = *(uint *)(param_1 + uVar12 * 0x10);
    uStack_15c = uVar1 & 0xffffff;
    uStack_160 = *(undefined4 *)(param_1 + 4 + uVar12 * 0x10);
    uStack_50 = *(undefined4 *)(param_1 + 8 + uVar12 * 0x10);
    uStack_54 = *(undefined4 *)(param_1 + 0xc + uVar12 * 0x10);
    switch(uVar1 >> 0x18) {
    case 1:
      pcStack_c0 = (code *)&uStack_50;
      pcStack_b8 = FUN_140d8d560;
      ppuStack_b0 = (undefined4 **)&uStack_54;
      pcStack_a8 = FUN_140d8d560;
      puStack_140 = (undefined4 *)&UNK_142d06be0;
      pcStack_138 = (code *)0x3;
      uStack_128 = 2;
      uStack_120 = 0;
      ppcStack_130 = &pcStack_c0;
      FUN_140d79f90(&uStack_1a8,&puStack_140);
      uStack_78 = (undefined4 *)CONCAT44(uStack_1a4,uStack_1a8);
      uStack_70 = (undefined4 *)CONCAT44(uStack_19c,uStack_1a0);
      uStack_68 = uStack_198;
      break;
    case 2:
      puVar10 = (undefined4 *)func_0x000140613c10(0x10,1);
      uVar5 = _UNK_142d06c1c;
      uVar4 = _UNK_142d06c18;
      uVar3 = _UNK_142d06c14;
      if (puVar10 == (undefined4 *)0x0) {
code_r0x0001418c4f4d:
        FUN_1428d8fe3(1,0x10,&UNK_142d06b10);
        pcVar2 = (code *)swi(3);
        (*pcVar2)();
        return;
      }
      *puVar10 = _UNK_142d06c10;
      puVar10[1] = uVar3;
      puVar10[2] = uVar4;
      puVar10[3] = uVar5;
      uStack_78 = (undefined4 *)0x10;
      uStack_68 = 0x10;
      uStack_70 = puVar10;
      break;
    case 3:
      pcStack_c0 = (code *)&uStack_50;
      pcStack_b8 = FUN_140d8d560;
      ppuStack_b0 = (undefined4 **)&uStack_54;
      pcStack_a8 = FUN_140d8d560;
      puStack_140 = (undefined4 *)&UNK_142d06c38;
      pcStack_138 = (code *)0x3;
      uStack_128 = 2;
      uStack_120 = 0;
      ppcStack_130 = &pcStack_c0;
      FUN_140d79f90(&uStack_1c0,&puStack_140);
      uStack_78 = (undefined4 *)CONCAT44(uStack_1bc,uStack_1c0);
      uStack_70 = (undefined4 *)CONCAT44(uStack_1b4,uStack_1b8);
      uStack_68 = uStack_1b0;
      break;
    case 4:
      pcStack_c0 = (code *)&uStack_50;
      pcStack_b8 = FUN_140d8d560;
      ppuStack_b0 = (undefined4 **)&uStack_54;
      pcStack_a8 = FUN_140d8d560;
      puStack_140 = (undefined4 *)&UNK_142d06c70;
      pcStack_138 = (code *)0x3;
      uStack_128 = 2;
      uStack_120 = 0;
      ppcStack_130 = &pcStack_c0;
      FUN_140d79f90(&uStack_1d8,&puStack_140);
      uStack_78 = (undefined4 *)CONCAT44(uStack_1d4,uStack_1d8);
      uStack_70 = (undefined4 *)CONCAT44(uStack_1cc,uStack_1d0);
      uStack_68 = uStack_1c8;
      break;
    case 5:
      pcStack_c0 = (code *)&uStack_50;
      pcStack_b8 = FUN_140d8d560;
      ppuStack_b0 = (undefined4 **)&uStack_54;
      pcStack_a8 = FUN_140d8d560;
      puStack_140 = (undefined4 *)&UNK_142d06ca8;
      pcStack_138 = (code *)0x3;
      uStack_128 = 2;
      uStack_120 = 0;
      ppcStack_130 = &pcStack_c0;
      FUN_140d79f90(&uStack_1f0,&puStack_140);
      uStack_78 = (undefined4 *)CONCAT44(uStack_1ec,uStack_1f0);
      uStack_70 = (undefined4 *)CONCAT44(uStack_1e4,uStack_1e8);
      uStack_68 = uStack_1e0;
      break;
    case 6:
      pcStack_c0 = (code *)&uStack_50;
      pcStack_b8 = FUN_140d8d560;
      ppuStack_b0 = (undefined4 **)&uStack_54;
      pcStack_a8 = FUN_140d8d560;
      puStack_140 = (undefined4 *)&UNK_142d06ce0;
      pcStack_138 = (code *)0x3;
      uStack_128 = 2;
      uStack_120 = 0;
      ppcStack_130 = &pcStack_c0;
      FUN_140d79f90(&uStack_208,&puStack_140);
      uStack_78 = (undefined4 *)CONCAT44(uStack_204,uStack_208);
      uStack_70 = (undefined4 *)CONCAT44(uStack_1fc,uStack_200);
      uStack_68 = uStack_1f8;
      break;
    default:
      puVar10 = (undefined4 *)func_0x000140613c10(3,1);
      if (puVar10 == (undefined4 *)0x0) {
        FUN_1428d8fe3(1,3,&UNK_142d06b10);
        goto code_r0x0001418c4f4d;
      }
      *(undefined1 *)((longlong)puVar10 + 2) = 0x3f;
      *(undefined2 *)puVar10 = 0x3f3f;
      uStack_78 = (undefined4 *)0x3;
      uStack_68 = 3;
      uStack_70 = puVar10;
    }
    puStack_158 = &uStack_160;
    pcStack_150 = FUN_140d8cc50;
    puStack_140 = (undefined4 *)0x2;
    ppcStack_130 = (code **)0x0;
    uStack_128 = 8;
    uStack_120 = 0;
    uStack_118 = 0x800000020;
    uStack_110 = CONCAT71(uStack_110._1_7_,3);
    pcStack_c0 = (code *)&UNK_142d06d30;
    pcStack_b8 = (code *)0x2;
    ppuStack_b0 = &puStack_158;
    pcStack_a8 = (code *)0x1;
    puStack_98 = (undefined *)0x1;
    ppuStack_a0 = &puStack_140;
    FUN_140d79f90(&lStack_190,&pcStack_c0);
    lStack_60 = lStack_190;
    puStack_80 = puStack_188;
    FUN_140d59140(&puStack_140,puStack_188,uStack_180);
    if (lStack_60 != 0) {
      func_0x000140613c20(puStack_80,lStack_60,1);
    }
    pcVar2 = pcStack_138;
    uVar1 = (uint)pcStack_138;
    if (SBORROW8(0,(longlong)puStack_140)) {
      pcStack_c0 = pcStack_138;
      pcVar9 = (code *)func_0x000140613c10(3,1);
      if (pcVar9 == (code *)0x0) goto code_r0x0001418c4f1f;
      pcVar9[2] = (code)0x3f;
      *(undefined2 *)pcVar9 = 0x3f3f;
      puStack_158 = (undefined4 *)0x3;
      uStack_148 = (code **)0x3;
      pcStack_150 = pcVar9;
      if ((uVar1 & 3) == 1) {
        pcStack_168 = pcVar2 + -1;
        lStack_60 = *(longlong *)(pcVar2 + -1);
        puStack_80 = *(undefined8 **)(pcVar2 + 7);
        if ((code *)*puStack_80 != (code *)0x0) {
          (*(code *)*puStack_80)(lStack_60);
        }
        if (puStack_80[1] != 0) {
          func_0x000140613c20(lStack_60,puStack_80[1],puStack_80[2]);
        }
        func_0x000140613c20(pcStack_168,0x18,8);
      }
      uVar11 = *puVar6;
    }
    else {
      uVar11 = *puVar6;
      uStack_148 = ppcStack_130;
      puStack_158 = puStack_140;
      pcStack_150 = pcStack_138;
    }
    if (2 < uVar11) {
      pcStack_c0 = (code *)&uStack_178;
      pcStack_b8 = FUN_140d8d6c0;
      ppuStack_b0 = (undefined4 **)&uStack_78;
      pcStack_a8 = (code *)&UNK_14003bad0;
      ppuStack_a0 = &puStack_158;
      puStack_98 = &UNK_14003bad0;
      puStack_90 = &uStack_15c;
      pcStack_88 = FUN_140d8d560;
      puVar8 = (undefined8 *)func_0x00014067dd80(&UNK_142d06da0);
      uStack_120 = *puVar8;
      uStack_118 = puVar8[1];
      uStack_f4 = *(undefined4 *)(puVar8 + 2);
      uStack_110 = 3;
      puStack_108 = &UNK_142d06ba0;
      uStack_100 = 0x14;
      puStack_f0 = &UNK_142d06d60;
      uStack_e8 = 4;
      uStack_d8 = 4;
      uStack_d0 = 0;
      puStack_140 = (undefined4 *)0x0;
      pcStack_138 = (code *)&UNK_142d06ba0;
      ppcStack_130 = (code **)0x14;
      uStack_128 = 0;
      uStack_f8 = 1;
      ppcStack_e0 = &pcStack_c0;
      func_0x000140d4a330(&uStack_49,&puStack_140);
    }
    if (puStack_158 != (undefined4 *)0x0) {
      func_0x000140613c20(pcStack_150,puStack_158,1);
    }
    if (uStack_78 != (undefined4 *)0x0) {
      func_0x000140613c20(uStack_70,uStack_78,1);
    }
    uVar12 = uVar12 + 1;
  }
  FUN_1428d9850(0xfffffffffffffffc,uVar11,&UNK_142d06bb8);
code_r0x0001418c4f0d:
  FUN_1428d9840(uVar11,param_2,&UNK_142d06bb8);
code_r0x0001418c4f1f:
  FUN_1428d8fe3(1,3,&UNK_142d06b10);
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

