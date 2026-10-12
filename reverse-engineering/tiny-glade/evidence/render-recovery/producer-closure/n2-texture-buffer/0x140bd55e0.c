
undefined8 * FUN_140bd55e0(undefined8 *param_1,undefined8 *param_2,uint *param_3)

{
  uint uVar1;
  code *pcVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  bool bVar6;
  
  uVar1 = *param_3;
  if (uVar1 != 0) {
    uVar3 = *(uint *)((longlong)param_2 + 0xc) / uVar1;
    uVar1 = (uint)(*(uint *)((longlong)param_2 + 0xc) % uVar1 != 0);
    *(uint *)((longlong)param_2 + 0xc) = uVar3 + uVar1 + (uint)(uVar3 + uVar1 == 0);
    uVar1 = param_3[1];
    if (uVar1 != 0) {
      uVar3 = *(uint *)(param_2 + 2) / uVar1;
      uVar1 = (uint)(*(uint *)(param_2 + 2) % uVar1 != 0);
      *(uint *)(param_2 + 2) = uVar3 + uVar1 + (uint)(uVar3 + uVar1 == 0);
      uVar1 = param_3[2];
      if (uVar1 != 0) {
        uVar3 = *(uint *)((longlong)param_2 + 0x14) / uVar1;
        bVar6 = *(uint *)((longlong)param_2 + 0x14) % uVar1 != 0;
        *(uint *)((longlong)param_2 + 0x14) = uVar3 + bVar6 + (uint)(uVar3 + bVar6 == 0);
        uVar4 = param_2[1];
        *param_1 = *param_2;
        param_1[1] = uVar4;
        *(undefined4 *)(param_1 + 2) = *(undefined4 *)(param_2 + 2);
        *(undefined4 *)((longlong)param_1 + 0x14) = *(undefined4 *)((longlong)param_2 + 0x14);
        param_1[3] = param_2[3];
        return param_1;
      }
    }
  }
  FUN_1428d99e0(&UNK_142b0fab0);
  pcVar2 = (code *)swi(3);
  puVar5 = (undefined8 *)(*pcVar2)();
  return puVar5;
}

