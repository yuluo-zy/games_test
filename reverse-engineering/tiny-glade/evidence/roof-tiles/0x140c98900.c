
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong *
FUN_140c98900(ulonglong *param_1,uint param_2,uint param_3,undefined4 param_4,undefined4 param_5,
             undefined8 param_6)

{
  code *pcVar1;
  ulonglong *puVar2;
  
  if ((param_2 & 0x7fffffff) < 0x7f800000) {
    if ((param_3 & 0x7fffffff) < 0x7f800000) {
      *param_1 = (ulonglong)_DAT_142925904;
      *(uint *)(param_1 + 2) = param_2;
      *(uint *)((longlong)param_1 + 0x14) = param_3;
      *(undefined4 *)(param_1 + 1) = param_4;
      *(undefined4 *)((longlong)param_1 + 0xc) = param_5;
      return param_1;
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e5f8,0x23,param_6);
  }
  FUN_1428d9430(&UNK_142b2e61b,0x24);
  pcVar1 = (code *)swi(3);
  puVar2 = (ulonglong *)(*pcVar1)();
  return puVar2;
}

