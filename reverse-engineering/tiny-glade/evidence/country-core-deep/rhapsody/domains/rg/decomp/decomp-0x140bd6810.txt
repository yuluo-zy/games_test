
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_140bd6810(longlong param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uStack_ca;
  undefined1 uStack_c9;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined1 *puStack_90;
  undefined8 uStack_88;
  undefined1 *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined1 auStack_50 [8];
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 uStack_18;
  
  uStack_ca = *(undefined1 *)(param_2 + 2);
  uStack_c9 = *(undefined1 *)((longlong)param_2 + 0x11);
  uStack_70 = *param_2;
  puStack_90 = &uStack_ca;
  uStack_88 = 1;
  puStack_80 = &uStack_c9;
  uStack_78 = 1;
  uStack_58 = param_2[1];
  uStack_68 = 0;
  uStack_60 = 0xffffffffffffffff;
  FUN_140c10940(auStack_50,&puStack_90);
  uStack_98 = uStack_18;
  uStack_a8 = uStack_28;
  uStack_a4 = uStack_24;
  uStack_a0 = uStack_20;
  uStack_9c = uStack_1c;
  uStack_b8 = uStack_38;
  uStack_b0 = uStack_30;
  uStack_c8 = uStack_48;
  uStack_c0 = uStack_40;
  *(ulonglong *)(param_1 + 0x48) =
       *(ulonglong *)(param_1 + 0x48) |
       CONCAT44(auStack_50._4_4_ & _UNK_142b0ff54,auStack_50._0_4_ & _UNK_142b0ff50);
  lVar2 = *(longlong *)(param_1 + 0x28);
  if (lVar2 == *(longlong *)(param_1 + 0x18)) {
    FUN_1418b0310(param_1 + 0x18,&UNK_142b0ffa0);
  }
  lVar3 = *(longlong *)(param_1 + 0x20);
  lVar4 = lVar2 * 0x38;
  *(undefined8 *)(lVar3 + 0x30 + lVar4) = uStack_98;
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar4);
  *puVar1 = CONCAT44(uStack_a4,uStack_a8);
  puVar1[1] = CONCAT44(uStack_9c,uStack_a0);
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar4);
  *puVar1 = uStack_b8;
  puVar1[1] = uStack_b0;
  *(undefined8 *)(lVar3 + lVar4) = uStack_c8;
  ((undefined8 *)(lVar3 + lVar4))[1] = uStack_c0;
  *(longlong *)(param_1 + 0x28) = lVar2 + 1;
  return param_1;
}

