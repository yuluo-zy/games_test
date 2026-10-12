
void FUN_140bd44b0(longlong param_1,longlong param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong lVar3;
  ulonglong uVar4;
  code *pcVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *plVar9;
  longlong lVar10;
  
  if (*(longlong *)(param_2 + 0x10) != 0) {
    plVar9 = *(longlong **)(param_2 + 8);
    uVar1 = *(ulonglong *)(param_1 + 0x70);
    lVar2 = *(longlong *)(param_1 + 0x68);
    lVar3 = *(longlong *)(param_1 + 0x50);
    uVar4 = *(ulonglong *)(param_1 + 0x58);
    lVar10 = *(longlong *)(param_2 + 0x10) * 0x48;
    uVar6 = 0;
    do {
      if (*plVar9 == 0) {
        lVar7 = (ulonglong)*(byte *)((longlong)plVar9 + 0x36) - 6;
        if (1 < (byte)(*(byte *)((longlong)plVar9 + 0x36) - 7)) {
          lVar7 = 0;
        }
        if (lVar7 == 0) {
          lVar8 = 0x18;
          lVar7 = lVar3;
          if (uVar4 <= uVar6) {
            FUN_1428d9518(uVar6,uVar4,&UNK_142b0f630);
            pcVar5 = (code *)swi(3);
            (*pcVar5)();
            return;
          }
        }
        else {
          if (lVar7 != 1) goto code_r0x000140bd44fa;
          lVar8 = 0x20;
          lVar7 = lVar2;
          if (uVar1 <= uVar6) {
            FUN_1428d9518(uVar6,uVar1,&UNK_142b0f648);
            return;
          }
        }
        *(undefined4 *)(lVar7 + uVar6 * 4) = *(undefined4 *)((longlong)plVar9 + lVar8);
      }
code_r0x000140bd44fa:
      plVar9 = plVar9 + 9;
      uVar6 = uVar6 + 1;
      lVar10 = lVar10 + -0x48;
    } while (lVar10 != 0);
  }
  return;
}

