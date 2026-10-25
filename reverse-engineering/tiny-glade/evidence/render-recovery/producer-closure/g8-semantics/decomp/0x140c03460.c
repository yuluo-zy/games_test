
longlong *
FUN_140c03460(longlong *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             ulonglong *param_5,longlong param_6,ulonglong param_7)

{
  code *pcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lStack_168;
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
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  longlong lStack_120;
  longlong lStack_118;
  ulonglong uStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong lStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  longlong lStack_90;
  ulonglong uStack_88;
  undefined4 uStack_80;
  undefined2 uStack_7c;
  undefined1 uStack_7a;
  undefined4 *puStack_78;
  longlong lStack_70;
  undefined4 *puStack_68;
  longlong *plStack_60;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_118 = param_6;
  uStack_110 = param_7;
  if (param_7 != 0 && param_6 != 0) {
    *(byte *)(param_5 + 1) = (byte)param_5[1] | 2;
  }
  lVar7 = param_2 + 0x870;
  lVar6 = 0;
  if (*(longlong *)(param_2 + 0x870) != 0) {
    lVar6 = lVar7;
  }
  uStack_108 = (undefined4)*param_5;
  uStack_104 = *(undefined4 *)((longlong)param_5 + 4);
  uStack_100 = (undefined4)param_5[1];
  uStack_fc = *(undefined4 *)((longlong)param_5 + 0xc);
  FUN_140bee200(&uStack_d8,param_2 + 0x78,lVar6,*(undefined8 *)(param_2 + 0x648),
                *(longlong *)(param_2 + 0x678) + 0x10,&uStack_108,param_3,param_4);
  uStack_50 = lStack_a8;
  lVar6 = uStack_50;
  if (CONCAT44(uStack_d4,uStack_d8) != 0xb) {
    param_1[5] = CONCAT44(uStack_ac,uStack_b0);
    param_1[6] = lStack_a8;
    param_1[3] = CONCAT44(uStack_bc,uStack_c0);
    param_1[4] = CONCAT44(uStack_b4,uStack_b8);
    param_1[1] = CONCAT44(uStack_cc,uStack_d0);
    param_1[2] = CONCAT44(uStack_c4,uStack_c8);
    *param_1 = CONCAT44(uStack_d4,uStack_d8);
    param_1[7] = lStack_a0;
    param_1[8] = uStack_98;
    param_1[9] = lStack_90;
    return param_1;
  }
  uStack_50._0_4_ = (undefined4)lStack_a8;
  uStack_50._4_4_ = (undefined4)((ulonglong)lStack_a8 >> 0x20);
  uStack_e8 = uStack_b0;
  uStack_e4 = uStack_ac;
  uStack_e0 = (undefined4)uStack_50;
  uStack_dc = uStack_50._4_4_;
  uStack_f8 = uStack_c0;
  uStack_f4 = uStack_bc;
  uStack_f0 = uStack_b8;
  uStack_ec = uStack_b4;
  uStack_108 = uStack_d0;
  uStack_104 = uStack_cc;
  uStack_100 = uStack_c8;
  uStack_fc = uStack_c4;
  if (param_7 == 0 || param_6 == 0) {
code_r0x000140c035ab:
    param_1[5] = CONCAT44(uStack_dc,uStack_e0);
    param_1[3] = CONCAT44(uStack_ec,uStack_f0);
    param_1[4] = CONCAT44(uStack_e4,uStack_e8);
    param_1[1] = CONCAT44(uStack_fc,uStack_100);
    param_1[2] = CONCAT44(uStack_f4,uStack_f8);
    *param_1 = 0xb;
    return param_1;
  }
  uStack_50 = lVar6;
  if (*param_5 < param_7) {
    FUN_1428d9430(&UNK_142b186a0,0x38,&UNK_142b18700);
  }
  else if (lStack_a0 == 0) {
    uStack_80 = 1;
    uStack_7a = 3;
    uStack_7c = 0;
    if (*(longlong *)(param_2 + 0x870) == 0) {
      lVar7 = 0;
    }
    uStack_88 = *param_5;
    FUN_140bee200(&uStack_d8,param_2 + 0x78,lVar7,*(undefined8 *)(param_2 + 0x648),
                  *(longlong *)(param_2 + 0x678) + 0x10,&uStack_88,&UNK_142b18730,0x13);
    uVar5 = uStack_b4;
    uVar4 = uStack_b8;
    uVar3 = uStack_c4;
    uVar2 = uStack_c8;
    puStack_78 = (undefined4 *)CONCAT44(uStack_cc,uStack_d0);
    lStack_70 = CONCAT44(uStack_c4,uStack_c8);
    puStack_68 = (undefined4 *)CONCAT44(uStack_bc,uStack_c0);
    plStack_60 = (longlong *)CONCAT44(uStack_b4,uStack_b8);
    uStack_50 = lStack_a8;
    lVar7 = uStack_50;
    if (CONCAT44(uStack_d4,uStack_d8) != 0xb) {
      param_1[5] = CONCAT44(uStack_ac,uStack_b0);
      param_1[6] = lStack_a8;
      param_1[3] = (longlong)puStack_68;
      param_1[4] = (longlong)plStack_60;
      *(undefined4 *)(param_1 + 1) = uStack_d0;
      *(undefined4 *)((longlong)param_1 + 0xc) = uStack_cc;
      *(undefined4 *)(param_1 + 2) = uStack_c8;
      *(undefined4 *)((longlong)param_1 + 0x14) = uStack_c4;
      *param_1 = CONCAT44(uStack_d4,uStack_d8);
      param_1[7] = lStack_a0;
      param_1[8] = uStack_98;
      param_1[9] = lStack_90;
code_r0x000140c037ce:
      FUN_140be24b0(&uStack_100);
      return param_1;
    }
    uStack_50._0_4_ = (undefined4)lStack_a8;
    uStack_50._4_4_ = (undefined4)((ulonglong)lStack_a8 >> 0x20);
    uStack_b8 = uStack_b0;
    uStack_b4 = uStack_ac;
    uStack_b0 = (undefined4)uStack_50;
    uStack_ac = uStack_50._4_4_;
    uStack_c8 = uStack_c0;
    uStack_c4 = uStack_bc;
    uStack_c0 = uVar4;
    uStack_bc = uVar5;
    uStack_d8 = uStack_d0;
    uStack_d4 = uStack_cc;
    uStack_d0 = uVar2;
    uStack_cc = uVar3;
    uStack_50 = lVar7;
    if (lStack_a0 == 0) {
      FUN_1428d9330(&UNK_142b18743,0x14,&UNK_142b18758);
    }
    else {
      if (param_7 <= uStack_98) {
        func_0x000142923c00(lStack_a0,param_6,param_7);
        puStack_68 = &uStack_108;
        plStack_60 = &lStack_118;
        puStack_78 = &uStack_d8;
        lStack_70 = param_2;
        FUN_140c04840(&lStack_168,param_2,&puStack_78);
        if (lStack_168 == 0xb) {
          FUN_140be24b0(&uStack_d0);
          goto code_r0x000140c035ab;
        }
        param_1[9] = lStack_120;
        *(undefined4 *)(param_1 + 7) = uStack_130;
        *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_12c;
        *(undefined4 *)(param_1 + 8) = uStack_128;
        *(undefined4 *)((longlong)param_1 + 0x44) = uStack_124;
        *(undefined4 *)(param_1 + 5) = uStack_140;
        *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_13c;
        *(undefined4 *)(param_1 + 6) = uStack_138;
        *(undefined4 *)((longlong)param_1 + 0x34) = uStack_134;
        *(undefined4 *)(param_1 + 3) = uStack_150;
        *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_14c;
        *(undefined4 *)(param_1 + 4) = uStack_148;
        *(undefined4 *)((longlong)param_1 + 0x24) = uStack_144;
        *(undefined4 *)(param_1 + 1) = uStack_160;
        *(undefined4 *)((longlong)param_1 + 0xc) = uStack_15c;
        *(undefined4 *)(param_1 + 2) = uStack_158;
        *(undefined4 *)((longlong)param_1 + 0x14) = uStack_154;
        *param_1 = lStack_168;
        FUN_140be24b0(&uStack_d0);
        goto code_r0x000140c037ce;
      }
      FUN_1428d9840(param_7,uStack_98,&UNK_142b18770);
    }
  }
  else {
    if (param_7 <= uStack_98) {
      func_0x000142923c00(lStack_a0,param_6,param_7);
      goto code_r0x000140c035ab;
    }
    FUN_1428d9840(param_7,uStack_98,&UNK_142b18718);
  }
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

