
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_141848770(longlong *param_1,undefined8 param_2,undefined8 param_3,longlong *param_4)

{
  bool bVar1;
  code *pcVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  char *pcVar6;
  longlong lVar7;
  byte *pbVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  longlong lStack_128;
  longlong lStack_120;
  longlong lStack_118;
  longlong lStack_110;
  longlong lStack_108;
  undefined8 uStack_100;
  char *pcStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong lStack_c8;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  char *pcStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong *plStack_50;
  undefined1 uStack_43;
  undefined1 uStack_42;
  undefined1 uStack_41;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  uStack_42 = 1;
  plStack_50 = param_4;
  FUN_140d59140(&uStack_b8);
  lVar7 = CONCAT44(uStack_ac,uStack_b0);
  if (SBORROW8(0,uStack_b8)) {
    uStack_42 = 1;
    lVar7 = FUN_1429171e0(lVar7);
    param_1[1] = lVar7;
    *param_1 = 0;
  }
  else {
    lStack_58 = uStack_b8;
    uStack_41 = 1;
    lStack_60 = lVar7;
    FUN_14212fb80(&lStack_108,lVar7,pcStack_a8);
    uStack_88 = uStack_100;
    uVar5 = uStack_88;
    uStack_80 = pcStack_f8;
    pcVar6 = uStack_80;
    lStack_78 = lStack_f0;
    uStack_88._0_4_ = (undefined4)uStack_100;
    uStack_88._4_4_ = (undefined4)((ulonglong)uStack_100 >> 0x20);
    uStack_80._0_4_ = SUB84(pcStack_f8,0);
    uStack_80._4_4_ = (undefined4)((ulonglong)pcStack_f8 >> 0x20);
    uStack_88 = uVar5;
    uStack_80 = pcVar6;
    if (SBORROW8(0,lStack_108)) {
      uStack_d8 = (undefined4)uStack_88;
      uStack_d4 = uStack_88._4_4_;
      uStack_d0 = (undefined4)uStack_80;
      uStack_cc = uStack_80._4_4_;
      lStack_c8 = lStack_f0;
      bVar1 = lStack_f0 != 0;
      if (lStack_f0 == 0) {
        lVar9 = *plStack_50;
        lVar11 = plStack_50[1];
        lVar10 = plStack_50[2];
        pbVar8 = (byte *)FUN_140d4d740();
        if ((*pbVar8 & 1) == 0) {
          lStack_70 = lVar11;
          lStack_68 = lVar9;
          uStack_43 = bVar1;
          lVar9 = FUN_140d64e00();
          pbVar8[0] = 1;
          pbVar8[1] = 0;
          pbVar8[2] = 0;
          pbVar8[3] = 0;
          pbVar8[4] = 0;
          pbVar8[5] = 0;
          pbVar8[6] = 0;
          pbVar8[7] = 0;
          *(longlong *)(pbVar8 + 8) = lVar9;
          *(longlong *)(pbVar8 + 0x10) = lVar7;
          lVar11 = lStack_70;
        }
        else {
          lVar9 = *(longlong *)(pbVar8 + 8);
          lVar7 = *(longlong *)(pbVar8 + 0x10);
        }
        *(longlong *)(pbVar8 + 8) = lVar9 + 1;
        lVar4 = _UNK_142cf1af8;
        param_1[4] = _DAT_142cf1af0;
        param_1[5] = lVar4;
        uVar3 = _UNK_142cf1aec;
        uVar13 = _UNK_142cf1ae8;
        uVar12 = _UNK_142cf1ae4;
        *(undefined4 *)(param_1 + 2) = _DAT_142cf1ae0;
        *(undefined4 *)((longlong)param_1 + 0x14) = uVar12;
        *(undefined4 *)(param_1 + 3) = uVar13;
        *(undefined4 *)((longlong)param_1 + 0x1c) = uVar3;
        param_1[6] = lVar9;
        param_1[7] = lVar7;
      }
      else {
        if (*pcStack_f8 != '\x05') {
          uStack_43 = bVar1;
          FUN_1428d9310(&UNK_142cf1bb8);
                    /* WARNING: Does not return */
          pcVar2 = (code *)invalidInstructionException();
          (*pcVar2)();
        }
        pcStack_a8 = *(char **)(pcStack_f8 + 0x20);
        lVar7 = *(longlong *)(pcStack_f8 + 0x28);
        if (lVar7 == 0) {
          uVar12 = 0;
          uVar13 = 0;
          uStack_b0 = 0;
          uStack_ac = 0;
        }
        else {
          uVar12 = *(undefined4 *)(lVar7 + 0x70);
          uVar13 = *(undefined4 *)(lVar7 + 0x74);
          uStack_b0 = *(undefined4 *)(lVar7 + 0x78);
          uStack_ac = *(undefined4 *)(lVar7 + 0x7c);
        }
        uStack_b8 = CONCAT44(uVar13,uVar12);
        uStack_43 = bVar1;
        FUN_142130580(&uStack_138,&uStack_b8);
        lVar11 = plStack_50[1];
        lVar10 = plStack_50[2];
        param_1[6] = lStack_118;
        param_1[7] = lStack_110;
        param_1[4] = lStack_128;
        param_1[5] = lStack_120;
        *(undefined4 *)(param_1 + 2) = uStack_138;
        *(undefined4 *)((longlong)param_1 + 0x14) = uStack_134;
        *(undefined4 *)(param_1 + 3) = uStack_130;
        *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_12c;
      }
      *param_1 = lVar11;
      param_1[1] = lVar10;
      uStack_41 = 0;
      FUN_141849b20(&uStack_d8);
      lVar7 = lStack_58;
      if (lStack_58 == 0) {
        return param_1;
      }
      goto LAB_1418489d5;
    }
    uStack_98 = uStack_e8;
    uStack_90 = uStack_e0;
    uStack_b0 = (undefined4)uStack_88;
    uStack_ac = uStack_88._4_4_;
    pcStack_a8 = pcStack_f8;
    lStack_a0 = lStack_f0;
    uStack_b8 = lStack_108;
    uStack_41 = 1;
    lVar7 = FUN_142917270(&uStack_b8);
    param_1[1] = lVar7;
    *param_1 = 0;
    if (lStack_58 != 0) {
      func_0x000140613c20(lStack_60,lStack_58,1);
    }
  }
  if (*plStack_50 == 0) {
    return param_1;
  }
  lStack_60 = plStack_50[1];
  lVar7 = *plStack_50;
LAB_1418489d5:
  func_0x000140613c20(lStack_60,lVar7,1);
  return param_1;
}

