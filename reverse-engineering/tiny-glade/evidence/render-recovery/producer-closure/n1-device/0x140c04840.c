
undefined8 * FUN_140c04840(undefined8 *param_1,longlong param_2,longlong *param_3)

{
  longlong *plVar1;
  char cVar2;
  longlong *plVar3;
  longlong lVar4;
  longlong *plVar5;
  longlong *plVar6;
  ulonglong *puVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  code *pcVar11;
  code *pcVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  code *pcVar15;
  ulonglong uVar16;
  char *pcVar17;
  bool bVar18;
  undefined8 in_stack_fffffffffffffed8;
  undefined4 uVar19;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  undefined8 *puStack_c0;
  code *pcStack_b8;
  longlong *plStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  char *pcStack_50;
  undefined1 uStack_41;
  undefined8 uStack_40;
  
  uVar19 = (undefined4)((ulonglong)in_stack_fffffffffffffed8 >> 0x20);
  uStack_40 = 0xfffffffffffffffe;
  pcVar17 = (char *)(param_2 + 0x9f0);
  LOCK();
  bVar18 = *(char *)(param_2 + 0x9f0) == '\0';
  if (bVar18) {
    *(char *)(param_2 + 0x9f0) = '\x01';
  }
  UNLOCK();
  if (!bVar18) {
    FUN_1428cd210(pcVar17,param_2,1000000000);
  }
  FUN_140c81c70(pcVar17);
  plVar1 = (longlong *)(param_2 + 0x9f8);
  if (*(longlong *)(param_2 + 0x9f8) == 4) {
    pcStack_50 = pcVar17;
    FUN_140bef4a0(&uStack_120,param_2,&UNK_142b18788,8,
                  CONCAT44(uVar19,*(undefined4 *)(param_2 + 0xad4)));
    uStack_41 = 1;
    FUN_140bed230(&uStack_120,param_2,0);
    plVar3 = (longlong *)*param_3;
    lVar4 = param_3[1];
    plVar5 = (longlong *)param_3[2];
    plVar6 = (longlong *)param_3[3];
    puVar7 = (ulonglong *)plVar3[2];
    if (*(int *)((longlong)puVar7 + 0xc) == (int)plVar3[3]) {
      if ((int)uStack_120 != 1) {
        pcStack_b8 = FUN_140bed620;
        uStack_a8._0_4_ = 0x42b14bb0;
        uStack_a8._4_4_ = 1;
        uStack_a0._0_4_ = 1;
        uStack_a0._4_4_ = 0;
        uStack_88 = 0;
        uStack_98 = &puStack_c0;
        uStack_90 = 1;
        uStack_41 = 1;
        puStack_c0 = &uStack_120;
        FUN_1428d9390(&uStack_a8,&UNK_142b14bf0);
        goto code_r0x000140c04cdb;
      }
      pcVar11 = (code *)*puVar7;
      do {
        pcVar15 = uStack_118;
        if (uStack_118 < pcVar11) {
          pcVar15 = pcVar11;
        }
        LOCK();
        pcVar12 = (code *)*puVar7;
        bVar18 = pcVar11 == pcVar12;
        if (bVar18) {
          *puVar7 = (ulonglong)pcVar15;
          pcVar12 = pcVar11;
        }
        UNLOCK();
        pcVar11 = pcVar12;
      } while (!bVar18);
    }
    uVar8 = *(undefined8 *)(*plVar3 + 0x20);
    puVar7 = (ulonglong *)plVar5[2];
    if (*(int *)((longlong)puVar7 + 0xc) == (int)plVar5[3]) {
      if ((int)uStack_120 != 1) {
        pcStack_b8 = FUN_140bed620;
        uStack_a8._0_4_ = 0x42b14bb0;
        uStack_a8._4_4_ = 1;
        uStack_a0._0_4_ = 1;
        uStack_a0._4_4_ = 0;
        uStack_88 = 0;
        uStack_98 = &puStack_c0;
        uStack_90 = 1;
        uStack_41 = 1;
        puStack_c0 = &uStack_120;
        FUN_1428d9390(&uStack_a8,&UNK_142b14bf0);
        goto code_r0x000140c04cdb;
      }
      pcVar11 = (code *)*puVar7;
      do {
        pcVar15 = uStack_118;
        if (uStack_118 < pcVar11) {
          pcVar15 = pcVar11;
        }
        LOCK();
        pcVar12 = (code *)*puVar7;
        bVar18 = pcVar11 == pcVar12;
        if (bVar18) {
          *puVar7 = (ulonglong)pcVar15;
          pcVar12 = pcVar11;
        }
        UNLOCK();
        pcVar11 = pcVar12;
      } while (!bVar18);
    }
    uStack_98 = (undefined8 **)0x0;
    if (*plVar6 != 0) {
      uStack_98 = (undefined8 **)plVar6[1];
    }
    uStack_a8 = 0;
    uStack_a0 = 0;
    (**(code **)(lVar4 + 0x370))(lStack_d8,uVar8,*(undefined8 *)(*plVar5 + 0x20),1,&uStack_a8);
    lStack_58 = lStack_d0;
    lStack_68 = lStack_e0;
    lStack_60 = lStack_d8;
    uStack_78 = uStack_f0;
    uStack_74 = uStack_ec;
    uStack_70 = uStack_e8;
    uStack_6c = uStack_e4;
    uStack_88 = CONCAT44(uStack_fc,uStack_100);
    uStack_80 = uStack_f8;
    uStack_7c = uStack_f4;
    uStack_98 = (undefined8 **)CONCAT44(uStack_10c,uStack_110);
    uStack_90 = CONCAT44(uStack_104,uStack_108);
    uStack_a8._0_4_ = (int)uStack_120;
    uStack_a8._4_4_ = uStack_120._4_4_;
    uStack_a0._0_4_ = (int)uStack_118;
    uStack_a0._4_4_ = uStack_118._4_4_;
    lVar4 = *plVar1;
    plStack_b0 = plVar1;
    if (lVar4 != 4) {
      if ((lVar4 != 0) && ((int)lVar4 != 3)) {
        lStack_c8 = param_2;
        FUN_1428d9430(&UNK_142b12b58,0x6e,&UNK_142b12bf0);
code_r0x000140c04cdb:
                    /* WARNING: Does not return */
        pcVar11 = (code *)invalidInstructionException();
        (*pcVar11)();
      }
      if (*(longlong *)(param_2 + 0xa10) != 0) {
        func_0x000140613c20(*(undefined8 *)(param_2 + 0xa18),*(longlong *)(param_2 + 0xa10) << 3,8);
      }
      FUN_1405df600(param_2 + 0xa28);
    }
    plStack_b0[10] = lStack_58;
    plStack_b0[8] = lStack_68;
    plStack_b0[9] = lStack_60;
    plStack_b0[6] = CONCAT44(uStack_74,uStack_78);
    plStack_b0[7] = CONCAT44(uStack_6c,uStack_70);
    plStack_b0[4] = uStack_88;
    plStack_b0[5] = CONCAT44(uStack_7c,uStack_80);
    plStack_b0[2] = (longlong)uStack_98;
    plStack_b0[3] = uStack_90;
    *(int *)plStack_b0 = (int)uStack_a8;
    *(int *)((longlong)plStack_b0 + 4) = uStack_a8._4_4_;
    *(int *)(plStack_b0 + 1) = (int)uStack_a0;
    *(int *)((longlong)plStack_b0 + 0xc) = uStack_a0._4_4_;
    pcVar17 = pcStack_50;
  }
  else {
    plVar3 = (longlong *)*param_3;
    lVar4 = param_3[1];
    plVar5 = (longlong *)param_3[2];
    plVar6 = (longlong *)param_3[3];
    puVar7 = (ulonglong *)plVar3[2];
    uStack_120 = plVar1;
    if (*(int *)((longlong)puVar7 + 0xc) == (int)plVar3[3]) {
      if ((int)*(longlong *)(param_2 + 0x9f8) != 1) {
        uStack_118 = FUN_140bed620;
        uStack_a8._0_4_ = 0x42b14bb0;
        uStack_a8._4_4_ = 1;
        uStack_a0._0_4_ = 1;
        uStack_a0._4_4_ = 0;
        uStack_88 = 0;
        uStack_98 = (undefined8 **)&uStack_120;
        uStack_90 = 1;
        pcStack_50 = pcVar17;
        FUN_1428d9390(&uStack_a8,&UNK_142b14bf0);
        goto code_r0x000140c04cdb;
      }
      uVar9 = *(ulonglong *)(param_2 + 0xa00);
      uVar13 = *puVar7;
      do {
        uVar16 = uVar9;
        if (uVar9 < uVar13) {
          uVar16 = uVar13;
        }
        LOCK();
        uVar14 = *puVar7;
        bVar18 = uVar13 == uVar14;
        if (bVar18) {
          *puVar7 = uVar16;
          uVar14 = uVar13;
        }
        UNLOCK();
        uVar13 = uVar14;
      } while (!bVar18);
    }
    uVar8 = *(undefined8 *)(param_2 + 0xa40);
    uVar10 = *(undefined8 *)(*plVar3 + 0x20);
    puVar7 = (ulonglong *)plVar5[2];
    if (*(int *)((longlong)puVar7 + 0xc) == (int)plVar5[3]) {
      if (*(int *)plVar1 != 1) {
        uStack_118 = FUN_140bed620;
        uStack_a8._0_4_ = 0x42b14bb0;
        uStack_a8._4_4_ = 1;
        uStack_a0._0_4_ = 1;
        uStack_a0._4_4_ = 0;
        uStack_88 = 0;
        uStack_98 = (undefined8 **)&uStack_120;
        uStack_90 = 1;
        pcStack_50 = pcVar17;
        FUN_1428d9390(&uStack_a8,&UNK_142b14bf0);
        goto code_r0x000140c04cdb;
      }
      uVar9 = *(ulonglong *)(param_2 + 0xa00);
      uVar13 = *puVar7;
      do {
        uVar16 = uVar9;
        if (uVar9 < uVar13) {
          uVar16 = uVar13;
        }
        LOCK();
        uVar14 = *puVar7;
        bVar18 = uVar13 == uVar14;
        if (bVar18) {
          *puVar7 = uVar16;
          uVar14 = uVar13;
        }
        UNLOCK();
        uVar13 = uVar14;
      } while (!bVar18);
    }
    uStack_98 = (undefined8 **)0x0;
    if (*plVar6 != 0) {
      uStack_98 = (undefined8 **)plVar6[1];
    }
    uStack_a8._0_4_ = 0;
    uStack_a8._4_4_ = 0;
    uStack_a0._0_4_ = 0;
    uStack_a0._4_4_ = 0;
    (**(code **)(lVar4 + 0x370))(uVar8,uVar10,*(undefined8 *)(*plVar5 + 0x20),1,&uStack_a8);
  }
  *param_1 = 0xb;
  FUN_140c81e10();
  LOCK();
  cVar2 = *pcVar17;
  if (cVar2 == '\x01') {
    *pcVar17 = '\0';
  }
  UNLOCK();
  if (cVar2 != '\x01') {
    FUN_1428cdb90(pcVar17,0);
  }
  return param_1;
}

