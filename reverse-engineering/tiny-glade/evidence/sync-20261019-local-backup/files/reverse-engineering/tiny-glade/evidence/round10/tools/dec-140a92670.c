
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a92670(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                  undefined8 *param_5,undefined8 param_6)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  code *pcVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong *plVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  longlong lStack_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_118;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_e8;
  longlong *plStack_e0;
  longlong lStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  longlong lStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined *puStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  FUN_140b0eeb0(&lStack_d8,&UNK_142ada260,0x18);
  if (SBORROW8(0,lStack_d8)) {
    lStack_168 = (longlong)uStack_d0;
    FUN_1428d9760(&UNK_142ad9fd0,0x2b,&lStack_168,&UNK_142ad9fb0,&UNK_142ada278);
  }
  else {
    uStack_f8 = uStack_68;
    uStack_100 = puStack_70;
    uStack_118 = uStack_88;
    uStack_130 = (undefined4)uStack_a0;
    uStack_12c = uStack_a0._4_4_;
    uStack_158 = (undefined4)uStack_c8;
    uStack_154 = uStack_c8._4_4_;
    lStack_168 = lStack_d8;
    uStack_160 = (undefined4)uStack_d0;
    uStack_15c = uStack_d0._4_4_;
    uVar14 = *param_3;
    uVar2 = *param_2;
    uStack_e8 = FUN_140b1aca0(&lStack_168,&UNK_142ada290,5,uVar14,uVar2);
    plVar11 = (longlong *)func_0x000140613c10(0x48,4);
    lVar7 = _UNK_142ad9a48;
    if (plVar11 == (longlong *)0x0) {
      FUN_1428d9000(4,0x48);
    }
    else {
      *plVar11 = _UNK_142ad9a40;
      plVar11[1] = lVar7;
      lVar7 = _UNK_142ad9a58;
      plVar11[2] = _UNK_142ad9a50;
      plVar11[3] = lVar7;
      lVar7 = _UNK_142ad9a68;
      plVar11[4] = _UNK_142ad9a60;
      plVar11[5] = lVar7;
      uVar10 = _UNK_142ad9a7c;
      uVar9 = _UNK_142ad9a78;
      uVar8 = _UNK_142ad9a74;
      *(undefined4 *)(plVar11 + 6) = _UNK_142ad9a70;
      *(undefined4 *)((longlong)plVar11 + 0x34) = uVar8;
      *(undefined4 *)(plVar11 + 7) = uVar9;
      *(undefined4 *)((longlong)plVar11 + 0x3c) = uVar10;
      plVar11[8] = 0x100000003;
      plStack_e0 = plVar11;
      plVar11 = (longlong *)FUN_1400d9b20(0);
      if (plVar11 != (longlong *)0x0) {
        uStack_80 = (undefined4)*plVar11;
        uStack_7c = *(undefined4 *)((longlong)plVar11 + 4);
        uStack_78 = (undefined4)plVar11[1];
        uStack_74 = *(undefined4 *)((longlong)plVar11 + 0xc);
        *plVar11 = *plVar11 + 1;
        uStack_150 = _UNK_142ac64c0;
        uStack_148 = _UNK_142ac64c8;
        uVar12 = uStack_148;
        uStack_140 = _UNK_142ac64d0;
        uVar13 = uStack_140;
        uStack_138 = _UNK_142ac64d8;
        lStack_d8 = 0x12;
        uStack_d0 = plStack_e0;
        uStack_c8 = (longlong *)0x12;
        uStack_c0 = 0x8000000000000000;
        uStack_b0 = CONCAT44(uStack_15c,uStack_160);
        lStack_b8 = lStack_168;
        uStack_a8 = CONCAT44(uStack_154,uStack_158);
        uStack_a0 = _UNK_142ac64c0;
        uStack_148._0_4_ = (undefined4)_UNK_142ac64c8;
        uStack_148._4_4_ = (undefined4)((ulonglong)_UNK_142ac64c8 >> 0x20);
        uStack_140._0_4_ = (undefined4)_UNK_142ac64d0;
        uStack_140._4_4_ = (undefined4)((ulonglong)_UNK_142ac64d0 >> 0x20);
        uStack_98 = (undefined4)uStack_148;
        uStack_94 = uStack_148._4_4_;
        uStack_90 = (undefined4)uStack_140;
        uStack_8c = uStack_140._4_4_;
        uStack_88 = _UNK_142ac64d8;
        puStack_70 = &UNK_142ada2fe;
        uStack_68 = 0xf;
        uStack_148 = uVar12;
        uStack_140 = uVar13;
        uVar12 = FUN_140b1aca0(&lStack_d8,&UNK_142ada295,9,uVar14,uVar2);
        plVar11 = (longlong *)*param_1;
        uVar14 = *param_5;
        uVar2 = *param_4;
        uVar13 = FUN_140b3fe40(*plVar11 + 0x10,&UNK_142ada29e,0x21,&UNK_142ada2bf,0xf,uVar14,uVar2);
        uVar14 = FUN_140b3fe40(*plVar11 + 0x10,&UNK_142ada2ce,0x21,&UNK_142ada2ef,0xf,uVar14,uVar2);
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412d6640;
        uVar6 = _UNK_14293e0c8;
        uVar2 = _UNK_14293e0c0;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = _UNK_14293e0c0;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uStack_e8;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar13;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 0;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412d6640;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uVar12;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar14;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 0;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412d6640;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uStack_e8;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar13;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 1;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412d6640;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uVar12;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar14;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 1;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412dd190;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uVar12;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar14;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 0x100;
        *(undefined1 *)(lVar4 + 0x4a + lVar3) = 0;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412dd190;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uVar12;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar14;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 0x100;
        *(undefined1 *)(lVar4 + 0x4a + lVar3) = 1;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        FUN_140d146a0(&lStack_d8,param_6);
        lVar7 = lStack_b8;
        plVar11 = uStack_d0;
        if (uStack_d0 == (longlong *)0x0) {
          plVar11 = uStack_c8;
        }
        lVar3 = plVar11[2];
        if ((ulonglong)(*plVar11 - lVar3) < 0x88) {
          FUN_1428a9a60(plVar11,lVar3,0x88,1,1);
        }
        lVar4 = plVar11[1];
        *(code **)(lVar4 + lVar3) = FUN_1412dd190;
        puVar1 = (undefined8 *)(lVar4 + 8 + lVar3);
        *puVar1 = uVar2;
        puVar1[1] = uVar6;
        *(undefined8 *)(lVar4 + 0x18 + lVar3) = 0;
        *(undefined4 *)(lVar4 + 0x20 + lVar3) = 0;
        *(undefined8 *)(lVar4 + 0x24 + lVar3) = 0x3f8000003f800000;
        *(undefined4 *)(lVar4 + 0x2c + lVar3) = 0x3f800000;
        *(undefined8 *)(lVar4 + 0x38 + lVar3) = uVar12;
        *(undefined8 *)(lVar4 + 0x40 + lVar3) = uVar14;
        *(undefined2 *)(lVar4 + 0x48 + lVar3) = 0x100;
        *(undefined1 *)(lVar4 + 0x4a + lVar3) = 2;
        *(undefined1 *)(lVar4 + 0x58 + lVar3) = 0;
        *(longlong *)(lVar4 + 0x68 + lVar3) = lVar7;
        *(code **)(lVar4 + 0x78 + lVar3) = FUN_141564d30;
        plVar11[2] = lVar3 + 0x88;
        return;
      }
    }
    FUN_1428d80d0(&UNK_142c3aaa0);
  }
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

