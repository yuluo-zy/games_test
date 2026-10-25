
undefined8 * FUN_140bd4f20(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  code *pcVar1;
  undefined8 *puVar2;
  undefined1 uStack_1;
  
  if (param_3 < 0x10000) {
    *(short *)((longlong)param_2 + 0xc) = (short)param_3;
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = *(undefined4 *)(param_2 + 1);
    *(undefined2 *)((longlong)param_1 + 0xc) = *(undefined2 *)((longlong)param_2 + 0xc);
    *(undefined2 *)((longlong)param_1 + 0xe) = *(undefined2 *)((longlong)param_2 + 0xe);
    return param_1;
  }
  FUN_1428d9760(&UNK_142b0f8c8,5,&uStack_1,&UNK_142b0f768,&UNK_142b0f8d0);
  pcVar1 = (code *)swi(3);
  puVar2 = (undefined8 *)(*pcVar1)();
  return puVar2;
}

