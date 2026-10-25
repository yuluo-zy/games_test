
undefined8 * FUN_1418ae9f0(longlong param_1,undefined8 *param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  uint uVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  bool bVar7;
  undefined4 extraout_XMM0_Da;
  undefined4 uVar8;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined8 uStack_10;
  
  puVar6 = *(undefined8 **)(param_1 + 0x170);
  do {
    puVar4 = puVar6;
    if (puVar4 == (undefined8 *)0x0) {
      uStack_30 = 0;
      uStack_28 = 1;
      uStack_20 = *(undefined4 *)param_2;
      uStack_1c = *(undefined4 *)((longlong)param_2 + 4);
      uStack_18 = *(undefined4 *)(param_2 + 1);
      uStack_14 = *(undefined4 *)((longlong)param_2 + 0xc);
      uStack_10 = param_2[2];
      puVar6 = &uStack_30;
      uVar5 = FUN_1418bba00(param_1,puVar6);
      if (uVar5 < *(ulonglong *)(param_1 + 0x160)) {
        uVar3 = func_0x000140c111b0(uVar5);
        uVar8 = extraout_XMM0_Da;
        if (uVar3 < 0x2c) {
          return (undefined8 *)
                 ((longlong)puVar6 * 0x28 + *(longlong *)(param_1 + (ulonglong)uVar3 * 8));
        }
      }
      else {
        uVar8 = FUN_1428d9430(&UNK_142d0528e,0x22,&UNK_142d02268);
      }
      FUN_1428d9518(uVar8,0x2c,&UNK_142d02268);
      pcVar1 = (code *)swi(3);
      puVar6 = (undefined8 *)(*pcVar1)();
      return puVar6;
    }
    LOCK();
    puVar6 = *(undefined8 **)(param_1 + 0x170);
    bVar7 = puVar4 == puVar6;
    if (bVar7) {
      *(undefined8 *)(param_1 + 0x170) = puVar4[2];
      puVar6 = puVar4;
    }
    UNLOCK();
  } while (!bVar7);
  puVar4[4] = param_2[2];
  uVar2 = param_2[1];
  puVar4[2] = *param_2;
  puVar4[3] = uVar2;
  *(undefined4 *)(puVar4 + 1) = 1;
  *puVar4 = 0;
  return puVar4;
}

