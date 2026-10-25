
ulonglong FUN_1416f6d90(longlong param_1,int param_2)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  longlong lVar4;
  code *pcVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  int iStack_44;
  int *piStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined4 **ppuStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  lVar4 = *(longlong *)(param_1 + 0x10);
  uVar6 = (ulonglong)*(uint *)(param_1 + 0x18);
  uVar7 = *(ulonglong *)(lVar4 + 0x70);
  iStack_44 = param_2;
  if (uVar7 <= uVar6) goto code_r0x0001416f6e76;
  uVar7 = *(ulonglong *)(lVar4 + 0x68);
  uVar2 = *(uint *)(uVar7 + uVar6 * 8);
  uVar3 = *(uint *)(uVar7 + 4 + uVar6 * 8);
  uVar6 = (ulonglong)uVar3;
  if (uVar3 < uVar2) {
    uVar6 = FUN_1428d9850((ulonglong)uVar2,uVar6,&UNK_142c91560);
code_r0x0001416f6e0c:
    FUN_1428d9840(uVar6,uVar7,&UNK_142c91560);
  }
  else {
    uVar7 = *(ulonglong *)(lVar4 + 0x58);
    if (uVar7 < uVar6) goto code_r0x0001416f6e0c;
    uVar8 = (ulonglong)uVar2 * 0x14;
    while (uVar6 * 0x14 != uVar8) {
      uVar7 = uVar8 + 0x14;
      piVar1 = (int *)(*(longlong *)(lVar4 + 0x50) + uVar8);
      uVar8 = uVar7;
      if (*piVar1 == param_2) {
        return (ulonglong)*(byte *)(*(longlong *)(lVar4 + 0x50) + -4 + uVar7);
      }
    }
  }
  piStack_40 = &iStack_44;
  pcStack_38 = FUN_1416f7e80;
  puStack_30 = &UNK_142c91598;
  uStack_28 = 1;
  uStack_10 = 0;
  ppuStack_20 = &piStack_40;
  uStack_18 = 1;
  uVar6 = FUN_1428d9390(&puStack_30,&UNK_142c915a8);
code_r0x0001416f6e76:
  FUN_1428d9518(uVar6,uVar7,&UNK_142c91548);
  pcVar5 = (code *)swi(3);
  uVar7 = (*pcVar5)();
  return uVar7;
}

