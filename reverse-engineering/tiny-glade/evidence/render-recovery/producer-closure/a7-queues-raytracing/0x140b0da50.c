
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140b0da50(undefined8 *param_1,longlong param_2,ulonglong param_3)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  undefined8 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined4 *puVar18;
  longlong lVar19;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  longlong lStack_1a0;
  longlong lStack_198;
  longlong lStack_190;
  longlong lStack_168;
  undefined8 uStack_160;
  undefined8 *puStack_148;
  code *pcStack_140;
  undefined8 uStack_138;
  undefined4 uStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  longlong lStack_118;
  undefined *puStack_110;
  ulonglong *puStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined8 uStack_e8;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined8 **ppuStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  longlong lStack_98;
  longlong lStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 uStack_78;
  ulonglong uStack_70;
  longlong lStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar19 = 0;
  auVar1._8_8_ = 0;
  auVar1._0_8_ = param_3;
  uVar11 = SUB168(auVar1 * ZEXT816(0x30),0);
  if ((SUB168(auVar1 * ZEXT816(0x30),8) == 0) && (uVar11 < 0x7ffffffffffffff1)) {
    if (uVar11 == 0) {
      lVar12 = 0x10;
      puVar16 = (undefined4 *)0x0;
      uVar10 = 0;
    }
    else {
      lVar19 = 0x10;
      puVar16 = (undefined4 *)0x10;
      lVar12 = func_0x000140613c10(uVar11,0x10);
      uVar10 = param_3;
      if (lVar12 == 0) goto code_r0x000140b0dee0;
    }
    if (param_3 == 0) {
      *param_1 = 0;
      param_1[1] = 4;
      param_1[2] = 0;
      param_1[3] = 0;
      param_1[4] = 4;
      param_1[5] = 0;
      param_1[6] = 0;
      param_1[7] = 4;
      param_1[8] = 0;
    }
    else {
      if (param_3 == 1) {
        uVar11 = 0;
      }
      else {
        puVar16 = (undefined4 *)(param_2 + 0x40);
        puVar18 = (undefined4 *)(lVar12 + 0x50);
        uVar11 = 0;
        do {
          uVar14 = puVar16[-0xf];
          uVar15 = puVar16[-0xe];
          uVar3 = puVar16[-0xd];
          uVar4 = puVar16[-0xc];
          uVar5 = puVar16[-0xb];
          uVar6 = puVar16[-10];
          uVar7 = puVar16[-9];
          uVar8 = puVar16[-8];
          puVar18[-0x14] = puVar16[-0x10];
          puVar18[-0x13] = uVar14;
          puVar18[-0x12] = uVar15;
          puVar18[-0x11] = uVar15;
          puVar18[-0x10] = uVar3;
          puVar18[-0xf] = uVar4;
          puVar18[-0xe] = uVar5;
          puVar18[-0xd] = uVar5;
          puVar18[-0xc] = uVar6;
          puVar18[-0xb] = uVar7;
          puVar18[-10] = uVar8;
          puVar18[-9] = uVar8;
          uVar14 = puVar16[-5];
          uVar15 = puVar16[-4];
          uVar3 = puVar16[-3];
          uVar4 = puVar16[-2];
          uVar5 = puVar16[-1];
          uVar6 = *puVar16;
          uVar7 = puVar16[1];
          uVar8 = puVar16[2];
          puVar18[-8] = puVar16[-6];
          puVar18[-7] = uVar14;
          puVar18[-6] = uVar15;
          puVar18[-5] = uVar15;
          puVar18[-4] = uVar3;
          puVar18[-3] = uVar4;
          puVar18[-2] = uVar5;
          puVar18[-1] = uVar5;
          *puVar18 = uVar6;
          puVar18[1] = uVar7;
          puVar18[2] = uVar8;
          puVar18[3] = uVar8;
          uVar11 = uVar11 + 2;
          puVar16 = puVar16 + 0x14;
          puVar18 = puVar18 + 0x18;
        } while ((param_3 & 0xfffffffffffffffe) != uVar11);
      }
      uVar14 = SUB84(puVar16,0);
      if ((param_3 & 1) != 0) {
        puVar16 = (undefined4 *)(param_2 + uVar11 * 0x28);
        uVar15 = puVar16[1];
        uVar3 = puVar16[2];
        puVar18 = (undefined4 *)(param_2 + 0xc + uVar11 * 0x28);
        uVar4 = *puVar18;
        uVar5 = puVar18[1];
        uVar6 = puVar18[2];
        puVar18 = (undefined4 *)(param_2 + 0x18 + uVar11 * 0x28);
        uVar7 = *puVar18;
        uVar8 = puVar18[1];
        uVar9 = puVar18[2];
        lVar19 = uVar11 * 0x30;
        puVar18 = (undefined4 *)(lVar12 + lVar19);
        *puVar18 = *puVar16;
        puVar18[1] = uVar15;
        puVar18[2] = uVar3;
        puVar18[3] = uVar3;
        puVar16 = (undefined4 *)(lVar12 + 0x10 + lVar19);
        *puVar16 = uVar4;
        puVar16[1] = uVar5;
        puVar16[2] = uVar6;
        puVar16[3] = uVar6;
        puVar16 = (undefined4 *)(lVar12 + 0x20 + lVar19);
        *puVar16 = uVar7;
        puVar16[1] = uVar8;
        puVar16[2] = uVar9;
        puVar16[3] = uVar9;
      }
      uStack_70 = uVar10;
      lStack_68 = lVar12;
      uStack_60 = param_3;
      uStack_58 = uVar10;
      uVar13 = FUN_140d635e0();
      uStack_128 = 0;
      uStack_120 = 0;
      uStack_100 = CONCAT53(uStack_100._3_5_,0x10000);
      uStack_100 = CONCAT71(uStack_100._1_7_,4);
      lStack_118 = 1;
      puStack_110 = (undefined *)_UNK_142ae78c0;
      puStack_108 = (ulonglong *)0x4040000000000003;
      uStack_58 = uVar10;
      FUN_1417c93d0(&uStack_1b8,lVar12,param_3,&lStack_118,&uStack_128);
      uVar15 = (undefined4)lVar12;
      uStack_138 = FUN_140d635e0();
      uStack_130 = uVar15;
      if (4 < *puRam000000014362c3a8) {
        uStack_88 = FUN_140d63680(&uStack_138,uVar13,uVar14);
        uStack_80 = (undefined4)uVar13;
        puStack_148 = &uStack_88;
        pcStack_140 = FUN_140d8a550;
        puVar16 = (undefined4 *)func_0x00014067dd80(&UNK_142ae7c50);
        uStack_f8 = *puVar16;
        uStack_f4 = puVar16[1];
        uStack_f0 = puVar16[2];
        uStack_ec = puVar16[3];
        uStack_cc = puVar16[4];
        uStack_e8 = 5;
        puStack_e0 = &UNK_142ae7c68;
        uStack_d8 = 0x20;
        puStack_c8 = &UNK_142ae7c10;
        uStack_c0 = 1;
        ppuStack_b8 = &puStack_148;
        uStack_b0 = 1;
        uStack_a8 = 0;
        lStack_118 = 0;
        puStack_110 = &UNK_142ae7c68;
        puStack_108 = (ulonglong *)0x20;
        uStack_100 = 0;
        uStack_d0 = 1;
        func_0x000140d4a330(&uStack_49,&lStack_118);
      }
      puStack_110 = (undefined *)(lStack_198 + lStack_190 * 4);
      lStack_118 = lStack_198;
      puStack_108 = &uStack_70;
      FUN_141309ae0(&uStack_88,&lStack_118,&UNK_142ae79d8);
      if (lStack_190 == 0) {
        lVar12 = 4;
      }
      else {
        lVar19 = lStack_190 * 4;
        lVar12 = func_0x000140613c10(lVar19);
        if (lVar12 == 0) goto code_r0x000140b0def2;
        lVar17 = 0;
        do {
          uVar11 = (ulonglong)*(uint *)(lStack_198 + lVar17 * 4);
          if (param_3 <= uVar11) {
            lStack_98 = lVar12;
            lStack_90 = lVar19;
            FUN_1428d9518(uVar11,param_3,&UNK_142c65638);
            goto code_r0x000140b0df06;
          }
          *(undefined4 *)(lVar12 + lVar17 * 4) = *(undefined4 *)(param_2 + 0x24 + uVar11 * 0x28);
          lVar17 = lVar17 + 1;
        } while (lStack_190 != lVar17);
      }
      param_1[2] = uStack_1a8;
      *param_1 = uStack_1b8;
      param_1[1] = uStack_1b0;
      *(undefined4 *)(param_1 + 3) = (undefined4)uStack_88;
      *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_88._4_4_;
      *(undefined4 *)(param_1 + 4) = uStack_80;
      *(undefined4 *)((longlong)param_1 + 0x24) = uStack_7c;
      param_1[5] = uStack_78;
      param_1[6] = lStack_190;
      param_1[7] = lVar12;
      param_1[8] = lStack_190;
      if (lStack_1a0 != 0) {
        func_0x000140613c20(lStack_198,lStack_1a0 << 2,4);
      }
      uVar10 = uStack_70;
      lVar12 = lStack_68;
      if ((!SBORROW8(0,lStack_168)) && (lStack_168 != 0)) {
        func_0x000140613c20(uStack_160,lStack_168 << 5,0x10);
        uVar10 = uStack_70;
        lVar12 = lStack_68;
      }
    }
    if (uVar10 != 0) {
      func_0x000140613c20(lVar12,uVar10 * 0x30,0x10);
    }
    return param_1;
  }
code_r0x000140b0dee0:
  FUN_1428d8fe3(lVar19,uVar11,&UNK_142ae79d8);
code_r0x000140b0def2:
  FUN_1428d8fe3(4,lVar19,&UNK_142ae79d8);
code_r0x000140b0df06:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

