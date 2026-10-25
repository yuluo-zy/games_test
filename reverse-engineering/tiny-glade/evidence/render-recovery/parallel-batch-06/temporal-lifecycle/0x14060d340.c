
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_14060d340(longlong *param_1)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  
  if (*param_1 != 0) {
    FUN_14004ceb0(param_1,param_1 + 4,0x68,0x10,param_1,0xfffffffffffffffe);
    lVar1 = param_1[7];
    if ((lVar1 != 0) && (lVar1 * 0x11 != -0x21)) {
      lVar2 = param_1[6];
      uVar3 = GetProcessHeap();
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      HeapFree(uVar3,0,lVar2 + lVar1 * -0x10 + -0x10);
      return;
    }
  }
  return;
}

