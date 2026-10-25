
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_1408b0560(undefined8 param_1,longlong param_2,longlong param_3)

{
  undefined4 *puVar1;
  uint *puVar2;
  undefined1 *puVar3;
  undefined1 *puVar4;
  undefined8 *puVar5;
  char cVar6;
  uint uVar7;
  int iVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int *piVar13;
  code *pcVar14;
  uint uVar15;
  undefined4 uVar16;
  longlong lVar17;
  longlong lVar18;
  longlong lVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  longlong *plVar22;
  undefined4 *puVar23;
  char *pcVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  longlong lVar27;
  undefined *puVar28;
  undefined8 unaff_RDI;
  longlong lVar29;
  uint uVar30;
  ushort uVar31;
  ulonglong uVar32;
  bool bVar33;
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined8 unaff_XMM6_Qa;
  undefined8 unaff_XMM6_Qb;
  undefined8 unaff_XMM7_Qa;
  undefined8 unaff_XMM7_Qb;
  undefined8 unaff_XMM8_Qa;
  undefined8 unaff_XMM8_Qb;
  undefined4 unaff_XMM9_Da;
  undefined4 unaff_XMM9_Db;
  undefined4 unaff_XMM9_Dc;
  undefined4 unaff_XMM9_Dd;
  undefined8 auStackX_8 [4];
  undefined *puStack_48;
  
  puStack_48 = &UNK_1408b0576;
  lVar17 = FUN_14289aa40();
  lVar17 = -lVar17;
  *(undefined4 *)(&stack0x00001000 + lVar17) = unaff_XMM9_Da;
  *(undefined4 *)(&stack0x00001004 + lVar17) = unaff_XMM9_Db;
  *(undefined4 *)(&stack0x00001008 + lVar17) = unaff_XMM9_Dc;
  *(undefined4 *)(&stack0x0000100c + lVar17) = unaff_XMM9_Dd;
  *(undefined8 *)(&stack0x00000ff0 + lVar17) = unaff_XMM8_Qa;
  *(undefined8 *)(&stack0x00000ff8 + lVar17) = unaff_XMM8_Qb;
  *(undefined8 *)(&stack0x00000fe0 + lVar17) = unaff_XMM7_Qa;
  *(undefined8 *)(&stack0x00000fe8 + lVar17) = unaff_XMM7_Qb;
  *(undefined8 *)(&stack0x00000fd0 + lVar17) = unaff_XMM6_Qa;
  *(undefined8 *)(&stack0x00000fd8 + lVar17) = unaff_XMM6_Qb;
  *(undefined8 *)(&stack0x00000fc8 + lVar17) = 0xfffffffffffffffe;
  uVar26 = CONCAT71((int7)((ulonglong)unaff_RDI >> 8),1);
  if (cRam0000000143634d90 != '\0') goto code_r0x0001408b1485;
  lVar19 = param_2 + 0x38;
  uVar25 = 0xaca0d6d443d3ae2e;
  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b05e6;
  lVar18 = FUN_1400dd6f0(lVar19,0xaca0d6d443d3ae2e,0x8d07dd0d83c93360);
  if (((lVar18 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar25)) ||
     (uVar25 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar25 * 8), uVar25 == 0)) {
code_r0x0001408b150e:
    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b151a;
    FUN_1428d9310(&UNK_142a9ce68);
  }
  else {
    lVar18 = ~uVar25 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar18) == 0)
    goto code_r0x0001408b150e;
    plVar22 = *(longlong **)(*(longlong *)(param_2 + 0x1e0) + lVar18 + 0x10);
    uVar25 = 0xd38dae617fb79c7a;
    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0651;
    lVar18 = FUN_1400dd6f0(lVar19,0xd38dae617fb79c7a,0x22aadf5c5dad1eae);
    if (((lVar18 != 0) && (uVar25 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar25 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar25 * 8), uVar25 != 0)) {
      lVar18 = ~uVar25 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar18) != 0) {
        piVar13 = *(int **)(*(longlong *)(param_2 + 0x1e0) + lVar18 + 0x10);
        if ((*piVar13 != 10) || ((char)piVar13[2] != '\x01')) {
code_r0x0001408b1485:
          return uVar26 & 0xffffffff;
        }
        lVar18 = plVar22[1];
        if (lVar18 == 0) {
          uVar26 = 0;
          goto code_r0x0001408b1485;
        }
        pcVar24 = (char *)(lVar18 + 0x10);
        LOCK();
        bVar33 = *(char *)(lVar18 + 0x10) == '\0';
        if (bVar33) {
          *(char *)(lVar18 + 0x10) = '\x01';
        }
        UNLOCK();
        if (!bVar33) {
          *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b14c7;
          FUN_1428cd210(pcVar24);
        }
        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b06d8;
        FUN_140c81c70();
        bVar33 = *(char *)(lVar18 + 0x4e) != '\a';
        lVar29 = 0;
        if (bVar33) {
          lVar29 = lVar18 + 0x18;
        }
        *(longlong *)(&stack0x00000f10 + lVar17) = lVar29;
        if (!bVar33) {
          *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b06f7;
          FUN_140c81e10(pcVar24);
          uVar26 = 0;
          LOCK();
          cVar6 = *pcVar24;
          if (cVar6 == '\x01') {
            *pcVar24 = '\0';
          }
          UNLOCK();
          if (cVar6 != '\x01') {
            uVar26 = 0;
code_r0x0001408b14fb:
            *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1502;
            FUN_1428cdb90(pcVar24,0);
          }
          goto code_r0x0001408b1485;
        }
        *(char **)(&stack0x00000f68 + lVar17) = pcVar24;
        uVar26 = 0x4a142e09642375b9;
        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b073b;
        lVar18 = FUN_1400dd6f0(lVar19);
        if (((lVar18 != 0) && (uVar26 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar26 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar26 * 8), uVar26 != 0)) {
          lVar18 = ~uVar26 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar18) != 0) {
            *(undefined8 *)(&stack0x00000fb0 + lVar17) =
                 *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar18 + 0x10);
            if (*(longlong *)(param_2 + 0x88) != 0) {
              lVar18 = *(longlong *)(param_2 + 0x70);
              uVar26 = *(ulonglong *)(param_2 + 0x78);
              uVar25 = uVar26 & 0x8b297ec5ae15f15c;
              lVar29 = 0;
              while( true ) {
                pcVar24 = (char *)(lVar18 + uVar25);
                auVar36[0] = -(*pcVar24 == UNK_1429505f0);
                auVar36[1] = -(pcVar24[1] == UNK_1429505f0._1_1_);
                auVar36[2] = -(pcVar24[2] == UNK_1429505f0._2_1_);
                auVar36[3] = -(pcVar24[3] == UNK_1429505f0._3_1_);
                auVar36[4] = -(pcVar24[4] == UNK_1429505f0._4_1_);
                auVar36[5] = -(pcVar24[5] == UNK_1429505f0._5_1_);
                auVar36[6] = -(pcVar24[6] == UNK_1429505f0._6_1_);
                auVar36[7] = -(pcVar24[7] == UNK_1429505f0._7_1_);
                auVar36[8] = -(pcVar24[8] == UNK_1429505f0._8_1_);
                auVar36[9] = -(pcVar24[9] == UNK_1429505f0._9_1_);
                auVar36[10] = -(pcVar24[10] == UNK_1429505f0._10_1_);
                auVar36[0xb] = -(pcVar24[0xb] == UNK_1429505f0._11_1_);
                auVar36[0xc] = -(pcVar24[0xc] == UNK_1429505f0._12_1_);
                auVar36[0xd] = -(pcVar24[0xd] == UNK_1429505f0._13_1_);
                auVar36[0xe] = -(pcVar24[0xe] == UNK_1429505f0._14_1_);
                auVar36[0xf] = -(pcVar24[0xf] == UNK_1429505f0._15_1_);
                uVar31 = (ushort)(SUB161(auVar36 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar36[0xf] >> 7) << 0xf;
                uVar30 = (uint)uVar31;
                while (uVar31 != 0) {
                  uVar7 = 0;
                  for (uVar15 = uVar30; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                    uVar7 = uVar7 + 1;
                  }
                  uVar32 = uVar7 + uVar25 & uVar26;
                  if ((*(longlong *)(lVar18 + -0x18 + uVar32 * -0x18) == -0x7a6811f8d84b9e4d) &&
                     (*(longlong *)(lVar18 + -0x10 + uVar32 * -0x18) == -0x74d6813a51ea0ea4)) {
                    uVar26 = *(ulonglong *)(lVar18 + uVar32 * -0x18 + -8);
                    goto code_r0x0001408b0952;
                  }
                  uVar31 = (ushort)(uVar30 - 1) & (ushort)uVar30;
                  uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar31);
                }
                auVar34[0] = -(*pcVar24 == -1);
                auVar34[1] = -(pcVar24[1] == -1);
                auVar34[2] = -(pcVar24[2] == -1);
                auVar34[3] = -(pcVar24[3] == -1);
                auVar34[4] = -(pcVar24[4] == -1);
                auVar34[5] = -(pcVar24[5] == -1);
                auVar34[6] = -(pcVar24[6] == -1);
                auVar34[7] = -(pcVar24[7] == -1);
                auVar34[8] = -(pcVar24[8] == -1);
                auVar34[9] = -(pcVar24[9] == -1);
                auVar34[10] = -(pcVar24[10] == -1);
                auVar34[0xb] = -(pcVar24[0xb] == -1);
                auVar34[0xc] = -(pcVar24[0xc] == -1);
                auVar34[0xd] = -(pcVar24[0xd] == -1);
                auVar34[0xe] = -(pcVar24[0xe] == -1);
                auVar34[0xf] = -(pcVar24[0xf] == -1);
                if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar34 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar34 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar34 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar34 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar34 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar34 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar34 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x5f,0) & 1) != 0) || (SUB161(auVar34 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
                lVar27 = uVar25 + lVar29;
                lVar29 = lVar29 + 0x10;
                uVar25 = lVar27 + 0x10U & uVar26;
              }
            }
            puVar2 = (uint *)(param_2 + 0x90);
            uVar30 = *(uint *)(param_2 + 0x90);
            *(uint **)(&stack0x00000fb8 + lVar17) = puVar2;
            if (uVar30 < 0x3ffffffe) {
              LOCK();
              uVar7 = *puVar2;
              if (uVar30 == uVar7) {
                *puVar2 = uVar30 + 1;
              }
              UNLOCK();
              if (uVar30 != uVar7) goto code_r0x0001408b14cc;
            }
            else {
code_r0x0001408b14cc:
              *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b14d1;
              FUN_1428d8a30();
            }
            if (*(longlong *)(param_2 + 0xf0) != 0) {
              lVar18 = *(longlong *)(param_2 + 0xd8);
              uVar26 = *(ulonglong *)(param_2 + 0xe0);
              uVar25 = uVar26 & 0x8b297ec5ae15f15c;
              lVar29 = 0;
              do {
                auVar36 = *(undefined1 (*) [16])(lVar18 + uVar25);
                auVar37[0] = -(auVar36[0] == UNK_1429505f0);
                auVar37[1] = -(auVar36[1] == UNK_1429505f0._1_1_);
                auVar37[2] = -(auVar36[2] == UNK_1429505f0._2_1_);
                auVar37[3] = -(auVar36[3] == UNK_1429505f0._3_1_);
                auVar37[4] = -(auVar36[4] == UNK_1429505f0._4_1_);
                auVar37[5] = -(auVar36[5] == UNK_1429505f0._5_1_);
                auVar37[6] = -(auVar36[6] == UNK_1429505f0._6_1_);
                auVar37[7] = -(auVar36[7] == UNK_1429505f0._7_1_);
                auVar37[8] = -(auVar36[8] == UNK_1429505f0._8_1_);
                auVar37[9] = -(auVar36[9] == UNK_1429505f0._9_1_);
                auVar37[10] = -(auVar36[10] == UNK_1429505f0._10_1_);
                auVar37[0xb] = -(auVar36[0xb] == UNK_1429505f0._11_1_);
                auVar37[0xc] = -(auVar36[0xc] == UNK_1429505f0._12_1_);
                auVar37[0xd] = -(auVar36[0xd] == UNK_1429505f0._13_1_);
                auVar37[0xe] = -(auVar36[0xe] == UNK_1429505f0._14_1_);
                auVar37[0xf] = -(auVar36[0xf] == UNK_1429505f0._15_1_);
                uVar31 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar37[0xf] >> 7) << 0xf;
                uVar30 = (uint)uVar31;
                if (uVar31 != 0) {
code_r0x0001408b08fd:
                  uVar7 = 0;
                  for (uVar15 = uVar30; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                    uVar7 = uVar7 + 1;
                  }
                  lVar27 = (uVar7 + uVar25 & uVar26) * -0x88;
                  if ((*(longlong *)(lVar18 + -0x88 + lVar27) != -0x7a6811f8d84b9e4d) ||
                     (*(longlong *)(lVar18 + -0x80 + lVar27) != -0x74d6813a51ea0ea4))
                  goto code_r0x0001408b08f0;
                  uVar26 = *(ulonglong *)(lVar18 + lVar27 + -8);
                  piVar13 = *(int **)(&stack0x00000fb8 + lVar17);
                  LOCK();
                  iVar8 = *piVar13;
                  *piVar13 = *piVar13 + -1;
                  UNLOCK();
                  if (SBORROW4(0,iVar8 - 1U & 0xbfffffff)) {
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1509;
                    FUN_1428d8d40();
                  }
code_r0x0001408b0952:
                  if ((*(ulonglong *)(param_2 + 0x218) <= uVar26) ||
                     (uVar26 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar26 * 8),
                     uVar26 == 0)) goto code_r0x0001408b156d;
                  lVar18 = ~uVar26 * 0x60;
                  if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar18) == 0)
                  goto code_r0x0001408b156d;
                  uVar21 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar18 + 0x10);
                  puVar28 = &UNK_142a9ce12;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b09aa;
                  uVar26 = FUN_140937aa0(uVar21,&UNK_142a9ce12,0x15);
                  *(undefined **)(&stack0x00000f20 + lVar17) = puVar28;
                  if ((uVar26 & 1) == 0) {
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1587;
                    FUN_1428d9310(&UNK_142a9cec8);
                    goto code_r0x0001408b1587;
                  }
                  uVar26 = 0xd387f1aea6230aea;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b09d5;
                  lVar19 = FUN_1400dd6f0(lVar19,0xd387f1aea6230aea,0xd95b8da689a4c7a3);
                  if (((lVar19 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar26)) ||
                     (uVar26 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar26 * 8),
                     uVar26 == 0)) {
code_r0x0001408b1542:
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b154e;
                    FUN_1428d9310(&UNK_142a9cee0);
                    goto code_r0x0001408b1587;
                  }
                  lVar19 = ~uVar26 * 0x60;
                  if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar19) == 0)
                  goto code_r0x0001408b1542;
                  lVar18 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar19 + 0x10);
                  lVar29 = *plVar22;
                  lVar19 = lVar29 + 0x10;
                  *(longlong *)(&stack0x00000f80 + lVar17) = lVar29;
                  LOCK();
                  bVar33 = *(char *)(lVar29 + 0x10) == '\0';
                  if (bVar33) {
                    *(char *)(lVar29 + 0x10) = '\x01';
                  }
                  UNLOCK();
                  if (!bVar33) {
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b14e4;
                    FUN_1428cd210(lVar19,lVar29,1000000000);
                  }
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0a4a;
                  FUN_140c81c70(lVar19);
                  *(longlong *)(&stack0x00000f70 + lVar17) = lVar19;
                  lVar19 = *(longlong *)(&stack0x00000fb0 + lVar17);
                  uVar21 = *(undefined8 *)(lVar19 + 0x10);
                  *(undefined8 *)(&stack0x00000cb0 + lVar17) = *(undefined8 *)(lVar19 + 8);
                  *(undefined8 *)(&stack0x00000cb8 + lVar17) = uVar21;
                  *(undefined1 **)(&stack0x00000da8 + lVar17) = &stack0x00000cb0 + lVar17;
                  *(undefined **)(&stack0x00000db0 + lVar17) = &UNK_140109e90;
                  *(undefined **)(&stack0x000007d0 + lVar17) = &UNK_142ae6ca8;
                  *(undefined8 *)(&stack0x000007d8 + lVar17) = 1;
                  puVar3 = &stack0x00000da8 + lVar17;
                  *(undefined1 **)(&stack0x000007e0 + lVar17) = puVar3;
                  *(undefined8 *)(&stack0x000007e8 + lVar17) = 1;
                  *(undefined8 *)(&stack0x000007f0 + lVar17) = 0;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0ace;
                  FUN_140d79f90((longlong)auStackX_8 + lVar17,&stack0x000007d0 + lVar17);
                  uVar9 = *(undefined4 *)(lVar19 + 0x18);
                  *(undefined8 *)(&stack0x00000fa0 + lVar17) =
                       *(undefined8 *)((longlong)auStackX_8 + lVar17);
                  *(undefined8 *)(&stack0x00000f90 + lVar17) =
                       *(undefined8 *)((longlong)auStackX_8 + lVar17 + 8);
                  uVar21 = *(undefined8 *)((longlong)auStackX_8 + lVar17 + 0x10);
                  *(undefined8 *)(&stack0x00000fb8 + lVar17) = *(undefined8 *)(lVar18 + 8);
                  lVar19 = *(longlong *)(lVar18 + 0x10);
                  (&stack0x00000fc7)[lVar17] = 1;
                  *(longlong *)(&stack0x00000f28 + lVar17) = param_3 + 4;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0b13;
                  uVar20 = func_0x000140bd5790();
                  *(undefined8 *)(&stack0x000007d0 + lVar17) = _UNK_142a9cc30;
                  *(undefined8 *)(&stack0x000007d8 + lVar17) = _UNK_142a9cc38;
                  *(undefined4 *)(&stack0x000007e0 + lVar17) = 0x3f800000;
                  *(longlong *)(&stack0x000007e8 + lVar17) = param_3;
                  *(undefined4 *)(&stack0x00000da8 + lVar17) = 2;
                  (&stack0x00000fc7)[lVar17] = 1;
                  *(undefined8 *)((longlong)auStackX_8 + lVar17 + -8) = param_1;
                  *(longlong *)(&stack0xfffffffffffffff8 + lVar17) = param_2;
                  *(undefined8 *)(&stack0xfffffffffffffff0 + lVar17) = uVar20;
                  *(undefined1 **)(&stack0xffffffffffffffe8 + lVar17) = puVar3;
                  *(undefined8 *)(&stack0xffffffffffffffe0 + lVar17) = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0b7f;
                  FUN_140867970((longlong)auStackX_8 + lVar17,&UNK_142a9cef8,0xd,
                                &stack0x000007d0 + lVar17);
                  puVar5 = (undefined8 *)
                           (*(longlong *)(&stack0x00000fb0 + lVar17) + 8 +
                           (ulonglong)*(byte *)(*(longlong *)(&stack0x00000fb0 + lVar17) + 0xc0) *
                           0x40);
                  uVar20 = puVar5[1];
                  *(undefined8 *)(&stack0x00000cb0 + lVar17) = *puVar5;
                  *(undefined8 *)(&stack0x00000cb8 + lVar17) = uVar20;
                  *(undefined1 **)(&stack0x00000da8 + lVar17) = &stack0x00000cb0 + lVar17;
                  *(undefined **)(&stack0x00000db0 + lVar17) = &UNK_140109e90;
                  *(undefined **)(&stack0x000007d0 + lVar17) = &UNK_142ae6ca8;
                  *(undefined8 *)(&stack0x000007d8 + lVar17) = 1;
                  *(undefined1 **)(&stack0x000007e0 + lVar17) = puVar3;
                  *(undefined8 *)(&stack0x000007e8 + lVar17) = 1;
                  *(undefined8 *)(&stack0x000007f0 + lVar17) = 0;
                  (&stack0x00000fc3)[lVar17] = 1;
                  (&stack0x00000fc2)[lVar17] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0c11;
                  FUN_140d79f90(&stack0x00000f48 + lVar17,&stack0x000007d0 + lVar17);
                  uVar10 = *(undefined4 *)(*(longlong *)(&stack0x00000f80 + lVar17) + 0x3c);
                  *(longlong *)(&stack0x00000da8 + lVar17) =
                       *(longlong *)(&stack0x00000f80 + lVar17) + 0x14;
                  *(undefined4 *)(&stack0x00000db0 + lVar17) = 0;
                  *(undefined4 *)(&stack0x00000db4 + lVar17) = uVar10;
                  (&stack0x00000fc6)[lVar17] = 1;
                  (&stack0x00000fc5)[lVar17] = 1;
                  puVar3 = &stack0x000007d0 + lVar17;
                  puVar4 = &stack0x00000da8 + lVar17;
                  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0c64;
                  FUN_141304fb0(puVar3,puVar4,&UNK_142ab2438);
                  uVar10 = _UNK_142925904;
                  lVar19 = *(longlong *)(&stack0x00000fb8 + lVar17) + lVar19 * 0x28;
                  puVar23 = *(undefined4 **)(&stack0x000007d8 + lVar17);
                  *(undefined1 (*) [16])(&stack0x00000ef0 + lVar17) = (undefined1  [16])0x0;
                  *(undefined1 (*) [16])(&stack0x00000f00 + lVar17) = (undefined1  [16])0x0;
                  lVar18 = *(longlong *)(&stack0x00000fa0 + lVar17);
                  *(longlong *)(&stack0x00000ea0 + lVar17) = lVar18;
                  *(undefined8 *)(&stack0x00000ea8 + lVar17) =
                       *(undefined8 *)(&stack0x00000f90 + lVar17);
                  *(undefined8 *)(&stack0x00000eb0 + lVar17) = uVar21;
                  *(undefined4 *)(&stack0x00000eb8 + lVar17) = uVar9;
                  *(longlong *)(&stack0x00000ec0 + lVar17) =
                       *(longlong *)(&stack0x00000fb8 + lVar17);
                  *(longlong *)(&stack0x00000f30 + lVar17) = lVar19;
                  *(longlong *)(&stack0x00000ec8 + lVar17) = lVar19;
                  *(undefined4 **)(&stack0x00000ed0 + lVar17) = puVar23;
                  *(undefined8 *)(&stack0x00000f38 + lVar17) =
                       *(undefined8 *)(&stack0x000007d0 + lVar17);
                  *(undefined8 *)(&stack0x00000ee0 + lVar17) =
                       *(undefined8 *)(&stack0x000007d0 + lVar17);
                  *(undefined4 **)(&stack0x00000f18 + lVar17) =
                       puVar23 + *(longlong *)(&stack0x000007e0 + lVar17) * 2;
                  *(undefined4 **)(&stack0x00000ee8 + lVar17) =
                       puVar23 + *(longlong *)(&stack0x000007e0 + lVar17) * 2;
                  *(longlong *)(&stack0x00000f88 + lVar17) = lVar18;
                  lVar19 = 0;
                  *(undefined4 **)(&stack0x00000f40 + lVar17) = puVar23;
                  do {
                    if (puVar23 == *(undefined4 **)(&stack0x00000f18 + lVar17)) {
code_r0x0001408b13b4:
                      if (*(longlong *)(&stack0x00000f38 + lVar17) != 0) {
                        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b13d6;
                        func_0x000140613c20(*(undefined8 *)(&stack0x00000f40 + lVar17),
                                            *(longlong *)(&stack0x00000f38 + lVar17) << 3,4);
                      }
                      lVar18 = *(longlong *)(&stack0x00000f88 + lVar17);
                      if ((-0x7fffffffffffffff < lVar18) && (lVar18 != 0)) {
                        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1403;
                        func_0x000140613c20(*(undefined8 *)(&stack0x00000f90 + lVar17),lVar18,1);
                      }
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b141c;
                      func_0x000142923c00(&stack0x000007d0 + lVar17,(longlong)auStackX_8 + lVar17,
                                          1000);
                      (&stack0x00000fc6)[lVar17] = 0;
                      (&stack0x00000fc5)[lVar17] = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1432;
                      FUN_140868370(&stack0x000007d0 + lVar17);
                      if (*(longlong *)(&stack0x00000f48 + lVar17) != 0) {
                        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1450;
                        func_0x000140613c20(*(undefined8 *)(&stack0x00000f50 + lVar17),
                                            *(longlong *)(&stack0x00000f48 + lVar17),1);
                      }
                      pcVar24 = *(char **)(&stack0x00000f70 + lVar17);
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b145f;
                      FUN_140c81e10();
                      uVar26 = CONCAT71((int7)((ulonglong)lVar19 >> 8),1);
                      LOCK();
                      cVar6 = *pcVar24;
                      if (cVar6 == '\x01') {
                        *pcVar24 = '\0';
                      }
                      UNLOCK();
                      if (cVar6 != '\x01') {
                        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b14f3;
                        FUN_1428cdb90(pcVar24,0);
                      }
                      pcVar24 = *(char **)(&stack0x00000f68 + lVar17);
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b147b;
                      FUN_140c81e10();
                      LOCK();
                      cVar6 = *pcVar24;
                      if (cVar6 == '\x01') {
                        *pcVar24 = '\0';
                      }
                      UNLOCK();
                      if (cVar6 == '\x01') goto code_r0x0001408b1485;
                      goto code_r0x0001408b14fb;
                    }
                    puVar1 = puVar23 + 2;
                    *(undefined4 **)(&stack0x00000ed8 + lVar17) = puVar1;
                    uVar11 = *puVar23;
                    uVar9 = puVar23[1];
                    if (lVar18 == -0x7fffffffffffffff) {
code_r0x0001408b0d7c:
                      lVar18 = *(longlong *)(&stack0x00000fb8 + lVar17);
                      if ((lVar18 != 0) && (lVar18 != *(longlong *)(&stack0x00000f30 + lVar17))) {
                        lVar29 = lVar18 + 0x28;
                        *(longlong *)(&stack0x00000ec0 + lVar17) = lVar29;
                        uVar21 = *(undefined8 *)(lVar18 + 0x10);
                        *(undefined8 *)(&stack0x000006d8 + lVar17) = *(undefined8 *)(lVar18 + 8);
                        *(undefined8 *)(&stack0x000006e0 + lVar17) = uVar21;
                        *(undefined1 **)(&stack0x00000cb0 + lVar17) = &stack0x000006d8 + lVar17;
                        *(undefined **)(&stack0x00000cb8 + lVar17) = &UNK_140109e90;
                        *(undefined **)(&stack0x000007d0 + lVar17) = &UNK_142ae6ca8;
                        *(undefined8 *)(&stack0x000007d8 + lVar17) = 1;
                        *(undefined1 **)(&stack0x000007e0 + lVar17) = &stack0x00000cb0 + lVar17;
                        *(undefined8 *)(&stack0x000007e8 + lVar17) = 1;
                        *(undefined8 *)(&stack0x000007f0 + lVar17) = 0;
                        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0e15;
                        FUN_140d79f90(puVar4,puVar3);
                        lVar18 = *(longlong *)(&stack0x00000da8 + lVar17);
                        if (!SBORROW8(0,lVar18)) {
                          *(undefined8 *)(&stack0x00000f60 + lVar17) = 0x8000000000000001;
                          uVar12 = uVar10;
                          goto code_r0x0001408b0e37;
                        }
                      }
                      goto code_r0x0001408b13b4;
                    }
                    *(undefined8 *)(&stack0x00000ea0 + lVar17) = 0x8000000000000000;
                    if (SBORROW8(0,lVar18)) {
                      *(undefined8 *)(&stack0x00000ea0 + lVar17) = 0x8000000000000001;
                      *(undefined8 *)(&stack0x00000f88 + lVar17) = 0x8000000000000001;
                      goto code_r0x0001408b0d7c;
                    }
                    *(undefined8 *)(&stack0x00000dc0 + lVar17) =
                         *(undefined8 *)(&stack0x00000eb8 + lVar17);
                    *(undefined8 *)(&stack0x00000db0 + lVar17) =
                         *(undefined8 *)(&stack0x00000ea8 + lVar17);
                    *(undefined8 *)(&stack0x00000db8 + lVar17) =
                         *(undefined8 *)(&stack0x00000eb0 + lVar17);
                    uVar12 = *(undefined4 *)(&stack0x00000dc0 + lVar17);
                    *(undefined8 *)(&stack0x00000f88 + lVar17) = 0x8000000000000000;
                    *(undefined8 *)(&stack0x00000f60 + lVar17) = 0x8000000000000000;
                    lVar29 = *(longlong *)(&stack0x00000fb8 + lVar17);
code_r0x0001408b0e37:
                    lVar27 = *(longlong *)(&stack0x00000db8 + lVar17);
                    lVar19 = lVar19 + 1;
                    *(longlong *)(&stack0x00000f08 + lVar17) = lVar19;
                    *(longlong *)(&stack0x00000fb8 + lVar17) = lVar29;
                    *(longlong *)(&stack0x00000f78 + lVar17) = lVar18;
                    *(undefined8 *)(&stack0x00000fb0 + lVar17) =
                         *(undefined8 *)(&stack0x00000db0 + lVar17);
                    if (*(longlong *)(&stack0x00000f58 + lVar17) == lVar27) {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0e7d;
                      uVar21 = func_0x000142923bf0(*(undefined8 *)(&stack0x00000f50 + lVar17),
                                                   *(undefined8 *)(&stack0x00000db0 + lVar17),lVar27
                                                  );
                      *(int *)(&stack0x00000fac + lVar17) =
                           (int)CONCAT71((int7)((ulonglong)uVar21 >> 8),(int)uVar21 == 0);
                    }
                    else {
                      *(undefined4 *)(&stack0x00000fac + lVar17) = 0;
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0e9f;
                    uVar16 = func_0x000140be6750();
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0ea8;
                    plVar22 = (longlong *)FUN_1400d9b20(0);
                    if (plVar22 == (longlong *)0x0) goto code_r0x0001408b1526;
                    *(undefined4 *)(&stack0x00000e50 + lVar17) = uVar16;
                    *(undefined2 *)(&stack0x00000e58 + lVar17) = 0;
                    *(undefined4 *)(&stack0x00000e54 + lVar17) = 0x3f800000;
                    *(undefined8 *)(&stack0x00000da8 + lVar17) = 0;
                    *(undefined8 *)(&stack0x00000e48 + lVar17) =
                         *(undefined8 *)(&stack0x00000f20 + lVar17);
                    *(undefined8 *)(&stack0x00000db8 + lVar17) = 0;
                    *(undefined8 *)(&stack0x00000dc0 + lVar17) = 4;
                    *(undefined8 *)(&stack0x00000dc8 + lVar17) = 0;
                    lVar18 = *plVar22;
                    lVar29 = plVar22[1];
                    *plVar22 = *plVar22 + 1;
                    *(undefined8 *)(&stack0x00000e28 + lVar17) = _UNK_142a96730;
                    *(undefined8 *)(&stack0x00000e30 + lVar17) = _UNK_142a96738;
                    *(undefined8 *)(&stack0x00000e18 + lVar17) = _UNK_142a96720;
                    *(undefined8 *)(&stack0x00000e20 + lVar17) = _UNK_142a96728;
                    *(longlong *)(&stack0x00000e38 + lVar17) = lVar18;
                    *(longlong *)(&stack0x00000e40 + lVar17) = lVar29;
                    *(undefined8 *)(&stack0x00000de8 + lVar17) = 0x100000003;
                    *(undefined8 *)(&stack0x00000dd0 + lVar17) = 0;
                    *(undefined8 *)(&stack0x00000dd8 + lVar17) = 1;
                    *(undefined8 *)(&stack0x00000de0 + lVar17) = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0f76;
                    FUN_140905be0(puVar3,puVar4,(longlong)auStackX_8 + lVar17,0);
                    *(undefined8 *)(&stack0x00000f98 + lVar17) =
                         *(undefined8 *)(&stack0x00000f10 + lVar17);
                    *(undefined4 *)(&stack0x00000da8 + lVar17) = 2;
                    *(undefined1 **)(&stack0xffffffffffffffe0 + lVar17) = puVar4;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0fb2;
                    FUN_1408fd6c0(&stack0x00000f98 + lVar17,puVar3,0x11ba5697cc40b3dd,0x1d);
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0fc3;
                    func_0x000142923c00(puVar4,puVar3,0xf8);
                    *(undefined8 *)(&stack0x00000f98 + lVar17) = 0xa647e80af4e9395f;
                    *(undefined4 *)(&stack0x000007d0 + lVar17) = 2;
                    *(undefined1 **)(&stack0xffffffffffffffe0 + lVar17) = puVar3;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b0ffb;
                    FUN_140908610(&stack0x00000f98 + lVar17,puVar4,0xc0b052b653ac9db6,0x1d);
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b100c;
                    func_0x000142923c00(puVar3,puVar4,0xf8);
                    (&stack0x00000fc4)[lVar17] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1022;
                    uVar21 = FUN_140c9cd80(*(undefined8 *)(&stack0x00000fb0 + lVar17),lVar27);
                    *(undefined8 *)(&stack0x00000f98 + lVar17) = uVar21;
                    *(undefined4 *)(&stack0x00000da8 + lVar17) = 2;
                    *(undefined1 **)(&stack0xffffffffffffffe0 + lVar17) = puVar4;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1054;
                    FUN_140908610(&stack0x00000f98 + lVar17,puVar3,0x422b14c27be4803,0x1d);
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1065;
                    func_0x000142923c00(puVar4,puVar3,0xf8);
                    (&stack0x00000fc1)[lVar17] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1078;
                    func_0x000140bd5790(*(undefined8 *)(&stack0x00000f28 + lVar17));
                    (&stack0x00000fc1)[lVar17] = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b109b;
                    FUN_141629d20(&stack0x00000cb0 + lVar17,puVar4,0x531bb9318eea4971);
                    if ((&stack0x00000d8e)[lVar17] != '\b') {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b10cd;
                      FUN_14096ba30(&stack0x00000d68 + lVar17,
                                    *(longlong *)(&stack0x00000d98 + lVar17) + 0xe8,
                                    0x9c91ae27fdb736fb,uVar12);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b10e6;
                    func_0x000142923c00(&stack0x000006d8 + lVar17,&stack0x00000cb0 + lVar17,0xf8);
                    if ((&stack0x000007b6)[lVar17] != '\b') {
                      *(undefined4 *)(&stack0xffffffffffffffe0 + lVar17) = uVar9;
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b111f;
                      FUN_14096b940(&stack0x00000790 + lVar17,
                                    *(longlong *)(&stack0x000007c0 + lVar17) + 0xe8,
                                    0x1d06ec44e216b191,uVar11);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1138;
                    func_0x000142923c00(&stack0x000003f0 + lVar17,&stack0x000006d8 + lVar17,0xf8);
                    if ((&stack0x000004ce)[lVar17] != '\b') {
                      uVar9 = *(undefined4 *)(*(longlong *)(&stack0x00000f80 + lVar17) + 0x24);
                      *(undefined4 *)(&stack0xffffffffffffffe0 + lVar17) =
                           *(undefined4 *)(*(longlong *)(&stack0x00000f80 + lVar17) + 0x28);
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b117d;
                      FUN_14096b940(&stack0x000004a8 + lVar17,
                                    *(longlong *)(&stack0x000004d8 + lVar17) + 0xe8,
                                    0x7038b90f28755919,uVar9);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1196;
                    func_0x000142923c00(&stack0x000004e8 + lVar17,&stack0x000003f0 + lVar17,0xf8);
                    if ((&stack0x000005c6)[lVar17] != '\b') {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b11d8;
                      FUN_14096b850(&stack0x000005a0 + lVar17,
                                    *(longlong *)(&stack0x000005d0 + lVar17) + 0xe8,
                                    0xfed4ed8f674ddb11);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b11f1;
                    func_0x000142923c00(&stack0x000005e0 + lVar17,&stack0x000004e8 + lVar17,0xf8);
                    if ((&stack0x000006be)[lVar17] != '\b') {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1226;
                      FUN_14096b850(&stack0x00000698 + lVar17,
                                    *(longlong *)(&stack0x000006c8 + lVar17) + 0xe8,
                                    0xfe45681f9b784bd8);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b123f;
                    func_0x000142923c00(&stack0x00000bb8 + lVar17,&stack0x000005e0 + lVar17,0xf8);
                    if ((&stack0x00000c96)[lVar17] != '\b') {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1274;
                      FUN_14096b850(&stack0x00000c70 + lVar17,
                                    *(longlong *)(&stack0x00000ca0 + lVar17) + 0xe8,
                                    0xfa9824c7f6ddb734);
                    }
                    uVar21 = *(undefined8 *)(&stack0x00000bb8 + lVar17);
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1297;
                    func_0x000142923c00(&stack0x000007d8 + lVar17,&stack0x00000bc0 + lVar17,0xa0);
                    *(undefined8 *)(&stack0x0000087c + lVar17) =
                         *(undefined8 *)(&stack0x00000c64 + lVar17);
                    *(undefined8 *)(&stack0x00000884 + lVar17) =
                         *(undefined8 *)(&stack0x00000c6c + lVar17);
                    *(undefined8 *)(&stack0x0000088c + lVar17) =
                         *(undefined8 *)(&stack0x00000c74 + lVar17);
                    *(undefined8 *)(&stack0x00000894 + lVar17) =
                         *(undefined8 *)(&stack0x00000c7c + lVar17);
                    *(undefined8 *)(&stack0x0000089c + lVar17) =
                         *(undefined8 *)(&stack0x00000c84 + lVar17);
                    *(undefined8 *)(&stack0x000008a4 + lVar17) =
                         *(undefined8 *)(&stack0x00000c8c + lVar17);
                    *(undefined8 *)(&stack0x000008ac + lVar17) =
                         *(undefined8 *)(&stack0x00000c94 + lVar17);
                    *(undefined8 *)(&stack0x000008b4 + lVar17) =
                         *(undefined8 *)(&stack0x00000c9c + lVar17);
                    *(undefined8 *)(&stack0x000008b8 + lVar17) =
                         *(undefined8 *)(&stack0x00000ca0 + lVar17);
                    *(undefined8 *)(&stack0x000008c0 + lVar17) =
                         *(undefined8 *)(&stack0x00000ca8 + lVar17);
                    *(undefined8 *)(&stack0x000007d0 + lVar17) = uVar21;
                    (&stack0x00000878)[lVar17] = (&stack0x00000c60)[lVar17] & 0x7f;
                    *(undefined2 *)(&stack0x00000879 + lVar17) =
                         *(undefined2 *)(&stack0x00000c61 + lVar17);
                    (&stack0x0000087b)[lVar17] = (&stack0x00000c63)[lVar17] & 0x9f | 0x40;
                    *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1326;
                    FUN_140906a90(puVar3,6);
                    lVar18 = *(longlong *)(&stack0x00000f60 + lVar17);
                    puVar23 = puVar1;
                    if (*(longlong *)(&stack0x00000f78 + lVar17) != 0) {
                      *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1358;
                      func_0x000140613c20(*(undefined8 *)(&stack0x00000fb0 + lVar17),
                                          *(longlong *)(&stack0x00000f78 + lVar17),1);
                    }
                  } while( true );
                }
code_r0x0001408b08c7:
                auVar35[0] = -(auVar36[0] == -1);
                auVar35[1] = -(auVar36[1] == -1);
                auVar35[2] = -(auVar36[2] == -1);
                auVar35[3] = -(auVar36[3] == -1);
                auVar35[4] = -(auVar36[4] == -1);
                auVar35[5] = -(auVar36[5] == -1);
                auVar35[6] = -(auVar36[6] == -1);
                auVar35[7] = -(auVar36[7] == -1);
                auVar35[8] = -(auVar36[8] == -1);
                auVar35[9] = -(auVar36[9] == -1);
                auVar35[10] = -(auVar36[10] == -1);
                auVar35[0xb] = -(auVar36[0xb] == -1);
                auVar35[0xc] = -(auVar36[0xc] == -1);
                auVar35[0xd] = -(auVar36[0xd] == -1);
                auVar35[0xe] = -(auVar36[0xe] == -1);
                auVar35[0xf] = -(auVar36[0xf] == -1);
                if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar35 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar35 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar35 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar35 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar35 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar35 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar35 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar35 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar35 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar35 >> 0x5f,0) & 1) != 0) || (SUB161(auVar35 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') break;
                lVar27 = uVar25 + lVar29;
                lVar29 = lVar29 + 0x10;
                uVar25 = lVar27 + 0x10U & uVar26;
              } while( true );
            }
            piVar13 = *(int **)(&stack0x00000fb8 + lVar17);
            LOCK();
            iVar8 = *piVar13;
            *piVar13 = *piVar13 + -1;
            UNLOCK();
            if (SBORROW4(0,iVar8 - 1U & 0xbfffffff)) {
              *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b158e;
              FUN_1428d8d40();
            }
code_r0x0001408b156d:
            *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1579;
            FUN_1428d9310(&UNK_142a9ceb0);
            goto code_r0x0001408b1587;
          }
        }
        *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1540;
        FUN_1428d9310(&UNK_142a9ce98);
        goto code_r0x0001408b1587;
      }
    }
  }
  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1526;
  FUN_1428d9310(&UNK_142a9ce80);
code_r0x0001408b1526:
  *(undefined **)((longlong)&puStack_48 + lVar17) = &UNK_1408b1532;
  FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001408b1587:
                    /* WARNING: Does not return */
  pcVar14 = (code *)invalidInstructionException();
  (*pcVar14)();
code_r0x0001408b08f0:
  uVar31 = (ushort)(uVar30 - 1) & (ushort)uVar30;
  uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar31);
  if (uVar31 == 0) goto code_r0x0001408b08c7;
  goto code_r0x0001408b08fd;
}

