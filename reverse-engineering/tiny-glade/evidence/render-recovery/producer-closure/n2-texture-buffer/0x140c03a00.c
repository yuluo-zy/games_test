
void FUN_140c03a00(longlong *param_1,int *param_2)

{
  ulonglong uVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  ulonglong *puVar7;
  code *pcVar8;
  ulonglong uVar9;
  longlong *plVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  ulonglong *puVar15;
  ulonglong uVar16;
  bool bVar17;
  undefined8 in_stack_ffffffffffffff38;
  undefined4 uVar18;
  undefined *puStack_90;
  undefined8 uStack_88;
  int **ppiStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  int *piStack_60;
  code *pcStack_58;
  ulonglong *puStack_50;
  undefined8 uStack_48;
  
  uVar18 = (undefined4)((ulonglong)in_stack_ffffffffffffff38 >> 0x20);
  uStack_48 = 0xfffffffffffffffe;
  puVar15 = *(ulonglong **)(*param_1 + 8);
  if (*(int *)((longlong)puVar15 + 0xc) == *(int *)(*param_1 + 0x10)) {
    if (*param_2 == 1) {
      uVar1 = *(ulonglong *)(param_2 + 2);
      uVar16 = *puVar15;
      do {
        uVar12 = uVar1;
        if (uVar1 < uVar16) {
          uVar12 = uVar16;
        }
        LOCK();
        uVar9 = *puVar15;
        bVar17 = uVar16 == uVar9;
        if (bVar17) {
          *puVar15 = uVar12;
          uVar9 = uVar16;
        }
        UNLOCK();
        uVar16 = uVar9;
      } while (!bVar17);
      goto code_r0x000140c03a61;
    }
code_r0x000140c03cb0:
    pcStack_58 = FUN_140bed620;
    puStack_90 = &UNK_142b14bb0;
    uStack_88 = 1;
    uStack_70 = 0;
    ppiStack_80 = &piStack_60;
    uStack_78 = 1;
    piStack_60 = param_2;
    FUN_1428d9390(&puStack_90,&UNK_142b14bf0);
code_r0x000140c03cfa:
    FUN_1428d9330(&UNK_142b185f2,0x22,&UNK_142b18670);
  }
  else {
code_r0x000140c03a61:
    plVar2 = (longlong *)param_1[1];
    puVar15 = (ulonglong *)plVar2[2];
    if (*(int *)((longlong)puVar15 + 0xc) == (int)plVar2[3]) {
      if (*param_2 != 1) goto code_r0x000140c03cb0;
      uVar1 = *(ulonglong *)(param_2 + 2);
      uVar16 = *puVar15;
      do {
        uVar12 = uVar1;
        if (uVar1 < uVar16) {
          uVar12 = uVar16;
        }
        LOCK();
        uVar9 = *puVar15;
        bVar17 = uVar16 == uVar9;
        if (bVar17) {
          *puVar15 = uVar12;
          uVar9 = uVar16;
        }
        UNLOCK();
        uVar16 = uVar9;
      } while (!bVar17);
    }
    lVar3 = *plVar2;
    lVar4 = param_1[2];
    func_0x000140d4a3b0(&piStack_60);
    uVar1 = param_1[4];
    if (0x555555555555555 < uVar1) {
      FUN_1428d79d0();
      goto code_r0x000140c03d20;
    }
    puVar15 = (ulonglong *)param_1[3];
    plVar2 = (longlong *)param_1[5];
    uVar16 = puStack_50[4] & 0xfffffffffffffff8;
    if (uVar1 * 0x18 <= uVar16 - *puStack_50 && *puStack_50 <= uVar16) {
      uVar16 = uVar16 + uVar1 * -0x18;
      puStack_50[4] = uVar16;
      if (uVar16 != 0) goto code_r0x000140c03b22;
    }
    uVar16 = FUN_1428d7780(&piStack_60,8);
    if (uVar16 != 0) {
code_r0x000140c03b22:
      if (uVar1 != 0) {
        plVar10 = (longlong *)(uVar16 + 0x10);
        lVar11 = uVar1 << 4;
        lVar13 = 0;
        uVar12 = uVar1;
        do {
          if (lVar11 == 0) goto code_r0x000140c03cfa;
          uVar9 = *puVar15;
          lVar14 = 0;
          if (uVar9 <= puVar15[1]) {
            lVar14 = puVar15[1] - uVar9;
          }
          puVar15 = puVar15 + 2;
          lVar5 = *plVar2;
          plVar10[-2] = lVar13;
          lVar13 = lVar13 + lVar14;
          plVar10[-1] = uVar9 + lVar5;
          *plVar10 = lVar14;
          plVar10 = plVar10 + 3;
          lVar11 = lVar11 + -0x10;
          uVar12 = uVar12 - 1;
        } while (uVar12 != 0);
      }
      uVar6 = *(undefined8 *)(param_2 + 0x12);
      puStack_90._0_4_ = 0x2e;
      uStack_88 = 0;
      ppiStack_80 = (int **)0x1800000018000;
      (**(code **)(lVar4 + 0x3e0))(uVar6,0x10000,0x10000,0,CONCAT44(uVar18,1),&puStack_90,0,8,0,8);
      (**(code **)(lVar4 + 0x370))
                (uVar6,*(undefined8 *)(lVar3 + 0x20),*(undefined8 *)(param_1[6] + 0x20),
                 uVar1 & 0xffffffff,uVar16);
      puStack_90 = (undefined *)CONCAT44(puStack_90._4_4_,0x2e);
      uStack_88 = 0;
      ppiStack_80 = (int **)0x1800000018000;
      (**(code **)(lVar4 + 0x3e0))(uVar6,0x10000,0x10000,0,1,&puStack_90,0,8,0,8);
      puVar15 = puRam000000014362c3e0;
      while (puStack_50 != puVar15) {
        puVar7 = (ulonglong *)puStack_50[3];
        func_0x000140613c20(*puStack_50,puStack_50[2],puStack_50[1]);
        puStack_50 = puVar7;
      }
      return;
    }
    FUN_1428d79d0();
  }
code_r0x000140c03d20:
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

