
undefined4 * FUN_140fe7cb0(undefined4 *param_1,longlong param_2,int param_3,uint param_4)

{
  code *pcVar1;
  undefined4 *puVar2;
  longlong lVar3;
  
  if ((ulonglong)param_4 < *(ulonglong *)(param_2 + 0x10)) {
    lVar3 = (ulonglong)param_4 * 0x330;
    if (*(int *)(*(longlong *)(param_2 + 8) + 0x328 + lVar3) == param_3) {
      *(longlong *)(param_1 + 2) = *(longlong *)(param_2 + 8) + lVar3;
      *param_1 = 4;
      return param_1;
    }
  }
  FUN_1428d8725(&UNK_142f94fb8,0x18,&UNK_142bc2ad8);
  pcVar1 = (code *)swi(3);
  puVar2 = (undefined4 *)(*pcVar1)();
  return puVar2;
}

