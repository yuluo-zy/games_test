
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_1405fc3d0(longlong param_1)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  
  FUN_14004ceb0(param_1,param_1 + 0x20,0x68,0x10,param_1,0xfffffffffffffffe);
  lVar1 = *(longlong *)(param_1 + 0x38);
  if ((lVar1 != 0) && (lVar1 * 0x11 != -0x21)) {
    lVar2 = *(longlong *)(param_1 + 0x30);
    uVar3 = GetProcessHeap();
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    HeapFree(uVar3,0,lVar2 + lVar1 * -0x10 + -0x10);
    return;
  }
  return;
}

