
undefined8 * FUN_140bd5540(undefined8 *param_1,undefined8 *param_2,int param_3)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  
  if ((param_3 != 1) &&
     ((6 < *(byte *)((longlong)param_2 + 0x1e) ||
      ((0x4aU >> (*(byte *)((longlong)param_2 + 0x1e) & 0x1f) & 1) == 0)))) {
    FUN_1428d9430(&UNK_142b0fa21,0x73,&UNK_142b0fa98);
    pcVar1 = (code *)swi(3);
    puVar3 = (undefined8 *)(*pcVar1)();
    return puVar3;
  }
  *(int *)(param_2 + 3) = param_3;
  uVar2 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = uVar2;
  param_1[2] = param_2[2];
  *(undefined4 *)(param_1 + 3) = *(undefined4 *)(param_2 + 3);
  *(undefined4 *)((longlong)param_1 + 0x1c) = *(undefined4 *)((longlong)param_2 + 0x1c);
  return param_1;
}

