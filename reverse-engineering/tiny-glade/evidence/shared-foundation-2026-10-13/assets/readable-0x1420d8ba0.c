
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 * asset__texture__scan__scan_asset_tree(undefined8 *param_1,undefined8 param_2,undefined8 param_3)
{
  byte *pbVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined4 auStack_80 [3];
  undefined4 uStack_74;
  undefined2 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 uStack_58;
  undefined8 uStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  undefined8 uStack_30;
  uStack_30 = 0xfffffffffffffffe;
  uVar3 = param_2;
  pbVar1 = (byte *)FUN_140d4d740();
  if ((*pbVar1 & 1) == 0) {
    lStack_48 = FUN_140d64e00();
    pbVar1[0] = 1;
    pbVar1[1] = 0;
    pbVar1[2] = 0;
    pbVar1[3] = 0;
    pbVar1[4] = 0;
    pbVar1[5] = 0;
    pbVar1[6] = 0;
    pbVar1[7] = 0;
    *(longlong *)(pbVar1 + 8) = lStack_48;
    *(undefined8 *)(pbVar1 + 0x10) = uVar3;
  }
  else {
    lStack_48 = *(longlong *)(pbVar1 + 8);
    uVar3 = *(undefined8 *)(pbVar1 + 0x10);
  }
  *(longlong *)(pbVar1 + 8) = lStack_48 + 1;
  uStack_58 = _DAT_142fdc0a0;
  uStack_50 = _UNK_142fdc0a8;
  uStack_68 = _DAT_142fdc090;
  uStack_64 = _UNK_142fdc094;
  uStack_60 = _UNK_142fdc098;
  uStack_5c = _UNK_142fdc09c;
  auStack_80[0] = 0;
  uStack_6c = 0;
  uStack_74 = 0;
  uStack_40 = uVar3;
  lVar2 = asset__texture__scan__scan_asset_tree_walker(param_2,param_3,auStack_80,1,0,&uStack_68);
  if (lVar2 == 0) {
    param_1[4] = lStack_48;
    param_1[5] = uStack_40;
    param_1[2] = uStack_58;
    param_1[3] = uStack_50;
    *param_1 = CONCAT44(uStack_64,uStack_68);
    param_1[1] = CONCAT44(uStack_5c,uStack_60);
  }
  else {
    param_1[1] = lVar2;
    *param_1 = 0;
    hashbrown__raw__RawTableInner__drop_inner_table(&uStack_68,&lStack_48,0x68,0x10);
  }
  return param_1;
}
