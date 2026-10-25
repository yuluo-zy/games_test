
undefined8 * FUN_1418aead0(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  code *pcVar1;
  uint uVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  undefined8 *puVar5;
  bool bVar6;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  puVar5 = *(undefined8 **)(param_1 + 0x170);
  do {
    puVar3 = puVar5;
    if (puVar3 == (undefined8 *)0x0) {
      uStack_28 = 0;
      uStack_20 = 1;
      puVar5 = &uStack_28;
      uStack_18 = param_2;
      uStack_10 = param_3;
      uVar4 = FUN_1418bc030(param_1,puVar5);
      if (uVar4 < *(ulonglong *)(param_1 + 0x160)) {
        uVar2 = func_0x000140c111b0(uVar4);
        if (uVar2 < 0x2c) {
          return (undefined8 *)
                 ((longlong)puVar5 * 0x20 + *(longlong *)(param_1 + (ulonglong)uVar2 * 8));
        }
      }
      else {
        FUN_1428d9430(&UNK_142d0528e,0x22,&UNK_142d02268);
      }
      FUN_1428d9518();
      pcVar1 = (code *)swi(3);
      puVar5 = (undefined8 *)(*pcVar1)();
      return puVar5;
    }
    LOCK();
    puVar5 = *(undefined8 **)(param_1 + 0x170);
    bVar6 = puVar3 == puVar5;
    if (bVar6) {
      *(undefined8 *)(param_1 + 0x170) = puVar3[2];
      puVar5 = puVar3;
    }
    UNLOCK();
  } while (!bVar6);
  puVar3[2] = param_2;
  puVar3[3] = param_3;
  *(undefined4 *)(puVar3 + 1) = 1;
  *puVar3 = 0;
  return puVar3;
}

