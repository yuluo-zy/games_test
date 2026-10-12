
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_140be22c0(longlong *param_1)

{
  longlong lVar1;
  undefined8 uVar2;
  
  lVar1 = param_1[1];
  FUN_140be1bf0(lVar1,param_1[2]);
  if (*param_1 == 0) {
    return;
  }
  uVar2 = GetProcessHeap(lVar1,*param_1 << 6);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar2,0,lVar1);
  return;
}

