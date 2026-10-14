
longlong * FUN_140f77ad0(longlong *param_1,longlong *param_2,longlong *param_3)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar5;
  code *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  undefined4 *puVar15;
  longlong *plVar16;
  ulonglong uVar17;
  longlong *plVar18;
  longlong *plVar19;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_98;
  undefined4 uStack_94;
  longlong *plVar4;
  
  lVar1 = *param_3;
  plVar2 = (longlong *)param_3[1];
  plVar19 = plVar2 + param_3[2] * 6;
  plVar18 = plVar2;
  plVar4 = plVar2;
  if (param_3[2] != 0) {
    do {
      plVar16 = plVar4 + 6;
      lVar3 = *plVar4;
      plVar18 = plVar16;
      if (SBORROW8(0,lVar3)) break;
      lVar5 = plVar4[5];
      lVar11 = plVar4[1];
      uVar7 = *(undefined4 *)((longlong)plVar4 + 0xc);
      lVar12 = plVar4[2];
      uVar8 = *(undefined4 *)((longlong)plVar4 + 0x14);
      lVar13 = plVar4[3];
      uVar9 = *(undefined4 *)((longlong)plVar4 + 0x1c);
      lVar14 = plVar4[4];
      uVar10 = *(undefined4 *)((longlong)plVar4 + 0x24);
      puVar15 = (undefined4 *)func_0x000140613c10(0x30,8);
      if (puVar15 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x30);
                    /* WARNING: Does not return */
        pcVar6 = (code *)invalidInstructionException();
        (*pcVar6)();
      }
      uStack_c0 = (undefined4)lVar3;
      uStack_bc = (undefined4)((ulonglong)lVar3 >> 0x20);
      uStack_98 = (undefined4)lVar5;
      uStack_94 = (undefined4)((ulonglong)lVar5 >> 0x20);
      puVar15[8] = (int)lVar14;
      puVar15[9] = uVar10;
      puVar15[10] = uStack_98;
      puVar15[0xb] = uStack_94;
      puVar15[4] = (int)lVar12;
      puVar15[5] = uVar8;
      puVar15[6] = (int)lVar13;
      puVar15[7] = uVar9;
      *puVar15 = uStack_c0;
      puVar15[1] = uStack_bc;
      puVar15[2] = (int)lVar11;
      puVar15[3] = uVar7;
      lVar3 = param_2[2];
      if (lVar3 == *param_2) {
        FUN_140562630(param_2,&UNK_142bab5c8);
      }
      lVar5 = param_2[1];
      *(undefined4 **)(lVar5 + lVar3 * 0x10) = puVar15;
      *(undefined **)(lVar5 + 8 + lVar3 * 0x10) = &UNK_142bab5a8;
      param_2[2] = lVar3 + 1;
      plVar18 = plVar19;
      plVar4 = plVar16;
    } while (plVar16 != plVar19);
  }
  if (plVar19 != plVar18) {
    uVar17 = (ulonglong)((longlong)plVar19 - (longlong)plVar18) / 0x30;
    plVar18 = plVar18 + 4;
    do {
      if (plVar18[-4] != 0) {
        func_0x000140613c20(plVar18[-3],plVar18[-4],1);
      }
      if (plVar18[-1] != 0) {
        func_0x000140613c20(*plVar18,plVar18[-1],1);
      }
      plVar18 = plVar18 + 6;
      uVar17 = uVar17 - 1;
    } while (uVar17 != 0);
  }
  if (lVar1 != 0) {
    func_0x000140613c20(plVar2,lVar1 * 0x30,8);
  }
  param_1[2] = param_2[2];
  lVar1 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = lVar1;
  return param_1;
}

