
void FUN_140be78b0(longlong *param_1,longlong param_2,ulonglong param_3)

{
  undefined1 uVar1;
  undefined1 uVar2;
  ushort uVar3;
  code *pcVar4;
  ulonglong uVar5;
  ulonglong in_R10;
  
  param_3 = param_3 & 0xffffffff;
  uVar5 = *(ulonglong *)(param_2 + 0x10);
  if (param_3 < uVar5) {
    uVar5 = *(ulonglong *)(param_2 + 0x40);
    uVar3 = *(ushort *)(*(longlong *)(param_2 + 8) + param_3 * 4);
    in_R10 = (ulonglong)uVar3;
    if (in_R10 < uVar5) {
      if (param_3 < *(ulonglong *)(param_2 + 0x28)) {
        uVar1 = *(undefined1 *)(*(longlong *)(param_2 + 8) + 2 + param_3 * 4);
        uVar2 = *(undefined1 *)(*(longlong *)(param_2 + 0x20) + param_3);
        *param_1 = (ulonglong)uVar3 * 0x40 + *(longlong *)(param_2 + 0x38);
        *(undefined1 *)(param_1 + 1) = uVar2;
        *(undefined1 *)((longlong)param_1 + 9) = uVar1;
        return;
      }
      goto code_r0x000140be7925;
    }
  }
  else {
    FUN_1428d9518(param_3,uVar5,&UNK_142b13c60);
  }
  FUN_1428d9518(in_R10,uVar5,&UNK_142b13c78);
code_r0x000140be7925:
  FUN_1428d9518();
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

