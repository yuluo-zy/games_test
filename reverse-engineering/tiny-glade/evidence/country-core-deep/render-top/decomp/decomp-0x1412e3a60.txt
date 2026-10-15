
void FUN_1412e3a60(longlong *param_1,longlong param_2,longlong param_3)

{
  byte bVar1;
  code *pcVar2;
  uint uVar3;
  uint *puVar4;
  ulonglong uVar5;
  uint uVar6;
  uint *puVar7;
  uint uVar8;
  uint uVar9;
  longlong lVar10;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar10 = *(longlong *)(param_3 + 8);
  if (lVar10 == 0) {
    FUN_1428d9310(&UNK_142c2ec00);
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  }
  puVar7 = *(uint **)(lVar10 + 8);
  puVar4 = puVar7 + *(longlong *)(lVar10 + 0x10);
  lStack_60 = 0;
  lStack_58 = 4;
  lStack_50 = 0;
  lVar10 = 0;
  uVar6 = 0;
  uVar8 = 0;
  while( true ) {
    if (puVar7 == puVar4) {
      if (uVar6 <= uVar8 && uVar8 - uVar6 != 0) {
        if (lVar10 == lStack_60) {
          FUN_140562e70(&lStack_60,&UNK_142c2ec18);
        }
        *(uint *)(lStack_58 + lVar10 * 8) = uVar6;
        *(uint *)(lStack_58 + 4 + lVar10 * 8) = uVar8 - uVar6;
        lStack_50 = lVar10 + 1;
      }
      param_1[2] = lStack_50;
      *param_1 = lStack_60;
      param_1[1] = lStack_58;
      return;
    }
    uVar5 = (ulonglong)*puVar7;
    if (*(ulonglong *)(param_2 + 0x28) <= uVar5) break;
    uVar9 = *puVar7 * 0x20;
    bVar1 = *(byte *)(*(longlong *)(param_2 + 0x20) + uVar5);
    uVar3 = uVar6;
    if ((uVar8 != uVar9) && (uVar3 = uVar9, uVar6 <= uVar8 && uVar8 - uVar6 != 0)) {
      if (lVar10 == lStack_60) {
        FUN_140562e70(&lStack_60,&UNK_142c2ec48);
      }
      *(uint *)(lStack_58 + lVar10 * 8) = uVar6;
      *(uint *)(lStack_58 + 4 + lVar10 * 8) = uVar8 - uVar6;
      lVar10 = lVar10 + 1;
      lStack_50 = lVar10;
    }
    uVar6 = uVar3;
    puVar7 = puVar7 + 1;
    uVar8 = uVar9 + bVar1;
  }
  FUN_1428d9518(uVar5,*(ulonglong *)(param_2 + 0x28),&UNK_142c2ec30);
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

