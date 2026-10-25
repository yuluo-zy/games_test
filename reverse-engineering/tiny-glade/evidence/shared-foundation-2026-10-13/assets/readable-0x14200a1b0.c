
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 * asset__prefab__scan__scan_asset_tree(undefined8 *param_1,undefined8 param_2,undefined8 param_3)
{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte *pbVar5;
  longlong lVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined8 *puVar9;
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
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  undefined8 uStack_50;
  uStack_50 = 0xfffffffffffffffe;
  uVar7 = param_2;
  pbVar5 = (byte *)FUN_140d4d740();
  if ((*pbVar5 & 1) == 0) {
    uStack_78 = FUN_140d64e00();
    pbVar5[0] = 1;
    pbVar5[1] = 0;
    pbVar5[2] = 0;
    pbVar5[3] = 0;
    pbVar5[4] = 0;
    pbVar5[5] = 0;
    pbVar5[6] = 0;
    pbVar5[7] = 0;
    *(longlong *)(pbVar5 + 8) = uStack_78;
    *(undefined8 *)(pbVar5 + 0x10) = uVar7;
  }
  else {
    uStack_78 = *(longlong *)(pbVar5 + 8);
    uVar7 = *(undefined8 *)(pbVar5 + 0x10);
  }
  *(longlong *)(pbVar5 + 8) = uStack_78 + 1;
  uVar4 = _UNK_142fce618;
  uVar3 = _DAT_142fce610;
  uVar2 = _UNK_142fce608;
  uVar1 = _DAT_142fce600;
  uStack_88 = _DAT_142fce610;
  uStack_80 = _UNK_142fce618;
  uStack_98 = _DAT_142fce600;
  uStack_90 = _UNK_142fce608;
  uStack_70 = uVar7;
  pbVar5 = (byte *)FUN_140d4d740();
  if ((*pbVar5 & 1) == 0) {
    lStack_a8 = FUN_140d64e00();
    pbVar5[0] = 1;
    pbVar5[1] = 0;
    pbVar5[2] = 0;
    pbVar5[3] = 0;
    pbVar5[4] = 0;
    pbVar5[5] = 0;
    pbVar5[6] = 0;
    pbVar5[7] = 0;
    *(longlong *)(pbVar5 + 8) = lStack_a8;
    *(undefined8 *)(pbVar5 + 0x10) = uVar7;
  }
  else {
    lStack_a8 = *(longlong *)(pbVar5 + 8);
    uVar7 = *(undefined8 *)(pbVar5 + 0x10);
  }
  *(longlong *)(pbVar5 + 8) = lStack_a8 + 1;
  uStack_d8 = (undefined4)uStack_78;
  uStack_d4 = uStack_78._4_4_;
  uStack_d0 = (undefined4)uStack_70;
  uStack_cc = uStack_70._4_4_;
  uStack_e8 = (undefined4)uStack_88;
  uStack_e4 = uStack_88._4_4_;
  uStack_e0 = (undefined4)uStack_80;
  uStack_dc = uStack_80._4_4_;
  uStack_f8 = (undefined4)uStack_98;
  uStack_f4 = uStack_98._4_4_;
  uStack_f0 = (undefined4)uStack_90;
  uStack_ec = uStack_90._4_4_;
  uStack_c8 = uVar1;
  uStack_c0 = uVar2;
  uStack_b8 = uVar3;
  uStack_b0 = uVar4;
  lStack_68 = 0;
  lStack_60 = 8;
  lStack_58 = 0;
  uStack_a0 = uVar7;
  pbVar5 = (byte *)FUN_140d4d740();
  if ((*pbVar5 & 1) == 0) {
    uStack_78 = FUN_140d64e00();
    pbVar5[0] = 1;
    pbVar5[1] = 0;
    pbVar5[2] = 0;
    pbVar5[3] = 0;
    pbVar5[4] = 0;
    pbVar5[5] = 0;
    pbVar5[6] = 0;
    pbVar5[7] = 0;
    *(longlong *)(pbVar5 + 8) = uStack_78;
    *(undefined8 *)(pbVar5 + 0x10) = uVar7;
  }
  else {
    uStack_78 = *(longlong *)(pbVar5 + 8);
    uVar7 = *(undefined8 *)(pbVar5 + 0x10);
  }
  *(longlong *)(pbVar5 + 8) = uStack_78 + 1;
  uStack_88 = uVar3;
  uStack_80 = uVar4;
  uStack_98 = uVar1;
  uStack_90 = uVar2;
  uStack_70 = uVar7;
  lVar6 = asset__prefab__scan__scan_asset_tree_walker(param_2,param_3,&uStack_98,1,0,&lStack_68,&uStack_f8);
  if (lVar6 == 0) {
    hashbrown__raw__RawTableInner__drop_inner_table(&uStack_98,&uStack_78,0xd8,0x10);
    lVar6 = lStack_60;
    param_1[10] = lStack_a8;
    param_1[0xb] = uStack_a0;
    param_1[8] = uStack_b8;
    param_1[9] = uStack_b0;
    param_1[6] = uStack_c8;
    param_1[7] = uStack_c0;
    param_1[4] = CONCAT44(uStack_d4,uStack_d8);
    param_1[5] = CONCAT44(uStack_cc,uStack_d0);
    param_1[2] = CONCAT44(uStack_e4,uStack_e8);
    param_1[3] = CONCAT44(uStack_dc,uStack_e0);
    *(undefined4 *)param_1 = uStack_f8;
    *(undefined4 *)((longlong)param_1 + 4) = uStack_f4;
    *(undefined4 *)(param_1 + 1) = uStack_f0;
    *(undefined4 *)((longlong)param_1 + 0xc) = uStack_ec;
    if (lStack_58 != 0) {
      puVar9 = (undefined8 *)(lStack_60 + 8);
      lVar8 = lStack_58;
      do {
        if (puVar9[-1] != 0) {
          func_0x000140613c20(*puVar9,puVar9[-1],1);
        }
        puVar9 = puVar9 + 4;
        lVar8 = lVar8 + -1;
      } while (lVar8 != 0);
    }
    if (lStack_68 != 0) {
      func_0x000140613c20(lVar6,lStack_68 << 5,8);
    }
  }
  else {
    param_1[1] = lVar6;
    *param_1 = 0;
    hashbrown__raw__RawTableInner__drop_inner_table(&uStack_98,&uStack_78,0xd8,0x10);
    lVar6 = lStack_60;
    if (lStack_58 != 0) {
      puVar9 = (undefined8 *)(lStack_60 + 8);
      lVar8 = lStack_58;
      do {
        if (puVar9[-1] != 0) {
          func_0x000140613c20(*puVar9,puVar9[-1],1);
        }
        puVar9 = puVar9 + 4;
        lVar8 = lVar8 + -1;
      } while (lVar8 != 0);
    }
    if (lStack_68 != 0) {
      func_0x000140613c20(lVar6,lStack_68 << 5,8);
    }
    hashbrown__raw__RawTableInner__drop_inner_table(&uStack_f8,&uStack_d8,0x40,0x10);
    hashbrown__raw__RawTableInner__drop_inner_table(&uStack_c8,&lStack_a8,0x38,0x10);
  }
  return param_1;
}
