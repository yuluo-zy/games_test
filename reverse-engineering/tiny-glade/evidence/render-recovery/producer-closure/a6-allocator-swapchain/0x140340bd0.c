
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140340bd0(undefined8 *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 *puVar17;
  longlong *plVar18;
  longlong *plVar19;
  longlong lVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  longlong *plVar23;
  undefined8 *puStack_148;
  undefined *puStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  longlong alStack_128 [5];
  undefined8 uStack_100;
  longlong lStack_f8;
  ulonglong uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  longlong lStack_d8;
  longlong *plStack_d0;
  longlong *plStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 **ppuStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_50;
  undefined1 uStack_41;
  undefined8 uStack_40;
  
  puVar7 = puRam000000014362c3a8;
  uStack_40 = 0xfffffffffffffffe;
  if (3 < *puRam000000014362c3a8) {
    puVar17 = (undefined4 *)func_0x00014067dd80(&UNK_1429c2538);
    uStack_b0 = *puVar17;
    uStack_ac = puVar17[1];
    uStack_a8 = puVar17[2];
    uStack_a4 = puVar17[3];
    uStack_84 = puVar17[4];
    uStack_a0 = 4;
    puStack_98 = &UNK_1429c2550;
    uStack_90 = 0x13;
    puStack_80 = &UNK_1429c2528;
    uStack_78 = 1;
    ppuStack_70 = (undefined8 **)0x8;
    plStack_d0 = (longlong *)0x0;
    uStack_68 = 0;
    uStack_60 = 0;
    plStack_c8 = (longlong *)&UNK_1429c2550;
    uStack_c0 = 0x13;
    uStack_b8 = (longlong *)0x0;
    uStack_88 = 1;
    func_0x000140d4a330(&uStack_41,&plStack_d0);
  }
  FUN_1400315e0(&plStack_d0,*param_2 + 0x10,param_3,param_4);
  uVar14 = uStack_a4;
  uVar5 = uStack_a8;
  uVar4 = uStack_ac;
  uVar3 = uStack_b0;
  plVar23 = plStack_c8;
  plVar19 = plStack_d0;
  if (plStack_d0 == (longlong *)0x0) {
    *param_1 = 0x16;
    param_1[1] = &UNK_1429c261e;
    param_1[2] = 0x1e;
    *(undefined4 *)(param_1 + 3) = plStack_c8._0_4_;
    return param_1;
  }
  uVar10 = (undefined4)uStack_c0;
  uVar16 = uStack_a0._4_4_;
  uVar15 = (undefined4)uStack_a0;
  uVar11 = uStack_c0._4_4_;
  uVar12 = (undefined4)uStack_b8;
  uVar13 = uStack_b8._4_4_;
  plVar18 = (longlong *)func_0x000140613c10(0x48,8);
  if (plVar18 == (longlong *)0x0) {
    FUN_1428d9000(8,0x48);
    goto code_r0x0001403411a7;
  }
  *plVar18 = 1;
  plVar18[1] = 1;
  plVar18[2] = (longlong)plVar19;
  plVar18[3] = (longlong)plVar23;
  *(undefined4 *)(plVar18 + 4) = uVar10;
  *(undefined4 *)((longlong)plVar18 + 0x24) = uVar11;
  *(undefined4 *)(plVar18 + 5) = uVar12;
  *(undefined4 *)((longlong)plVar18 + 0x2c) = uVar13;
  *(undefined4 *)(plVar18 + 6) = uVar3;
  *(undefined4 *)((longlong)plVar18 + 0x34) = uVar4;
  *(undefined4 *)(plVar18 + 7) = uVar5;
  *(undefined4 *)((longlong)plVar18 + 0x3c) = uVar14;
  *(undefined4 *)(plVar18 + 8) = uVar15;
  *(undefined4 *)((longlong)plVar18 + 0x44) = uVar16;
  plStack_50 = plVar18;
  if (3 < *puVar7) {
    puVar17 = (undefined4 *)func_0x00014067dd80(&UNK_1429c2598);
    uStack_b0 = *puVar17;
    uStack_ac = puVar17[1];
    uStack_a8 = puVar17[2];
    uStack_a4 = puVar17[3];
    uStack_84 = puVar17[4];
    uStack_a0 = 4;
    puStack_98 = &UNK_1429c2550;
    uStack_90 = 0x13;
    puStack_80 = &UNK_1429c2588;
    uStack_78 = 1;
    ppuStack_70 = (undefined8 **)0x8;
    plStack_d0 = (longlong *)0x0;
    uStack_68 = 0;
    uStack_60 = 0;
    plStack_c8 = (longlong *)&UNK_1429c2550;
    uStack_c0 = 0x13;
    uStack_b8 = (longlong *)0x0;
    uStack_88 = 1;
    func_0x000140d4a330(&uStack_41,&plStack_d0);
  }
  FUN_140c01e00(&plStack_d0,param_2);
  if (SBORROW8(0,(longlong)plStack_d0)) {
    *param_1 = 0x16;
    param_1[1] = &UNK_1429c263c;
    param_1[2] = 0x1a;
    *(undefined4 *)(param_1 + 3) = plStack_c8._0_4_;
code_r0x000140340f9f:
    LOCK();
    *plStack_50 = *plStack_50 + -1;
    UNLOCK();
    if (*plStack_50 == 0) {
      func_0x000140b5d090(&plStack_50);
    }
    return param_1;
  }
  uStack_138 = plStack_d0;
  uStack_130 = plStack_c8;
  alStack_128[0] = uStack_c0;
  FUN_140bdcf90(&uStack_e8,&uStack_138,plStack_50 + 2);
  uStack_c0 = CONCAT44(uStack_e4,uStack_e8);
  plStack_d0 = (longlong *)CONCAT44(uStack_dc,uStack_e0);
  uStack_b8 = plStack_d0 + lStack_d8 * 0xd2;
  plStack_c8 = plStack_d0;
  FUN_1405620f0(&uStack_100,&plStack_d0,&UNK_1429ae548);
  uVar9 = uStack_f0;
  lVar8 = lStack_f8;
  if (uStack_f0 == 0) {
    puVar17 = (undefined4 *)func_0x000140613c10(0x26,1);
    uVar6 = _UNK_1429c2610;
    if (puVar17 != (undefined4 *)0x0) {
      *(undefined8 *)(puVar17 + 4) = _UNK_1429c2608;
      *(undefined8 *)(puVar17 + 6) = uVar6;
      uVar5 = _UNK_1429c2604;
      uVar4 = _UNK_1429c2600;
      uVar3 = _UNK_1429c25fc;
      *puVar17 = _UNK_1429c25f8;
      puVar17[1] = uVar3;
      puVar17[2] = uVar4;
      puVar17[3] = uVar5;
      *(undefined8 *)((longlong)puVar17 + 0x1e) = 0x6e6f697461746e65;
      *param_1 = 0x11;
      param_1[1] = 0x26;
      param_1[2] = puVar17;
      param_1[3] = 0x26;
      FUN_1405d5020(&uStack_100);
      goto code_r0x000140340f9f;
    }
    FUN_1428d8fe3(1,0x26,&UNK_1429c1ee0);
    goto code_r0x0001403411d1;
  }
  if (*puVar7 < 3) goto code_r0x000140341161;
  plVar23 = (longlong *)0x0;
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uStack_f0;
  plVar19 = SUB168(auVar1 * ZEXT816(0x18),0);
  if ((SUB168(auVar1 * ZEXT816(0x18),8) == 0) && (plVar19 < (longlong *)0x7ffffffffffffff9)) {
    if (plVar19 == (longlong *)0x0) {
      lVar20 = 8;
      uVar21 = 0;
    }
    else {
      plVar23 = (longlong *)0x8;
      lVar20 = func_0x000140613c10(plVar19,8);
      uVar21 = uVar9;
      if (lVar20 == 0) goto code_r0x0001403411a7;
    }
    alStack_128[0] = 0;
    plStack_c8 = (longlong *)0x0;
    uStack_138 = (longlong *)uVar21;
    uStack_130 = (longlong *)lVar20;
    plStack_d0 = alStack_128;
    uStack_c0 = lVar20;
    FUN_14048ea40(lVar8,lVar8 + uVar9 * 8,&plStack_d0);
    lVar20 = alStack_128[0];
    lVar8 = (longlong)uStack_130;
    uStack_e8 = (undefined4)uStack_138;
    uStack_e4 = uStack_138._4_4_;
    uStack_e0 = (undefined4)uStack_130;
    uStack_dc = uStack_130._4_4_;
    lStack_d8 = alStack_128[0];
    FUN_14048bd20(&uStack_138,uStack_130,alStack_128[0],&UNK_1429c25b0,1);
    puStack_140 = &UNK_14003bad0;
    puStack_148 = &uStack_138;
    puVar17 = (undefined4 *)func_0x00014067dd80(&UNK_1429c25e0);
    uStack_b0 = *puVar17;
    uStack_ac = puVar17[1];
    uStack_a8 = puVar17[2];
    uStack_a4 = puVar17[3];
    uStack_84 = puVar17[4];
    uStack_a0 = 3;
    puStack_98 = &UNK_1429c2550;
    uStack_90 = 0x13;
    puStack_80 = &UNK_1429c25d0;
    uStack_78 = 1;
    ppuStack_70 = &puStack_148;
    uStack_68 = 1;
    uStack_60 = 0;
    plStack_d0 = (longlong *)0x0;
    plStack_c8 = (longlong *)&UNK_1429c2550;
    uStack_c0 = 0x13;
    uStack_b8 = (longlong *)0x0;
    uStack_88 = 1;
    func_0x000140d4a330(&uStack_41,&plStack_d0);
    if (uStack_138 != (longlong *)0x0) {
      func_0x000140613c20(uStack_130,uStack_138,1);
    }
    if (lVar20 != 0) {
      puVar22 = (undefined8 *)(lVar8 + 8);
      do {
        if (puVar22[-1] != 0) {
          func_0x000140613c20(*puVar22,puVar22[-1],1);
        }
        puVar22 = puVar22 + 3;
        lVar20 = lVar20 + -1;
      } while (lVar20 != 0);
    }
    if (CONCAT44(uStack_e4,uStack_e8) != 0) {
      func_0x000140613c20(lVar8,CONCAT44(uStack_e4,uStack_e8) * 0x18,8);
    }
code_r0x000140341161:
    param_1[3] = uStack_f0;
    param_1[1] = uStack_100;
    param_1[2] = lStack_f8;
    param_1[4] = plStack_50;
    *param_1 = 0x18;
    return param_1;
  }
code_r0x0001403411a7:
  FUN_1428d8fe3(plVar23,plVar19,&UNK_1429c1e78);
code_r0x0001403411d1:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

