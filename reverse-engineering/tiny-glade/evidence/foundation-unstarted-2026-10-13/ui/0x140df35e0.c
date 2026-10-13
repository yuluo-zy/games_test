
longlong * FUN_140df35e0(longlong *param_1,longlong *param_2,longlong *param_3,undefined8 param_4)

{
  ulonglong *puVar1;
  ulonglong *puVar2;
  ulonglong *puVar3;
  longlong lVar4;
  code *pcVar5;
  ulonglong *puVar6;
  undefined4 *puVar7;
  ulonglong *puVar8;
  int iVar9;
  longlong lVar10;
  undefined4 *puVar11;
  longlong lVar12;
  ulonglong uVar13;
  ulonglong unaff_R12;
  undefined8 uVar14;
  longlong *plVar15;
  ulonglong *puVar16;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  ulonglong *puStack_e8;
  ulonglong *puStack_e0;
  longlong lStack_d8;
  ulonglong *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  longlong *plStack_98;
  ulonglong uStack_90;
  longlong lStack_78;
  longlong *plStack_70;
  longlong lStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_78 = 0;
  plStack_70 = (longlong *)0x8;
  lStack_68 = 0;
  lStack_d8 = *param_2;
  puStack_e8 = (ulonglong *)param_2[1];
  puStack_d0 = puStack_e8 + param_2[2] * 6;
  uStack_f0 = param_4;
  puStack_e0 = puStack_e8;
  plStack_98 = param_3;
  while (puStack_e0 != puStack_d0) {
    puVar16 = puStack_e0 + 6;
    uVar13 = *puStack_e0;
    puVar1 = puStack_e0 + 5;
    puVar2 = puStack_e0 + 3;
    puVar8 = puStack_e0 + 4;
    puVar3 = puStack_e0 + 1;
    puVar11 = (undefined4 *)((longlong)puStack_e0 + 0xc);
    puVar6 = puStack_e0 + 2;
    puVar7 = (undefined4 *)((longlong)puStack_e0 + 0x14);
    puStack_e0 = puVar16;
    if (SBORROW8(0,uVar13)) break;
    uStack_138 = (undefined4)*puVar2;
    uStack_134 = (undefined4)(*puVar2 >> 0x20);
    uStack_130 = (undefined4)*puVar8;
    uStack_12c = (undefined4)(*puVar8 >> 0x20);
    uStack_108 = uStack_138;
    uStack_104 = uStack_134;
    uStack_100 = uStack_130;
    uStack_fc = uStack_12c;
    uStack_49 = 1;
    uStack_120 = uVar13;
    uStack_118 = (int)*puVar3;
    uStack_114 = *puVar11;
    uStack_110 = (int)*puVar6;
    uStack_10c = *puVar7;
    uStack_f8 = *puVar1;
    lVar10 = FUN_140bd7bc0(&uStack_120);
    if (lVar10 == 0) {
      if (lStack_68 != 0) {
        lVar10 = lStack_68 * 0x30;
        plVar15 = plStack_70;
        do {
          if (SBORROW8(0,*plVar15)) {
            uStack_90 = 0x8000000000000000;
            uStack_58 = unaff_R12;
            goto LAB_140df38d2;
          }
          plVar15 = plVar15 + 6;
          lVar10 = lVar10 + -0x30;
        } while (lVar10 != 0);
      }
      uStack_58 = 0x8000000000000000;
LAB_140df3833:
      puVar11 = (undefined4 *)func_0x000140613c10(0x30,8);
      lVar10 = lStack_68;
      if (puVar11 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x30);
LAB_140df3a88:
                    /* WARNING: Does not return */
        pcVar5 = (code *)invalidInstructionException();
        (*pcVar5)();
      }
      puVar11[8] = uStack_100;
      puVar11[9] = uStack_fc;
      puVar11[10] = (undefined4)uStack_f8;
      puVar11[0xb] = uStack_f8._4_4_;
      puVar11[4] = uStack_110;
      puVar11[5] = uStack_10c;
      puVar11[6] = uStack_108;
      puVar11[7] = uStack_104;
      *puVar11 = (undefined4)uStack_120;
      puVar11[1] = uStack_120._4_4_;
      puVar11[2] = uStack_118;
      puVar11[3] = uStack_114;
      uStack_b0 = 1;
      uStack_a0 = 1;
      uStack_c8 = uStack_58;
      uStack_c0 = uStack_60;
      uStack_b8 = unaff_R12;
      uStack_a8 = puVar11;
      if (lStack_68 == lStack_78) {
        FUN_1405629f0(&lStack_78,&UNK_142b78e40);
      }
      plStack_70[lVar10 * 6 + 4] = (longlong)uStack_a8;
      (plStack_70 + lVar10 * 6 + 4)[1] = uStack_a0;
      plStack_70[lVar10 * 6 + 2] = uStack_b8;
      (plStack_70 + lVar10 * 6 + 2)[1] = uStack_b0;
      plVar15 = plStack_70 + lVar10 * 6;
      *(undefined4 *)plVar15 = (undefined4)uStack_c8;
      *(undefined4 *)((longlong)plVar15 + 4) = uStack_c8._4_4_;
      *(undefined4 *)(plVar15 + 1) = (undefined4)uStack_c0;
      *(undefined4 *)((longlong)plVar15 + 0xc) = uStack_c0._4_4_;
      lStack_68 = lVar10 + 1;
    }
    else {
      if ((longlong)uVar13 < 0) {
        uVar14 = 0;
LAB_140df3a6b:
        uStack_49 = 1;
        FUN_1428d8fe3(uVar14,uStack_60,&UNK_142b78d78);
        goto LAB_140df3a88;
      }
      uStack_58 = uVar13;
      if (uVar13 == 0) {
        uStack_60 = 1;
      }
      else {
        uVar14 = 1;
        uStack_60 = func_0x000140613c10(uVar13,1);
        if (uStack_60 == 0) {
          uStack_60 = uStack_58;
          goto LAB_140df3a6b;
        }
      }
      unaff_R12 = uStack_58;
      func_0x000142923c00(uStack_60,lVar10,uStack_58);
      if (lStack_68 == 0) goto LAB_140df3833;
      lVar10 = lStack_68 * 0x30;
      plVar15 = plStack_70;
      while (((SBORROW8(0,*plVar15) || (plVar15[2] != unaff_R12)) ||
             (iVar9 = func_0x000142923bf0(plVar15[1],uStack_60), unaff_R12 = uStack_58, iVar9 != 0))
            ) {
        plVar15 = plVar15 + 6;
        lVar10 = lVar10 + -0x30;
        if (lVar10 == 0) goto LAB_140df3833;
      }
      uStack_90 = uStack_58;
LAB_140df38d2:
      uStack_a8 = (undefined4 *)CONCAT44(uStack_fc,uStack_100);
      uStack_a0 = uStack_f8;
      uStack_b8 = CONCAT44(uStack_10c,uStack_110);
      uStack_b0 = CONCAT44(uStack_104,uStack_108);
      uStack_c8 = uStack_120;
      uStack_c0 = CONCAT44(uStack_114,uStack_118);
      lVar10 = plVar15[5];
      if (lVar10 == plVar15[3]) {
        FUN_1405629f0(plVar15 + 3,&UNK_142b78e28);
      }
      lVar4 = plVar15[4];
      lVar12 = lVar10 * 0x30;
      puVar11 = (undefined4 *)(lVar4 + 0x20 + lVar12);
      *puVar11 = (undefined4)uStack_a8;
      puVar11[1] = uStack_a8._4_4_;
      puVar11[2] = (undefined4)uStack_a0;
      puVar11[3] = uStack_a0._4_4_;
      puVar11 = (undefined4 *)(lVar4 + 0x10 + lVar12);
      *puVar11 = (undefined4)uStack_b8;
      puVar11[1] = uStack_b8._4_4_;
      puVar11[2] = (undefined4)uStack_b0;
      puVar11[3] = uStack_b0._4_4_;
      puVar11 = (undefined4 *)(lVar4 + lVar12);
      *puVar11 = (undefined4)uStack_c8;
      puVar11[1] = uStack_c8._4_4_;
      puVar11[2] = (undefined4)uStack_c0;
      puVar11[3] = uStack_c0._4_4_;
      plVar15[5] = lVar10 + 1;
      unaff_R12 = uStack_58;
      if ((uStack_90 & 0x7fffffffffffffff) != 0) {
        func_0x000140613c20(uStack_60,uStack_90,1);
        unaff_R12 = uStack_58;
      }
    }
  }
  if (puStack_d0 != puStack_e0) {
    uVar13 = (ulonglong)((longlong)puStack_d0 - (longlong)puStack_e0) / 0x30;
    puVar16 = puStack_e0 + 4;
    do {
      if (puVar16[-4] != 0) {
        func_0x000140613c20(puVar16[-3],puVar16[-4],1);
      }
      if (puVar16[-1] != 0) {
        func_0x000140613c20(*puVar16,puVar16[-1],1);
      }
      puVar16 = puVar16 + 6;
      uVar13 = uVar13 - 1;
    } while (uVar13 != 0);
  }
  if (lStack_d8 != 0) {
    func_0x000140613c20(puStack_e8,lStack_d8 * 0x30,8);
  }
  param_1[2] = lStack_68;
  *param_1 = lStack_78;
  param_1[1] = (longlong)plStack_70;
  lVar10 = *plStack_98;
  lVar4 = plStack_98[1];
  lVar12 = plStack_98[3];
  param_1[5] = plStack_98[2];
  param_1[6] = lVar12;
  param_1[3] = lVar10;
  param_1[4] = lVar4;
  *(short *)((longlong)param_1 + 100) = (short)((ulonglong)uStack_f0 >> 0x20);
  *(int *)(param_1 + 0xc) = (int)uStack_f0;
  param_1[7] = -0x7fffffffffffffff;
  return param_1;
}

