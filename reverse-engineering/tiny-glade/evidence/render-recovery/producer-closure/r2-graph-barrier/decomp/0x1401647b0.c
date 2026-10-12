
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_1401647b0(undefined8 param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  code *pcVar1;
  undefined1 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  char cVar22;
  undefined4 uVar23;
  longlong lVar24;
  longlong lVar25;
  undefined8 uVar26;
  char *pcVar27;
  longlong lVar28;
  longlong lVar29;
  longlong *plVar30;
  ulonglong uVar31;
  longlong lVar32;
  ulonglong uVar33;
  longlong lVar34;
  undefined8 *puVar35;
  undefined4 unaff_XMM6_Da;
  float fVar36;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined1 auStackX_8 [8];
  undefined1 auStackX_10 [24];
  undefined *puStack_48;
  
  puStack_48 = &UNK_1401647c6;
  lVar24 = FUN_14289aa40();
  lVar24 = -lVar24;
  *(undefined4 *)(&stack0x000019c0 + lVar24) = unaff_XMM6_Da;
  *(undefined4 *)(&stack0x000019c4 + lVar24) = unaff_XMM6_Db;
  *(undefined4 *)(&stack0x000019c8 + lVar24) = unaff_XMM6_Dc;
  *(undefined4 *)(&stack0x000019cc + lVar24) = unaff_XMM6_Dd;
  *(undefined8 *)(&stack0x000019b8 + lVar24) = 0xfffffffffffffffe;
  *(undefined8 *)(&stack0x00001928 + lVar24) = param_1;
  lVar29 = *param_2;
  *param_2 = -0x8000000000000000;
  if (SBORROW8(0,lVar29)) {
    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165bef;
    FUN_1428d9310(&UNK_14295d5d8);
  }
  else {
    *(undefined8 *)(&stack0x00001910 + lVar24) = param_4;
    *(longlong *)(&stack0x000011b0 + lVar24) = lVar29;
    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164830;
    func_0x000142923c00(&stack0x000011b8 + lVar24,param_2 + 1,0x88);
    lVar29 = param_2[0x12];
    param_2[0x12] = 0;
    *(longlong *)(&stack0x00001920 + lVar24) = lVar29;
    if (lVar29 != 0) {
      plVar30 = *(longlong **)(&stack0x00001a40 + lVar24);
      *(longlong *)(&stack0x00001368 + lVar24) = param_2[0x1d];
      lVar34 = param_2[0x1c];
      *(longlong *)(&stack0x00001358 + lVar24) = param_2[0x1b];
      *(longlong *)(&stack0x00001360 + lVar24) = lVar34;
      lVar34 = param_2[0x13];
      uVar23 = *(undefined4 *)((longlong)param_2 + 0x9c);
      lVar28 = param_2[0x14];
      uVar3 = *(undefined4 *)((longlong)param_2 + 0xa4);
      lVar25 = param_2[0x15];
      uVar4 = *(undefined4 *)((longlong)param_2 + 0xac);
      lVar32 = param_2[0x16];
      uVar5 = *(undefined4 *)((longlong)param_2 + 0xb4);
      lVar19 = param_2[0x17];
      uVar6 = *(undefined4 *)((longlong)param_2 + 0xbc);
      lVar20 = param_2[0x18];
      uVar7 = *(undefined4 *)((longlong)param_2 + 0xc4);
      uVar8 = *(undefined4 *)((longlong)param_2 + 0xcc);
      lVar21 = param_2[0x1a];
      uVar9 = *(undefined4 *)((longlong)param_2 + 0xd4);
      *(int *)(&stack0x00001348 + lVar24) = (int)param_2[0x19];
      *(undefined4 *)(&stack0x0000134c + lVar24) = uVar8;
      *(int *)(&stack0x00001350 + lVar24) = (int)lVar21;
      *(undefined4 *)(&stack0x00001354 + lVar24) = uVar9;
      *(int *)(&stack0x00001338 + lVar24) = (int)lVar19;
      *(undefined4 *)(&stack0x0000133c + lVar24) = uVar6;
      *(int *)(&stack0x00001340 + lVar24) = (int)lVar20;
      *(undefined4 *)(&stack0x00001344 + lVar24) = uVar7;
      *(int *)(&stack0x00001328 + lVar24) = (int)lVar25;
      *(undefined4 *)(&stack0x0000132c + lVar24) = uVar4;
      *(int *)(&stack0x00001330 + lVar24) = (int)lVar32;
      *(undefined4 *)(&stack0x00001334 + lVar24) = uVar5;
      *(int *)(&stack0x00001318 + lVar24) = (int)lVar34;
      *(undefined4 *)(&stack0x0000131c + lVar24) = uVar23;
      *(int *)(&stack0x00001320 + lVar24) = (int)lVar28;
      *(undefined4 *)(&stack0x00001324 + lVar24) = uVar3;
      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401648c6;
      func_0x000142923c00(&stack0x00001260 + lVar24,&stack0x000011b0 + lVar24,0x90);
      lVar34 = *plVar30;
      lVar28 = plVar30[1];
      *(longlong *)(&stack0x00001998 + lVar24) = plVar30[2];
      *(longlong *)(&stack0x00001900 + lVar24) = plVar30[3];
      *(longlong *)(&stack0x00001908 + lVar24) = plVar30[4];
      *(undefined8 *)(&stack0x000012f0 + lVar24) = 0;
      *(undefined8 *)(&stack0x000012f8 + lVar24) = 8;
      *(undefined8 *)(&stack0x00001300 + lVar24) = 0;
      *(undefined8 *)(&stack0x00001918 + lVar24) = param_3;
      *(undefined8 *)(&stack0x00001308 + lVar24) = param_3;
      *(longlong *)(&stack0x00001310 + lVar24) = lVar29;
      lVar29 = lVar34 + 0x38;
      (&stack0x000019b5)[lVar24] = 1;
      uVar33 = 0xa68561b396784eaa;
      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016494b;
      lVar25 = FUN_1400dd6f0(lVar29,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
      if (((lVar25 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
         (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8), uVar33 != 0)) {
        lVar25 = ~uVar33 * 0x60;
        if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar25) != 0) {
          lVar25 = *(longlong *)(lVar34 + 0x1e0) + lVar25;
          uVar26 = *(undefined8 *)(lVar25 + 0x10);
          uVar23 = *(undefined4 *)(lVar34 + 0x5b0);
          *(undefined4 *)(lVar25 + 0x5c) = uVar23;
          (&stack0x000019b5)[lVar24] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401649ba;
          FUN_14093cd30(uVar26,&stack0x00001260 + lVar24);
          *(undefined4 *)(lVar25 + 0x5c) = uVar23;
          (&stack0x000019b5)[lVar24] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401649cf;
          uVar26 = FUN_14093d040(uVar26,lVar28);
          (&stack0x000019b5)[lVar24] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401649e5;
          func_0x000140bd6b00(&stack0x000018c8 + lVar24,uVar26);
          (&stack0x000019b6)[lVar24] = 1;
          uVar33 = 0x8fd9dc19c7973bca;
          *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164a08;
          lVar25 = FUN_1400dd6f0(lVar29,0x8fd9dc19c7973bca,0x361ff4c5eb9b45fd);
          if (((lVar25 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
             (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8), uVar33 != 0)) {
            lVar25 = ~uVar33 * 0x60;
            if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar25) != 0) {
              uVar26 = *(undefined8 *)(*(longlong *)(lVar34 + 0x1e0) + lVar25 + 0x10);
              *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164a69;
              func_0x000142923c00(&stack0x00000120 + lVar24,uVar26,0x1090);
              (&stack0x000019b6)[lVar24] = 1;
              uVar33 = 0x47c94cb78283a72e;
              *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164a8c;
              lVar25 = FUN_1400dd6f0(lVar29,0x47c94cb78283a72e,0x8c173ded97323e9d);
              if (((lVar25 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                 (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8), uVar33 != 0))
              {
                lVar25 = ~uVar33 * 0x60;
                if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar25) != 0) {
                  uVar26 = *(undefined8 *)(*(longlong *)(lVar34 + 0x1e0) + lVar25 + 0x10);
                  (&stack0x000019b6)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164ae7;
                  pcVar27 = (char *)func_0x00014067dd80(uVar26);
                  LOCK();
                  cVar22 = *pcVar27;
                  if (cVar22 == '\0') {
                    *pcVar27 = '\x01';
                  }
                  UNLOCK();
                  if (cVar22 != '\0') {
                    (&stack0x000019b6)[lVar24] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b1e;
                    FUN_1428cd210(pcVar27);
                  }
                  (&stack0x000019b6)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164b07;
                  FUN_140c81c70(pcVar27);
                  *(char **)(&stack0x000019a0 + lVar24) = pcVar27;
                  *(undefined8 *)(&stack0xffffffffffffffe0 + lVar24) = 0;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164b33;
                  FUN_140c02740(&stack0x00001730 + lVar24,pcVar27 + 8,lVar28,0x1090);
                  if (*(longlong *)(&stack0x00001738 + lVar24) != 0x1090) {
                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b34;
                    FUN_1428d98e0(*(longlong *)(&stack0x00001738 + lVar24),0x1090,&UNK_1429de8b0);
                    goto code_r0x000140165c52;
                  }
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164b60;
                  func_0x000142923c00(*(undefined8 *)(&stack0x00001730 + lVar24),
                                      &stack0x00000120 + lVar24,0x1090);
                  *(undefined4 *)(&stack0x00001480 + lVar24) =
                       *(undefined4 *)(&stack0x00001730 + lVar24);
                  *(undefined4 *)(&stack0x00001484 + lVar24) =
                       *(undefined4 *)(&stack0x00001734 + lVar24);
                  *(undefined4 *)(&stack0x00001488 + lVar24) =
                       *(undefined4 *)(&stack0x00001738 + lVar24);
                  *(undefined4 *)(&stack0x0000148c + lVar24) =
                       *(undefined4 *)(&stack0x0000173c + lVar24);
                  *(undefined4 *)(&stack0x00001490 + lVar24) =
                       *(undefined4 *)(&stack0x00001740 + lVar24);
                  *(undefined4 *)(&stack0x00001494 + lVar24) =
                       *(undefined4 *)(&stack0x00001744 + lVar24);
                  *(undefined4 *)(&stack0x00001498 + lVar24) =
                       *(undefined4 *)(&stack0x00001748 + lVar24);
                  *(undefined4 *)(&stack0x0000149c + lVar24) =
                       *(undefined4 *)(&stack0x0000174c + lVar24);
                  *(undefined4 *)(&stack0x000014a0 + lVar24) =
                       *(undefined4 *)(&stack0x00001750 + lVar24);
                  *(undefined4 *)(&stack0x000014a4 + lVar24) =
                       *(undefined4 *)(&stack0x00001754 + lVar24);
                  *(undefined4 *)(&stack0x000014a8 + lVar24) =
                       *(undefined4 *)(&stack0x00001758 + lVar24);
                  *(undefined4 *)(&stack0x000014ac + lVar24) =
                       *(undefined4 *)(&stack0x0000175c + lVar24);
                  *(undefined4 *)(&stack0x000014b0 + lVar24) =
                       *(undefined4 *)(&stack0x00001760 + lVar24);
                  *(undefined4 *)(&stack0x000014b4 + lVar24) =
                       *(undefined4 *)(&stack0x00001764 + lVar24);
                  *(undefined4 *)(&stack0x000014b8 + lVar24) =
                       *(undefined4 *)(&stack0x00001768 + lVar24);
                  *(undefined4 *)(&stack0x000014bc + lVar24) =
                       *(undefined4 *)(&stack0x0000176c + lVar24);
                  *(undefined8 *)(&stack0x000014c0 + lVar24) =
                       *(undefined8 *)(&stack0x00001770 + lVar24);
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164bbc;
                  FUN_140c02370(&stack0x00001390 + lVar24,&stack0x00001480 + lVar24,lVar28);
                  (&stack0x000019b0)[lVar24] = 1;
                  pcVar27 = *(char **)(&stack0x000019a0 + lVar24);
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164bd2;
                  FUN_140c81e10();
                  LOCK();
                  cVar22 = *pcVar27;
                  if (cVar22 == '\x01') {
                    *pcVar27 = '\0';
                  }
                  UNLOCK();
                  if (cVar22 != '\x01') {
                    (&stack0x000019b0)[lVar24] = 1;
                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b4a;
                    FUN_1428cdb90(pcVar27,0);
                  }
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164bf9;
                  func_0x000142923c00(&stack0x00001480 + lVar24,&stack0x00001260 + lVar24,0x110);
                  *(undefined8 *)(&stack0x00001590 + lVar24) =
                       *(undefined8 *)(&stack0x00001390 + lVar24);
                  *(undefined8 *)(&stack0x00001598 + lVar24) =
                       *(undefined8 *)(&stack0x00001398 + lVar24);
                  *(undefined4 *)(&stack0x000015a0 + lVar24) =
                       *(undefined4 *)(&stack0x000013a0 + lVar24);
                  *(undefined4 *)(&stack0x000015a4 + lVar24) =
                       *(undefined4 *)(&stack0x000013a4 + lVar24);
                  *(undefined4 *)(&stack0x000015a8 + lVar24) =
                       *(undefined4 *)(&stack0x000013a8 + lVar24);
                  *(undefined4 *)(&stack0x000015ac + lVar24) =
                       *(undefined4 *)(&stack0x000013ac + lVar24);
                  *(undefined4 *)(&stack0x000015b0 + lVar24) =
                       *(undefined4 *)(&stack0x000013b0 + lVar24);
                  *(undefined4 *)(&stack0x000015b4 + lVar24) =
                       *(undefined4 *)(&stack0x000013b4 + lVar24);
                  *(undefined4 *)(&stack0x000015b8 + lVar24) =
                       *(undefined4 *)(&stack0x000013b8 + lVar24);
                  *(undefined4 *)(&stack0x000015bc + lVar24) =
                       *(undefined4 *)(&stack0x000013bc + lVar24);
                  *(undefined4 *)(&stack0x000015c0 + lVar24) =
                       *(undefined4 *)(&stack0x000013c0 + lVar24);
                  *(undefined4 *)(&stack0x000015c4 + lVar24) =
                       *(undefined4 *)(&stack0x000013c4 + lVar24);
                  *(undefined4 *)(&stack0x000015c8 + lVar24) =
                       *(undefined4 *)(&stack0x000013c8 + lVar24);
                  *(undefined4 *)(&stack0x000015cc + lVar24) =
                       *(undefined4 *)(&stack0x000013cc + lVar24);
                  *(undefined8 *)(&stack0x000015d0 + lVar24) =
                       *(undefined8 *)(&stack0x000013d0 + lVar24);
                  *(undefined8 *)(&stack0x000015d8 + lVar24) =
                       *(undefined8 *)(&stack0x000013d8 + lVar24);
                  *(undefined8 *)(&stack0x000015e0 + lVar24) =
                       *(undefined8 *)(&stack0x000013e0 + lVar24);
                  *(undefined8 *)(&stack0x000015e8 + lVar24) =
                       *(undefined8 *)(&stack0x000013e8 + lVar24);
                  *(undefined4 *)(&stack0x000015f0 + lVar24) =
                       *(undefined4 *)(&stack0x000018c8 + lVar24);
                  *(undefined4 *)(&stack0x000015f4 + lVar24) =
                       *(undefined4 *)(&stack0x000018cc + lVar24);
                  *(undefined4 *)(&stack0x000015f8 + lVar24) =
                       *(undefined4 *)(&stack0x000018d0 + lVar24);
                  *(undefined4 *)(&stack0x000015fc + lVar24) =
                       *(undefined4 *)(&stack0x000018d4 + lVar24);
                  *(undefined8 *)(&stack0x00001600 + lVar24) =
                       *(undefined8 *)(&stack0x000018d8 + lVar24);
                  *(undefined8 *)(&stack0x00001608 + lVar24) =
                       *(undefined8 *)(&stack0x00001998 + lVar24);
                  *(longlong *)(&stack0x00001610 + lVar24) = lVar28;
                  if (cRam000000014362b0e6 == '\b') {
                    (&stack0x000019b7)[lVar24] = 1;
                    uVar33 = 0x4da5153112b09210;
                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164cae;
                    lVar25 = FUN_1400dd6f0(lVar29,0x4da5153112b09210,0x5b807edb4e205af1);
                    if (((lVar25 == 0) || (*(ulonglong *)(lVar34 + 0x218) <= uVar33)) ||
                       (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                       uVar33 == 0)) {
code_r0x00014016509e:
                      (&stack0x000019a0)[lVar24] = 7;
                    }
                    else {
                      lVar25 = *(longlong *)(lVar34 + 0x1e0);
                      lVar32 = ~uVar33 * 0x60;
                      if (*(longlong *)(lVar25 + 0x20 + lVar32) == 0) goto code_r0x00014016509e;
                      (&stack0x000019a0)[lVar24] = 7;
                      lVar25 = *(longlong *)(lVar25 + lVar32 + 0x10);
                      if (*(longlong *)(lVar25 + 0x10) != 0) {
                        uVar26 = *(undefined8 *)(lVar25 + 8);
                        (&stack0x000019b7)[lVar24] = 1;
                        *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164d26;
                        FUN_140b46a50(&stack0x00001370 + lVar24,uVar26);
                        (&stack0x000019b7)[lVar24] = 1;
                        *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164d40;
                        FUN_1400c6970(&stack0x00001730 + lVar24,&stack0x00001370 + lVar24);
                        if (*(longlong *)(&stack0x00001730 + lVar24) == -0x7ffffffffffffff6) {
                          *(undefined8 *)(&stack0x00001960 + lVar24) =
                               *(undefined8 *)(&stack0x00001740 + lVar24);
                          *(undefined8 *)(&stack0x00001968 + lVar24) =
                               *(undefined8 *)(&stack0x00001748 + lVar24);
                          *(undefined4 *)(&stack0x00001970 + lVar24) =
                               *(undefined4 *)(&stack0x00001750 + lVar24);
                          *(undefined4 *)(&stack0x00001974 + lVar24) =
                               *(undefined4 *)(&stack0x00001754 + lVar24);
                          *(undefined4 *)(&stack0x00001978 + lVar24) =
                               *(undefined4 *)(&stack0x00001758 + lVar24);
                          *(undefined4 *)(&stack0x0000197c + lVar24) =
                               *(undefined4 *)(&stack0x0000175c + lVar24);
                          if (*(longlong *)(&stack0x00001738 + lVar24) != 10) {
                            *(undefined4 *)(&stack0x00001638 + lVar24) =
                                 *(undefined4 *)(&stack0x00001970 + lVar24);
                            *(undefined4 *)(&stack0x0000163c + lVar24) =
                                 *(undefined4 *)(&stack0x00001974 + lVar24);
                            *(undefined4 *)(&stack0x00001640 + lVar24) =
                                 *(undefined4 *)(&stack0x00001978 + lVar24);
                            *(undefined4 *)(&stack0x00001644 + lVar24) =
                                 *(undefined4 *)(&stack0x0000197c + lVar24);
                            *(undefined4 *)(&stack0x00001628 + lVar24) =
                                 *(undefined4 *)(&stack0x00001960 + lVar24);
                            *(undefined4 *)(&stack0x0000162c + lVar24) =
                                 *(undefined4 *)(&stack0x00001964 + lVar24);
                            *(undefined4 *)(&stack0x00001630 + lVar24) =
                                 *(undefined4 *)(&stack0x00001968 + lVar24);
                            *(undefined4 *)(&stack0x00001634 + lVar24) =
                                 *(undefined4 *)(&stack0x0000196c + lVar24);
                            *(longlong *)(&stack0x00001620 + lVar24) =
                                 *(longlong *)(&stack0x00001738 + lVar24);
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164dbd;
                            FUN_140bad8d0(&stack0x000018e0 + lVar24,&stack0x00001620 + lVar24);
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164dc9;
                            func_0x000140161c60(&stack0x00001620 + lVar24);
                            (&stack0x000019af)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164de9;
                            func_0x000140bd52a0(&stack0x00001930 + lVar24,
                                                *(undefined8 *)(&stack0x000018f8 + lVar24),0x2b);
                            (&stack0x000019af)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164e06;
                            func_0x000140bd5370(&stack0x00001240 + lVar24,&stack0x00001930 + lVar24)
                            ;
                            *(undefined8 *)(&stack0x00001998 + lVar24) =
                                 *(undefined8 *)(&stack0x000018e0 + lVar24);
                            *(undefined8 *)(&stack0xfffffffffffffff0 + lVar24) =
                                 *(undefined8 *)(&stack0x000018f0 + lVar24);
                            *(undefined8 *)(&stack0x00001958 + lVar24) =
                                 *(undefined8 *)(&stack0x000018e8 + lVar24);
                            *(undefined8 *)(&stack0xffffffffffffffe8 + lVar24) =
                                 *(undefined8 *)(&stack0x000018e8 + lVar24);
                            *(undefined1 **)(&stack0xffffffffffffffe0 + lVar24) =
                                 &stack0x00001240 + lVar24;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164e54;
                            FUN_1405b9b00(&stack0x00001730 + lVar24,lVar28,&UNK_14295da20,0x10);
                            uVar33 = *(ulonglong *)(&stack0x00001730 + lVar24);
                            if (uVar33 == 0xb) {
                              *(undefined8 *)(&stack0x0000164e + lVar24) =
                                   *(undefined8 *)(&stack0x00001766 + lVar24);
                              *(undefined8 *)(&stack0x00001640 + lVar24) =
                                   *(undefined8 *)(&stack0x00001758 + lVar24);
                              *(undefined8 *)(&stack0x00001648 + lVar24) =
                                   *(undefined8 *)(&stack0x00001760 + lVar24);
                              *(undefined8 *)(&stack0x00001630 + lVar24) =
                                   *(undefined8 *)(&stack0x00001748 + lVar24);
                              *(undefined8 *)(&stack0x00001638 + lVar24) =
                                   *(undefined8 *)(&stack0x00001750 + lVar24);
                              *(undefined4 *)(&stack0x00001620 + lVar24) =
                                   *(undefined4 *)(&stack0x00001738 + lVar24);
                              *(undefined4 *)(&stack0x00001624 + lVar24) =
                                   *(undefined4 *)(&stack0x0000173c + lVar24);
                              *(undefined4 *)(&stack0x00001628 + lVar24) =
                                   *(undefined4 *)(&stack0x00001740 + lVar24);
                              *(undefined4 *)(&stack0x0000162c + lVar24) =
                                   *(undefined4 *)(&stack0x00001744 + lVar24);
                              if ((&stack0x0000176e)[lVar24] != '\a') {
                                *(undefined8 *)(&stack0x0000198e + lVar24) =
                                     *(undefined8 *)(&stack0x0000164e + lVar24);
                                *(undefined4 *)(&stack0x00001980 + lVar24) =
                                     *(undefined4 *)(&stack0x00001640 + lVar24);
                                *(undefined4 *)(&stack0x00001984 + lVar24) =
                                     *(undefined4 *)(&stack0x00001644 + lVar24);
                                *(undefined4 *)(&stack0x00001988 + lVar24) =
                                     *(undefined4 *)(&stack0x00001648 + lVar24);
                                *(undefined4 *)(&stack0x0000198c + lVar24) =
                                     *(undefined4 *)(&stack0x0000164c + lVar24);
                                *(undefined4 *)(&stack0x00001970 + lVar24) =
                                     *(undefined4 *)(&stack0x00001630 + lVar24);
                                *(undefined4 *)(&stack0x00001974 + lVar24) =
                                     *(undefined4 *)(&stack0x00001634 + lVar24);
                                *(undefined4 *)(&stack0x00001978 + lVar24) =
                                     *(undefined4 *)(&stack0x00001638 + lVar24);
                                *(undefined4 *)(&stack0x0000197c + lVar24) =
                                     *(undefined4 *)(&stack0x0000163c + lVar24);
                                *(undefined4 *)(&stack0x00001960 + lVar24) =
                                     *(undefined4 *)(&stack0x00001620 + lVar24);
                                *(undefined4 *)(&stack0x00001964 + lVar24) =
                                     *(undefined4 *)(&stack0x00001624 + lVar24);
                                *(undefined4 *)(&stack0x00001968 + lVar24) =
                                     *(undefined4 *)(&stack0x00001628 + lVar24);
                                *(undefined4 *)(&stack0x0000196c + lVar24) =
                                     *(undefined4 *)(&stack0x0000162c + lVar24);
                                (&stack0x00001996)[lVar24] = (&stack0x0000176e)[lVar24];
                                (&stack0x00001997)[lVar24] = (&stack0x0000176f)[lVar24];
                                if (*(longlong *)(&stack0x00001998 + lVar24) != 0) {
                                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164f15;
                                  func_0x000140613c20(*(undefined8 *)(&stack0x00001958 + lVar24),
                                                      *(longlong *)(&stack0x00001998 + lVar24),1);
                                }
                                uVar33 = 0xa284155e68bfeb3f;
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164f31;
                                lVar28 = FUN_1400dd6f0(lVar29,0xa284155e68bfeb3f,0x84a2b3441bc64a3a)
                                ;
                                if (((lVar28 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                                   (uVar33 = *(ulonglong *)
                                              (*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                                   uVar33 != 0)) {
                                  lVar28 = ~uVar33 * 0x60;
                                  if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar28)
                                      != 0) {
                                    lVar28 = *(longlong *)(lVar34 + 0x1e0) + lVar28;
                                    lVar25 = *(longlong *)(lVar28 + 0x10);
                                    *(undefined4 *)(lVar28 + 0x5c) = *(undefined4 *)(lVar34 + 0x5b0)
                                    ;
                                    if (*(int *)(lVar25 + 0x48) == 2) {
                                      *(undefined **)((longlong)&puStack_48 + lVar24) =
                                           &UNK_140165c52;
                                      FUN_1428d9430(&UNK_14295d9e0,0x28,&UNK_14295da48);
                                      goto code_r0x000140165c52;
                                    }
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140164fa7
                                    ;
                                    func_0x000140bd6b00(&stack0x00001730 + lVar24,
                                                        &stack0x00001960 + lVar24);
                                    *(undefined8 *)(&stack0x00001748 + lVar24) =
                                         *(undefined8 *)(&stack0x00001978 + lVar24);
                                    *(undefined8 *)(&stack0x00001750 + lVar24) =
                                         *(undefined8 *)(&stack0x00001980 + lVar24);
                                    *(undefined4 *)(&stack0x00001758 + lVar24) =
                                         *(undefined4 *)(&stack0x00001988 + lVar24);
                                    *(undefined4 *)(&stack0x0000175c + lVar24) =
                                         *(undefined4 *)(&stack0x0000198c + lVar24);
                                    *(undefined4 *)(&stack0x00001760 + lVar24) =
                                         *(undefined4 *)(&stack0x00001990 + lVar24);
                                    *(undefined4 *)(&stack0x00001764 + lVar24) =
                                         *(undefined4 *)(&stack0x00001994 + lVar24);
                                    *(undefined8 *)(&stack0x00001620 + lVar24) =
                                         *(undefined8 *)(&stack0x00001730 + lVar24);
                                    *(undefined8 *)(&stack0x00001628 + lVar24) =
                                         *(undefined8 *)(&stack0x00001738 + lVar24);
                                    *(undefined8 *)(&stack0x00001630 + lVar24) =
                                         *(undefined8 *)(&stack0x00001740 + lVar24);
                                    *(undefined8 *)(&stack0x00001638 + lVar24) =
                                         *(undefined8 *)(&stack0x00001748 + lVar24);
                                    *(undefined4 *)(&stack0x00001640 + lVar24) =
                                         *(undefined4 *)(&stack0x00001750 + lVar24);
                                    *(undefined4 *)(&stack0x00001644 + lVar24) =
                                         *(undefined4 *)(&stack0x00001754 + lVar24);
                                    *(undefined4 *)(&stack0x00001648 + lVar24) =
                                         *(undefined4 *)(&stack0x00001758 + lVar24);
                                    *(undefined4 *)(&stack0x0000164c + lVar24) =
                                         *(undefined4 *)(&stack0x0000175c + lVar24);
                                    *(undefined8 *)(&stack0x00001650 + lVar24) =
                                         *(undefined8 *)(&stack0x00001760 + lVar24);
                                    *(longlong *)(&stack0x000019a0 + lVar24) = lVar25 + 8;
                                    if (*(char *)(lVar25 + 0x3e) != '\a') {
                                      *(undefined **)((longlong)&puStack_48 + lVar24) =
                                           &UNK_140165018;
                                      FUN_1405ee5e0(*(undefined8 *)(&stack0x000019a0 + lVar24));
                                    }
                                    puVar35 = *(undefined8 **)(&stack0x000019a0 + lVar24);
                                    puVar35[6] = *(undefined8 *)(&stack0x00001650 + lVar24);
                                    uVar26 = *(undefined8 *)(&stack0x00001620 + lVar24);
                                    uVar15 = *(undefined8 *)(&stack0x00001628 + lVar24);
                                    uVar16 = *(undefined8 *)(&stack0x00001630 + lVar24);
                                    uVar17 = *(undefined8 *)(&stack0x00001638 + lVar24);
                                    uVar18 = *(undefined8 *)(&stack0x00001648 + lVar24);
                                    puVar35[4] = *(undefined8 *)(&stack0x00001640 + lVar24);
                                    puVar35[5] = uVar18;
                                    puVar35[2] = uVar16;
                                    puVar35[3] = uVar17;
                                    *puVar35 = uVar26;
                                    puVar35[1] = uVar15;
                                    *(undefined4 *)(&stack0x00001730 + lVar24) =
                                         *(undefined4 *)(&stack0x00001960 + lVar24);
                                    *(undefined4 *)(&stack0x00001734 + lVar24) =
                                         *(undefined4 *)(&stack0x00001964 + lVar24);
                                    *(undefined4 *)(&stack0x00001738 + lVar24) =
                                         *(undefined4 *)(&stack0x00001968 + lVar24);
                                    *(undefined4 *)(&stack0x0000173c + lVar24) =
                                         *(undefined4 *)(&stack0x0000196c + lVar24);
                                    *(undefined8 *)(&stack0x00001740 + lVar24) =
                                         *(undefined8 *)(&stack0x00001970 + lVar24);
                                    *(undefined8 *)(&stack0x00001748 + lVar24) =
                                         *(undefined8 *)(&stack0x00001978 + lVar24);
                                    *(undefined8 *)(&stack0x00001750 + lVar24) =
                                         *(undefined8 *)(&stack0x00001980 + lVar24);
                                    *(undefined8 *)(&stack0x00001758 + lVar24) =
                                         *(undefined8 *)(&stack0x00001988 + lVar24);
                                    *(undefined8 *)(&stack0x0000175e + lVar24) =
                                         *(undefined8 *)(&stack0x0000198e + lVar24);
                                    (&stack0x000019a0)[lVar24] = (&stack0x00001996)[lVar24];
                                    (&stack0x00001998)[lVar24] = (&stack0x00001997)[lVar24];
                                    goto code_r0x0001401650a5;
                                  }
                                }
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165c38;
                                FUN_1428d9310(&UNK_14295da30);
                                goto code_r0x000140165c52;
                              }
                            }
                            else {
                              lVar28 = 0;
                              if (uVar33 - 9 < 2) {
                                lVar28 = uVar33 - 8;
                              }
                              if (lVar28 == 0) {
                                if (((uVar33 < 6) && ((0x32UL >> (uVar33 & 0x3f) & 1) != 0)) &&
                                   (*(longlong *)(&stack0x00001738 + lVar24) != 0)) {
                                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165aac;
                                  func_0x000140613c20(*(undefined8 *)(&stack0x00001740 + lVar24),
                                                      *(longlong *)(&stack0x00001738 + lVar24),1);
                                }
                                lVar28 = *(longlong *)(&stack0x00001750 + lVar24);
                                if (lVar28 != 0) {
                                  puVar2 = &stack0x00001758;
code_r0x000140165ad4:
                                  uVar26 = *(undefined8 *)(puVar2 + lVar24);
                                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165ae2;
                                  func_0x000140613c20(uVar26,lVar28,1);
                                }
                              }
                              else if (lVar28 == 1) {
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165a18;
                                FUN_1405ebcf0(&stack0x00001738 + lVar24);
                              }
                              else {
                                lVar28 = *(longlong *)(&stack0x00001738 + lVar24);
                                if (lVar28 != 0) {
                                  puVar2 = &stack0x00001740;
                                  goto code_r0x000140165ad4;
                                }
                              }
                            }
                            if (*(longlong *)(&stack0x00001998 + lVar24) != 0) {
                              *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b04;
                              func_0x000140613c20(*(undefined8 *)(&stack0x00001958 + lVar24),
                                                  *(longlong *)(&stack0x00001998 + lVar24),1);
                            }
                          }
                        }
                        else {
                          uVar31 = *(longlong *)(&stack0x00001730 + lVar24) + 0x7ffffffffffffffc;
                          uVar33 = 4;
                          if (uVar31 < 6) {
                            uVar33 = uVar31;
                          }
                          switch(uVar33) {
                          case 0:
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401659e6;
                            FUN_1405ea740(&stack0x00001738 + lVar24);
                            break;
                          case 1:
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165a48;
                            FUN_1405ea740(&stack0x00001738 + lVar24);
                            break;
                          case 2:
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165a60;
                            FUN_1405eb290(&stack0x00001738 + lVar24);
                            break;
                          case 3:
                            break;
                          case 4:
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165a30;
                            FUN_1405ed2c0(&stack0x00001730 + lVar24);
                            break;
                          default:
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165a78;
                            FUN_1405e7a00(&stack0x00001738 + lVar24);
                          }
                        }
                      }
                    }
code_r0x0001401650a5:
                    if (1 < (byte)(cRam000000014362b0e6 - 7U)) {
                      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401650be;
                      func_0x00014003bb80(0x14362b0b0);
                      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401650ca;
                      FUN_140be24b0(0x14362b0b0);
                    }
                    uRam000000014362b0e0 =
                         (undefined6)((ulonglong)*(undefined8 *)(&stack0x0000175e + lVar24) >> 0x10)
                    ;
                    uRam000000014362b0b0 = *(undefined4 *)(&stack0x00001730 + lVar24);
                    uRam000000014362b0b4 = *(undefined4 *)(&stack0x00001734 + lVar24);
                    uRam000000014362b0b8 = *(undefined4 *)(&stack0x00001738 + lVar24);
                    uRam000000014362b0bc = *(undefined4 *)(&stack0x0000173c + lVar24);
                    uRam000000014362b0c0 = *(undefined4 *)(&stack0x00001740 + lVar24);
                    uRam000000014362b0c4 = *(undefined4 *)(&stack0x00001744 + lVar24);
                    uRam000000014362b0c8 = *(undefined4 *)(&stack0x00001748 + lVar24);
                    uRam000000014362b0cc = *(undefined4 *)(&stack0x0000174c + lVar24);
                    uRam000000014362b0d0 = *(undefined4 *)(&stack0x00001750 + lVar24);
                    uRam000000014362b0d4 = *(undefined4 *)(&stack0x00001754 + lVar24);
                    uRam000000014362b0d8 = *(undefined4 *)(&stack0x00001758 + lVar24);
                    uRam000000014362b0dc = (undefined2)*(undefined4 *)(&stack0x0000175c + lVar24);
                    uRam000000014362b0de =
                         (undefined2)((uint)*(undefined4 *)(&stack0x0000175c + lVar24) >> 0x10);
                    cRam000000014362b0e6 = (&stack0x000019a0)[lVar24];
                    uRam000000014362b0e7 = (&stack0x00001998)[lVar24];
                  }
                  uVar26 = **(undefined8 **)(&stack0x00001900 + lVar24);
                  (&stack0x000019b7)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016513f;
                  func_0x000140bd52a0(&stack0x00001930 + lVar24,uVar26,0x2b);
                  *(undefined4 *)(&stack0x00001630 + lVar24) =
                       *(undefined4 *)(&stack0x00001940 + lVar24);
                  *(undefined4 *)(&stack0x00001634 + lVar24) =
                       *(undefined4 *)(&stack0x00001944 + lVar24);
                  *(undefined4 *)(&stack0x00001638 + lVar24) =
                       *(undefined4 *)(&stack0x00001948 + lVar24);
                  *(undefined4 *)(&stack0x0000163c + lVar24) =
                       *(undefined4 *)(&stack0x0000194c + lVar24);
                  *(undefined4 *)(&stack0x00001620 + lVar24) =
                       *(undefined4 *)(&stack0x00001930 + lVar24);
                  *(undefined4 *)(&stack0x00001624 + lVar24) =
                       *(undefined4 *)(&stack0x00001934 + lVar24);
                  *(undefined4 *)(&stack0x00001628 + lVar24) =
                       *(undefined4 *)(&stack0x00001938 + lVar24);
                  *(undefined4 *)(&stack0x0000162c + lVar24) =
                       *(undefined4 *)(&stack0x0000193c + lVar24);
                  (&stack0x000019b7)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165175;
                  func_0x000140bd6ce0(&stack0x00001740 + lVar24,&stack0x00001620 + lVar24);
                  *(undefined **)(&stack0x00001730 + lVar24) = &UNK_14295d970;
                  *(undefined8 *)(&stack0x00001738 + lVar24) = 0xe;
                  (&stack0x000019b7)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401651a8;
                  uVar23 = FUN_140be94d0(&stack0x00001480 + lVar24,&stack0x00001730 + lVar24);
                  *(undefined4 *)(&stack0x00001964 + lVar24) =
                       *(undefined4 *)(&stack0x00001930 + lVar24);
                  *(undefined4 *)(&stack0x00001968 + lVar24) =
                       *(undefined4 *)(&stack0x00001934 + lVar24);
                  *(undefined4 *)(&stack0x0000196c + lVar24) =
                       *(undefined4 *)(&stack0x00001938 + lVar24);
                  *(undefined4 *)(&stack0x00001970 + lVar24) =
                       *(undefined4 *)(&stack0x0000193c + lVar24);
                  *(undefined4 *)(&stack0x00001974 + lVar24) =
                       *(undefined4 *)(&stack0x00001940 + lVar24);
                  *(undefined4 *)(&stack0x00001978 + lVar24) =
                       *(undefined4 *)(&stack0x00001944 + lVar24);
                  *(undefined4 *)(&stack0x0000197c + lVar24) =
                       *(undefined4 *)(&stack0x00001948 + lVar24);
                  *(undefined4 *)(&stack0x00001980 + lVar24) =
                       *(undefined4 *)(&stack0x0000194c + lVar24);
                  *(undefined4 *)(&stack0x00001960 + lVar24) = uVar23;
                  (&stack0x000019b7)[lVar24] = 1;
                  uVar33 = 0x109ca79d5cccc2d1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401651ed;
                  lVar28 = FUN_1400dd6f0(lVar29,0x109ca79d5cccc2d1,0x24eabc1830c9840c);
                  if (((lVar28 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                     (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                     uVar33 != 0)) {
                    lVar28 = ~uVar33 * 0x60;
                    if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar28) != 0) {
                      fVar36 = *(float *)(*(longlong *)
                                           (*(longlong *)(lVar34 + 0x1e0) + lVar28 + 0x10) + 0x58);
                      (&stack0x000019b7)[lVar24] = 1;
                      uVar33 = 0xa284155e68bfeb3f;
                      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165264;
                      lVar28 = FUN_1400dd6f0(lVar29,0xa284155e68bfeb3f,0x84a2b3441bc64a3a);
                      if (((lVar28 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                         (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                         uVar33 != 0)) {
                        lVar28 = ~uVar33 * 0x60;
                        if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar28) != 0) {
                          lVar28 = *(longlong *)(lVar34 + 0x1e0) + lVar28;
                          lVar25 = *(longlong *)(lVar28 + 0x10);
                          *(undefined4 *)(lVar28 + 0x5c) = *(undefined4 *)(lVar34 + 0x5b0);
                          if (*(int *)(lVar25 + 0x48) == 2) {
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165c1c;
                            FUN_1428d9430(&UNK_14295d9e0,0x28,&UNK_14295da08);
                          }
                          else {
                            if (_UNK_14295d580 <= fVar36) {
                              fVar36 = _UNK_14295d580;
                            }
                            fVar36 = fVar36 + *(float *)(lVar25 + 0x44);
                            *(float *)(lVar25 + 0x44) = fVar36;
                            uVar26 = 0x14362b0b0;
                            if (cRam000000014362b0e6 == '\a') {
                              uVar26 = 0;
                            }
                            (&stack0x000019b7)[lVar24] = 1;
                            *(float *)(&stack0xffffffffffffffe8 + lVar24) = fVar36;
                            *(undefined1 **)(&stack0xffffffffffffffe0 + lVar24) =
                                 &stack0x00001960 + lVar24;
                            auStackX_8[lVar24] = 1;
                            auStackX_10[lVar24 + -0x10] = 0;
                            (&stack0xfffffffffffffff8)[lVar24] = 0;
                            *(undefined4 *)(&stack0xfffffffffffffff0 + lVar24) = 0;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016532f;
                            FUN_1408a8d70(&stack0x00001480 + lVar24,lVar34,uVar26,0);
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016534f;
                            FUN_1409c6700(&stack0x00001480 + lVar24,lVar34,&stack0x00001960 + lVar24
                                          ,0);
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165373;
                            FUN_140837ff0(&stack0x00001480 + lVar24,lVar34,&stack0x00001960 + lVar24
                                          ,*(undefined8 *)(&stack0x00001908 + lVar24));
                            *(undefined4 *)(&stack0x00001730 + lVar24) =
                                 *(undefined4 *)(&stack0x00001960 + lVar24);
                            *(undefined4 *)(&stack0x00001734 + lVar24) =
                                 *(undefined4 *)(&stack0x00001964 + lVar24);
                            *(undefined4 *)(&stack0x00001738 + lVar24) =
                                 *(undefined4 *)(&stack0x00001968 + lVar24);
                            *(undefined4 *)(&stack0x0000173c + lVar24) =
                                 *(undefined4 *)(&stack0x0000196c + lVar24);
                            *(undefined4 *)(&stack0x00001750 + lVar24) =
                                 *(undefined4 *)(&stack0x00001980 + lVar24);
                            *(undefined4 *)(&stack0x00001740 + lVar24) =
                                 *(undefined4 *)(&stack0x00001970 + lVar24);
                            *(undefined4 *)(&stack0x00001744 + lVar24) =
                                 *(undefined4 *)(&stack0x00001974 + lVar24);
                            *(undefined4 *)(&stack0x00001748 + lVar24) =
                                 *(undefined4 *)(&stack0x00001978 + lVar24);
                            *(undefined4 *)(&stack0x0000174c + lVar24) =
                                 *(undefined4 *)(&stack0x0000197c + lVar24);
                            uVar23 = *(undefined4 *)(&stack0x00001730 + lVar24);
                            (&stack0x000019b7)[lVar24] = 1;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401653bf;
                            FUN_140bd6380(&stack0x00001730 + lVar24,&stack0x00001480 + lVar24,0x22);
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401653db;
                            func_0x000142923c00(&stack0x00001730 + lVar24,&stack0x00001480 + lVar24,
                                                0x198);
                            (&stack0x000019b7)[lVar24] = 0;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401653f1;
                            FUN_1409dda50(&stack0x00001620 + lVar24,&stack0x00001730 + lVar24);
                            uVar33 = 0xa68561b396784eaa;
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016540d;
                            lVar28 = FUN_1400dd6f0(lVar29,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
                            if (((lVar28 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                               (uVar33 = *(ulonglong *)(*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                               uVar33 != 0)) {
                              lVar28 = ~uVar33 * 0x60;
                              if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar28) != 0)
                              {
                                lVar28 = *(longlong *)(lVar34 + 0x1e0) + lVar28;
                                uVar26 = *(undefined8 *)(lVar28 + 0x10);
                                *(undefined4 *)(lVar28 + 0x5c) = *(undefined4 *)(lVar34 + 0x5b0);
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165472;
                                FUN_14093ce30(uVar26,&stack0x00001620 + lVar24);
                                uVar33 = 0x8226cc786e679355;
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016548e;
                                lVar29 = FUN_1400dd6f0(lVar29,0x8226cc786e679355,0x65cfda6a55a841ac)
                                ;
                                if (((lVar29 != 0) && (uVar33 < *(ulonglong *)(lVar34 + 0x218))) &&
                                   (uVar33 = *(ulonglong *)
                                              (*(longlong *)(lVar34 + 0x210) + uVar33 * 8),
                                   uVar33 != 0)) {
                                  lVar29 = ~uVar33 * 0x60;
                                  if (*(longlong *)(*(longlong *)(lVar34 + 0x1e0) + 0x20 + lVar29)
                                      != 0) {
                                    lVar29 = *(longlong *)(lVar34 + 0x1e0) + lVar29;
                                    uVar26 = *(undefined8 *)(lVar29 + 0x10);
                                    *(undefined4 *)(lVar29 + 0x5c) = *(undefined4 *)(lVar34 + 0x5b0)
                                    ;
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401654f3
                                    ;
                                    FUN_14091ebb0(uVar26,&stack0x00001620 + lVar24);
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016550c
                                    ;
                                    func_0x000142923c00(auStackX_10 + lVar24,
                                                        &stack0x00001620 + lVar24,0x110);
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016551b
                                    ;
                                    FUN_1404f7ca0(&stack0x00000120 + lVar24,auStackX_10 + lVar24);
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016553b
                                    ;
                                    func_0x000142923c00(&stack0x00001620 + lVar24,
                                                        &stack0x00000120 + lVar24,0xa8);
                                    *(undefined8 *)(&stack0x00001420 + lVar24) =
                                         *(undefined8 *)(&stack0x000001c8 + lVar24);
                                    *(undefined8 *)(&stack0x00001428 + lVar24) =
                                         *(undefined8 *)(&stack0x000001d0 + lVar24);
                                    *(undefined8 *)(&stack0x00001430 + lVar24) =
                                         *(undefined8 *)(&stack0x000001d8 + lVar24);
                                    *(undefined8 *)(&stack0x00001438 + lVar24) =
                                         *(undefined8 *)(&stack0x000001e0 + lVar24);
                                    *(undefined8 *)(&stack0x00001440 + lVar24) =
                                         *(undefined8 *)(&stack0x000001e8 + lVar24);
                                    *(undefined8 *)(&stack0x00001448 + lVar24) =
                                         *(undefined8 *)(&stack0x000001f0 + lVar24);
                                    *(undefined8 *)(&stack0x00001450 + lVar24) =
                                         *(undefined8 *)(&stack0x000001f8 + lVar24);
                                    *(undefined8 *)(&stack0x00001458 + lVar24) =
                                         *(undefined8 *)(&stack0x00000200 + lVar24);
                                    *(undefined8 *)(&stack0x00001460 + lVar24) =
                                         *(undefined8 *)(&stack0x00000208 + lVar24);
                                    *(undefined8 *)(&stack0x00001468 + lVar24) =
                                         *(undefined8 *)(&stack0x00000210 + lVar24);
                                    *(undefined8 *)(&stack0x00001470 + lVar24) =
                                         *(undefined8 *)(&stack0x00000218 + lVar24);
                                    *(undefined8 *)(&stack0x00001478 + lVar24) =
                                         *(undefined8 *)(&stack0x00000220 + lVar24);
                                    lVar29 = param_2[0x1d];
                                    *(longlong *)(&stack0x00000170 + lVar24) = param_2[0x1c];
                                    *(longlong *)(&stack0x00000178 + lVar24) = lVar29;
                                    lVar29 = param_2[0x1b];
                                    *(longlong *)(&stack0x00000160 + lVar24) = param_2[0x1a];
                                    *(longlong *)(&stack0x00000168 + lVar24) = lVar29;
                                    lVar29 = param_2[0x12];
                                    lVar34 = param_2[0x13];
                                    lVar28 = param_2[0x14];
                                    lVar25 = param_2[0x15];
                                    lVar32 = param_2[0x16];
                                    lVar19 = param_2[0x17];
                                    lVar20 = param_2[0x19];
                                    *(longlong *)(&stack0x00000150 + lVar24) = param_2[0x18];
                                    *(longlong *)(&stack0x00000158 + lVar24) = lVar20;
                                    *(longlong *)(&stack0x00000140 + lVar24) = lVar32;
                                    *(longlong *)(&stack0x00000148 + lVar24) = lVar19;
                                    *(longlong *)(&stack0x00000130 + lVar24) = lVar28;
                                    *(longlong *)(&stack0x00000138 + lVar24) = lVar25;
                                    *(longlong *)(&stack0x00000120 + lVar24) = lVar29;
                                    *(longlong *)(&stack0x00000128 + lVar24) = lVar34;
                                    lVar29 = *(longlong *)(&stack0x00001428 + lVar24);
                                    uVar3 = *(undefined4 *)(&stack0x00001430 + lVar24);
                                    uVar4 = *(undefined4 *)(&stack0x00001434 + lVar24);
                                    uVar5 = *(undefined4 *)(&stack0x00001438 + lVar24);
                                    uVar6 = *(undefined4 *)(&stack0x0000143c + lVar24);
                                    uVar7 = *(undefined4 *)(&stack0x00001440 + lVar24);
                                    uVar8 = *(undefined4 *)(&stack0x00001444 + lVar24);
                                    uVar9 = *(undefined4 *)(&stack0x00001448 + lVar24);
                                    uVar10 = *(undefined4 *)(&stack0x0000144c + lVar24);
                                    uVar11 = *(undefined4 *)(&stack0x00001450 + lVar24);
                                    uVar12 = *(undefined4 *)(&stack0x00001454 + lVar24);
                                    uVar13 = *(undefined4 *)(&stack0x00001458 + lVar24);
                                    uVar14 = *(undefined4 *)(&stack0x0000145c + lVar24);
                                    param_2[0x12] = *(longlong *)(&stack0x00001420 + lVar24);
                                    param_2[0x13] = lVar29;
                                    *(undefined4 *)(param_2 + 0x14) = uVar3;
                                    *(undefined4 *)((longlong)param_2 + 0xa4) = uVar4;
                                    *(undefined4 *)(param_2 + 0x15) = uVar5;
                                    *(undefined4 *)((longlong)param_2 + 0xac) = uVar6;
                                    *(undefined4 *)(param_2 + 0x16) = uVar7;
                                    *(undefined4 *)((longlong)param_2 + 0xb4) = uVar8;
                                    *(undefined4 *)(param_2 + 0x17) = uVar9;
                                    *(undefined4 *)((longlong)param_2 + 0xbc) = uVar10;
                                    *(undefined4 *)(param_2 + 0x18) = uVar11;
                                    *(undefined4 *)((longlong)param_2 + 0xc4) = uVar12;
                                    *(undefined4 *)(param_2 + 0x19) = uVar13;
                                    *(undefined4 *)((longlong)param_2 + 0xcc) = uVar14;
                                    lVar29 = *(longlong *)(&stack0x00001468 + lVar24);
                                    param_2[0x1a] = *(longlong *)(&stack0x00001460 + lVar24);
                                    param_2[0x1b] = lVar29;
                                    uVar3 = *(undefined4 *)(&stack0x00001474 + lVar24);
                                    uVar4 = *(undefined4 *)(&stack0x00001478 + lVar24);
                                    uVar5 = *(undefined4 *)(&stack0x0000147c + lVar24);
                                    *(undefined4 *)(param_2 + 0x1c) =
                                         *(undefined4 *)(&stack0x00001470 + lVar24);
                                    *(undefined4 *)((longlong)param_2 + 0xe4) = uVar3;
                                    *(undefined4 *)(param_2 + 0x1d) = uVar4;
                                    *(undefined4 *)((longlong)param_2 + 0xec) = uVar5;
                                    if (*(longlong *)(&stack0x00000120 + lVar24) != 0) {
                                      *(undefined **)((longlong)&puStack_48 + lVar24) =
                                           &UNK_14016563a;
                                      FUN_14004ceb0(&stack0x00000120 + lVar24,
                                                    &stack0x00000140 + lVar24,0x68,0x10);
                                      lVar29 = *(longlong *)(&stack0x00000158 + lVar24);
                                      if ((lVar29 != 0) &&
                                         (lVar34 = lVar29 * 0x11 + 0x21, lVar34 != 0)) {
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_140165672;
                                        func_0x000140613c20(*(longlong *)(&stack0x00000150 + lVar24)
                                                            + lVar29 * -0x10 + -0x10,lVar34,0x10);
                                      }
                                    }
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_14016568b
                                    ;
                                    func_0x000142923c00(&stack0x00001390 + lVar24,
                                                        &stack0x00001620 + lVar24,0x90);
                                    *(undefined8 *)(&stack0x00001940 + lVar24) =
                                         *(undefined8 *)(&stack0x000016c0 + lVar24);
                                    *(undefined4 *)(&stack0x00001930 + lVar24) =
                                         *(undefined4 *)(&stack0x000016b0 + lVar24);
                                    *(undefined4 *)(&stack0x00001934 + lVar24) =
                                         *(undefined4 *)(&stack0x000016b4 + lVar24);
                                    *(undefined4 *)(&stack0x00001938 + lVar24) =
                                         *(undefined4 *)(&stack0x000016b8 + lVar24);
                                    *(undefined4 *)(&stack0x0000193c + lVar24) =
                                         *(undefined4 *)(&stack0x000016bc + lVar24);
                                    *(undefined8 *)(&stack0x00000120 + lVar24) = 0;
                                    *(undefined8 *)(&stack0x00000128 + lVar24) = 8;
                                    *(undefined8 *)(&stack0x00000130 + lVar24) = 0;
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_1401656cf
                                    ;
                                    plVar30 = (longlong *)FUN_1400d9b20(0);
                                    if (plVar30 != (longlong *)0x0) {
                                      lVar29 = *plVar30;
                                      lVar34 = plVar30[1];
                                      *plVar30 = *plVar30 + 1;
                                      *(undefined8 *)(&stack0x00001568 + lVar24) = _UNK_1429b0030;
                                      *(undefined8 *)(&stack0x00001570 + lVar24) = _UNK_1429b0038;
                                      *(undefined8 *)(&stack0x00001558 + lVar24) = _UNK_1429b0020;
                                      *(undefined8 *)(&stack0x00001560 + lVar24) = _UNK_1429b0028;
                                      *(longlong *)(&stack0x00001578 + lVar24) = lVar29;
                                      *(longlong *)(&stack0x00001580 + lVar24) = lVar34;
                                      *(undefined8 *)(&stack0x00001480 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001488 + lVar24) = 2;
                                      *(undefined8 *)(&stack0x00001490 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001498 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x000014a0 + lVar24) = 1;
                                      *(undefined8 *)(&stack0x000014a8 + lVar24) = 0;
                                      *(undefined4 *)(&stack0x000014b0 + lVar24) =
                                           *(undefined4 *)(&stack0x00000120 + lVar24);
                                      *(undefined4 *)(&stack0x000014b4 + lVar24) =
                                           *(undefined4 *)(&stack0x00000124 + lVar24);
                                      *(undefined4 *)(&stack0x000014b8 + lVar24) =
                                           *(undefined4 *)(&stack0x00000128 + lVar24);
                                      *(undefined4 *)(&stack0x000014bc + lVar24) =
                                           *(undefined4 *)(&stack0x0000012c + lVar24);
                                      *(undefined8 *)(&stack0x000014c0 + lVar24) =
                                           *(undefined8 *)(&stack0x00000130 + lVar24);
                                      *(undefined8 *)(&stack0x000014c8 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x000014d0 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x000014d8 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x000014e0 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x000014e8 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x000014f0 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x000014f8 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001500 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x00001508 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001510 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001518 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x00001520 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001528 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001530 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x00001538 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001540 + lVar24) = 0;
                                      *(undefined8 *)(&stack0x00001548 + lVar24) = 4;
                                      *(undefined8 *)(&stack0x00001550 + lVar24) = 0;
                                      (&stack0x000019b2)[lVar24] = 1;
                                      (&stack0x000019b1)[lVar24] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar24) =
                                           &UNK_1401657fd;
                                      FUN_140530430(&stack0x000014c8 + lVar24,
                                                    &stack0x00001390 + lVar24,
                                                    *(undefined8 *)(&stack0x00001938 + lVar24),
                                                    *(undefined8 *)(&stack0x00001940 + lVar24));
                                      (&stack0x000019b2)[lVar24] = 1;
                                      (&stack0x000019b1)[lVar24] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar24) =
                                           &UNK_14016582c;
                                      cVar22 = FUN_140be7ba0(&stack0x00001480 + lVar24,
                                                             &stack0x00001390 + lVar24,
                                                             *(undefined8 *)
                                                              (&stack0x00001910 + lVar24),
                                                             *(undefined8 *)
                                                              (&stack0x00001918 + lVar24));
                                      if (cVar22 == '\0') {
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_1401658ff;
                                        func_0x000142923c00(&stack0x00001260 + lVar24,
                                                            &stack0x00001390 + lVar24,0x90);
                                        *(undefined8 *)(&stack0x00001970 + lVar24) =
                                             *(undefined8 *)(&stack0x00001940 + lVar24);
                                        *(undefined4 *)(&stack0x00001960 + lVar24) =
                                             *(undefined4 *)(&stack0x00001930 + lVar24);
                                        *(undefined4 *)(&stack0x00001964 + lVar24) =
                                             *(undefined4 *)(&stack0x00001934 + lVar24);
                                        *(undefined4 *)(&stack0x00001968 + lVar24) =
                                             *(undefined4 *)(&stack0x00001938 + lVar24);
                                        *(undefined4 *)(&stack0x0000196c + lVar24) =
                                             *(undefined4 *)(&stack0x0000193c + lVar24);
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_140165937;
                                        func_0x000142923c00(&stack0x00001730 + lVar24,
                                                            &stack0x00001480 + lVar24,0x108);
                                        (&stack0x000019b2)[lVar24] = 0;
                                        (&stack0x000019b1)[lVar24] = 0;
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_14016595e;
                                        FUN_1402c4280(&stack0x00000120 + lVar24,
                                                      &stack0x00001260 + lVar24,
                                                      &stack0x00001960 + lVar24,
                                                      &stack0x00001730 + lVar24);
                                        puVar35 = *(undefined8 **)(&stack0x00001928 + lVar24);
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_14016597a;
                                        func_0x000142923c00(puVar35,&stack0x00000120 + lVar24,0x218)
                                        ;
                                        *(undefined4 *)(puVar35 + 0x43) = uVar23;
                                      }
                                      else {
                                        (&stack0x000019b2)[lVar24] = 1;
                                        (&stack0x000019b1)[lVar24] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_14016584e;
                                        FUN_140be9420(&stack0x00001390 + lVar24);
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_140165863;
                                        func_0x000142923c00(&stack0x00000120 + lVar24,param_2,0x90);
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_140165878;
                                        func_0x000142923c00(param_2,&stack0x00001390 + lVar24,0x90);
                                        if (!SBORROW8(0,*(longlong *)(&stack0x00000120 + lVar24))) {
                                          (&stack0x000019b2)[lVar24] = 0;
                                          (&stack0x000019b1)[lVar24] = 1;
                                          *(undefined **)((longlong)&puStack_48 + lVar24) =
                                               &UNK_14016589d;
                                          FUN_1405ef210(&stack0x00000120 + lVar24);
                                        }
                                        puVar35 = *(undefined8 **)(&stack0x00001928 + lVar24);
                                        *puVar35 = 0x8000000000000000;
                                        (&stack0x000019b4)[lVar24] = 0;
                                        (&stack0x000019b3)[lVar24] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_1401658cb;
                                        FUN_1405fd1d0(&stack0x00001480 + lVar24);
                                        (&stack0x000019ad)[lVar24] = 0;
                                        *(undefined **)((longlong)&puStack_48 + lVar24) =
                                             &UNK_1401658de;
                                        FUN_1405da820(&stack0x00001930 + lVar24);
                                      }
                                      return puVar35;
                                    }
                                    *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165c2a
                                    ;
                                    FUN_1428d80d0(&UNK_142932a10);
                                    goto code_r0x000140165c52;
                                  }
                                }
                                *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165be1;
                                FUN_1428d9310(&UNK_14295d9c8);
                                goto code_r0x000140165c52;
                              }
                            }
                            *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165bd3;
                            FUN_1428d9310(&UNK_14295d9b0);
                          }
                          goto code_r0x000140165c52;
                        }
                      }
                      (&stack0x000019b7)[lVar24] = 1;
                      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165bc2;
                      FUN_1428d9310(&UNK_14295d998);
                      goto code_r0x000140165c52;
                    }
                  }
                  (&stack0x000019b7)[lVar24] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165baa;
                  FUN_1428d9310(&UNK_14295d980);
                  goto code_r0x000140165c52;
                }
              }
              (&stack0x000019b6)[lVar24] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b92;
              FUN_1428d9310(&UNK_14295d958);
              goto code_r0x000140165c52;
            }
          }
          (&stack0x000019b6)[lVar24] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b7a;
          FUN_1428d9310(&UNK_14295d940);
          goto code_r0x000140165c52;
        }
      }
      (&stack0x000019b5)[lVar24] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165b62;
      FUN_1428d9310(&UNK_14295d928);
      goto code_r0x000140165c52;
    }
  }
  *(undefined **)((longlong)&puStack_48 + lVar24) = &UNK_140165bfb;
  FUN_1428d9310(&UNK_14295d5f0);
code_r0x000140165c52:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

