
undefined8 * FUN_140bfd9d0(undefined8 *param_1,undefined8 param_2,undefined4 *param_3)

{
  code *pcVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined *puVar4;
  undefined *puVar5;
  undefined *puVar6;
  ulonglong uVar7;
  undefined **ppuVar8;
  undefined4 *puVar9;
  undefined1 uVar10;
  undefined8 *puStack_180;
  undefined8 *puStack_178;
  undefined8 *puStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined6 uStack_128;
  char cStack_122;
  undefined1 uStack_121;
  undefined8 uStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined **ppuStack_e8;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined8 **ppuStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 *puStack_98;
  undefined8 *puStack_90;
  undefined4 *puStack_88;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined *puStack_60;
  undefined **ppuStack_58;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_140c0eb40(&puStack_180,param_3,param_2);
  if (cStack_122 == '\a') {
    puVar6 = (undefined *)(uStack_168 & 0xffffffff);
    uVar7 = (ulonglong)uStack_160 & 0xffffffff;
    puVar5 = (undefined *)0x1;
    if ((undefined *)((longlong)puStack_180 + -2) < (undefined *)0x4) {
      puVar5 = (undefined *)((longlong)puStack_180 + -2);
    }
    puVar9 = (undefined4 *)CONCAT44(uStack_154,uStack_158);
    uVar10 = (longlong)*(int *)(&UNK_142b174dc + (longlong)puVar5 * 4) == -0x142b174dc;
    puStack_88 = param_3;
    switch(puStack_180) {
    case (undefined8 *)0x2:
      goto code_r0x000140bfda60;
    default:
      uStack_70 = (undefined **)CONCAT44(uStack_14c,uStack_150);
      uStack_68 = (undefined8 **)CONCAT44(uStack_144,uStack_148);
      ppuVar8 = (undefined **)puStack_170;
      uStack_160._0_4_ = (undefined4)uStack_168;
      puStack_80 = puVar6;
      uStack_78 = puVar9;
      if (3 < *puRam000000014362c3a8) {
        ppuStack_58 = (undefined **)puStack_170;
        puStack_98 = &uStack_78;
        puStack_90 = (undefined8 *)&UNK_14003bad0;
        puVar3 = (undefined8 *)func_0x00014067dd80(&UNK_142b16d58);
        uStack_f8 = *puVar3;
        uStack_f0 = *(undefined4 *)(puVar3 + 1);
        uStack_ec = *(undefined4 *)((longlong)puVar3 + 0xc);
        uStack_cc = *(undefined4 *)(puVar3 + 2);
        ppuStack_e8 = (undefined **)0x4;
        puStack_e0 = &UNK_142b16d70;
        uStack_d8 = 0x18;
        puStack_c8 = &UNK_142b16d38;
        uStack_c0 = 2;
        ppuStack_b8 = &puStack_98;
        uStack_b0 = 1;
        uStack_a8 = 0;
        puStack_118 = (undefined *)0x0;
        uStack_110 = (undefined8 *)&UNK_142b16d70;
        uStack_108 = (undefined8 *)0x18;
        uStack_100 = 0;
        uStack_d0 = 1;
        func_0x000140d4a330(&uStack_4c,&puStack_118);
        uStack_160._0_4_ = SUB84(puStack_80,0);
        ppuVar8 = ppuStack_58;
      }
      if (uStack_78 != (undefined4 *)0x0) {
        ppuStack_58 = (undefined **)puStack_178;
        func_0x000140613c20(uStack_70,uStack_78,1);
        uStack_160._0_4_ = SUB84(puStack_80,0);
        puStack_178 = ppuStack_58;
      }
      puVar9 = (undefined4 *)((ulonglong)uStack_160._4_4_ << 0x20 | uVar7);
      puStack_170 = puStack_178;
      break;
    case (undefined8 *)0x4:
      if (3 < *puRam000000014362c3a8) {
        ppuStack_58 = (undefined **)puStack_170;
        puStack_80 = puVar6;
        puVar3 = (undefined8 *)func_0x00014067dd80(&UNK_142b16db8);
        uStack_f8 = *puVar3;
        uStack_f0 = *(undefined4 *)(puVar3 + 1);
        uStack_ec = *(undefined4 *)((longlong)puVar3 + 0xc);
        uStack_cc = *(undefined4 *)(puVar3 + 2);
        ppuStack_e8 = (undefined **)0x4;
        puStack_e0 = &UNK_142b16d70;
        uStack_d8 = 0x18;
        puStack_c8 = &UNK_142b16da8;
        uStack_c0 = 1;
        ppuStack_b8 = (undefined8 **)0x8;
        puStack_118 = (undefined *)0x0;
        uStack_b0 = 0;
        uStack_a8 = 0;
        uStack_110 = (undefined8 *)&UNK_142b16d70;
        uStack_108 = (undefined8 *)0x18;
        uStack_100 = 0;
        uStack_d0 = 1;
        func_0x000140d4a330(&uStack_4c,&puStack_118);
        puStack_170 = ppuStack_58;
        puVar6 = puStack_80;
      }
      ppuVar8 = (undefined **)((ulonglong)uStack_168._4_4_ << 0x20 | (ulonglong)puVar6);
      puStack_180 = puStack_178;
      uStack_168._4_4_ = uStack_160._4_4_;
      break;
    case (undefined8 *)0x5:
      uStack_4c = (undefined4)uStack_168;
      puStack_98 = puStack_178;
      puStack_90 = puStack_170;
      uStack_78 = &uStack_4c;
      uStack_70 = (undefined **)&UNK_140c8f550;
      uStack_68 = &puStack_98;
      puStack_60 = &UNK_140109e90;
      puStack_118 = &UNK_142b16fc0;
      uStack_110 = (undefined8 *)0x2;
      uStack_f8 = 0;
      uStack_108 = &uStack_78;
      uStack_100 = 2;
      puVar5 = &UNK_142b16fe0;
      ppuVar8 = &puStack_118;
      FUN_1428d9390();
      goto code_r0x000140bfdf4d;
    }
    puVar2 = puStack_88;
    uStack_168 = CONCAT44(uStack_168._4_4_,(undefined4)uStack_160);
    puStack_178 = puStack_170;
    puStack_170 = ppuVar8;
    uStack_160 = puVar9;
    FUN_140bfd720(&puStack_118,param_2,&puStack_180,puStack_88);
    if (puStack_118 == (undefined *)0x6) {
      FUN_140be24b0(puVar2);
      puStack_88[4] = (undefined4)uStack_100;
      puStack_88[5] = uStack_100._4_4_;
      puStack_88[6] = (undefined4)uStack_f8;
      puStack_88[7] = uStack_f8._4_4_;
      *puStack_88 = (undefined4)uStack_110;
      puStack_88[1] = uStack_110._4_4_;
      puStack_88[2] = (undefined4)uStack_108;
      puStack_88[3] = uStack_108._4_4_;
      FUN_140c0eb40(&puStack_180,puStack_88,param_2);
      if (cStack_122 != '\a') goto code_r0x000140bfde23;
      switch(puStack_180) {
      case (undefined8 *)0x2:
code_r0x000140bfda60:
        *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
        return param_1;
      default:
        uStack_78 = (undefined4 *)CONCAT44(uStack_154,uStack_158);
        uStack_70 = (undefined **)CONCAT44(uStack_14c,uStack_150);
        if (1 < *puRam000000014362c3a8) {
          puStack_98 = &uStack_78;
          puStack_90 = (undefined8 *)&UNK_14003bad0;
          puVar3 = (undefined8 *)func_0x00014067dd80(&UNK_142b16e10);
          uStack_f8 = *puVar3;
          uStack_f0 = *(undefined4 *)(puVar3 + 1);
          uStack_ec = *(undefined4 *)((longlong)puVar3 + 0xc);
          uStack_cc = *(undefined4 *)(puVar3 + 2);
          ppuStack_e8 = (undefined **)0x2;
          puStack_e0 = &UNK_142b16d70;
          uStack_d8 = 0x18;
          puStack_c8 = &UNK_142b16e00;
          uStack_c0 = 1;
          ppuStack_b8 = &puStack_98;
          uStack_b0 = 1;
          uStack_a8 = 0;
          puStack_118 = (undefined *)0x0;
          uStack_110 = (undefined8 *)&UNK_142b16d70;
          uStack_108 = (undefined8 *)0x18;
          uStack_100 = 0;
          uStack_d0 = 1;
          func_0x000140d4a330(&uStack_4c,&puStack_118);
        }
        *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
        if (uStack_78 == (undefined4 *)0x0) {
          return param_1;
        }
        break;
      case (undefined8 *)0x5:
        uStack_4c = (undefined4)uStack_168;
        puStack_98 = puStack_178;
        puStack_90 = puStack_170;
        FUN_140befd60(param_2);
        uStack_78 = &uStack_4c;
        uStack_70 = (undefined **)&UNK_140c8f550;
        uStack_68 = &puStack_98;
        puStack_60 = &UNK_140109e90;
        puStack_118 = &UNK_142b16ea8;
        uStack_110 = (undefined8 *)0x2;
        uStack_f8 = 0;
        uStack_108 = &uStack_78;
        uStack_100 = 2;
        FUN_1428d9390(&puStack_118,&UNK_142b16ec8);
      case (undefined8 *)0x4:
        FUN_140befd60(param_2);
        puStack_118 = &UNK_142b16e58;
        uStack_110 = (undefined8 *)0x1;
        uStack_108 = (undefined8 *)0x8;
        uStack_100 = 0;
        uStack_f8 = 0;
        FUN_1428d9390(&puStack_118,&UNK_142b16e68);
        pcVar1 = (code *)swi(3);
        puVar3 = (undefined8 *)(*pcVar1)();
        return puVar3;
      }
    }
    else {
      puVar5 = (undefined *)CONCAT44(uStack_ec,uStack_f0);
      puVar6 = puStack_118 + -2;
      puVar4 = (undefined *)0x1;
      if (puVar6 < (undefined *)0x4) {
        puVar4 = puVar6;
      }
      uVar10 = puVar4 == (undefined *)0x2;
      ppuVar8 = ppuStack_e8;
      if ((undefined *)0x1 < puVar4) {
code_r0x000140bfdf4d:
        puStack_88 = (undefined4 *)puVar5;
        puStack_80 = puVar6;
        ppuStack_58 = ppuVar8;
        if ((bool)uVar10) {
          FUN_140befd60(param_2);
          puStack_180 = (undefined8 *)&UNK_142b16f18;
          puStack_178 = (undefined8 *)0x1;
          puStack_170 = (undefined8 *)0x8;
          uStack_168 = 0;
          uStack_160 = (undefined4 *)0x0;
          FUN_1428d9390(&puStack_180,&UNK_142b16f28);
        }
        else {
          uStack_4c = (undefined4)uStack_100;
          puStack_98 = uStack_110;
          puStack_90 = uStack_108;
          FUN_140befd60(param_2);
          uStack_78 = &uStack_4c;
          uStack_70 = (undefined **)&UNK_140c8f550;
          uStack_68 = &puStack_98;
          puStack_60 = &UNK_140109e90;
          puStack_180 = (undefined8 *)&UNK_142b16f70;
          puStack_178 = (undefined8 *)0x2;
          uStack_160 = (undefined4 *)0x0;
          puStack_170 = &uStack_78;
          uStack_168 = 2;
          FUN_1428d9390(&puStack_180,&UNK_142b16f90);
        }
                    /* WARNING: Does not return */
        pcVar1 = (code *)invalidInstructionException();
        (*pcVar1)();
      }
      *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
      uStack_70 = ppuStack_e8;
      uStack_78 = (undefined4 *)puVar5;
      if (puVar5 == (undefined *)0x0 || puVar6 != (undefined *)0x1 && puVar6 < (undefined *)0x4) {
        return param_1;
      }
    }
    func_0x000140613c20(uStack_70,uStack_78,1);
  }
  else {
code_r0x000140bfde23:
    param_1[0xc] = uStack_120;
    param_1[10] = uStack_130;
    param_1[0xb] = CONCAT17(uStack_121,CONCAT16(cStack_122,uStack_128));
    param_1[8] = uStack_140;
    param_1[9] = uStack_138;
    param_1[6] = CONCAT44(uStack_14c,uStack_150);
    param_1[7] = CONCAT44(uStack_144,uStack_148);
    param_1[4] = uStack_160;
    param_1[5] = CONCAT44(uStack_154,uStack_158);
    param_1[2] = puStack_170;
    param_1[3] = uStack_168;
    *param_1 = puStack_180;
    param_1[1] = puStack_178;
  }
  return param_1;
}

