
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1 (*) [16]
FUN_140c98890(undefined1 (*param_1) [16],uint param_2,uint param_3,undefined8 param_4)

{
  code *pcVar1;
  undefined1 (*pauVar2) [16];
  
  if ((param_2 & 0x7fffffff) < 0x7f800000) {
    if ((param_3 & 0x7fffffff) < 0x7f800000) {
      *(uint *)param_1[1] = param_2;
      *(uint *)(param_1[1] + 4) = param_3;
      *param_1 = ZEXT416(_DAT_142925904);
      return param_1;
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e5f8,0x23,param_4);
  }
  FUN_1428d9430(&UNK_142b2e61b,0x24,param_4);
  pcVar1 = (code *)swi(3);
  pauVar2 = (undefined1 (*) [16])(*pcVar1)();
  return pauVar2;
}

