
/* WARNING: Possible PIC construction at 0x000141469366: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x00014146936b) */
/* WARNING: Removing unreachable block (ram,0x000141469373) */
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_141469230(longlong *param_1,longlong *param_2)

{
  float fVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  code *pcVar5;
  undefined8 uVar6;
  longlong lVar7;
  float *pfVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  undefined8 *puVar12;
  undefined8 *unaff_RBP;
  longlong *unaff_RSI;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 auStack_70 [6];
  longlong *plStack_40;
  undefined8 uStack_38;
  
  puVar12 = &uStack_38;
  uStack_38 = 0xfffffffffffffffe;
  uVar3 = param_2[2];
  if (uVar3 < 2) {
    *(undefined1 *)(param_1 + 1) = 0;
    *(char *)((longlong)param_1 + 9) = (char)uVar3;
    *param_1 = -0x8000000000000000;
    if (*param_2 == 0) {
      return;
    }
    lVar7 = param_2[1];
    lVar10 = *param_2 * 0xc;
    puVar12 = unaff_RBP;
    param_1 = unaff_RSI;
  }
  else {
    lVar10 = uVar3 * 4;
    auStack_70[0] = 0x1414692ad;
    lVar7 = func_0x000140613c10(lVar10);
    if (lVar7 == 0) {
      auStack_70[0] = 0x1414693da;
      plStack_40 = param_2;
      FUN_1428d8fe3(4,lVar10,&UNK_142c637c0);
                    /* WARNING: Does not return */
      pcVar5 = (code *)invalidInstructionException();
      (*pcVar5)();
    }
    puVar4 = (undefined8 *)param_2[1];
    pfVar8 = (float *)(puVar4 + 1);
    fVar13 = 0.0;
    uVar11 = 0;
    fVar14 = *(float *)(puVar4 + 1);
    uVar6 = *puVar4;
    do {
      fVar1 = *pfVar8;
      fVar14 = fVar14 - fVar1;
      uVar2 = *(undefined8 *)(pfVar8 + -2);
      fVar15 = (float)uVar6 - (float)uVar2;
      fVar16 = (float)((ulonglong)uVar6 >> 0x20) - (float)((ulonglong)uVar2 >> 0x20);
      fVar13 = fVar13 + SQRT(fVar14 * fVar14 + fVar16 * fVar16 + fVar15 * fVar15);
      *(float *)(lVar7 + uVar11 * 4) = fVar13;
      uVar11 = uVar11 + 1;
      pfVar8 = pfVar8 + 3;
      fVar14 = fVar1;
      uVar6 = uVar2;
    } while (uVar3 != uVar11);
    if ((fVar13 != 0.0) || (NAN(fVar13))) {
      lVar9 = 0;
      do {
        if (lVar10 - lVar9 == 0) {
          param_1[2] = param_2[2];
          lVar10 = param_2[1];
          *param_1 = *param_2;
          param_1[1] = lVar10;
          param_1[3] = uVar3;
          param_1[4] = lVar7;
          param_1[5] = uVar3;
          *(float *)(param_1 + 6) = fVar13;
          return;
        }
        fVar14 = *(float *)(lVar7 + lVar9) / fVar13;
        *(float *)(lVar7 + lVar9) = fVar14;
        lVar9 = lVar9 + 4;
      } while ((uint)ABS(fVar14) < 0x7f800000);
    }
    *(undefined1 *)(param_1 + 1) = 1;
    *param_1 = -0x8000000000000000;
    register0x00000020 = (BADSPACEBASE *)auStack_70;
    auStack_70[0] = 0x14146936b;
  }
  *(undefined8 **)((longlong)register0x00000020 + -8) = puVar12;
  *(longlong **)((longlong)register0x00000020 + -0x10) = param_1;
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar6 = GetProcessHeap(lVar7,lVar10);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar6,0,lVar7);
  return;
}

