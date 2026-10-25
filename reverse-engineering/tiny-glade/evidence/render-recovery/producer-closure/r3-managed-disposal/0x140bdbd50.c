
undefined8 * FUN_140bdbd50(longlong param_1,undefined8 param_2)

{
  code *pcVar1;
  uint uVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  undefined8 *puVar5;
  bool bVar6;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined1 auStack_d8 [192];
  
  puVar5 = *(undefined8 **)(param_1 + 0x178);
  do {
    puVar3 = puVar5;
    if (puVar3 == (undefined8 *)0x0) {
      func_0x000142923c00(auStack_d8,param_2,0xc0);
      uStack_e8 = 0;
      uStack_e0 = 1;
      puVar5 = &uStack_e8;
      uVar4 = FUN_1418bbd30(param_1 + 8,puVar5);
      if (uVar4 < *(ulonglong *)(param_1 + 0x168)) {
        uVar2 = func_0x000140c111b0(uVar4);
        if (uVar2 < 0x2c) {
          return (undefined8 *)
                 ((longlong)puVar5 * 0xd0 + *(longlong *)(param_1 + 8 + (ulonglong)uVar2 * 8));
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
    puVar5 = *(undefined8 **)(param_1 + 0x178);
    bVar6 = puVar3 == puVar5;
    if (bVar6) {
      *(undefined8 *)(param_1 + 0x178) = puVar3[2];
      puVar5 = puVar3;
    }
    UNLOCK();
  } while (!bVar6);
  func_0x000142923c00(puVar3 + 2,param_2,0xc0);
  *(undefined4 *)(puVar3 + 1) = 1;
  *puVar3 = 0;
  return puVar3;
}

