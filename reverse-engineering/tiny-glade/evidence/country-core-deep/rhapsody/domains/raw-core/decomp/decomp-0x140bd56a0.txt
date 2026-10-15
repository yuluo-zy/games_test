
undefined8 * FUN_140bd56a0(undefined8 *param_1,undefined8 *param_2,uint *param_3)

{
  code *pcVar1;
  undefined8 uVar2;
  uint uVar3;
  undefined8 *puVar4;
  
  if (*param_3 != 0) {
    uVar3 = *(uint *)((longlong)param_2 + 0xc) / *param_3;
    *(uint *)((longlong)param_2 + 0xc) = uVar3 + (uVar3 == 0);
    if (param_3[1] != 0) {
      uVar3 = *(uint *)(param_2 + 2) / param_3[1];
      *(uint *)(param_2 + 2) = uVar3 + (uVar3 == 0);
      if (param_3[2] != 0) {
        uVar3 = *(uint *)((longlong)param_2 + 0x14) / param_3[2];
        *(uint *)((longlong)param_2 + 0x14) = uVar3 + (uVar3 == 0);
        uVar2 = param_2[1];
        *param_1 = *param_2;
        param_1[1] = uVar2;
        *(undefined4 *)(param_1 + 2) = *(undefined4 *)(param_2 + 2);
        *(undefined4 *)((longlong)param_1 + 0x14) = *(undefined4 *)((longlong)param_2 + 0x14);
        param_1[3] = param_2[3];
        return param_1;
      }
    }
  }
  FUN_1428d99e0(&UNK_142b0fac8);
  pcVar1 = (code *)swi(3);
  puVar4 = (undefined8 *)(*pcVar1)();
  return puVar4;
}

