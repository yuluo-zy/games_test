
void FUN_1412e4250(longlong *param_1,longlong param_2)

{
  ulonglong *puVar1;
  byte bVar2;
  code *pcVar3;
  uint uVar4;
  bool bVar5;
  int iVar6;
  ulonglong uVar7;
  int iVar8;
  ulonglong *puVar9;
  uint uVar10;
  uint uVar11;
  int unaff_R14D;
  ulonglong unaff_R15;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  uint uStack_4c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puVar9 = *(ulonglong **)(param_2 + 0x30);
  puVar1 = puVar9 + *(longlong *)(param_2 + 0x38);
  lStack_70 = 0;
  lStack_68 = 4;
  lStack_60 = 0;
  lStack_58 = 0;
  iVar6 = 0;
  bVar5 = false;
  uStack_4c = 0;
  uVar10 = 0;
  while( true ) {
    if (!bVar5) goto code_r0x0001412e4360;
    while (unaff_R15 == 0) {
code_r0x0001412e4360:
      if ((puVar9 == (ulonglong *)0x0) || (puVar9 == puVar1)) {
        iVar6 = uVar10 - uStack_4c;
        if (uStack_4c <= uVar10 && iVar6 != 0) {
          if (lStack_58 == lStack_70) {
            FUN_140562e70(&lStack_70,&UNK_142c2ec18);
          }
          *(uint *)(lStack_68 + lStack_58 * 8) = uStack_4c;
          *(int *)(lStack_68 + 4 + lStack_58 * 8) = iVar6;
          lStack_60 = lStack_58 + 1;
        }
        param_1[2] = lStack_60;
        *param_1 = lStack_70;
        param_1[1] = lStack_68;
        return;
      }
      unaff_R14D = iVar6 << 6;
      unaff_R15 = *puVar9;
      puVar9 = puVar9 + 1;
      iVar6 = iVar6 + 1;
    }
    iVar8 = 0;
    for (uVar7 = unaff_R15; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x8000000000000000) {
      iVar8 = iVar8 + 1;
    }
    uVar7 = (ulonglong)(uint)(iVar8 + unaff_R14D);
    if (*(ulonglong *)(param_2 + 0x28) <= uVar7) break;
    uVar11 = (iVar8 + unaff_R14D) * 0x20;
    bVar2 = *(byte *)(*(longlong *)(param_2 + 0x20) + uVar7);
    uVar4 = uStack_4c;
    if ((uVar10 != uVar11) &&
       (iVar8 = uVar10 - uStack_4c, uVar4 = uVar11, uStack_4c <= uVar10 && iVar8 != 0)) {
      if (lStack_58 == lStack_70) {
        FUN_140562e70(&lStack_70,&UNK_142c2ec48);
      }
      *(uint *)(lStack_68 + lStack_58 * 8) = uStack_4c;
      *(int *)(lStack_68 + 4 + lStack_58 * 8) = iVar8;
      lStack_60 = lStack_58 + 1;
      lStack_58 = lStack_60;
    }
    uStack_4c = uVar4;
    unaff_R15 = unaff_R15 & unaff_R15 - 1;
    uVar10 = uVar11 + bVar2;
    bVar5 = true;
  }
  FUN_1428d9518(uVar7,*(ulonglong *)(param_2 + 0x28),&UNK_142c2ec30);
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

