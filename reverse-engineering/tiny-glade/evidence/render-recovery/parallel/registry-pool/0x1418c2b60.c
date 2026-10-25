
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1418c2b60(undefined8 *param_1,undefined4 *param_2)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  byte *pbVar4;
  undefined4 *puVar5;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined4 **ppuStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined4 *puStack_50;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  puVar5 = param_2;
  pbVar4 = (byte *)FUN_140d4d740();
  if ((*pbVar4 & 1) == 0) {
    lStack_58 = FUN_140d64e00();
    pbVar4[0] = 1;
    pbVar4[1] = 0;
    pbVar4[2] = 0;
    pbVar4[3] = 0;
    pbVar4[4] = 0;
    pbVar4[5] = 0;
    pbVar4[6] = 0;
    pbVar4[7] = 0;
    *(longlong *)(pbVar4 + 8) = lStack_58;
    *(undefined4 **)(pbVar4 + 0x10) = puVar5;
  }
  else {
    lStack_58 = *(longlong *)(pbVar4 + 8);
    puVar5 = *(undefined4 **)(pbVar4 + 0x10);
  }
  *(longlong *)(pbVar4 + 8) = lStack_58 + 1;
  uStack_68 = _UNK_142d063f0;
  uStack_60 = _UNK_142d063f8;
  uStack_78 = _UNK_142d063e0;
  uStack_74 = _UNK_142d063e4;
  uStack_70 = _UNK_142d063e8;
  uStack_6c = _UNK_142d063ec;
  lVar1 = *(longlong *)(param_2 + 8);
  uVar2 = *(undefined8 *)(param_2 + 10);
  uVar3 = *(undefined8 *)(param_2 + 0xc);
  puStack_50 = puVar5;
  if (lVar1 != 0) {
    FUN_1428c0200(&uStack_78,lVar1,&lStack_58,1);
  }
  puStack_98 = &uStack_78;
  ppuStack_80 = &puStack_98;
  uStack_b8 = *param_2;
  uStack_b4 = param_2[1];
  uStack_b0 = param_2[2];
  uStack_ac = param_2[3];
  uStack_a8 = param_2[4];
  uStack_a4 = param_2[5];
  uStack_a0 = param_2[6];
  uStack_9c = param_2[7];
  uStack_90 = uVar2;
  uStack_88 = uVar3;
  FUN_1417e78e0(&uStack_b8,lVar1,&ppuStack_80);
  param_1[4] = lStack_58;
  param_1[5] = puStack_50;
  param_1[2] = uStack_68;
  param_1[3] = uStack_60;
  *param_1 = CONCAT44(uStack_74,uStack_78);
  param_1[1] = CONCAT44(uStack_6c,uStack_70);
  return param_1;
}

