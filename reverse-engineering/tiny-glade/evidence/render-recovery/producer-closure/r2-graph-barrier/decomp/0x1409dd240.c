
void FUN_1409dd240(longlong *param_1,undefined4 *param_2)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  code *pcVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  longlong lVar24;
  undefined4 *puVar25;
  longlong lVar26;
  
  lVar2 = *param_1;
  if (SBORROW8(0,lVar2)) {
    FUN_1428d9310(&UNK_142ac3a78);
  }
  else {
    puVar25 = (undefined4 *)func_0x000140613c10(0x30,8);
    if (puVar25 == (undefined4 *)0x0) {
      FUN_1428d9000(8,0x30);
    }
    else {
      uVar10 = *param_2;
      uVar11 = param_2[1];
      uVar12 = param_2[2];
      uVar13 = param_2[3];
      uVar14 = param_2[4];
      uVar15 = param_2[5];
      uVar16 = param_2[6];
      uVar17 = param_2[7];
      uVar18 = param_2[9];
      uVar19 = param_2[10];
      uVar20 = param_2[0xb];
      puVar25[8] = param_2[8];
      puVar25[9] = uVar18;
      puVar25[10] = uVar19;
      puVar25[0xb] = uVar20;
      puVar25[4] = uVar14;
      puVar25[5] = uVar15;
      puVar25[6] = uVar16;
      puVar25[7] = uVar17;
      *puVar25 = uVar10;
      puVar25[1] = uVar11;
      puVar25[2] = uVar12;
      puVar25[3] = uVar13;
      lVar3 = param_1[5];
      param_1[5] = (longlong)puVar25;
      param_1[6] = (longlong)&UNK_142ac3b08;
      if (lVar3 == 0) {
        lVar3 = param_1[8];
        *param_1 = -0x8000000000000000;
        lVar4 = param_1[7];
        lVar21 = param_1[1];
        lVar22 = param_1[2];
        lVar23 = param_1[3];
        lVar24 = param_1[4];
        lVar5 = param_1[5];
        lVar6 = param_1[6];
        lVar7 = *(longlong *)(lVar3 + 0xa0);
        if (lVar7 == *(longlong *)(lVar3 + 0x90)) {
          FUN_1405627b0(lVar3 + 0x90,&UNK_142abc8c0);
        }
        lVar8 = *(longlong *)(lVar3 + 0x98);
        lVar26 = lVar7 * 0x40;
        plVar1 = (longlong *)(lVar8 + 0x30 + lVar26);
        *plVar1 = lVar6;
        plVar1[1] = lVar4;
        plVar1 = (longlong *)(lVar8 + 0x20 + lVar26);
        *plVar1 = lVar24;
        plVar1[1] = lVar5;
        plVar1 = (longlong *)(lVar8 + 0x10 + lVar26);
        *plVar1 = lVar22;
        plVar1[1] = lVar23;
        *(longlong *)(lVar8 + lVar26) = lVar2;
        ((longlong *)(lVar8 + lVar26))[1] = lVar21;
        *(longlong *)(lVar3 + 0xa0) = lVar7 + 1;
        return;
      }
      FUN_1428d9430(&UNK_142ac3ab0,0x20,&UNK_142ac3ad0);
    }
  }
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}

