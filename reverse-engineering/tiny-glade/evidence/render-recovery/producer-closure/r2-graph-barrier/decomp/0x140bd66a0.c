
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_140bd66a0(longlong param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined1 uStack_f2;
  undefined1 uStack_f1;
  undefined1 *puStack_f0;
  undefined8 uStack_e8;
  undefined1 *puStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined2 uStack_ac;
  undefined1 uStack_aa;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined1 auStack_60 [8];
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 uStack_18;
  
  uStack_c0 = *(undefined4 *)(param_2 + 1);
  uStack_f2 = *(undefined1 *)((longlong)param_2 + 0x14);
  uStack_f1 = *(undefined1 *)((longlong)param_2 + 0x15);
  uStack_aa = *(undefined1 *)((longlong)param_2 + 0x16);
  uStack_d0 = *param_2;
  puStack_f0 = &uStack_f2;
  uStack_e8 = 1;
  puStack_e0 = &uStack_f1;
  uStack_d8 = 1;
  uStack_ac = 0;
  uStack_c8 = *(undefined8 *)((longlong)param_2 + 0xc);
  uStack_bc = _UNK_142b0ff40;
  uStack_b8 = _UNK_142b0ff44;
  uStack_b4 = _UNK_142b0ff48;
  uStack_b0 = _UNK_142b0ff4c;
  FUN_140c10af0(auStack_60,&puStack_f0);
  uStack_68 = uStack_18;
  uStack_78 = uStack_28;
  uStack_74 = uStack_24;
  uStack_70 = uStack_20;
  uStack_6c = uStack_1c;
  uStack_88 = uStack_38;
  uStack_84 = uStack_34;
  uStack_80 = uStack_30;
  uStack_7c = uStack_2c;
  uStack_98 = uStack_48;
  uStack_90 = uStack_40;
  uStack_a8 = uStack_58;
  uStack_a0 = uStack_50;
  *(ulonglong *)(param_1 + 0x48) =
       *(ulonglong *)(param_1 + 0x48) |
       CONCAT44(auStack_60._4_4_ & _UNK_142b0ff54,auStack_60._0_4_ & _UNK_142b0ff50);
  lVar2 = *(longlong *)(param_1 + 0x40);
  if (lVar2 == *(longlong *)(param_1 + 0x30)) {
    FUN_1418b03d0(param_1 + 0x30,&UNK_142b0ff88);
  }
  lVar3 = *(longlong *)(param_1 + 0x38);
  *(undefined8 *)(lVar3 + 0x40 + lVar2 * 0x48) = uStack_68;
  puVar1 = (undefined8 *)(lVar3 + 0x30 + lVar2 * 0x48);
  *puVar1 = CONCAT44(uStack_74,uStack_78);
  puVar1[1] = CONCAT44(uStack_6c,uStack_70);
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar2 * 0x48);
  *puVar1 = CONCAT44(uStack_84,uStack_88);
  puVar1[1] = CONCAT44(uStack_7c,uStack_80);
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar2 * 0x48);
  *puVar1 = uStack_98;
  puVar1[1] = uStack_90;
  puVar1 = (undefined8 *)(lVar3 + lVar2 * 0x48);
  *puVar1 = uStack_a8;
  puVar1[1] = uStack_a0;
  *(longlong *)(param_1 + 0x40) = lVar2 + 1;
  return param_1;
}

