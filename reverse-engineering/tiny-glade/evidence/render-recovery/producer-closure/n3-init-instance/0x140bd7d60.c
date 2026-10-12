
undefined8 FUN_140bd7d60(longlong param_1,undefined8 param_2,longlong param_3)

{
  longlong lVar1;
  longlong lVar2;
  int iVar3;
  longlong lVar4;
  
  lVar1 = *(longlong *)(param_1 + 0x10);
  if (lVar1 != 0) {
    lVar2 = *(longlong *)(param_1 + 8);
    lVar4 = 0;
    do {
      if (((*(int *)(lVar2 + lVar4) == 1) && (*(longlong *)(lVar2 + 0x18 + lVar4) == param_3)) &&
         (iVar3 = func_0x000142923bf0(*(undefined8 *)(lVar2 + 0x10 + lVar4),param_2,param_3),
         iVar3 == 0)) {
        return 1;
      }
      lVar4 = lVar4 + 0x20;
    } while (lVar1 << 5 != lVar4);
  }
  return 0;
}

