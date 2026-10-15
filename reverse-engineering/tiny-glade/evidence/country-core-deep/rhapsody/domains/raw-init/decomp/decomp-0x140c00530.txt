
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_140c00530(longlong param_1)

{
  uint uVar1;
  code *pcVar2;
  uint *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  byte *pbVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  
  uVar1 = *(uint *)(param_1 + 0x504);
  uVar5 = (ulonglong)uVar1;
  if (0x10 < uVar5) {
    FUN_1428d9840(uVar5,0x10,&UNK_142b17578);
    pcVar2 = (code *)swi(3);
    lVar4 = (*pcVar2)();
    return lVar4;
  }
  if (uVar1 == 0) {
    lVar4 = 0;
  }
  else {
    if (uVar1 < 5) {
      lVar4 = 0;
      lVar8 = 0;
    }
    else {
      uVar9 = 4;
      if ((uVar1 & 3) != 0) {
        uVar9 = (ulonglong)(uVar1 & 3);
      }
      lVar8 = uVar5 - uVar9;
      puVar3 = (uint *)(param_1 + 0x540);
      lVar11 = 0;
      lVar12 = 0;
      lVar6 = 0;
      lVar10 = 0;
      lVar4 = lVar8;
      do {
        auVar13._8_8_ = *(undefined8 *)(puVar3 + -10);
        auVar13._0_8_ = *(undefined8 *)(puVar3 + -0xe);
        auVar15._8_8_ = *(undefined8 *)(puVar3 + -2);
        auVar15._0_8_ = *(undefined8 *)(puVar3 + -6);
        auVar16._0_4_ = -(uint)((puVar3[-0xc] & _UNK_142958fe0) == 0);
        auVar16._4_4_ = -(uint)((puVar3[-0xc] & _UNK_142958fe4) == 0);
        auVar16._8_4_ = -(uint)((puVar3[-8] & _UNK_142958fe8) == 0);
        auVar16._12_4_ = -(uint)((puVar3[-8] & _UNK_142958fec) == 0);
        lVar11 = lVar11 + SUB168(~auVar16 & auVar13,0);
        lVar12 = lVar12 + SUB168(~auVar16 & auVar13,8);
        auVar14._0_4_ = -(uint)((puVar3[-4] & _UNK_142958fe0) == 0);
        auVar14._4_4_ = -(uint)((puVar3[-4] & _UNK_142958fe4) == 0);
        auVar14._8_4_ = -(uint)((*puVar3 & _UNK_142958fe8) == 0);
        auVar14._12_4_ = -(uint)((*puVar3 & _UNK_142958fec) == 0);
        lVar6 = lVar6 + SUB168(~auVar14 & auVar15,0);
        lVar10 = lVar10 + SUB168(~auVar14 & auVar15,8);
        puVar3 = puVar3 + 0x10;
        lVar4 = lVar4 + -4;
      } while (lVar4 != 0);
      lVar4 = lVar10 + lVar12 + lVar6 + lVar11;
    }
    lVar6 = uVar5 - lVar8;
    pbVar7 = (byte *)(param_1 + lVar8 * 0x10 + 0x510);
    do {
      while ((*pbVar7 & 1) == 0) {
        pbVar7 = pbVar7 + 0x10;
        lVar6 = lVar6 + -1;
        if (lVar6 == 0) {
          return lVar4;
        }
      }
      lVar4 = lVar4 + *(longlong *)(pbVar7 + -8);
      pbVar7 = pbVar7 + 0x10;
      lVar6 = lVar6 + -1;
    } while (lVar6 != 0);
  }
  return lVar4;
}

