
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_1415fc230(undefined8 param_1,undefined8 param_2,longlong param_3)

{
  undefined1 auVar1 [16];
  undefined4 uVar2;
  undefined8 uVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  undefined1 uVar7;
  int iVar8;
  longlong lVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  uint uVar17;
  ushort uVar18;
  ulonglong uVar19;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined8 auStack_380 [3];
  longlong lStack_368;
  longlong lStack_360;
  longlong lStack_358;
  undefined1 auStack_320 [176];
  longlong lStack_270;
  ulonglong uStack_268;
  char cStack_24a;
  longlong lStack_248;
  undefined8 uStack_240;
  undefined1 auStack_238 [72];
  undefined *puStack_1f0;
  undefined8 uStack_1e8;
  longlong lStack_1d8;
  longlong lStack_1d0;
  longlong lStack_1c8;
  undefined1 auStack_190 [176];
  longlong lStack_e0;
  ulonglong uStack_d8;
  char cStack_ba;
  longlong lStack_b8;
  undefined8 uStack_b0;
  undefined1 auStack_a8 [72];
  longlong lStack_60;
  undefined8 uStack_58;
  int iStack_50;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_49 = 1;
  uVar12 = 0x8226cc786e679355;
  uStack_58 = param_2;
  lVar9 = FUN_1400dd6f0(param_3 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
  if (((lVar9 != 0) && (uVar12 < *(ulonglong *)(param_3 + 0x218))) &&
     (uVar12 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar12 * 8), uVar12 != 0)) {
    lVar9 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar9) != 0) {
      uVar3 = *(undefined8 *)(*(longlong *)(param_3 + 0x1e0) + lVar9 + 0x10);
      uStack_49 = 1;
      puVar10 = (undefined8 *)FUN_140969720(uVar3,&UNK_142c2e3d8);
      if (puVar10 != (undefined8 *)0x0) {
        puStack_1f0 = &UNK_142a9dc30;
        uStack_1e8 = 0x2c;
        if ((puVar10[3] != 0x2c) ||
           (iVar8 = func_0x000142923bf0(&UNK_142a9dc30,puVar10[2],0x2c), iVar8 != 0)) {
          auStack_380[0] = 0;
          uStack_49 = 1;
          FUN_1428cab41(0,&puStack_1f0,puVar10 + 2,auStack_380,&UNK_142c2e180);
          goto code_r0x0001415fccd8;
        }
        uStack_49 = 1;
        puVar10 = (undefined8 *)(**(code **)(puVar10[1] + 0x18))(*puVar10);
        if (puVar10 != (undefined8 *)0x0) {
          uStack_49 = 1;
          puVar11 = (undefined8 *)FUN_140969720(uVar3,&UNK_142c2e350);
          if (puVar11 != (undefined8 *)0x0) {
            puStack_1f0 = &UNK_142a9e726;
            uStack_1e8 = 0x26;
            if ((puVar11[3] != 0x26) ||
               (iVar8 = func_0x000142923bf0(&UNK_142a9e726,puVar11[2],0x26), iVar8 != 0)) {
              auStack_380[0] = 0;
              uStack_49 = 1;
              FUN_1428cab41(0,&puStack_1f0,puVar11 + 2,auStack_380,&UNK_142c2e180);
              goto code_r0x0001415fccd8;
            }
            uStack_49 = 1;
            puVar11 = (undefined8 *)(**(code **)(puVar11[1] + 0x18))(*puVar11);
            if (puVar11 != (undefined8 *)0x0) {
              func_0x000142923c00(auStack_380,uStack_58,400);
              uVar7 = FUN_140972c20(auStack_238,uStack_240,auStack_320,auStack_380,0x1a);
              if (*(longlong *)(lStack_248 + 0xb0) != 0) {
                lVar9 = *(longlong *)(lStack_248 + 0x98);
                uVar12 = *(ulonglong *)(lStack_248 + 0xa0);
                uVar13 = uVar12 & 0x168ddb2c3b5f38a9;
                lVar16 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar9 + uVar13);
                  auVar26[0] = -(auVar1[0] == UNK_1429a4ba0);
                  auVar26[1] = -(auVar1[1] == UNK_1429a4ba0._1_1_);
                  auVar26[2] = -(auVar1[2] == UNK_1429a4ba0._2_1_);
                  auVar26[3] = -(auVar1[3] == UNK_1429a4ba0._3_1_);
                  auVar26[4] = -(auVar1[4] == UNK_1429a4ba0._4_1_);
                  auVar26[5] = -(auVar1[5] == UNK_1429a4ba0._5_1_);
                  auVar26[6] = -(auVar1[6] == UNK_1429a4ba0._6_1_);
                  auVar26[7] = -(auVar1[7] == UNK_1429a4ba0._7_1_);
                  auVar26[8] = -(auVar1[8] == UNK_1429a4ba0._8_1_);
                  auVar26[9] = -(auVar1[9] == UNK_1429a4ba0._9_1_);
                  auVar26[10] = -(auVar1[10] == UNK_1429a4ba0._10_1_);
                  auVar26[0xb] = -(auVar1[0xb] == UNK_1429a4ba0._11_1_);
                  auVar26[0xc] = -(auVar1[0xc] == UNK_1429a4ba0._12_1_);
                  auVar26[0xd] = -(auVar1[0xd] == UNK_1429a4ba0._13_1_);
                  auVar26[0xe] = -(auVar1[0xe] == UNK_1429a4ba0._14_1_);
                  auVar26[0xf] = -(auVar1[0xf] == UNK_1429a4ba0._15_1_);
                  uVar18 = (ushort)(SUB161(auVar26 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar26[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    if (*(longlong *)(lVar9 + -0x10 + uVar19 * -0x10) == 0x168ddb2c3b5f38a9) {
                      lVar16 = uVar19 * -0x10;
                      iStack_50 = *(int *)(lVar9 + -8 + lVar16);
                      uVar2 = *(undefined4 *)(lVar9 + -4 + lVar16);
                      if (iStack_50 != 0) {
                        puStack_1f0 = (undefined *)0x0;
                        FUN_1428a994f(0,&iStack_50,&UNK_142c7d9e8,&puStack_1f0,&UNK_142c7dc60);
                        goto code_r0x0001415fccd8;
                      }
                      if (lStack_358 == lStack_368) {
                        FUN_140562e70(&lStack_368,&UNK_142c7dc78);
                      }
                      *(undefined4 *)(lStack_360 + lStack_358 * 8) = uVar2;
                      *(undefined1 *)(lStack_360 + 4 + lStack_358 * 8) = uVar7;
                      lStack_358 = lStack_358 + 1;
                      goto code_r0x0001415fc51b;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar20[0] = -(auVar1[0] == -1);
                  auVar20[1] = -(auVar1[1] == -1);
                  auVar20[2] = -(auVar1[2] == -1);
                  auVar20[3] = -(auVar1[3] == -1);
                  auVar20[4] = -(auVar1[4] == -1);
                  auVar20[5] = -(auVar1[5] == -1);
                  auVar20[6] = -(auVar1[6] == -1);
                  auVar20[7] = -(auVar1[7] == -1);
                  auVar20[8] = -(auVar1[8] == -1);
                  auVar20[9] = -(auVar1[9] == -1);
                  auVar20[10] = -(auVar1[10] == -1);
                  auVar20[0xb] = -(auVar1[0xb] == -1);
                  auVar20[0xc] = -(auVar1[0xc] == -1);
                  auVar20[0xd] = -(auVar1[0xd] == -1);
                  auVar20[0xe] = -(auVar1[0xe] == -1);
                  auVar20[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar20 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar20 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar20 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar20 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0
                      ) || auVar20[0xf] < '\0') break;
                  lVar14 = uVar13 + lVar16;
                  lVar16 = lVar16 + 0x10;
                  uVar13 = lVar14 + 0x10U & uVar12;
                }
              }
code_r0x0001415fc51b:
              func_0x000142923c00(&puStack_1f0,auStack_380,400);
              uVar7 = FUN_140971d20(auStack_a8,uStack_b0,auStack_190,&puStack_1f0,0x1a);
              if (*(longlong *)(lStack_b8 + 0xb0) != 0) {
                lVar9 = *(longlong *)(lStack_b8 + 0x98);
                uVar12 = *(ulonglong *)(lStack_b8 + 0xa0);
                uVar13 = uVar12 & 0x47ddebfb264383d0;
                lVar16 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar9 + uVar13);
                  auVar27[0] = -(auVar1[0] == UNK_142950630);
                  auVar27[1] = -(auVar1[1] == UNK_142950630._1_1_);
                  auVar27[2] = -(auVar1[2] == UNK_142950630._2_1_);
                  auVar27[3] = -(auVar1[3] == UNK_142950630._3_1_);
                  auVar27[4] = -(auVar1[4] == UNK_142950630._4_1_);
                  auVar27[5] = -(auVar1[5] == UNK_142950630._5_1_);
                  auVar27[6] = -(auVar1[6] == UNK_142950630._6_1_);
                  auVar27[7] = -(auVar1[7] == UNK_142950630._7_1_);
                  auVar27[8] = -(auVar1[8] == UNK_142950630._8_1_);
                  auVar27[9] = -(auVar1[9] == UNK_142950630._9_1_);
                  auVar27[10] = -(auVar1[10] == UNK_142950630._10_1_);
                  auVar27[0xb] = -(auVar1[0xb] == UNK_142950630._11_1_);
                  auVar27[0xc] = -(auVar1[0xc] == UNK_142950630._12_1_);
                  auVar27[0xd] = -(auVar1[0xd] == UNK_142950630._13_1_);
                  auVar27[0xe] = -(auVar1[0xe] == UNK_142950630._14_1_);
                  auVar27[0xf] = -(auVar1[0xf] == UNK_142950630._15_1_);
                  uVar18 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar27[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    if (*(longlong *)(lVar9 + -0x10 + uVar19 * -0x10) == 0x47ddebfb264383d0) {
                      lVar16 = uVar19 * -0x10;
                      iStack_50 = *(int *)(lVar9 + -8 + lVar16);
                      uVar2 = *(undefined4 *)(lVar9 + -4 + lVar16);
                      if (iStack_50 != 0) {
                        auStack_380[0] = 0;
                        FUN_1428a994f(0,&iStack_50,&UNK_142c7d9e8,auStack_380,&UNK_142c7dc60);
                        goto code_r0x0001415fccd8;
                      }
                      if (lStack_1c8 == lStack_1d8) {
                        FUN_140562e70(&lStack_1d8,&UNK_142c7dc78);
                      }
                      *(undefined4 *)(lStack_1d0 + lStack_1c8 * 8) = uVar2;
                      *(undefined1 *)(lStack_1d0 + 4 + lStack_1c8 * 8) = uVar7;
                      lStack_1c8 = lStack_1c8 + 1;
                      goto code_r0x0001415fc66a;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar21[0] = -(auVar1[0] == -1);
                  auVar21[1] = -(auVar1[1] == -1);
                  auVar21[2] = -(auVar1[2] == -1);
                  auVar21[3] = -(auVar1[3] == -1);
                  auVar21[4] = -(auVar1[4] == -1);
                  auVar21[5] = -(auVar1[5] == -1);
                  auVar21[6] = -(auVar1[6] == -1);
                  auVar21[7] = -(auVar1[7] == -1);
                  auVar21[8] = -(auVar1[8] == -1);
                  auVar21[9] = -(auVar1[9] == -1);
                  auVar21[10] = -(auVar1[10] == -1);
                  auVar21[0xb] = -(auVar1[0xb] == -1);
                  auVar21[0xc] = -(auVar1[0xc] == -1);
                  auVar21[0xd] = -(auVar1[0xd] == -1);
                  auVar21[0xe] = -(auVar1[0xe] == -1);
                  auVar21[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar21 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar21 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar21 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar21 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0
                      ) || auVar21[0xf] < '\0') break;
                  lVar14 = uVar13 + lVar16;
                  lVar16 = lVar16 + 0x10;
                  uVar13 = lVar14 + 0x10U & uVar12;
                }
              }
code_r0x0001415fc66a:
              func_0x000142923c00(auStack_380,&puStack_1f0,400);
              uVar7 = FUN_140970120(auStack_238,uStack_240,auStack_320,auStack_380,0x1a);
              lVar9 = lStack_358;
              if (*(longlong *)(lStack_248 + 0xb0) != 0) {
                lVar16 = *(longlong *)(lStack_248 + 0x98);
                uVar12 = *(ulonglong *)(lStack_248 + 0xa0);
                uVar13 = uVar12 & 0x339e76846042a271;
                lVar14 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar16 + uVar13);
                  auVar28[0] = -(auVar1[0] == UNK_1429506d0);
                  auVar28[1] = -(auVar1[1] == UNK_1429506d0._1_1_);
                  auVar28[2] = -(auVar1[2] == UNK_1429506d0._2_1_);
                  auVar28[3] = -(auVar1[3] == UNK_1429506d0._3_1_);
                  auVar28[4] = -(auVar1[4] == UNK_1429506d0._4_1_);
                  auVar28[5] = -(auVar1[5] == UNK_1429506d0._5_1_);
                  auVar28[6] = -(auVar1[6] == UNK_1429506d0._6_1_);
                  auVar28[7] = -(auVar1[7] == UNK_1429506d0._7_1_);
                  auVar28[8] = -(auVar1[8] == UNK_1429506d0._8_1_);
                  auVar28[9] = -(auVar1[9] == UNK_1429506d0._9_1_);
                  auVar28[10] = -(auVar1[10] == UNK_1429506d0._10_1_);
                  auVar28[0xb] = -(auVar1[0xb] == UNK_1429506d0._11_1_);
                  auVar28[0xc] = -(auVar1[0xc] == UNK_1429506d0._12_1_);
                  auVar28[0xd] = -(auVar1[0xd] == UNK_1429506d0._13_1_);
                  auVar28[0xe] = -(auVar1[0xe] == UNK_1429506d0._14_1_);
                  auVar28[0xf] = -(auVar1[0xf] == UNK_1429506d0._15_1_);
                  uVar18 = (ushort)(SUB161(auVar28 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar28[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    if (*(longlong *)(lVar16 + -0x10 + uVar19 * -0x10) == 0x339e76846042a271) {
                      lVar14 = uVar19 * -0x10;
                      iStack_50 = *(int *)(lVar16 + -8 + lVar14);
                      uVar2 = *(undefined4 *)(lVar16 + -4 + lVar14);
                      if (iStack_50 != 0) {
                        puStack_1f0 = (undefined *)0x0;
                        FUN_1428a994f(0,&iStack_50,&UNK_142c7d9e8,&puStack_1f0,&UNK_142c7dc60);
                        goto code_r0x0001415fccd8;
                      }
                      if (lStack_358 == lStack_368) {
                        FUN_140562e70(&lStack_368,&UNK_142c7dc78);
                      }
                      *(undefined4 *)(lStack_360 + lVar9 * 8) = uVar2;
                      *(undefined1 *)(lStack_360 + 4 + lVar9 * 8) = uVar7;
                      lStack_358 = lVar9 + 1;
                      goto code_r0x0001415fc79b;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar22[0] = -(auVar1[0] == -1);
                  auVar22[1] = -(auVar1[1] == -1);
                  auVar22[2] = -(auVar1[2] == -1);
                  auVar22[3] = -(auVar1[3] == -1);
                  auVar22[4] = -(auVar1[4] == -1);
                  auVar22[5] = -(auVar1[5] == -1);
                  auVar22[6] = -(auVar1[6] == -1);
                  auVar22[7] = -(auVar1[7] == -1);
                  auVar22[8] = -(auVar1[8] == -1);
                  auVar22[9] = -(auVar1[9] == -1);
                  auVar22[10] = -(auVar1[10] == -1);
                  auVar22[0xb] = -(auVar1[0xb] == -1);
                  auVar22[0xc] = -(auVar1[0xc] == -1);
                  auVar22[0xd] = -(auVar1[0xd] == -1);
                  auVar22[0xe] = -(auVar1[0xe] == -1);
                  auVar22[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar22 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar22 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar22 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar22 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar22 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar22 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar22 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0
                      ) || auVar22[0xf] < '\0') break;
                  lVar15 = uVar13 + lVar14;
                  lVar14 = lVar14 + 0x10;
                  uVar13 = lVar15 + 0x10U & uVar12;
                }
              }
code_r0x0001415fc79b:
              func_0x000142923c00(&puStack_1f0,auStack_380,400);
              uVar7 = FUN_140972b20(auStack_a8,uStack_b0,auStack_190,&puStack_1f0,0x1a);
              lVar9 = lStack_1c8;
              if (*(longlong *)(lStack_b8 + 0xb0) != 0) {
                lVar16 = *(longlong *)(lStack_b8 + 0x98);
                uVar12 = *(ulonglong *)(lStack_b8 + 0xa0);
                uVar13 = uVar12 & 0xe9701a257aa8828c;
                lVar14 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar16 + uVar13);
                  auVar29[0] = -(auVar1[0] == UNK_1429fc8f0);
                  auVar29[1] = -(auVar1[1] == UNK_1429fc8f0._1_1_);
                  auVar29[2] = -(auVar1[2] == UNK_1429fc8f0._2_1_);
                  auVar29[3] = -(auVar1[3] == UNK_1429fc8f0._3_1_);
                  auVar29[4] = -(auVar1[4] == UNK_1429fc8f0._4_1_);
                  auVar29[5] = -(auVar1[5] == UNK_1429fc8f0._5_1_);
                  auVar29[6] = -(auVar1[6] == UNK_1429fc8f0._6_1_);
                  auVar29[7] = -(auVar1[7] == UNK_1429fc8f0._7_1_);
                  auVar29[8] = -(auVar1[8] == UNK_1429fc8f0._8_1_);
                  auVar29[9] = -(auVar1[9] == UNK_1429fc8f0._9_1_);
                  auVar29[10] = -(auVar1[10] == UNK_1429fc8f0._10_1_);
                  auVar29[0xb] = -(auVar1[0xb] == UNK_1429fc8f0._11_1_);
                  auVar29[0xc] = -(auVar1[0xc] == UNK_1429fc8f0._12_1_);
                  auVar29[0xd] = -(auVar1[0xd] == UNK_1429fc8f0._13_1_);
                  auVar29[0xe] = -(auVar1[0xe] == UNK_1429fc8f0._14_1_);
                  auVar29[0xf] = -(auVar1[0xf] == UNK_1429fc8f0._15_1_);
                  uVar18 = (ushort)(SUB161(auVar29 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar29[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    if (*(longlong *)(lVar16 + -0x10 + uVar19 * -0x10) == -0x168fe5da85577d74) {
                      lVar14 = uVar19 * -0x10;
                      iStack_50 = *(int *)(lVar16 + -8 + lVar14);
                      uVar2 = *(undefined4 *)(lVar16 + -4 + lVar14);
                      if (iStack_50 != 0) {
                        auStack_380[0] = 0;
                        FUN_1428a994f(0,&iStack_50,&UNK_142c7d9e8,auStack_380,&UNK_142c7dc60);
                        goto code_r0x0001415fccd8;
                      }
                      if (lStack_1c8 == lStack_1d8) {
                        FUN_140562e70(&lStack_1d8,&UNK_142c7dc78);
                      }
                      *(undefined4 *)(lStack_1d0 + lVar9 * 8) = uVar2;
                      *(undefined1 *)(lStack_1d0 + 4 + lVar9 * 8) = uVar7;
                      lStack_1c8 = lVar9 + 1;
                      goto code_r0x0001415fc8ea;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar23[0] = -(auVar1[0] == -1);
                  auVar23[1] = -(auVar1[1] == -1);
                  auVar23[2] = -(auVar1[2] == -1);
                  auVar23[3] = -(auVar1[3] == -1);
                  auVar23[4] = -(auVar1[4] == -1);
                  auVar23[5] = -(auVar1[5] == -1);
                  auVar23[6] = -(auVar1[6] == -1);
                  auVar23[7] = -(auVar1[7] == -1);
                  auVar23[8] = -(auVar1[8] == -1);
                  auVar23[9] = -(auVar1[9] == -1);
                  auVar23[10] = -(auVar1[10] == -1);
                  auVar23[0xb] = -(auVar1[0xb] == -1);
                  auVar23[0xc] = -(auVar1[0xc] == -1);
                  auVar23[0xd] = -(auVar1[0xd] == -1);
                  auVar23[0xe] = -(auVar1[0xe] == -1);
                  auVar23[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar23 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar23 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar23 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar23 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar23 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar23 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar23 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0
                      ) || auVar23[0xf] < '\0') break;
                  lVar15 = uVar13 + lVar14;
                  lVar14 = lVar14 + 0x10;
                  uVar13 = lVar15 + 0x10U & uVar12;
                }
              }
code_r0x0001415fc8ea:
              func_0x000142923c00(auStack_380,&puStack_1f0,400);
              if (((cStack_24a != '\b') && (*(longlong *)(lStack_248 + 0xe8) != 0)) &&
                 (*(longlong *)(lStack_248 + 0x130) != 0)) {
                lVar9 = *(longlong *)(lStack_248 + 0x118);
                uVar12 = *(ulonglong *)(lStack_248 + 0x120);
                uVar13 = uVar12 & 0x1c9f2eef0ac739ac;
                lVar16 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar9 + uVar13);
                  auVar30[0] = -(auVar1[0] == UNK_1429504e0);
                  auVar30[1] = -(auVar1[1] == UNK_1429504e0._1_1_);
                  auVar30[2] = -(auVar1[2] == UNK_1429504e0._2_1_);
                  auVar30[3] = -(auVar1[3] == UNK_1429504e0._3_1_);
                  auVar30[4] = -(auVar1[4] == UNK_1429504e0._4_1_);
                  auVar30[5] = -(auVar1[5] == UNK_1429504e0._5_1_);
                  auVar30[6] = -(auVar1[6] == UNK_1429504e0._6_1_);
                  auVar30[7] = -(auVar1[7] == UNK_1429504e0._7_1_);
                  auVar30[8] = -(auVar1[8] == UNK_1429504e0._8_1_);
                  auVar30[9] = -(auVar1[9] == UNK_1429504e0._9_1_);
                  auVar30[10] = -(auVar1[10] == UNK_1429504e0._10_1_);
                  auVar30[0xb] = -(auVar1[0xb] == UNK_1429504e0._11_1_);
                  auVar30[0xc] = -(auVar1[0xc] == UNK_1429504e0._12_1_);
                  auVar30[0xd] = -(auVar1[0xd] == UNK_1429504e0._13_1_);
                  auVar30[0xe] = -(auVar1[0xe] == UNK_1429504e0._14_1_);
                  auVar30[0xf] = -(auVar1[0xf] == UNK_1429504e0._15_1_);
                  uVar18 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar30[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    lStack_60 = lVar16;
                    if (*(longlong *)(lVar9 + -0x10 + uVar19 * -0x10) == 0x1c9f2eef0ac739ac) {
                      uVar12 = (ulonglong)*(uint *)(lVar9 + -8 + uVar19 * -0x10);
                      if (uStack_268 < uVar12) {
                        FUN_1428d9830(uVar12,uStack_268,&UNK_142ab5bc0);
                        goto code_r0x0001415fccd8;
                      }
                      *(int *)(lStack_270 + uVar12) = (int)*puVar10;
                      goto code_r0x0001415fca17;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar24[0] = -(auVar1[0] == -1);
                  auVar24[1] = -(auVar1[1] == -1);
                  auVar24[2] = -(auVar1[2] == -1);
                  auVar24[3] = -(auVar1[3] == -1);
                  auVar24[4] = -(auVar1[4] == -1);
                  auVar24[5] = -(auVar1[5] == -1);
                  auVar24[6] = -(auVar1[6] == -1);
                  auVar24[7] = -(auVar1[7] == -1);
                  auVar24[8] = -(auVar1[8] == -1);
                  auVar24[9] = -(auVar1[9] == -1);
                  auVar24[10] = -(auVar1[10] == -1);
                  auVar24[0xb] = -(auVar1[0xb] == -1);
                  auVar24[0xc] = -(auVar1[0xc] == -1);
                  auVar24[0xd] = -(auVar1[0xd] == -1);
                  auVar24[0xe] = -(auVar1[0xe] == -1);
                  auVar24[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar24 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar24 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar24 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar24 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar24 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar24 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x6f,0) & 1) != 0) || (SUB161(auVar24 >> 0x77,0) & 1) != 0
                      ) || auVar24[0xf] < '\0') break;
                  lVar14 = uVar13 + lVar16;
                  lVar16 = lVar16 + 0x10;
                  uVar13 = lVar14 + 0x10U & uVar12;
                }
              }
code_r0x0001415fca17:
              func_0x000142923c00(&puStack_1f0,auStack_380,400);
              if (((cStack_ba != '\b') && (*(longlong *)(lStack_b8 + 0xe8) != 0)) &&
                 (*(longlong *)(lStack_b8 + 0x130) != 0)) {
                lVar9 = *(longlong *)(lStack_b8 + 0x118);
                uVar12 = *(ulonglong *)(lStack_b8 + 0x120);
                uVar13 = uVar12 & 0xc2b7f2fdf5e21521;
                lVar16 = 0;
                while( true ) {
                  auVar1 = *(undefined1 (*) [16])(lVar9 + uVar13);
                  auVar31[0] = -(auVar1[0] == UNK_142950400);
                  auVar31[1] = -(auVar1[1] == UNK_142950400._1_1_);
                  auVar31[2] = -(auVar1[2] == UNK_142950400._2_1_);
                  auVar31[3] = -(auVar1[3] == UNK_142950400._3_1_);
                  auVar31[4] = -(auVar1[4] == UNK_142950400._4_1_);
                  auVar31[5] = -(auVar1[5] == UNK_142950400._5_1_);
                  auVar31[6] = -(auVar1[6] == UNK_142950400._6_1_);
                  auVar31[7] = -(auVar1[7] == UNK_142950400._7_1_);
                  auVar31[8] = -(auVar1[8] == UNK_142950400._8_1_);
                  auVar31[9] = -(auVar1[9] == UNK_142950400._9_1_);
                  auVar31[10] = -(auVar1[10] == UNK_142950400._10_1_);
                  auVar31[0xb] = -(auVar1[0xb] == UNK_142950400._11_1_);
                  auVar31[0xc] = -(auVar1[0xc] == UNK_142950400._12_1_);
                  auVar31[0xd] = -(auVar1[0xd] == UNK_142950400._13_1_);
                  auVar31[0xe] = -(auVar1[0xe] == UNK_142950400._14_1_);
                  auVar31[0xf] = -(auVar1[0xf] == UNK_142950400._15_1_);
                  uVar18 = (ushort)(SUB161(auVar31 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar31[0xf] >> 7) << 0xf;
                  uVar17 = (uint)uVar18;
                  while (uVar18 != 0) {
                    uVar5 = 0;
                    for (uVar6 = uVar17; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar19 = uVar5 + uVar13 & uVar12;
                    if (*(longlong *)(lVar9 + -0x10 + uVar19 * -0x10) == -0x3d480d020a1deadf) {
                      uVar12 = (ulonglong)*(uint *)(lVar9 + -8 + uVar19 * -0x10);
                      if (uStack_d8 < uVar12) {
                        FUN_1428d9830(uVar12,uStack_d8,&UNK_142ab5bc0);
                        goto code_r0x0001415fccd8;
                      }
                      *(int *)(lStack_e0 + uVar12) = (int)*puVar11;
                      goto code_r0x0001415fcb18;
                    }
                    uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
                    uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
                  }
                  auVar25[0] = -(auVar1[0] == -1);
                  auVar25[1] = -(auVar1[1] == -1);
                  auVar25[2] = -(auVar1[2] == -1);
                  auVar25[3] = -(auVar1[3] == -1);
                  auVar25[4] = -(auVar1[4] == -1);
                  auVar25[5] = -(auVar1[5] == -1);
                  auVar25[6] = -(auVar1[6] == -1);
                  auVar25[7] = -(auVar1[7] == -1);
                  auVar25[8] = -(auVar1[8] == -1);
                  auVar25[9] = -(auVar1[9] == -1);
                  auVar25[10] = -(auVar1[10] == -1);
                  auVar25[0xb] = -(auVar1[0xb] == -1);
                  auVar25[0xc] = -(auVar1[0xc] == -1);
                  auVar25[0xd] = -(auVar1[0xd] == -1);
                  auVar25[0xe] = -(auVar1[0xe] == -1);
                  auVar25[0xf] = -(auVar1[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar25 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar25 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar25 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0
                      ) || auVar25[0xf] < '\0') break;
                  lVar14 = uVar13 + lVar16;
                  lVar16 = lVar16 + 0x10;
                  uVar13 = lVar14 + 0x10U & uVar12;
                }
              }
code_r0x0001415fcb18:
              func_0x000142923c00(param_1,&puStack_1f0,400);
              return param_1;
            }
          }
          uStack_49 = 1;
          FUN_1428d9310(&UNK_142c7d958);
          goto code_r0x0001415fccd8;
        }
      }
      uStack_49 = 1;
      FUN_1428d9310(&UNK_142c7d940);
      goto code_r0x0001415fccd8;
    }
  }
  uStack_49 = 1;
  FUN_1428d9310(&UNK_142c7d928);
code_r0x0001415fccd8:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

