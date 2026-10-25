
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140166510(undefined8 *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  undefined1 auVar1 [16];
  undefined1 uVar2;
  undefined4 uVar3;
  undefined8 *puVar4;
  longlong lVar5;
  code *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  longlong lVar18;
  undefined8 uVar19;
  longlong lVar20;
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  longlong lVar24;
  char cVar25;
  longlong lVar26;
  longlong lVar27;
  undefined8 uVar28;
  char *pcVar29;
  longlong *plVar30;
  ulonglong uVar31;
  undefined1 auVar32 [16];
  undefined *puStack_48;
  
  puStack_48 = &UNK_140166526;
  lVar26 = FUN_14289aa40();
  lVar26 = -lVar26;
  *(undefined8 *)(&stack0x00001950 + lVar26) = 0xfffffffffffffffe;
  puVar4 = *(undefined8 **)(&stack0x000019c0 + lVar26);
  lVar5 = *param_2;
  *param_2 = -0x8000000000000000;
  if (SBORROW8(0,lVar5)) {
    (&stack0x00001944)[lVar26] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401671fc;
    FUN_1428d9310(&UNK_14295d5d8);
  }
  else {
    *(longlong *)(&stack0x00001190 + lVar26) = lVar5;
    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166588;
    func_0x000142923c00(&stack0x00001198 + lVar26,param_2 + 1,0x88);
    lVar5 = param_2[0x12];
    param_2[0x12] = 0;
    *(longlong *)(&stack0x00001908 + lVar26) = lVar5;
    if (lVar5 == 0) {
      *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016720a;
      FUN_1428d9310(&UNK_14295d5f0);
    }
    else {
      *(longlong *)(&stack0x000015f8 + lVar26) = param_2[0x1d];
      lVar27 = param_2[0x1c];
      *(longlong *)(&stack0x000015e8 + lVar26) = param_2[0x1b];
      *(longlong *)(&stack0x000015f0 + lVar26) = lVar27;
      lVar27 = param_2[0x13];
      lVar18 = param_2[0x14];
      lVar20 = param_2[0x15];
      uVar3 = *(undefined4 *)((longlong)param_2 + 0xac);
      lVar21 = param_2[0x16];
      uVar7 = *(undefined4 *)((longlong)param_2 + 0xb4);
      lVar22 = param_2[0x17];
      uVar8 = *(undefined4 *)((longlong)param_2 + 0xbc);
      lVar23 = param_2[0x18];
      uVar9 = *(undefined4 *)((longlong)param_2 + 0xc4);
      uVar10 = *(undefined4 *)((longlong)param_2 + 0xcc);
      lVar24 = param_2[0x1a];
      uVar11 = *(undefined4 *)((longlong)param_2 + 0xd4);
      *(int *)(&stack0x000015d8 + lVar26) = (int)param_2[0x19];
      *(undefined4 *)(&stack0x000015dc + lVar26) = uVar10;
      *(int *)(&stack0x000015e0 + lVar26) = (int)lVar24;
      *(undefined4 *)(&stack0x000015e4 + lVar26) = uVar11;
      *(int *)(&stack0x000015c8 + lVar26) = (int)lVar22;
      *(undefined4 *)(&stack0x000015cc + lVar26) = uVar8;
      *(int *)(&stack0x000015d0 + lVar26) = (int)lVar23;
      *(undefined4 *)(&stack0x000015d4 + lVar26) = uVar9;
      *(int *)(&stack0x000015b8 + lVar26) = (int)lVar20;
      *(undefined4 *)(&stack0x000015bc + lVar26) = uVar3;
      *(int *)(&stack0x000015c0 + lVar26) = (int)lVar21;
      *(undefined4 *)(&stack0x000015c4 + lVar26) = uVar7;
      *(longlong *)(&stack0x000015a8 + lVar26) = lVar27;
      *(longlong *)(&stack0x000015b0 + lVar26) = lVar18;
      *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166617;
      func_0x000142923c00(&stack0x000014f0 + lVar26,&stack0x00001190 + lVar26,0x90);
      uVar28 = puVar4[0xb];
      *(undefined8 *)(&stack0x000018f0 + lVar26) = puVar4[10];
      *(undefined8 *)(&stack0x000018f8 + lVar26) = uVar28;
      uVar28 = puVar4[9];
      *(undefined8 *)(&stack0x000018e0 + lVar26) = puVar4[8];
      *(undefined8 *)(&stack0x000018e8 + lVar26) = uVar28;
      uVar28 = *puVar4;
      uVar19 = puVar4[1];
      uVar3 = *(undefined4 *)(puVar4 + 2);
      uVar7 = *(undefined4 *)((longlong)puVar4 + 0x14);
      uVar8 = *(undefined4 *)(puVar4 + 3);
      uVar9 = *(undefined4 *)((longlong)puVar4 + 0x1c);
      uVar10 = *(undefined4 *)(puVar4 + 4);
      uVar11 = *(undefined4 *)((longlong)puVar4 + 0x24);
      uVar12 = *(undefined4 *)(puVar4 + 5);
      uVar13 = *(undefined4 *)((longlong)puVar4 + 0x2c);
      uVar14 = *(undefined4 *)((longlong)puVar4 + 0x34);
      uVar15 = *(undefined4 *)(puVar4 + 7);
      uVar16 = *(undefined4 *)((longlong)puVar4 + 0x3c);
      *(undefined4 *)(&stack0x000018d0 + lVar26) = *(undefined4 *)(puVar4 + 6);
      *(undefined4 *)(&stack0x000018d4 + lVar26) = uVar14;
      *(undefined4 *)(&stack0x000018d8 + lVar26) = uVar15;
      *(undefined4 *)(&stack0x000018dc + lVar26) = uVar16;
      *(undefined4 *)(&stack0x000018c0 + lVar26) = uVar10;
      *(undefined4 *)(&stack0x000018c4 + lVar26) = uVar11;
      *(undefined4 *)(&stack0x000018c8 + lVar26) = uVar12;
      *(undefined4 *)(&stack0x000018cc + lVar26) = uVar13;
      *(undefined4 *)(&stack0x000018b0 + lVar26) = uVar3;
      *(undefined4 *)(&stack0x000018b4 + lVar26) = uVar7;
      *(undefined4 *)(&stack0x000018b8 + lVar26) = uVar8;
      *(undefined4 *)(&stack0x000018bc + lVar26) = uVar9;
      *(undefined8 *)(&stack0x000018a0 + lVar26) = uVar28;
      *(undefined8 *)(&stack0x000018a8 + lVar26) = uVar19;
      *(undefined8 *)(&stack0x00001580 + lVar26) = 0;
      *(undefined8 *)(&stack0x00001588 + lVar26) = 8;
      *(undefined8 *)(&stack0x00001590 + lVar26) = 0;
      *(undefined8 *)(&stack0x00001598 + lVar26) = param_3;
      *(longlong *)(&stack0x000015a0 + lVar26) = lVar5;
      lVar5 = *(longlong *)(&stack0x000018a0 + lVar26);
      (&stack0x0000194d)[lVar26] = 1;
      (&stack0x0000194c)[lVar26] = 1;
      uVar31 = 0xa68561b396784eaa;
      *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401666c0;
      lVar27 = FUN_1400dd6f0(lVar5 + 0x38,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
      if (((lVar27 != 0) && (uVar31 < *(ulonglong *)(lVar5 + 0x218))) &&
         (uVar31 = *(ulonglong *)(*(longlong *)(lVar5 + 0x210) + uVar31 * 8), uVar31 != 0)) {
        lVar27 = ~uVar31 * 0x60;
        if (*(longlong *)(*(longlong *)(lVar5 + 0x1e0) + 0x20 + lVar27) != 0) {
          lVar27 = *(longlong *)(lVar5 + 0x1e0) + lVar27;
          uVar28 = *(undefined8 *)(lVar27 + 0x10);
          uVar3 = *(undefined4 *)(lVar5 + 0x5b0);
          *(undefined4 *)(lVar27 + 0x5c) = uVar3;
          (&stack0x0000194d)[lVar26] = 1;
          (&stack0x0000194c)[lVar26] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016673b;
          FUN_14093cd30(uVar28,&stack0x000014f0 + lVar26);
          *(undefined4 *)(lVar27 + 0x5c) = uVar3;
          (&stack0x0000194d)[lVar26] = 1;
          (&stack0x0000194c)[lVar26] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016675c;
          uVar28 = FUN_14093d040(uVar28,*(undefined8 *)(&stack0x000018a8 + lVar26));
          (&stack0x0000194d)[lVar26] = 1;
          (&stack0x0000194c)[lVar26] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166779;
          func_0x000140bd6b00(&stack0x00001888 + lVar26,uVar28);
          lVar5 = *(longlong *)(&stack0x000018a0 + lVar26);
          (&stack0x0000194f)[lVar26] = 1;
          (&stack0x0000194e)[lVar26] = 1;
          uVar31 = 0x8fd9dc19c7973bca;
          *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401667af;
          lVar27 = FUN_1400dd6f0(lVar5 + 0x38,0x8fd9dc19c7973bca,0x361ff4c5eb9b45fd);
          if (((lVar27 != 0) && (uVar31 < *(ulonglong *)(lVar5 + 0x218))) &&
             (uVar31 = *(ulonglong *)(*(longlong *)(lVar5 + 0x210) + uVar31 * 8), uVar31 != 0)) {
            lVar27 = ~uVar31 * 0x60;
            if (*(longlong *)(*(longlong *)(lVar5 + 0x1e0) + 0x20 + lVar27) != 0) {
              uVar28 = *(undefined8 *)(*(longlong *)(lVar5 + 0x1e0) + lVar27 + 0x10);
              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166813;
              func_0x000142923c00(&stack0x00000100 + lVar26,uVar28,0x1090);
              (&stack0x0000194f)[lVar26] = 1;
              (&stack0x0000194e)[lVar26] = 1;
              uVar31 = 0x47c94cb78283a72e;
              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016683d;
              lVar27 = FUN_1400dd6f0(lVar5 + 0x38,0x47c94cb78283a72e,0x8c173ded97323e9d);
              if (((lVar27 != 0) && (uVar31 < *(ulonglong *)(lVar5 + 0x218))) &&
                 (uVar31 = *(ulonglong *)(*(longlong *)(lVar5 + 0x210) + uVar31 * 8), uVar31 != 0))
              {
                lVar27 = ~uVar31 * 0x60;
                if (*(longlong *)(*(longlong *)(lVar5 + 0x1e0) + 0x20 + lVar27) != 0) {
                  uVar28 = *(undefined8 *)(*(longlong *)(lVar5 + 0x1e0) + lVar27 + 0x10);
                  (&stack0x0000194f)[lVar26] = 1;
                  (&stack0x0000194e)[lVar26] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401668a2;
                  pcVar29 = (char *)func_0x00014067dd80(uVar28);
                  LOCK();
                  cVar25 = *pcVar29;
                  if (cVar25 == '\0') {
                    *pcVar29 = '\x01';
                  }
                  UNLOCK();
                  if (cVar25 != '\0') {
                    (&stack0x0000194f)[lVar26] = 1;
                    (&stack0x0000194e)[lVar26] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016713e;
                    FUN_1428cd210(pcVar29);
                  }
                  (&stack0x0000194f)[lVar26] = 1;
                  (&stack0x0000194e)[lVar26] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401668ca;
                  FUN_140c81c70(pcVar29);
                  *(char **)(&stack0x00001938 + lVar26) = pcVar29;
                  uVar28 = *(undefined8 *)(&stack0x000018a8 + lVar26);
                  *(undefined8 *)(&stack0xffffffffffffffe0 + lVar26) = 0;
                  *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401668fd;
                  FUN_140c02740(&stack0x00001330 + lVar26,pcVar29 + 8,uVar28,0x1090);
                  if (*(longlong *)(&stack0x00001338 + lVar26) == 0x1090) {
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016692a;
                    func_0x000142923c00(*(undefined8 *)(&stack0x00001330 + lVar26),
                                        &stack0x00000100 + lVar26,0x1090);
                    *(undefined8 *)(&stack0x000016f0 + lVar26) =
                         *(undefined8 *)(&stack0x00001330 + lVar26);
                    *(undefined8 *)(&stack0x000016f8 + lVar26) =
                         *(undefined8 *)(&stack0x00001338 + lVar26);
                    *(undefined4 *)(&stack0x00001700 + lVar26) =
                         *(undefined4 *)(&stack0x00001340 + lVar26);
                    *(undefined4 *)(&stack0x00001704 + lVar26) =
                         *(undefined4 *)(&stack0x00001344 + lVar26);
                    *(undefined4 *)(&stack0x00001708 + lVar26) =
                         *(undefined4 *)(&stack0x00001348 + lVar26);
                    *(undefined4 *)(&stack0x0000170c + lVar26) =
                         *(undefined4 *)(&stack0x0000134c + lVar26);
                    *(undefined4 *)(&stack0x00001710 + lVar26) =
                         *(undefined4 *)(&stack0x00001350 + lVar26);
                    *(undefined4 *)(&stack0x00001714 + lVar26) =
                         *(undefined4 *)(&stack0x00001354 + lVar26);
                    *(undefined4 *)(&stack0x00001718 + lVar26) =
                         *(undefined4 *)(&stack0x00001358 + lVar26);
                    *(undefined4 *)(&stack0x0000171c + lVar26) =
                         *(undefined4 *)(&stack0x0000135c + lVar26);
                    *(undefined4 *)(&stack0x00001720 + lVar26) =
                         *(undefined4 *)(&stack0x00001360 + lVar26);
                    *(undefined4 *)(&stack0x00001724 + lVar26) =
                         *(undefined4 *)(&stack0x00001364 + lVar26);
                    *(undefined4 *)(&stack0x00001728 + lVar26) =
                         *(undefined4 *)(&stack0x00001368 + lVar26);
                    *(undefined4 *)(&stack0x0000172c + lVar26) =
                         *(undefined4 *)(&stack0x0000136c + lVar26);
                    *(undefined8 *)(&stack0x00001730 + lVar26) =
                         *(undefined8 *)(&stack0x00001370 + lVar26);
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166986;
                    FUN_140c02370(&stack0x00001600 + lVar26,&stack0x000016f0 + lVar26,uVar28);
                    (&stack0x00001946)[lVar26] = 1;
                    (&stack0x00001945)[lVar26] = 1;
                    pcVar29 = *(char **)(&stack0x00001938 + lVar26);
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401669a3;
                    FUN_140c81e10();
                    LOCK();
                    cVar25 = *pcVar29;
                    if (cVar25 == '\x01') {
                      *pcVar29 = '\0';
                    }
                    UNLOCK();
                    if (cVar25 != '\x01') {
                      (&stack0x00001946)[lVar26] = 1;
                      (&stack0x00001945)[lVar26] = 1;
                      *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167171;
                      FUN_1428cdb90(pcVar29,0);
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401669ce;
                    func_0x000142923c00(&stack0x000016f0 + lVar26,&stack0x000014f0 + lVar26,0x110);
                    *(undefined8 *)(&stack0x00001800 + lVar26) =
                         *(undefined8 *)(&stack0x00001600 + lVar26);
                    *(undefined8 *)(&stack0x00001808 + lVar26) =
                         *(undefined8 *)(&stack0x00001608 + lVar26);
                    *(undefined8 *)(&stack0x00001810 + lVar26) =
                         *(undefined8 *)(&stack0x00001610 + lVar26);
                    *(undefined8 *)(&stack0x00001818 + lVar26) =
                         *(undefined8 *)(&stack0x00001618 + lVar26);
                    *(undefined8 *)(&stack0x00001820 + lVar26) =
                         *(undefined8 *)(&stack0x00001620 + lVar26);
                    *(undefined8 *)(&stack0x00001828 + lVar26) =
                         *(undefined8 *)(&stack0x00001628 + lVar26);
                    *(undefined4 *)(&stack0x00001830 + lVar26) =
                         *(undefined4 *)(&stack0x00001630 + lVar26);
                    *(undefined4 *)(&stack0x00001834 + lVar26) =
                         *(undefined4 *)(&stack0x00001634 + lVar26);
                    *(undefined4 *)(&stack0x00001838 + lVar26) =
                         *(undefined4 *)(&stack0x00001638 + lVar26);
                    *(undefined4 *)(&stack0x0000183c + lVar26) =
                         *(undefined4 *)(&stack0x0000163c + lVar26);
                    *(undefined8 *)(&stack0x00001840 + lVar26) =
                         *(undefined8 *)(&stack0x00001640 + lVar26);
                    *(undefined8 *)(&stack0x00001848 + lVar26) =
                         *(undefined8 *)(&stack0x00001648 + lVar26);
                    *(undefined8 *)(&stack0x00001850 + lVar26) =
                         *(undefined8 *)(&stack0x00001650 + lVar26);
                    *(undefined8 *)(&stack0x00001858 + lVar26) =
                         *(undefined8 *)(&stack0x00001658 + lVar26);
                    *(undefined8 *)(&stack0x00001860 + lVar26) =
                         *(undefined8 *)(&stack0x00001888 + lVar26);
                    *(undefined8 *)(&stack0x00001868 + lVar26) =
                         *(undefined8 *)(&stack0x00001890 + lVar26);
                    *(undefined8 *)(&stack0x00001870 + lVar26) =
                         *(undefined8 *)(&stack0x00001898 + lVar26);
                    auVar1 = *(undefined1 (*) [16])(&stack0x000018a8 + lVar26);
                    auVar32._0_8_ = auVar1._8_8_;
                    auVar32._8_4_ = auVar1._0_4_;
                    auVar32._12_4_ = auVar1._4_4_;
                    *(undefined1 (*) [16])(&stack0x00001878 + lVar26) = auVar32;
                    *(undefined8 *)(&stack0x00001360 + lVar26) =
                         *(undefined8 *)(&stack0x000018f8 + lVar26);
                    *(undefined4 *)(&stack0x00001350 + lVar26) =
                         *(undefined4 *)(&stack0x000018e8 + lVar26);
                    *(undefined4 *)(&stack0x00001354 + lVar26) =
                         *(undefined4 *)(&stack0x000018ec + lVar26);
                    *(undefined4 *)(&stack0x00001358 + lVar26) =
                         *(undefined4 *)(&stack0x000018f0 + lVar26);
                    *(undefined4 *)(&stack0x0000135c + lVar26) =
                         *(undefined4 *)(&stack0x000018f4 + lVar26);
                    *(undefined4 *)(&stack0x00001340 + lVar26) =
                         *(undefined4 *)(&stack0x000018d8 + lVar26);
                    *(undefined4 *)(&stack0x00001344 + lVar26) =
                         *(undefined4 *)(&stack0x000018dc + lVar26);
                    *(undefined4 *)(&stack0x00001348 + lVar26) =
                         *(undefined4 *)(&stack0x000018e0 + lVar26);
                    *(undefined4 *)(&stack0x0000134c + lVar26) =
                         *(undefined4 *)(&stack0x000018e4 + lVar26);
                    *(undefined8 *)(&stack0x00001330 + lVar26) =
                         *(undefined8 *)(&stack0x000018c8 + lVar26);
                    *(undefined8 *)(&stack0x00001338 + lVar26) =
                         *(undefined8 *)(&stack0x000018d0 + lVar26);
                    (&stack0x0000194b)[lVar26] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166aab;
                    FUN_140bd6220(&stack0x000014cc + lVar26,&stack0x00001330 + lVar26,
                                  &stack0x000016f0 + lVar26,0x22);
                    (&stack0x0000194b)[lVar26] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166acf;
                    FUN_140bd60a0(&stack0x00001910 + lVar26,
                                  *(undefined8 *)(&stack0x000018b8 + lVar26),
                                  &stack0x000016f0 + lVar26,0);
                    lVar5 = *(longlong *)(&stack0x000018a0 + lVar26);
                    uVar2 = *(undefined1 *)(*(longlong *)(&stack0x000018c0 + lVar26) + 0xc2);
                    (&stack0x0000194b)[lVar26] = 1;
                    (&stack0xffffffffffffffe0)[lVar26] = uVar2;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166b0c;
                    FUN_14090d850(&stack0x000016f0 + lVar26,lVar5,&stack0x000014cc + lVar26,
                                  &stack0x00001910 + lVar26);
                    *(undefined4 *)(&stack0x00001350 + lVar26) =
                         *(undefined4 *)(&stack0x00001930 + lVar26);
                    *(undefined4 *)(&stack0x00001340 + lVar26) =
                         *(undefined4 *)(&stack0x00001920 + lVar26);
                    *(undefined4 *)(&stack0x00001344 + lVar26) =
                         *(undefined4 *)(&stack0x00001924 + lVar26);
                    *(undefined4 *)(&stack0x00001348 + lVar26) =
                         *(undefined4 *)(&stack0x00001928 + lVar26);
                    *(undefined4 *)(&stack0x0000134c + lVar26) =
                         *(undefined4 *)(&stack0x0000192c + lVar26);
                    *(undefined8 *)(&stack0x00001330 + lVar26) =
                         *(undefined8 *)(&stack0x00001910 + lVar26);
                    *(undefined8 *)(&stack0x00001338 + lVar26) =
                         *(undefined8 *)(&stack0x00001918 + lVar26);
                    (&stack0x0000194b)[lVar26] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166b51;
                    FUN_140bd6380(&stack0x00001330 + lVar26,&stack0x000016f0 + lVar26,0x27);
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166b6d;
                    func_0x000142923c00(&stack0x00001330 + lVar26,&stack0x000016f0 + lVar26,0x198);
                    (&stack0x0000194b)[lVar26] = 0;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166b83;
                    FUN_1409dda50(&stack0x00001220 + lVar26,&stack0x00001330 + lVar26);
                    uVar31 = 0xa68561b396784eaa;
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166ba0;
                    lVar27 = FUN_1400dd6f0(lVar5 + 0x38,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
                    if (((lVar27 != 0) && (uVar31 < *(ulonglong *)(lVar5 + 0x218))) &&
                       (uVar31 = *(ulonglong *)(*(longlong *)(lVar5 + 0x210) + uVar31 * 8),
                       uVar31 != 0)) {
                      lVar27 = ~uVar31 * 0x60;
                      if (*(longlong *)(*(longlong *)(lVar5 + 0x1e0) + 0x20 + lVar27) != 0) {
                        lVar27 = *(longlong *)(lVar5 + 0x1e0) + lVar27;
                        uVar28 = *(undefined8 *)(lVar27 + 0x10);
                        *(undefined4 *)(lVar27 + 0x5c) = *(undefined4 *)(lVar5 + 0x5b0);
                        *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166c05;
                        FUN_14093ce30(uVar28,&stack0x00001220 + lVar26);
                        lVar5 = *(longlong *)(&stack0x000018a0 + lVar26);
                        uVar31 = 0x8226cc786e679355;
                        *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166c29;
                        lVar27 = FUN_1400dd6f0(lVar5 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
                        if (((lVar27 != 0) && (uVar31 < *(ulonglong *)(lVar5 + 0x218))) &&
                           (uVar31 = *(ulonglong *)(*(longlong *)(lVar5 + 0x210) + uVar31 * 8),
                           uVar31 != 0)) {
                          lVar27 = ~uVar31 * 0x60;
                          if (*(longlong *)(*(longlong *)(lVar5 + 0x1e0) + 0x20 + lVar27) != 0) {
                            lVar27 = *(longlong *)(lVar5 + 0x1e0) + lVar27;
                            uVar28 = *(undefined8 *)(lVar27 + 0x10);
                            *(undefined4 *)(lVar27 + 0x5c) = *(undefined4 *)(lVar5 + 0x5b0);
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166c8e;
                            FUN_14091ebb0(uVar28,&stack0x00001220 + lVar26);
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166ca7;
                            func_0x000142923c00(&stack0xfffffffffffffff0 + lVar26,
                                                &stack0x00001220 + lVar26,0x110);
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166cb6;
                            FUN_1404f7ca0(&stack0x00000100 + lVar26,
                                          &stack0xfffffffffffffff0 + lVar26);
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166cd6;
                            func_0x000142923c00(&stack0x00001220 + lVar26,&stack0x00000100 + lVar26,
                                                0xa8);
                            *(undefined8 *)(&stack0x00001690 + lVar26) =
                                 *(undefined8 *)(&stack0x000001a8 + lVar26);
                            *(undefined8 *)(&stack0x00001698 + lVar26) =
                                 *(undefined8 *)(&stack0x000001b0 + lVar26);
                            *(undefined8 *)(&stack0x000016a0 + lVar26) =
                                 *(undefined8 *)(&stack0x000001b8 + lVar26);
                            *(undefined8 *)(&stack0x000016a8 + lVar26) =
                                 *(undefined8 *)(&stack0x000001c0 + lVar26);
                            *(undefined8 *)(&stack0x000016b0 + lVar26) =
                                 *(undefined8 *)(&stack0x000001c8 + lVar26);
                            *(undefined8 *)(&stack0x000016b8 + lVar26) =
                                 *(undefined8 *)(&stack0x000001d0 + lVar26);
                            *(undefined8 *)(&stack0x000016c0 + lVar26) =
                                 *(undefined8 *)(&stack0x000001d8 + lVar26);
                            *(undefined8 *)(&stack0x000016c8 + lVar26) =
                                 *(undefined8 *)(&stack0x000001e0 + lVar26);
                            *(undefined8 *)(&stack0x000016d0 + lVar26) =
                                 *(undefined8 *)(&stack0x000001e8 + lVar26);
                            *(undefined8 *)(&stack0x000016d8 + lVar26) =
                                 *(undefined8 *)(&stack0x000001f0 + lVar26);
                            *(undefined8 *)(&stack0x000016e0 + lVar26) =
                                 *(undefined8 *)(&stack0x000001f8 + lVar26);
                            *(undefined8 *)(&stack0x000016e8 + lVar26) =
                                 *(undefined8 *)(&stack0x00000200 + lVar26);
                            lVar5 = param_2[0x1d];
                            *(longlong *)(&stack0x00000150 + lVar26) = param_2[0x1c];
                            *(longlong *)(&stack0x00000158 + lVar26) = lVar5;
                            lVar5 = param_2[0x1b];
                            *(longlong *)(&stack0x00000140 + lVar26) = param_2[0x1a];
                            *(longlong *)(&stack0x00000148 + lVar26) = lVar5;
                            lVar5 = param_2[0x12];
                            lVar27 = param_2[0x13];
                            lVar18 = param_2[0x14];
                            lVar20 = param_2[0x15];
                            lVar21 = param_2[0x16];
                            lVar22 = param_2[0x17];
                            lVar23 = param_2[0x19];
                            *(longlong *)(&stack0x00000130 + lVar26) = param_2[0x18];
                            *(longlong *)(&stack0x00000138 + lVar26) = lVar23;
                            *(longlong *)(&stack0x00000120 + lVar26) = lVar21;
                            *(longlong *)(&stack0x00000128 + lVar26) = lVar22;
                            *(longlong *)(&stack0x00000110 + lVar26) = lVar18;
                            *(longlong *)(&stack0x00000118 + lVar26) = lVar20;
                            *(longlong *)(&stack0x00000100 + lVar26) = lVar5;
                            *(longlong *)(&stack0x00000108 + lVar26) = lVar27;
                            lVar5 = *(longlong *)(&stack0x00001698 + lVar26);
                            uVar3 = *(undefined4 *)(&stack0x000016a0 + lVar26);
                            uVar7 = *(undefined4 *)(&stack0x000016a4 + lVar26);
                            uVar8 = *(undefined4 *)(&stack0x000016a8 + lVar26);
                            uVar9 = *(undefined4 *)(&stack0x000016ac + lVar26);
                            uVar10 = *(undefined4 *)(&stack0x000016b0 + lVar26);
                            uVar11 = *(undefined4 *)(&stack0x000016b4 + lVar26);
                            uVar12 = *(undefined4 *)(&stack0x000016b8 + lVar26);
                            uVar13 = *(undefined4 *)(&stack0x000016bc + lVar26);
                            uVar14 = *(undefined4 *)(&stack0x000016c0 + lVar26);
                            uVar15 = *(undefined4 *)(&stack0x000016c4 + lVar26);
                            uVar16 = *(undefined4 *)(&stack0x000016c8 + lVar26);
                            uVar17 = *(undefined4 *)(&stack0x000016cc + lVar26);
                            param_2[0x12] = *(longlong *)(&stack0x00001690 + lVar26);
                            param_2[0x13] = lVar5;
                            *(undefined4 *)(param_2 + 0x14) = uVar3;
                            *(undefined4 *)((longlong)param_2 + 0xa4) = uVar7;
                            *(undefined4 *)(param_2 + 0x15) = uVar8;
                            *(undefined4 *)((longlong)param_2 + 0xac) = uVar9;
                            *(undefined4 *)(param_2 + 0x16) = uVar10;
                            *(undefined4 *)((longlong)param_2 + 0xb4) = uVar11;
                            *(undefined4 *)(param_2 + 0x17) = uVar12;
                            *(undefined4 *)((longlong)param_2 + 0xbc) = uVar13;
                            *(undefined4 *)(param_2 + 0x18) = uVar14;
                            *(undefined4 *)((longlong)param_2 + 0xc4) = uVar15;
                            *(undefined4 *)(param_2 + 0x19) = uVar16;
                            *(undefined4 *)((longlong)param_2 + 0xcc) = uVar17;
                            lVar5 = *(longlong *)(&stack0x000016d8 + lVar26);
                            param_2[0x1a] = *(longlong *)(&stack0x000016d0 + lVar26);
                            param_2[0x1b] = lVar5;
                            lVar5 = *(longlong *)(&stack0x000016e8 + lVar26);
                            param_2[0x1c] = *(longlong *)(&stack0x000016e0 + lVar26);
                            param_2[0x1d] = lVar5;
                            if (*(longlong *)(&stack0x00000100 + lVar26) != 0) {
                              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166de1;
                              FUN_14004ceb0(&stack0x00000100 + lVar26,&stack0x00000120 + lVar26,0x68
                                            ,0x10);
                              lVar5 = *(longlong *)(&stack0x00000138 + lVar26);
                              if ((lVar5 != 0) && (lVar27 = lVar5 * 0x11 + 0x21, lVar27 != 0)) {
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166e19;
                                func_0x000140613c20(*(longlong *)(&stack0x00000130 + lVar26) +
                                                    lVar5 * -0x10 + -0x10,lVar27,0x10);
                              }
                            }
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166e32;
                            func_0x000142923c00(&stack0x00001600 + lVar26,&stack0x00001220 + lVar26,
                                                0x90);
                            *(undefined8 *)(&stack0x00001920 + lVar26) =
                                 *(undefined8 *)(&stack0x000012c0 + lVar26);
                            *(undefined8 *)(&stack0x00001910 + lVar26) =
                                 *(undefined8 *)(&stack0x000012b0 + lVar26);
                            *(undefined8 *)(&stack0x00001918 + lVar26) =
                                 *(undefined8 *)(&stack0x000012b8 + lVar26);
                            *(undefined8 *)(&stack0x00000100 + lVar26) = 0;
                            *(undefined8 *)(&stack0x00000108 + lVar26) = 8;
                            *(undefined8 *)(&stack0x00000110 + lVar26) = 0;
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166e76;
                            plVar30 = (longlong *)FUN_1400d9b20(0);
                            if (plVar30 != (longlong *)0x0) {
                              lVar5 = *plVar30;
                              lVar27 = plVar30[1];
                              *plVar30 = *plVar30 + 1;
                              *(undefined8 *)(&stack0x000017d8 + lVar26) = _UNK_1429b0030;
                              *(undefined8 *)(&stack0x000017e0 + lVar26) = _UNK_1429b0038;
                              *(undefined8 *)(&stack0x000017c8 + lVar26) = _UNK_1429b0020;
                              *(undefined8 *)(&stack0x000017d0 + lVar26) = _UNK_1429b0028;
                              *(longlong *)(&stack0x000017e8 + lVar26) = lVar5;
                              *(longlong *)(&stack0x000017f0 + lVar26) = lVar27;
                              *(undefined8 *)(&stack0x000016f0 + lVar26) = 0;
                              *(undefined8 *)(&stack0x000016f8 + lVar26) = 2;
                              *(undefined8 *)(&stack0x00001700 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001708 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001710 + lVar26) = 1;
                              *(undefined8 *)(&stack0x00001718 + lVar26) = 0;
                              *(undefined4 *)(&stack0x00001720 + lVar26) =
                                   *(undefined4 *)(&stack0x00000100 + lVar26);
                              *(undefined4 *)(&stack0x00001724 + lVar26) =
                                   *(undefined4 *)(&stack0x00000104 + lVar26);
                              *(undefined4 *)(&stack0x00001728 + lVar26) =
                                   *(undefined4 *)(&stack0x00000108 + lVar26);
                              *(undefined4 *)(&stack0x0000172c + lVar26) =
                                   *(undefined4 *)(&stack0x0000010c + lVar26);
                              *(undefined8 *)(&stack0x00001730 + lVar26) =
                                   *(undefined8 *)(&stack0x00000110 + lVar26);
                              *(undefined8 *)(&stack0x00001738 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001740 + lVar26) = 4;
                              *(undefined8 *)(&stack0x00001748 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001750 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001758 + lVar26) = 4;
                              *(undefined8 *)(&stack0x00001760 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001768 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001770 + lVar26) = 4;
                              *(undefined8 *)(&stack0x00001778 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001780 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001788 + lVar26) = 4;
                              *(undefined8 *)(&stack0x00001790 + lVar26) = 0;
                              *(undefined8 *)(&stack0x00001798 + lVar26) = 0;
                              *(undefined8 *)(&stack0x000017a0 + lVar26) = 4;
                              *(undefined8 *)(&stack0x000017a8 + lVar26) = 0;
                              *(undefined8 *)(&stack0x000017b0 + lVar26) = 0;
                              *(undefined8 *)(&stack0x000017b8 + lVar26) = 4;
                              *(undefined8 *)(&stack0x000017c0 + lVar26) = 0;
                              (&stack0x00001948)[lVar26] = 1;
                              (&stack0x00001947)[lVar26] = 1;
                              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166fa4;
                              FUN_140530430(&stack0x00001738 + lVar26,&stack0x00001600 + lVar26,
                                            *(undefined8 *)(&stack0x00001918 + lVar26),
                                            *(undefined8 *)(&stack0x00001920 + lVar26));
                              (&stack0x00001948)[lVar26] = 1;
                              (&stack0x00001947)[lVar26] = 1;
                              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166fcb;
                              cVar25 = FUN_140be7ba0(&stack0x000016f0 + lVar26,
                                                     &stack0x00001600 + lVar26,param_4,param_3);
                              if (cVar25 == '\0') {
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167097;
                                func_0x000142923c00(&stack0x000014f0 + lVar26,
                                                    &stack0x00001600 + lVar26,0x90);
                                *(undefined8 *)(&stack0x000018b0 + lVar26) =
                                     *(undefined8 *)(&stack0x00001920 + lVar26);
                                *(undefined8 *)(&stack0x000018a0 + lVar26) =
                                     *(undefined8 *)(&stack0x00001910 + lVar26);
                                *(undefined8 *)(&stack0x000018a8 + lVar26) =
                                     *(undefined8 *)(&stack0x00001918 + lVar26);
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401670cf;
                                func_0x000142923c00(&stack0x00001330 + lVar26,
                                                    &stack0x000016f0 + lVar26,0x108);
                                (&stack0x00001948)[lVar26] = 0;
                                (&stack0x00001947)[lVar26] = 0;
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401670f6;
                                FUN_1402c4280(&stack0x00000100 + lVar26,&stack0x000014f0 + lVar26,
                                              &stack0x000018a0 + lVar26,&stack0x00001330 + lVar26);
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016710b;
                                func_0x000142923c00(param_1,&stack0x00000100 + lVar26,0x218);
                              }
                              else {
                                (&stack0x00001948)[lVar26] = 1;
                                (&stack0x00001947)[lVar26] = 1;
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140166fed;
                                FUN_140be9420(&stack0x00001600 + lVar26);
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167002;
                                func_0x000142923c00(&stack0x00000100 + lVar26,param_2,0x90);
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167017;
                                func_0x000142923c00(param_2,&stack0x00001600 + lVar26,0x90);
                                if (!SBORROW8(0,*(longlong *)(&stack0x00000100 + lVar26))) {
                                  (&stack0x00001948)[lVar26] = 0;
                                  (&stack0x00001947)[lVar26] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_14016703c;
                                  FUN_1405ef210(&stack0x00000100 + lVar26);
                                }
                                *param_1 = 0x8000000000000000;
                                (&stack0x0000194a)[lVar26] = 0;
                                (&stack0x00001949)[lVar26] = 1;
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167063;
                                FUN_1405fd1d0(&stack0x000016f0 + lVar26);
                                (&stack0x00001942)[lVar26] = 0;
                                *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167076;
                                FUN_1405da820(&stack0x00001910 + lVar26);
                              }
                              return param_1;
                            }
                            *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167218;
                            FUN_1428d80d0(&UNK_142932a10);
                            goto code_r0x000140167218;
                          }
                        }
                        *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401671e7;
                        FUN_1428d9310(&UNK_14295d910);
                        goto code_r0x000140167218;
                      }
                    }
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401671d9;
                    FUN_1428d9310(&UNK_14295d8f8);
                  }
                  else {
                    *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167154;
                    FUN_1428d98e0(*(longlong *)(&stack0x00001338 + lVar26),0x1090,&UNK_1429de8b0);
                  }
                  goto code_r0x000140167218;
                }
              }
              (&stack0x0000194f)[lVar26] = 1;
              (&stack0x0000194e)[lVar26] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401671cb;
              FUN_1428d9310(&UNK_14295d8e0);
              goto code_r0x000140167218;
            }
          }
          (&stack0x0000194f)[lVar26] = 1;
          (&stack0x0000194e)[lVar26] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_1401671af;
          FUN_1428d9310(&UNK_14295d8c8);
          goto code_r0x000140167218;
        }
      }
      (&stack0x0000194d)[lVar26] = 1;
      (&stack0x0000194c)[lVar26] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar26) = &UNK_140167190;
      FUN_1428d9310(&UNK_14295d8b0);
    }
  }
code_r0x000140167218:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

