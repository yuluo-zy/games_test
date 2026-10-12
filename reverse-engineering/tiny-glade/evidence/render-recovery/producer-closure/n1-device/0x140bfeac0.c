
void FUN_140bfeac0(longlong *param_1,longlong param_2,longlong param_3,undefined8 param_4,
                  undefined4 param_5,longlong param_6,ulonglong param_7)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *plVar9;
  ulonglong uVar10;
  longlong lStack_150;
  undefined8 uStack_148;
  longlong lStack_138;
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
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong *plStack_68;
  undefined1 *puStack_60;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  if (param_7 == 0) {
    lVar8 = 0;
  }
  else {
    if (param_7 < 4) {
      lVar8 = 0;
      uVar10 = 0;
    }
    else {
      plVar9 = (longlong *)(param_6 + 0x98);
      lVar8 = 0;
      uVar10 = 0;
      do {
        lVar8 = lVar8 + plVar9[-0x12] + plVar9[-0xc] + plVar9[-6] + *plVar9;
        uVar10 = uVar10 + 4;
        plVar9 = plVar9 + 0x18;
      } while ((param_7 & 0xfffffffffffffffc) != uVar10);
    }
    if ((param_7 & 3) != 0) {
      lVar7 = 0;
      do {
        lVar8 = lVar8 + *(longlong *)(uVar10 * 0x30 + param_6 + 8 + lVar7);
        lVar7 = lVar7 + 0x30;
      } while ((ulonglong)(((uint)param_7 & 3) << 4) * 3 != lVar7);
    }
  }
  uStack_f8 = FUN_1417390e0(param_5);
  if (lVar8 != 0) {
    lVar7 = param_2 + 0x870;
    if (*(longlong *)(param_2 + 0x870) == 0) {
      lVar7 = 0;
    }
    uStack_130 = 1;
    uStack_12c = CONCAT13(uStack_12c._3_1_,0x30000);
    lStack_138 = lVar8;
    uStack_f0 = param_4;
    FUN_140bee200(&uStack_e8,param_2 + 0x78,lVar7,*(undefined8 *)(param_2 + 0x648),
                  *(longlong *)(param_2 + 0x678) + 0x10,&lStack_138,&UNK_142b17168,0x14);
    lVar7 = lStack_a8;
    lStack_98 = uStack_e0;
    uStack_90 = uStack_d8;
    lVar2 = uStack_90;
    uStack_88 = (longlong *)uStack_d0;
    lVar3 = (longlong)uStack_88;
    uStack_80 = (longlong)uStack_c8;
    lVar4 = uStack_80;
    uStack_78 = (undefined1 *)uStack_c0;
    lVar5 = (longlong)uStack_78;
    uStack_70 = uStack_b8;
    lVar6 = (longlong)uStack_70;
    lVar8 = CONCAT44(uStack_ac,uStack_b0);
    if (uStack_e8 == 0xb) {
      uStack_90._0_4_ = (undefined4)uStack_d8;
      uStack_90._4_4_ = (undefined4)((ulonglong)uStack_d8 >> 0x20);
      uStack_88._0_4_ = (undefined4)uStack_d0;
      uStack_88._4_4_ = (undefined4)((ulonglong)uStack_d0 >> 0x20);
      uStack_80._0_4_ = SUB84(uStack_c8,0);
      uStack_80._4_4_ = (undefined4)((ulonglong)uStack_c8 >> 0x20);
      uStack_78._0_4_ = SUB84(uStack_c0,0);
      uStack_78._4_4_ = (undefined4)((ulonglong)uStack_c0 >> 0x20);
      uStack_70._0_4_ = SUB84(uStack_b8,0);
      uStack_70._4_4_ = (undefined4)((ulonglong)uStack_b8 >> 0x20);
      uStack_118 = (undefined4)uStack_78;
      uStack_114 = uStack_78._4_4_;
      uStack_110 = (undefined4)uStack_70;
      uStack_10c = uStack_70._4_4_;
      uStack_128 = (undefined4)uStack_88;
      uStack_124 = uStack_88._4_4_;
      uStack_120 = (undefined4)uStack_80;
      uStack_11c = uStack_80._4_4_;
      uStack_130 = (undefined4)uStack_90;
      uStack_12c = uStack_90._4_4_;
      uStack_90 = lVar2;
      uStack_88 = (longlong *)lVar3;
      uStack_80 = lVar4;
      uStack_78 = (undefined1 *)lVar5;
      uStack_70 = (undefined4 *)lVar6;
      if (lVar8 == 0) goto code_r0x000140bfede8;
      uStack_100 = 0;
      uStack_4c = FUN_141738fe0(param_5);
      uStack_e0 = param_7 * 0x30 + param_6;
      uStack_e8 = param_6;
      uStack_d0 = lVar7;
      uStack_c8 = &uStack_100;
      uStack_c0 = &uStack_f8;
      uStack_d8 = lVar8;
      uStack_b8 = &uStack_4c;
      FUN_1418b7630(&lStack_150,&uStack_e8,&UNK_142b15090);
      uStack_80 = uStack_f0;
      uStack_78 = &stack0x00000040;
      puStack_60 = &stack0x00000048;
      lStack_98 = param_3;
      uStack_90 = param_2;
      uStack_88 = &lStack_138;
      uStack_70 = &uStack_4c;
      plStack_68 = &lStack_150;
      FUN_140c04560(&uStack_e8,param_2,&lStack_98);
      param_1[8] = lStack_a8;
      param_1[9] = lStack_a0;
      *(undefined4 *)(param_1 + 6) = (undefined4)uStack_b8;
      *(undefined4 *)((longlong)param_1 + 0x34) = uStack_b8._4_4_;
      *(undefined4 *)(param_1 + 7) = uStack_b0;
      *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_ac;
      *(undefined4 *)(param_1 + 4) = (undefined4)uStack_c8;
      *(undefined4 *)((longlong)param_1 + 0x24) = uStack_c8._4_4_;
      *(undefined4 *)(param_1 + 5) = (undefined4)uStack_c0;
      *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_c0._4_4_;
      *(undefined4 *)(param_1 + 2) = (undefined4)uStack_d8;
      *(undefined4 *)((longlong)param_1 + 0x14) = uStack_d8._4_4_;
      *(undefined4 *)(param_1 + 3) = (undefined4)uStack_d0;
      *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_d0._4_4_;
      *(undefined4 *)param_1 = (undefined4)uStack_e8;
      *(undefined4 *)((longlong)param_1 + 4) = uStack_e8._4_4_;
      *(undefined4 *)(param_1 + 1) = (undefined4)uStack_e0;
      *(undefined4 *)((longlong)param_1 + 0xc) = uStack_e0._4_4_;
      if (lStack_150 != 0) {
        func_0x000140613c20(uStack_148,lStack_150 * 0x38,8);
      }
      FUN_140be24b0(&uStack_130);
    }
    else {
      param_1[5] = (longlong)uStack_c0;
      param_1[6] = (longlong)uStack_b8;
      param_1[3] = uStack_d0;
      param_1[4] = (longlong)uStack_c8;
      param_1[1] = uStack_e0;
      param_1[2] = uStack_d8;
      *param_1 = uStack_e8;
      param_1[7] = lVar8;
      param_1[8] = lStack_a8;
      param_1[9] = lStack_a0;
    }
    return;
  }
  FUN_1428d9430(&UNK_142b0f830,0x1a,&UNK_142b0f898);
code_r0x000140bfede8:
  FUN_1428d9330(&UNK_142b1717c,0x24,&UNK_142b171a0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

