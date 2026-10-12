
void FUN_140bd4860(longlong param_1)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  ulonglong uVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  longlong lVar8;
  uint *puVar9;
  longlong lVar10;
  uint uVar11;
  longlong lVar12;
  undefined1 uStack_49;
  longlong lStack_48;
  
  lVar8 = *(longlong *)(param_1 + 0x40);
  if (lVar8 != 0) {
    lVar10 = *(longlong *)(param_1 + 0x38);
    lStack_48 = param_1 + 0x78;
    lVar12 = 0;
    uVar7 = 0;
    do {
      if ((*(int *)(lVar10 + lVar12) == 1) && (*(int *)(lVar10 + 8 + lVar12) == 1)) {
        iVar1 = *(int *)(lVar10 + 4 + lVar12);
        iVar2 = *(int *)(lVar10 + 0xc + lVar12);
        lVar3 = *(longlong *)(param_1 + 0x88);
        if (lVar3 == *(longlong *)(param_1 + 0x78)) {
          FUN_140b5f870(lStack_48,&UNK_142b0f6c0);
        }
        *(uint *)(*(longlong *)(param_1 + 0x80) + lVar3 * 4) = iVar1 << 0x11 | uVar7;
        *(longlong *)(param_1 + 0x88) = lVar3 + 1;
        if (lVar3 + 1 == *(longlong *)(param_1 + 0x78)) {
          FUN_140b5f870(lStack_48,&UNK_142b0f6d8);
        }
        *(uint *)(*(longlong *)(param_1 + 0x80) + 4 + lVar3 * 4) = iVar2 << 0x11 | uVar7 | 0x10000;
        *(longlong *)(param_1 + 0x88) = lVar3 + 2;
      }
      lVar12 = lVar12 + 0x18;
      uVar7 = uVar7 + 1;
    } while (lVar8 * 0x18 != lVar12);
  }
  uVar4 = *(ulonglong *)(param_1 + 0x88);
  if (1 < uVar4) {
    puVar5 = *(uint **)(param_1 + 0x80);
    if (uVar4 < 0x15) {
      puVar6 = puVar5 + 1;
      lVar8 = 4;
      do {
        uVar11 = puVar6[-1];
        uVar7 = *puVar6;
        lVar10 = lVar8;
        if (uVar7 < uVar11) {
          do {
            *(uint *)((longlong)puVar5 + lVar10) = uVar11;
            puVar9 = puVar5;
            if (lVar10 == 4) goto code_r0x000140bd497f;
            uVar11 = *(uint *)((longlong)puVar5 + lVar10 + -8);
            lVar10 = lVar10 + -4;
          } while (uVar7 < uVar11);
          puVar9 = (uint *)(lVar10 + (longlong)puVar5);
code_r0x000140bd497f:
          *puVar9 = uVar7;
        }
        puVar6 = puVar6 + 1;
        lVar8 = lVar8 + 4;
      } while (puVar6 != puVar5 + uVar4);
    }
    else {
      FUN_140958a90(puVar5,uVar4,&uStack_49);
    }
  }
  return;
}

