
longlong FUN_1402c4280(longlong param_1,undefined8 param_2,undefined4 *param_3,longlong param_4)

{
  undefined1 auStack_258 [144];
  longlong lStack_1c8;
  longlong lStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  longlong lStack_40;
  undefined4 *puStack_38;
  undefined8 uStack_30;
  undefined1 uStack_21;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  lStack_1c8 = *(longlong *)(param_4 + 0x80);
  lStack_1c0 = lStack_1c8 + *(longlong *)(param_4 + 0x88) * 0x18;
  uStack_1b8 = 0;
  uStack_21 = 1;
  lStack_40 = param_4;
  puStack_38 = param_3;
  uStack_30 = param_2;
  FUN_1402bf880(&uStack_70,&lStack_1c8,&UNK_1429ae8a0);
  func_0x000142923c00(auStack_258,uStack_30,0x90);
  uStack_48 = *(undefined8 *)(puStack_38 + 4);
  uStack_58 = *puStack_38;
  uStack_54 = puStack_38[1];
  uStack_50 = puStack_38[2];
  uStack_4c = puStack_38[3];
  func_0x000142923c00(&lStack_1c8,lStack_40,0x108);
  func_0x000140bd6650(&uStack_c0);
  *(undefined8 *)(param_1 + 0x210) = uStack_60;
  *(undefined4 *)(param_1 + 0x200) = uStack_70;
  *(undefined4 *)(param_1 + 0x204) = uStack_6c;
  *(undefined4 *)(param_1 + 0x208) = uStack_68;
  *(undefined4 *)(param_1 + 0x20c) = uStack_64;
  func_0x000142923c00(param_1,auStack_258,0x90);
  *(undefined4 *)(param_1 + 0x90) = uStack_58;
  *(undefined4 *)(param_1 + 0x94) = uStack_54;
  *(undefined4 *)(param_1 + 0x98) = uStack_50;
  *(undefined4 *)(param_1 + 0x9c) = uStack_4c;
  *(undefined8 *)(param_1 + 0xa0) = uStack_48;
  func_0x000142923c00(param_1 + 0xa8,&lStack_1c8,0x108);
  *(undefined8 *)(param_1 + 0x1b0) = uStack_c0;
  *(undefined8 *)(param_1 + 0x1b8) = uStack_b8;
  *(undefined8 *)(param_1 + 0x1c0) = uStack_b0;
  *(undefined8 *)(param_1 + 0x1c8) = uStack_a8;
  *(undefined8 *)(param_1 + 0x1d0) = uStack_a0;
  *(undefined8 *)(param_1 + 0x1d8) = uStack_98;
  *(undefined8 *)(param_1 + 0x1e0) = uStack_90;
  *(undefined8 *)(param_1 + 0x1e8) = uStack_88;
  *(undefined8 *)(param_1 + 0x1f0) = uStack_80;
  *(undefined8 *)(param_1 + 0x1f8) = uStack_78;
  return param_1;
}

