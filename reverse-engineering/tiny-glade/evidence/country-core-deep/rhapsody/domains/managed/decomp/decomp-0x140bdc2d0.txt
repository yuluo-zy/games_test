
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong *
FUN_140bdc2d0(longlong *param_1,longlong param_2,longlong param_3,longlong param_4,longlong param_5,
             longlong *param_6,longlong *param_7)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong **pplVar5;
  byte *pbVar6;
  longlong lVar7;
  longlong lVar8;
  longlong **pplStack_108;
  longlong **pplStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e0;
  undefined8 uStack_d8;
  undefined1 uStack_d0;
  longlong *plStack_c8;
  longlong lStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  longlong *plStack_a8;
  longlong **pplStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  pplStack_108 = (longlong **)((ulonglong)pplStack_108 & 0xffffffff00000000);
  pplStack_100 = (longlong **)((ulonglong)pplStack_100 & 0xffffffff00000000);
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e8 = 0;
  uStack_e0 = 0;
  uStack_d8 = 0;
  uStack_d0 = 0;
  lVar8 = *(longlong *)(param_5 + 0x140);
  uStack_49 = 0;
  FUN_140bdcba0(&plStack_a8,lVar8,*(undefined8 *)(param_5 + 0x5c8),&pplStack_108,param_7,param_4);
  pplVar5 = pplStack_a0;
  if (plStack_a8 == (longlong *)0xb) {
    uStack_68 = param_6[8];
    plStack_a8 = (longlong *)*param_6;
    pplStack_a0 = (longlong **)param_6[1];
    uStack_98 = (longlong ***)param_6[2];
    uStack_90 = (undefined4)param_6[3];
    uStack_8c = *(undefined4 *)((longlong)param_6 + 0x1c);
    uStack_88 = (undefined4)param_6[4];
    uStack_84 = *(undefined4 *)((longlong)param_6 + 0x24);
    uStack_80 = (undefined4)param_6[5];
    uStack_7c = *(undefined4 *)((longlong)param_6 + 0x2c);
    uStack_78 = (undefined4)param_6[6];
    uStack_74 = *(undefined4 *)((longlong)param_6 + 0x34);
    uStack_70 = (undefined4)param_6[7];
    uStack_6c = *(undefined4 *)((longlong)param_6 + 0x3c);
    pbVar6 = (byte *)FUN_140d4d740();
    if ((*pbVar6 & 1) == 0) {
      lVar7 = FUN_140d64e00();
      pbVar6[0] = 1;
      pbVar6[1] = 0;
      pbVar6[2] = 0;
      pbVar6[3] = 0;
      pbVar6[4] = 0;
      pbVar6[5] = 0;
      pbVar6[6] = 0;
      pbVar6[7] = 0;
      *(longlong *)(pbVar6 + 8) = lVar7;
      *(longlong *)(pbVar6 + 0x10) = lVar8;
    }
    else {
      lVar7 = *(longlong *)(pbVar6 + 8);
      lVar8 = *(longlong *)(pbVar6 + 0x10);
    }
    *(longlong *)(pbVar6 + 8) = lVar7 + 1;
    param_1[0x17] = uStack_68;
    param_1[0x15] = CONCAT44(uStack_74,uStack_78);
    param_1[0x16] = CONCAT44(uStack_6c,uStack_70);
    param_1[0x13] = CONCAT44(uStack_84,uStack_88);
    param_1[0x14] = CONCAT44(uStack_7c,uStack_80);
    param_1[0x11] = (longlong)uStack_98;
    param_1[0x12] = CONCAT44(uStack_8c,uStack_90);
    param_1[0xf] = (longlong)plStack_a8;
    param_1[0x10] = (longlong)pplStack_a0;
    lVar2 = param_7[1];
    lVar3 = param_7[2];
    lVar4 = param_7[3];
    *param_1 = *param_7;
    param_1[1] = lVar2;
    param_1[2] = lVar3;
    param_1[3] = lVar4;
    param_1[4] = param_2;
    param_1[5] = param_3;
    param_1[6] = param_4;
    param_1[7] = 0;
    lVar2 = _UNK_142b125b8;
    param_1[8] = _UNK_142b125b0;
    param_1[9] = lVar2;
    lVar2 = _UNK_142b125c8;
    param_1[10] = _UNK_142b125c0;
    param_1[0xb] = lVar2;
    param_1[0xc] = lVar7;
    param_1[0xd] = lVar8;
    param_1[0xe] = (longlong)pplVar5;
  }
  else {
    *(undefined4 *)(param_1 + 8) = (undefined4)uStack_68;
    *(undefined4 *)((longlong)param_1 + 0x44) = uStack_68._4_4_;
    *(undefined4 *)(param_1 + 9) = uStack_60;
    *(undefined4 *)((longlong)param_1 + 0x4c) = uStack_5c;
    *(undefined4 *)(param_1 + 6) = uStack_78;
    *(undefined4 *)((longlong)param_1 + 0x34) = uStack_74;
    *(undefined4 *)(param_1 + 7) = uStack_70;
    *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_6c;
    *(undefined4 *)(param_1 + 4) = uStack_88;
    *(undefined4 *)((longlong)param_1 + 0x24) = uStack_84;
    *(undefined4 *)(param_1 + 5) = uStack_80;
    *(undefined4 *)((longlong)param_1 + 0x2c) = uStack_7c;
    *(undefined4 *)(param_1 + 2) = (undefined4)uStack_98;
    *(undefined4 *)((longlong)param_1 + 0x14) = uStack_98._4_4_;
    *(undefined4 *)(param_1 + 3) = uStack_90;
    *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_8c;
    *param_1 = (longlong)plStack_a8;
    param_1[1] = (longlong)pplStack_a0;
    *(undefined1 *)((longlong)param_1 + 0xbc) = 3;
    if (*(char *)((longlong)param_6 + 0x44) != '\x02') {
      plStack_c8 = param_6 + 7;
      lVar8 = param_6[7];
      plStack_b0 = param_6 + 8;
      pplStack_108 = &plStack_b0;
      plStack_a8 = &lStack_c0;
      pplStack_a0 = &plStack_c8;
      pplStack_100 = &plStack_b8;
      lStack_c0 = lVar8;
      plStack_b8 = plStack_c8;
      uStack_98 = &pplStack_108;
      FUN_1418bc7d0(&plStack_a8);
      if (lVar8 != 0) {
        plStack_a8 = (longlong *)(lVar8 + -0x10);
        LOCK();
        plVar1 = (longlong *)(lVar8 + -0x10);
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (*plVar1 == 0) {
          FUN_140c15f40(&plStack_a8);
        }
      }
    }
  }
  return param_1;
}

