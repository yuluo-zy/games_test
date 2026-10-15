
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140bea2d0(undefined8 *param_1,longlong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  int iVar10;
  byte *pbVar11;
  longlong lVar12;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined8 uStack_b4;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  longlong lStack_88;
  longlong lStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  uStack_ac = 0;
  uStack_b4 = 0;
  uStack_a8 = CONCAT44(uStack_b8,0xb);
  uStack_a0 = 0;
  uStack_98 = 0;
  uStack_94 = 2;
  uStack_90 = 2;
  uStack_8c = 0;
  iVar10 = (**(code **)(param_2 + 0x168))(*(undefined8 *)(param_2 + 0x640),&uStack_a8,0,&uStack_108)
  ;
  uVar6 = uStack_108;
  if (iVar10 == 0) {
    uStack_a8 = 0x10;
    uStack_a0 = CONCAT17(uStack_a0._7_1_,0x4000000000002);
    uStack_a0 = CONCAT26(uStack_a0._6_2_,2);
    FUN_140c03460(&uStack_108,param_2,&UNK_142b14678,0x1d,&uStack_a8,&UNK_142b14668,0x10);
    uVar9 = uStack_f0;
    uVar8 = uStack_f8;
    uVar7 = uStack_100;
    if ((int)uStack_108 == 0xb) {
      uStack_108 = 0;
      uStack_f8 = 0;
      uStack_f0 = 0;
      uStack_100 = 8;
      pbVar11 = (byte *)FUN_140d4d740();
      if ((*pbVar11 & 1) == 0) {
        lStack_88 = FUN_140d64e00();
        pbVar11[0] = 1;
        pbVar11[1] = 0;
        pbVar11[2] = 0;
        pbVar11[3] = 0;
        pbVar11[4] = 0;
        pbVar11[5] = 0;
        pbVar11[6] = 0;
        pbVar11[7] = 0;
        *(longlong *)(pbVar11 + 8) = lStack_88;
        *(longlong *)(pbVar11 + 0x10) = param_2;
      }
      else {
        lStack_88 = *(longlong *)(pbVar11 + 8);
        param_2 = *(longlong *)(pbVar11 + 0x10);
      }
      *(longlong *)(pbVar11 + 8) = lStack_88 + 1;
      uVar5 = _UNK_142b145e8;
      uVar4 = _UNK_142b145e0;
      uVar3 = _UNK_142b145d8;
      uVar2 = _UNK_142b145d0;
      uStack_98 = (undefined4)_UNK_142b145e0;
      uStack_94 = (undefined4)((ulonglong)_UNK_142b145e0 >> 0x20);
      uStack_90 = (undefined4)_UNK_142b145e8;
      uStack_8c = (undefined4)((ulonglong)_UNK_142b145e8 >> 0x20);
      uStack_a8 = _UNK_142b145d0;
      uStack_a0 = _UNK_142b145d8;
      uStack_78 = 0;
      uStack_70 = 0;
      lStack_80 = param_2;
      pbVar11 = (byte *)FUN_140d4d740();
      if ((*pbVar11 & 1) == 0) {
        lVar12 = FUN_140d64e00();
        pbVar11[0] = 1;
        pbVar11[1] = 0;
        pbVar11[2] = 0;
        pbVar11[3] = 0;
        pbVar11[4] = 0;
        pbVar11[5] = 0;
        pbVar11[6] = 0;
        pbVar11[7] = 0;
        *(longlong *)(pbVar11 + 8) = lVar12;
        *(longlong *)(pbVar11 + 0x10) = param_2;
      }
      else {
        lVar12 = *(longlong *)(pbVar11 + 8);
        param_2 = *(longlong *)(pbVar11 + 0x10);
      }
      *(longlong *)(pbVar11 + 8) = lVar12 + 1;
      param_1[0x17] = 0;
      param_1[0x18] = 0;
      *param_1 = uStack_108;
      param_1[1] = uStack_100;
      param_1[2] = uStack_f8;
      param_1[3] = uStack_f0;
      param_1[0x19] = uVar6;
      param_1[8] = CONCAT44(uStack_dc,uStack_e0);
      param_1[6] = uVar9;
      param_1[7] = CONCAT44(uStack_e4,uStack_e8);
      param_1[4] = uVar7;
      param_1[5] = uVar8;
      param_1[9] = uStack_a8;
      param_1[10] = uStack_a0;
      param_1[0xb] = CONCAT44(uStack_94,uStack_98);
      param_1[0xc] = CONCAT44(uStack_8c,uStack_90);
      param_1[0xd] = lStack_88;
      param_1[0xe] = lStack_80;
      param_1[0xf] = uStack_78;
      param_1[0x10] = uStack_70;
      param_1[0x13] = uVar4;
      param_1[0x14] = uVar5;
      param_1[0x11] = uVar2;
      param_1[0x12] = uVar3;
      param_1[0x15] = lVar12;
      param_1[0x16] = param_2;
      return param_1;
    }
  }
  else {
    uStack_108 = CONCAT44(uStack_108._4_4_,iVar10);
    FUN_1428d9760(&UNK_142b14630,0x1b,&uStack_108,&UNK_142b14340,&UNK_142b14650);
  }
  uStack_68 = uStack_c8;
  uStack_60 = uStack_c0;
  uStack_98 = (undefined4)uStack_f8;
  uStack_94 = uStack_f8._4_4_;
  uStack_90 = (undefined4)uStack_f0;
  uStack_8c = uStack_f0._4_4_;
  uStack_a8 = uStack_108;
  FUN_1428d9760(&UNK_142b14678,0x1d,&uStack_a8,&UNK_142b14360,&UNK_142b14698);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

