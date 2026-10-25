
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140161cd0(undefined8 param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  undefined1 *puVar1;
  undefined1 uVar2;
  int iVar3;
  undefined8 *puVar4;
  code *pcVar5;
  undefined4 uVar6;
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
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  longlong lVar23;
  longlong lVar24;
  longlong lVar25;
  char cVar26;
  byte bVar27;
  undefined4 uVar28;
  undefined4 uVar29;
  longlong lVar30;
  longlong lVar31;
  undefined8 uVar32;
  char *pcVar33;
  longlong lVar34;
  undefined4 *puVar35;
  undefined8 uVar36;
  undefined8 uVar37;
  longlong lVar38;
  longlong *plVar39;
  longlong lVar40;
  ulonglong uVar41;
  longlong lVar42;
  byte bVar43;
  byte *pbVar44;
  bool bVar45;
  undefined4 unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined4 auStackX_8 [2];
  undefined1 auStackX_10 [24];
  undefined *puStack_48;
  
  puStack_48 = &UNK_140161ce6;
  lVar30 = FUN_14289aa40();
  lVar30 = -lVar30;
  *(undefined4 *)(&stack0x00001ae0 + lVar30) = unaff_XMM6_Da;
  *(undefined4 *)(&stack0x00001ae4 + lVar30) = unaff_XMM6_Db;
  *(undefined4 *)(&stack0x00001ae8 + lVar30) = unaff_XMM6_Dc;
  *(undefined4 *)(&stack0x00001aec + lVar30) = unaff_XMM6_Dd;
  *(undefined8 *)(&stack0x00001ad8 + lVar30) = 0xfffffffffffffffe;
  lVar38 = *param_2;
  *(undefined8 *)(&stack0x00001a48 + lVar30) = 0x8000000000000000;
  *param_2 = -0x8000000000000000;
  if (SBORROW8(0,lVar38)) {
    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140164069;
    FUN_1428d9310(&UNK_14295d5d8,-lVar38);
  }
  else {
    *(undefined8 *)(&stack0x000017d8 + lVar30) = param_4;
    *(undefined8 *)(&stack0x000017d0 + lVar30) = param_1;
    *(longlong *)(&stack0x000011d8 + lVar30) = lVar38;
    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161d57;
    func_0x000142923c00(&stack0x000011e0 + lVar30,param_2 + 1,0x88);
    lVar38 = param_2[0x12];
    param_2[0x12] = 0;
    *(longlong *)(&stack0x000017e8 + lVar30) = lVar38;
    if (lVar38 != 0) {
      plVar39 = *(longlong **)(&stack0x00001b60 + lVar30);
      *(longlong *)(&stack0x00001370 + lVar30) = param_2[0x1d];
      lVar42 = param_2[0x1c];
      *(longlong *)(&stack0x00001360 + lVar30) = param_2[0x1b];
      *(longlong *)(&stack0x00001368 + lVar30) = lVar42;
      lVar42 = param_2[0x13];
      uVar28 = *(undefined4 *)((longlong)param_2 + 0x9c);
      lVar34 = param_2[0x14];
      uVar29 = *(undefined4 *)((longlong)param_2 + 0xa4);
      lVar31 = param_2[0x15];
      uVar6 = *(undefined4 *)((longlong)param_2 + 0xac);
      lVar40 = param_2[0x16];
      uVar7 = *(undefined4 *)((longlong)param_2 + 0xb4);
      lVar23 = param_2[0x17];
      uVar8 = *(undefined4 *)((longlong)param_2 + 0xbc);
      lVar24 = param_2[0x18];
      uVar9 = *(undefined4 *)((longlong)param_2 + 0xc4);
      *(longlong **)(&stack0x000019f0 + lVar30) = param_2;
      uVar10 = *(undefined4 *)((longlong)param_2 + 0xcc);
      lVar25 = param_2[0x1a];
      uVar11 = *(undefined4 *)((longlong)param_2 + 0xd4);
      *(int *)(&stack0x00001350 + lVar30) = (int)param_2[0x19];
      *(undefined4 *)(&stack0x00001354 + lVar30) = uVar10;
      *(int *)(&stack0x00001358 + lVar30) = (int)lVar25;
      *(undefined4 *)(&stack0x0000135c + lVar30) = uVar11;
      *(int *)(&stack0x00001340 + lVar30) = (int)lVar23;
      *(undefined4 *)(&stack0x00001344 + lVar30) = uVar8;
      *(int *)(&stack0x00001348 + lVar30) = (int)lVar24;
      *(undefined4 *)(&stack0x0000134c + lVar30) = uVar9;
      *(int *)(&stack0x00001330 + lVar30) = (int)lVar31;
      *(undefined4 *)(&stack0x00001334 + lVar30) = uVar6;
      *(int *)(&stack0x00001338 + lVar30) = (int)lVar40;
      *(undefined4 *)(&stack0x0000133c + lVar30) = uVar7;
      *(int *)(&stack0x00001320 + lVar30) = (int)lVar42;
      *(undefined4 *)(&stack0x00001324 + lVar30) = uVar28;
      *(int *)(&stack0x00001328 + lVar30) = (int)lVar34;
      *(undefined4 *)(&stack0x0000132c + lVar30) = uVar29;
      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161df4;
      func_0x000142923c00(&stack0x00001268 + lVar30,&stack0x000011d8 + lVar30,0x90);
      lVar42 = *plVar39;
      lVar34 = plVar39[1];
      *(longlong *)(&stack0x00001a88 + lVar30) = plVar39[2];
      pbVar44 = (byte *)plVar39[3];
      *(longlong *)(&stack0x00001a40 + lVar30) = plVar39[4];
      *(longlong *)(&stack0x00001a30 + lVar30) = plVar39[5];
      *(longlong *)(&stack0x000017c0 + lVar30) = plVar39[6];
      *(longlong *)(&stack0x000019f8 + lVar30) = plVar39[7];
      *(undefined8 *)(&stack0x000012f8 + lVar30) = 0;
      *(undefined8 *)(&stack0x00001300 + lVar30) = 8;
      *(undefined8 *)(&stack0x00001308 + lVar30) = 0;
      *(undefined8 *)(&stack0x000017c8 + lVar30) = param_3;
      *(undefined8 *)(&stack0x00001310 + lVar30) = param_3;
      *(longlong *)(&stack0x00001318 + lVar30) = lVar38;
      lVar38 = lVar42 + 0x38;
      (&stack0x00001ad3)[lVar30] = 1;
      uVar41 = 0xa68561b396784eaa;
      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161e93;
      lVar31 = FUN_1400dd6f0(lVar38,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
      if (((lVar31 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
         (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8), uVar41 != 0)) {
        lVar31 = ~uVar41 * 0x60;
        if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar31) != 0) {
          lVar31 = *(longlong *)(lVar42 + 0x1e0) + lVar31;
          uVar32 = *(undefined8 *)(lVar31 + 0x10);
          uVar28 = *(undefined4 *)(lVar42 + 0x5b0);
          *(undefined4 *)(lVar31 + 0x5c) = uVar28;
          (&stack0x00001ad3)[lVar30] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161f03;
          FUN_14093cd30(uVar32,&stack0x00001268 + lVar30);
          *(undefined4 *)(lVar31 + 0x5c) = uVar28;
          (&stack0x00001ad3)[lVar30] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161f19;
          uVar32 = FUN_14093d040(uVar32,lVar34);
          (&stack0x00001ad3)[lVar30] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161f2f;
          func_0x000140bd6b00(&stack0x000015a8 + lVar30,uVar32);
          (&stack0x00001ad4)[lVar30] = 1;
          uVar41 = 0x662978458699a5ef;
          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161f52;
          lVar31 = FUN_1400dd6f0(lVar38,0x662978458699a5ef,0x3ec057c162419b46);
          if (((lVar31 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
             (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8), uVar41 != 0)) {
            lVar31 = ~uVar41 * 0x60;
            if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar31) != 0) {
              lVar31 = *(longlong *)(lVar42 + 0x1e0) + lVar31;
              uVar32 = *(undefined8 *)(lVar31 + 0x10);
              *(undefined4 *)(lVar31 + 0x5c) = *(undefined4 *)(lVar42 + 0x5b0);
              (&stack0x00001ad4)[lVar30] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161fc1;
              FUN_1409921c0(uVar32,&stack0x00001268 + lVar30);
              (&stack0x00001ad4)[lVar30] = 1;
              uVar41 = 0x8fd9dc19c7973bca;
              *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140161fe4;
              lVar31 = FUN_1400dd6f0(lVar38,0x8fd9dc19c7973bca,0x361ff4c5eb9b45fd);
              if (((lVar31 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                 (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8), uVar41 != 0))
              {
                lVar31 = ~uVar41 * 0x60;
                if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar31) != 0) {
                  uVar32 = *(undefined8 *)(*(longlong *)(lVar42 + 0x1e0) + lVar31 + 0x10);
                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162045;
                  func_0x000142923c00(&stack0x00000120 + lVar30,uVar32,0x1090);
                  (&stack0x00001ad4)[lVar30] = 1;
                  uVar41 = 0x47c94cb78283a72e;
                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162068;
                  lVar31 = FUN_1400dd6f0(lVar38,0x47c94cb78283a72e,0x8c173ded97323e9d);
                  if (((lVar31 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                     (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                     uVar41 != 0)) {
                    lVar31 = ~uVar41 * 0x60;
                    if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar31) != 0) {
                      uVar32 = *(undefined8 *)(*(longlong *)(lVar42 + 0x1e0) + lVar31 + 0x10);
                      (&stack0x00001ad4)[lVar30] = 1;
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401620c3;
                      pcVar33 = (char *)func_0x00014067dd80(uVar32);
                      LOCK();
                      cVar26 = *pcVar33;
                      if (cVar26 == '\0') {
                        *pcVar33 = '\x01';
                      }
                      UNLOCK();
                      if (cVar26 != '\0') {
                        (&stack0x00001ad4)[lVar30] = 1;
                        *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163ea5;
                        FUN_1428cd210(pcVar33);
                      }
                      (&stack0x00001ad4)[lVar30] = 1;
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401620e3;
                      FUN_140c81c70(pcVar33);
                      *(char **)(&stack0x00001ac0 + lVar30) = pcVar33;
                      *(undefined8 *)(&stack0xffffffffffffffe0 + lVar30) = 0;
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016210f;
                      FUN_140c02740(&stack0x00001850 + lVar30,pcVar33 + 8,lVar34,0x1090);
                      if (*(longlong *)(&stack0x00001858 + lVar30) != 0x1090) {
                        *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163ebb;
                        FUN_1428d98e0(*(longlong *)(&stack0x00001858 + lVar30),0x1090,&UNK_1429de8b0
                                     );
                        goto code_r0x0001401640c2;
                      }
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016213c;
                      func_0x000142923c00(*(undefined8 *)(&stack0x00001850 + lVar30),
                                          &stack0x00000120 + lVar30,0x1090);
                      *(undefined4 *)(&stack0x00001620 + lVar30) =
                           *(undefined4 *)(&stack0x00001850 + lVar30);
                      *(undefined4 *)(&stack0x00001624 + lVar30) =
                           *(undefined4 *)(&stack0x00001854 + lVar30);
                      *(undefined4 *)(&stack0x00001628 + lVar30) =
                           *(undefined4 *)(&stack0x00001858 + lVar30);
                      *(undefined4 *)(&stack0x0000162c + lVar30) =
                           *(undefined4 *)(&stack0x0000185c + lVar30);
                      *(undefined4 *)(&stack0x00001630 + lVar30) =
                           *(undefined4 *)(&stack0x00001860 + lVar30);
                      *(undefined4 *)(&stack0x00001634 + lVar30) =
                           *(undefined4 *)(&stack0x00001864 + lVar30);
                      *(undefined4 *)(&stack0x00001638 + lVar30) =
                           *(undefined4 *)(&stack0x00001868 + lVar30);
                      *(undefined4 *)(&stack0x0000163c + lVar30) =
                           *(undefined4 *)(&stack0x0000186c + lVar30);
                      *(undefined4 *)(&stack0x00001640 + lVar30) =
                           *(undefined4 *)(&stack0x00001870 + lVar30);
                      *(undefined4 *)(&stack0x00001644 + lVar30) =
                           *(undefined4 *)(&stack0x00001874 + lVar30);
                      *(undefined4 *)(&stack0x00001648 + lVar30) =
                           *(undefined4 *)(&stack0x00001878 + lVar30);
                      *(undefined4 *)(&stack0x0000164c + lVar30) =
                           *(undefined4 *)(&stack0x0000187c + lVar30);
                      *(undefined4 *)(&stack0x00001650 + lVar30) =
                           *(undefined4 *)(&stack0x00001880 + lVar30);
                      *(undefined4 *)(&stack0x00001654 + lVar30) =
                           *(undefined4 *)(&stack0x00001884 + lVar30);
                      *(undefined4 *)(&stack0x00001658 + lVar30) =
                           *(undefined4 *)(&stack0x00001888 + lVar30);
                      *(undefined4 *)(&stack0x0000165c + lVar30) =
                           *(undefined4 *)(&stack0x0000188c + lVar30);
                      *(undefined8 *)(&stack0x00001660 + lVar30) =
                           *(undefined8 *)(&stack0x00001890 + lVar30);
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162198;
                      FUN_140c02370(&stack0x00001378 + lVar30,&stack0x00001620 + lVar30,lVar34);
                      (&stack0x00001ace)[lVar30] = 1;
                      pcVar33 = *(char **)(&stack0x00001ac0 + lVar30);
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401621ae;
                      FUN_140c81e10();
                      LOCK();
                      cVar26 = *pcVar33;
                      if (cVar26 == '\x01') {
                        *pcVar33 = '\0';
                      }
                      UNLOCK();
                      if (cVar26 != '\x01') {
                        (&stack0x00001ace)[lVar30] = 1;
                        *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163ed1;
                        FUN_1428cdb90(pcVar33,0);
                      }
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401621d5;
                      func_0x000142923c00(&stack0x00001620 + lVar30,&stack0x00001268 + lVar30,0x110)
                      ;
                      *(undefined8 *)(&stack0x00001730 + lVar30) =
                           *(undefined8 *)(&stack0x00001378 + lVar30);
                      *(undefined8 *)(&stack0x00001738 + lVar30) =
                           *(undefined8 *)(&stack0x00001380 + lVar30);
                      *(undefined4 *)(&stack0x00001740 + lVar30) =
                           *(undefined4 *)(&stack0x00001388 + lVar30);
                      *(undefined4 *)(&stack0x00001744 + lVar30) =
                           *(undefined4 *)(&stack0x0000138c + lVar30);
                      *(undefined4 *)(&stack0x00001748 + lVar30) =
                           *(undefined4 *)(&stack0x00001390 + lVar30);
                      *(undefined4 *)(&stack0x0000174c + lVar30) =
                           *(undefined4 *)(&stack0x00001394 + lVar30);
                      *(undefined4 *)(&stack0x00001750 + lVar30) =
                           *(undefined4 *)(&stack0x00001398 + lVar30);
                      *(undefined4 *)(&stack0x00001754 + lVar30) =
                           *(undefined4 *)(&stack0x0000139c + lVar30);
                      *(undefined4 *)(&stack0x00001758 + lVar30) =
                           *(undefined4 *)(&stack0x000013a0 + lVar30);
                      *(undefined4 *)(&stack0x0000175c + lVar30) =
                           *(undefined4 *)(&stack0x000013a4 + lVar30);
                      *(undefined4 *)(&stack0x00001760 + lVar30) =
                           *(undefined4 *)(&stack0x000013a8 + lVar30);
                      *(undefined4 *)(&stack0x00001764 + lVar30) =
                           *(undefined4 *)(&stack0x000013ac + lVar30);
                      *(undefined4 *)(&stack0x00001768 + lVar30) =
                           *(undefined4 *)(&stack0x000013b0 + lVar30);
                      *(undefined4 *)(&stack0x0000176c + lVar30) =
                           *(undefined4 *)(&stack0x000013b4 + lVar30);
                      *(undefined8 *)(&stack0x00001770 + lVar30) =
                           *(undefined8 *)(&stack0x000013b8 + lVar30);
                      *(undefined8 *)(&stack0x00001778 + lVar30) =
                           *(undefined8 *)(&stack0x000013c0 + lVar30);
                      *(undefined8 *)(&stack0x00001780 + lVar30) =
                           *(undefined8 *)(&stack0x000013c8 + lVar30);
                      *(undefined8 *)(&stack0x00001788 + lVar30) =
                           *(undefined8 *)(&stack0x000013d0 + lVar30);
                      *(undefined4 *)(&stack0x00001790 + lVar30) =
                           *(undefined4 *)(&stack0x000015a8 + lVar30);
                      *(undefined4 *)(&stack0x00001794 + lVar30) =
                           *(undefined4 *)(&stack0x000015ac + lVar30);
                      *(undefined4 *)(&stack0x00001798 + lVar30) =
                           *(undefined4 *)(&stack0x000015b0 + lVar30);
                      *(undefined4 *)(&stack0x0000179c + lVar30) =
                           *(undefined4 *)(&stack0x000015b4 + lVar30);
                      *(undefined8 *)(&stack0x000017a0 + lVar30) =
                           *(undefined8 *)(&stack0x000015b8 + lVar30);
                      *(undefined8 *)(&stack0x000017a8 + lVar30) =
                           *(undefined8 *)(&stack0x00001a88 + lVar30);
                      *(longlong *)(&stack0x000017b0 + lVar30) = lVar34;
                      (&stack0x00001ad7)[lVar30] = 1;
                      uVar41 = 0x162d45f443f2465a;
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016227d;
                      lVar34 = FUN_1400dd6f0(lVar38,0x162d45f443f2465a,0x7db0d9595508fc0f);
                      if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                         (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                         uVar41 != 0)) {
                        lVar34 = ~uVar41 * 0x60;
                        if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) != 0) {
                          lVar34 = *(longlong *)(lVar42 + 0x1e0) + lVar34;
                          uVar32 = *(undefined8 *)(lVar34 + 0x10);
                          *(undefined4 *)(lVar34 + 0x5c) = *(undefined4 *)(lVar42 + 0x5b0);
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401622ec;
                          FUN_14090e280(uVar32,&stack0x00001620 + lVar30,lVar42);
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162302;
                          FUN_14090a870(&stack0x00001620 + lVar30,lVar42);
                          uVar32 = *(undefined8 *)pbVar44;
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016231e;
                          func_0x000140bd52a0(&stack0x00001850 + lVar30,uVar32,0x7a);
                          (&stack0x00001ad7)[lVar30] = 1;
                          puVar1 = &stack0x00001a50 + lVar30;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016233e;
                          func_0x000140bd5460(puVar1,&stack0x00001850 + lVar30,6);
                          *(undefined4 *)(&stack0x000014a0 + lVar30) =
                               *(undefined4 *)(&stack0x00001a60 + lVar30);
                          *(undefined4 *)(&stack0x000014a4 + lVar30) =
                               *(undefined4 *)(&stack0x00001a64 + lVar30);
                          *(undefined4 *)(&stack0x000014a8 + lVar30) =
                               *(undefined4 *)(&stack0x00001a68 + lVar30);
                          *(undefined4 *)(&stack0x000014ac + lVar30) =
                               *(undefined4 *)(&stack0x00001a6c + lVar30);
                          *(undefined4 *)(&stack0x00001490 + lVar30) =
                               *(undefined4 *)(&stack0x00001a50 + lVar30);
                          *(undefined4 *)(&stack0x00001494 + lVar30) =
                               *(undefined4 *)(&stack0x00001a54 + lVar30);
                          *(undefined4 *)(&stack0x00001498 + lVar30) =
                               *(undefined4 *)(&stack0x00001a58 + lVar30);
                          *(undefined4 *)(&stack0x0000149c + lVar30) =
                               *(undefined4 *)(&stack0x00001a5c + lVar30);
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162374;
                          func_0x000140bd6ce0(&stack0x00001860 + lVar30,&stack0x00001490 + lVar30);
                          *(undefined **)(&stack0x00001850 + lVar30) = &UNK_14295d6a8;
                          *(undefined8 *)(&stack0x00001858 + lVar30) = 10;
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401623a7;
                          uVar28 = FUN_140be94d0(&stack0x00001620 + lVar30,&stack0x00001850 + lVar30
                                                );
                          *(undefined4 *)(&stack0x00001a04 + lVar30) =
                               *(undefined4 *)(&stack0x00001a50 + lVar30);
                          *(undefined4 *)(&stack0x00001a08 + lVar30) =
                               *(undefined4 *)(&stack0x00001a54 + lVar30);
                          *(undefined4 *)(&stack0x00001a0c + lVar30) =
                               *(undefined4 *)(&stack0x00001a58 + lVar30);
                          *(undefined4 *)(&stack0x00001a10 + lVar30) =
                               *(undefined4 *)(&stack0x00001a5c + lVar30);
                          *(undefined4 *)(&stack0x00001a14 + lVar30) =
                               *(undefined4 *)(&stack0x00001a60 + lVar30);
                          *(undefined4 *)(&stack0x00001a18 + lVar30) =
                               *(undefined4 *)(&stack0x00001a64 + lVar30);
                          *(undefined4 *)(&stack0x00001a1c + lVar30) =
                               *(undefined4 *)(&stack0x00001a68 + lVar30);
                          *(undefined4 *)(&stack0x00001a20 + lVar30) =
                               *(undefined4 *)(&stack0x00001a6c + lVar30);
                          *(undefined4 *)(&stack0x00001a00 + lVar30) = uVar28;
                          (&stack0x00001ad7)[lVar30] = 1;
                          uVar41 = 0xd38dae617fb79c7a;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401623ec;
                          lVar34 = FUN_1400dd6f0(lVar38,0xd38dae617fb79c7a,0x22aadf5c5dad1eae);
                          if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                             (uVar41 = *(ulonglong *)(*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                             uVar41 != 0)) {
                            lVar34 = ~uVar41 * 0x60;
                            if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) != 0) {
                              uVar32 = *(undefined8 *)
                                        (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                              (&stack0x00001ad7)[lVar30] = 1;
                              uVar41 = 0x35385c45b6f578e9;
                              *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016245e;
                              lVar34 = FUN_1400dd6f0(lVar38,0x35385c45b6f578e9,0xf1623d81d5dd0c9f);
                              if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                 (uVar41 = *(ulonglong *)
                                            (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                 uVar41 != 0)) {
                                lVar34 = ~uVar41 * 0x60;
                                if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) !=
                                    0) {
                                  if (**(char **)(*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10) !=
                                      '\x01') {
code_r0x0001401627b4:
                                    *(undefined4 *)(&stack0x00001a88 + lVar30) = 0;
code_r0x0001401627be:
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    uVar41 = 0x6831e8f98c915159;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401627e1
                                    ;
                                    lVar34 = FUN_1400dd6f0(lVar38,0x6831e8f98c915159,
                                                           0xd515c479e4000725);
                                    if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218)))
                                       && (uVar41 = *(ulonglong *)
                                                     (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                          uVar41 != 0)) {
                                      lVar34 = ~uVar41 * 0x60;
                                      if (*(longlong *)
                                           (*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) != 0) {
                                        uVar37 = *(undefined8 *)
                                                  (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_14016283c;
                                        uVar41 = func_0x00014091f0f0(uVar37);
                                        if ((char)uVar41 != '\0') goto code_r0x000140162840;
                                        *(undefined4 *)(&stack0x00001ac0 + lVar30) = 0;
                                        goto code_r0x00014016287e;
                                      }
                                    }
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016405b
                                    ;
                                    FUN_1428d9310(&UNK_14295d780);
                                    goto code_r0x0001401640c2;
                                  }
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401624c5;
                                  cVar26 = func_0x00014093fc20(uVar32);
                                  if (cVar26 == '\0') goto code_r0x0001401627b4;
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  uVar41 = 0xd38dae617fb79c7a;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401624f0;
                                  lVar34 = FUN_1400dd6f0(lVar38,0xd38dae617fb79c7a,
                                                         0x22aadf5c5dad1eae);
                                  if (((lVar34 == 0) || (*(ulonglong *)(lVar42 + 0x218) <= uVar41))
                                     || (uVar41 = *(ulonglong *)
                                                   (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                        uVar41 == 0)) {
code_r0x000140164077:
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016408a
                                    ;
                                    FUN_1428d9310(&UNK_14295d6e8);
                                    goto code_r0x0001401640c2;
                                  }
                                  lVar34 = ~uVar41 * 0x60;
                                  if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34)
                                      == 0) goto code_r0x000140164077;
                                  uVar37 = *(undefined8 *)
                                            (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016254b;
                                  uVar28 = func_0x00014093fbc0(uVar37);
                                  *(undefined4 *)(&stack0x00001a88 + lVar30) = uVar28;
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162560;
                                  uVar41 = func_0x00014093fbe0(uVar32);
                                  if ((char)uVar41 != '\0') goto code_r0x000140162840;
                                  if ((&stack0x00001a88)[lVar30] != '\0') {
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    uVar41 = 0x1f18d9c9bdba6c60;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162598
                                    ;
                                    lVar34 = FUN_1400dd6f0(lVar38,0x1f18d9c9bdba6c60,
                                                           0x8547986648e90b67);
                                    if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218)))
                                       && (uVar41 = *(ulonglong *)
                                                     (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                          uVar41 != 0)) {
                                      lVar34 = ~uVar41 * 0x60;
                                      if (*(longlong *)
                                           (*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) != 0) {
                                        uVar37 = *(undefined8 *)
                                                  (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_1401625f3;
                                        cVar26 = func_0x0001408a63d0(uVar37);
                                        *(undefined **)(&stack0x00001a50 + lVar30) = &UNK_14295d718;
                                        *(undefined8 *)(&stack0x00001a58 + lVar30) = 0x18;
                                        if (cVar26 == '\0') {
                                          uVar41 = *puRam000000014362c3a8;
                                          if (4 < uVar41) {
                                            *(undefined1 **)(&stack0x00001490 + lVar30) = puVar1;
                                            *(undefined **)(&stack0x00001498 + lVar30) =
                                                 &UNK_140109e90;
                                            (&stack0x00001ad7)[lVar30] = 1;
                                            *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                 &UNK_140163dcb;
                                            puVar35 = (undefined4 *)
                                                      func_0x00014067dd80(&UNK_1429ebb58);
                                            uVar29 = *puVar35;
                                            uVar6 = puVar35[1];
                                            uVar7 = puVar35[2];
                                            uVar8 = puVar35[3];
                                            uVar28 = puVar35[4];
                                            *(undefined8 *)(&stack0x00001880 + lVar30) = 5;
                                            *(undefined **)(&stack0x00001888 + lVar30) =
                                                 &UNK_1429ebb70;
                                            *(undefined8 *)(&stack0x00001890 + lVar30) = 0x17;
                                            *(undefined **)(&stack0x000018a0 + lVar30) =
                                                 &UNK_1429ebb20;
                                            *(undefined8 *)(&stack0x000018a8 + lVar30) = 1;
                                            *(undefined1 **)(&stack0x000018b0 + lVar30) =
                                                 &stack0x00001490 + lVar30;
                                            *(undefined8 *)(&stack0x000018b8 + lVar30) = 1;
                                            *(undefined8 *)(&stack0x000018c0 + lVar30) = 0;
                                            *(undefined8 *)(&stack0x00001850 + lVar30) = 0;
                                            *(undefined **)(&stack0x00001858 + lVar30) =
                                                 &UNK_1429ebb70;
                                            *(undefined8 *)(&stack0x00001860 + lVar30) = 0x17;
                                            *(undefined8 *)(&stack0x00001868 + lVar30) = 0;
                                            *(undefined4 *)(&stack0x00001870 + lVar30) = uVar29;
                                            *(undefined4 *)(&stack0x00001874 + lVar30) = uVar6;
                                            *(undefined4 *)(&stack0x00001878 + lVar30) = uVar7;
                                            *(undefined4 *)(&stack0x0000187c + lVar30) = uVar8;
                                            *(undefined4 *)(&stack0x00001898 + lVar30) = 1;
                                            *(undefined4 *)(&stack0x0000189c + lVar30) = uVar28;
                                            (&stack0x00001ad7)[lVar30] = 1;
                                            *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                 &UNK_140163e8b;
                                            uVar41 = func_0x000140d4a330(&stack0x00001a90 + lVar30,
                                                                         &stack0x00001850 + lVar30);
                                          }
                                          goto code_r0x000140162840;
                                        }
                                        goto code_r0x000140162614;
                                      }
                                    }
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401640c2
                                    ;
                                    FUN_1428d9310(&UNK_14295d700);
                                    goto code_r0x0001401640c2;
                                  }
code_r0x000140162614:
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  uVar41 = 0x8fd9dc19c7973bca;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162637;
                                  lVar34 = FUN_1400dd6f0(lVar38,0x8fd9dc19c7973bca,
                                                         0x361ff4c5eb9b45fd);
                                  if (((lVar34 == 0) || (*(ulonglong *)(lVar42 + 0x218) <= uVar41))
                                     || (uVar41 = *(ulonglong *)
                                                   (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                        uVar41 == 0)) {
code_r0x00014016409a:
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401640ad
                                    ;
                                    FUN_1428d9310(&UNK_14295d730);
                                    goto code_r0x0001401640c2;
                                  }
                                  lVar34 = ~uVar41 * 0x60;
                                  if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34)
                                      == 0) goto code_r0x00014016409a;
                                  uVar37 = *(undefined8 *)
                                            (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162692;
                                  cVar26 = func_0x000140976880(uVar37);
                                  *(undefined **)(&stack0x00001a50 + lVar30) = &UNK_14295d748;
                                  *(undefined8 *)(&stack0x00001a58 + lVar30) = 0x14;
                                  if (cVar26 == '\0') {
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163c6d
                                    ;
                                    cVar26 = func_0x00014093fc60(uVar32);
                                    *(undefined **)(&stack0x00001a50 + lVar30) = &UNK_14295d75c;
                                    *(undefined8 *)(&stack0x00001a58 + lVar30) = 0x1f;
                                    if (cVar26 == '\0') {
                                      uVar41 = *puRam000000014362c3a8;
                                      if (4 < uVar41) {
                                        *(undefined1 **)(&stack0x00001490 + lVar30) = puVar1;
                                        *(undefined **)(&stack0x00001498 + lVar30) = &UNK_140109e90;
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140163cca;
                                        puVar35 = (undefined4 *)func_0x00014067dd80(&UNK_1429ebb58);
                                        uVar29 = *puVar35;
                                        uVar6 = puVar35[1];
                                        uVar7 = puVar35[2];
                                        uVar8 = puVar35[3];
                                        uVar28 = puVar35[4];
                                        *(undefined8 *)(&stack0x00001880 + lVar30) = 5;
                                        *(undefined **)(&stack0x00001888 + lVar30) = &UNK_1429ebb70;
                                        *(undefined8 *)(&stack0x00001890 + lVar30) = 0x17;
                                        *(undefined **)(&stack0x000018a0 + lVar30) = &UNK_1429ebb20;
                                        *(undefined8 *)(&stack0x000018a8 + lVar30) = 1;
                                        *(undefined1 **)(&stack0x000018b0 + lVar30) =
                                             &stack0x00001490 + lVar30;
                                        *(undefined8 *)(&stack0x000018b8 + lVar30) = 1;
                                        *(undefined8 *)(&stack0x000018c0 + lVar30) = 0;
                                        *(undefined8 *)(&stack0x00001850 + lVar30) = 0;
                                        *(undefined **)(&stack0x00001858 + lVar30) = &UNK_1429ebb70;
                                        *(undefined8 *)(&stack0x00001860 + lVar30) = 0x17;
                                        *(undefined8 *)(&stack0x00001868 + lVar30) = 0;
                                        *(undefined4 *)(&stack0x00001870 + lVar30) = uVar29;
                                        *(undefined4 *)(&stack0x00001874 + lVar30) = uVar6;
                                        *(undefined4 *)(&stack0x00001878 + lVar30) = uVar7;
                                        *(undefined4 *)(&stack0x0000187c + lVar30) = uVar8;
                                        *(undefined4 *)(&stack0x00001898 + lVar30) = 1;
                                        *(undefined4 *)(&stack0x0000189c + lVar30) = uVar28;
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140163d8a;
                                        uVar41 = func_0x000140d4a330(&stack0x00001a90 + lVar30,
                                                                     &stack0x00001850 + lVar30);
                                      }
                                      goto code_r0x000140162840;
                                    }
                                    goto code_r0x0001401627be;
                                  }
                                  uVar41 = *puRam000000014362c3a8;
                                  if (4 < uVar41) {
                                    *(undefined1 **)(&stack0x00001490 + lVar30) = puVar1;
                                    *(undefined **)(&stack0x00001498 + lVar30) = &UNK_140109e90;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401626ef
                                    ;
                                    puVar35 = (undefined4 *)func_0x00014067dd80(&UNK_1429ebb58);
                                    uVar29 = *puVar35;
                                    uVar6 = puVar35[1];
                                    uVar7 = puVar35[2];
                                    uVar8 = puVar35[3];
                                    uVar28 = puVar35[4];
                                    *(undefined8 *)(&stack0x00001880 + lVar30) = 5;
                                    *(undefined **)(&stack0x00001888 + lVar30) = &UNK_1429ebb70;
                                    *(undefined8 *)(&stack0x00001890 + lVar30) = 0x17;
                                    *(undefined **)(&stack0x000018a0 + lVar30) = &UNK_1429ebb20;
                                    *(undefined8 *)(&stack0x000018a8 + lVar30) = 1;
                                    *(undefined1 **)(&stack0x000018b0 + lVar30) =
                                         &stack0x00001490 + lVar30;
                                    *(undefined8 *)(&stack0x000018b8 + lVar30) = 1;
                                    *(undefined8 *)(&stack0x000018c0 + lVar30) = 0;
                                    *(undefined8 *)(&stack0x00001850 + lVar30) = 0;
                                    *(undefined **)(&stack0x00001858 + lVar30) = &UNK_1429ebb70;
                                    *(undefined8 *)(&stack0x00001860 + lVar30) = 0x17;
                                    *(undefined8 *)(&stack0x00001868 + lVar30) = 0;
                                    *(undefined4 *)(&stack0x00001870 + lVar30) = uVar29;
                                    *(undefined4 *)(&stack0x00001874 + lVar30) = uVar6;
                                    *(undefined4 *)(&stack0x00001878 + lVar30) = uVar7;
                                    *(undefined4 *)(&stack0x0000187c + lVar30) = uVar8;
                                    *(undefined4 *)(&stack0x00001898 + lVar30) = 1;
                                    *(undefined4 *)(&stack0x0000189c + lVar30) = uVar28;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401627af
                                    ;
                                    uVar41 = func_0x000140d4a330(&stack0x00001a90 + lVar30,
                                                                 &stack0x00001850 + lVar30);
                                  }
code_r0x000140162840:
                                  if (cRam0000000143634b90 == '\x01') {
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(int *)(&stack0x00001ac0 + lVar30) =
                                         (int)CONCAT71((int7)(uVar41 >> 8),1);
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162872
                                    ;
                                    FUN_1409beca0(&stack0x00001620 + lVar30,lVar42,
                                                  &stack0x00001a00 + lVar30);
code_r0x00014016287e:
                                    cVar26 = '\a';
code_r0x000140162880:
                                    bVar43 = (byte)pcVar33;
                                    uVar37 = *(undefined8 *)pbVar44;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_14016289c
                                    ;
                                    func_0x000140bd52a0(&stack0x00001a50 + lVar30,uVar37,0x82);
                                    *(undefined4 *)(&stack0x000014a0 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a60 + lVar30);
                                    *(undefined4 *)(&stack0x000014a4 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a64 + lVar30);
                                    *(undefined4 *)(&stack0x000014a8 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x000014ac + lVar30) =
                                         *(undefined4 *)(&stack0x00001a6c + lVar30);
                                    *(undefined4 *)(&stack0x00001490 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a50 + lVar30);
                                    *(undefined4 *)(&stack0x00001494 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a54 + lVar30);
                                    *(undefined4 *)(&stack0x00001498 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a58 + lVar30);
                                    *(undefined4 *)(&stack0x0000149c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a5c + lVar30);
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401628d2
                                    ;
                                    func_0x000140bd6ce0(&stack0x00001860 + lVar30,
                                                        &stack0x00001490 + lVar30);
                                    *(undefined **)(&stack0x00001850 + lVar30) = &UNK_14295d798;
                                    *(undefined8 *)(&stack0x00001858 + lVar30) = 10;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162905
                                    ;
                                    uVar28 = FUN_140be94d0(&stack0x00001620 + lVar30,
                                                           &stack0x00001850 + lVar30);
                                    *(undefined4 *)(&stack0x00001824 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a50 + lVar30);
                                    *(undefined4 *)(&stack0x00001828 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a54 + lVar30);
                                    *(undefined4 *)(&stack0x0000182c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a58 + lVar30);
                                    *(undefined4 *)(&stack0x00001830 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a5c + lVar30);
                                    *(undefined4 *)(&stack0x00001834 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a60 + lVar30);
                                    *(undefined4 *)(&stack0x00001838 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a64 + lVar30);
                                    *(undefined4 *)(&stack0x0000183c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x00001840 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a6c + lVar30);
                                    *(undefined4 *)(&stack0x00001820 + lVar30) = uVar28;
                                    if (cVar26 == '\a') goto code_r0x000140162930;
code_r0x000140162bed:
                                    *(undefined2 *)(&stack0x00001810 + lVar30) =
                                         *(undefined2 *)(&stack0x000015e0 + lVar30);
                                    uVar28 = *(undefined4 *)(&stack0x000015d0 + lVar30);
                                    *(undefined4 *)(&stack0x00001800 + lVar30) = uVar28;
                                    *(undefined4 *)(&stack0x00001804 + lVar30) =
                                         *(undefined4 *)(&stack0x000015d4 + lVar30);
                                    *(undefined4 *)(&stack0x00001808 + lVar30) =
                                         *(undefined4 *)(&stack0x000015d8 + lVar30);
                                    *(undefined4 *)(&stack0x0000180c + lVar30) =
                                         *(undefined4 *)(&stack0x000015dc + lVar30);
                                    *(undefined4 *)(&stack0x000017f0 + lVar30) =
                                         *(undefined4 *)(&stack0x000015c0 + lVar30);
                                    *(undefined4 *)(&stack0x000017f4 + lVar30) =
                                         *(undefined4 *)(&stack0x000015c4 + lVar30);
                                    *(undefined4 *)(&stack0x000017f8 + lVar30) =
                                         *(undefined4 *)(&stack0x000015c8 + lVar30);
                                    *(undefined4 *)(&stack0x000017fc + lVar30) =
                                         *(undefined4 *)(&stack0x000015cc + lVar30);
                                    (&stack0x00001812)[lVar30] = cVar26;
                                    (&stack0x00001813)[lVar30] = bVar43;
                                  }
                                  else {
                                    uVar37 = *(undefined8 *)pbVar44;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401629ff
                                    ;
                                    func_0x000140bd52a0(&stack0x00001a50 + lVar30,uVar37,0x82);
                                    *(undefined4 *)(&stack0x000014a0 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a60 + lVar30);
                                    *(undefined4 *)(&stack0x000014a4 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a64 + lVar30);
                                    *(undefined4 *)(&stack0x000014a8 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x000014ac + lVar30) =
                                         *(undefined4 *)(&stack0x00001a6c + lVar30);
                                    *(undefined4 *)(&stack0x00001490 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a50 + lVar30);
                                    *(undefined4 *)(&stack0x00001494 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a54 + lVar30);
                                    *(undefined4 *)(&stack0x00001498 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a58 + lVar30);
                                    *(undefined4 *)(&stack0x0000149c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a5c + lVar30);
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162a38
                                    ;
                                    func_0x000140bd6ce0(&stack0x00001860 + lVar30,
                                                        &stack0x00001490 + lVar30);
                                    *(undefined **)(&stack0x00001850 + lVar30) = &UNK_14295d798;
                                    *(undefined8 *)(&stack0x00001858 + lVar30) = 10;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162a6b
                                    ;
                                    uVar28 = FUN_140be94d0(&stack0x00001620 + lVar30,
                                                           &stack0x00001850 + lVar30);
                                    *(undefined8 *)(&stack0x00001410 + lVar30) =
                                         *(undefined8 *)(&stack0x00001a50 + lVar30);
                                    *(undefined8 *)(&stack0x00001418 + lVar30) =
                                         *(undefined8 *)(&stack0x00001a58 + lVar30);
                                    *(undefined8 *)(&stack0x00001420 + lVar30) =
                                         *(undefined8 *)(&stack0x00001a60 + lVar30);
                                    *(undefined8 *)(&stack0x00001428 + lVar30) =
                                         *(undefined8 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x0000140c + lVar30) = uVar28;
                                    uVar37 = *(undefined8 *)pbVar44;
                                    *(undefined4 *)(&stack0x000014b0 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a20 + lVar30);
                                    *(undefined4 *)(&stack0x000014a0 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a10 + lVar30);
                                    *(undefined4 *)(&stack0x000014a4 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a14 + lVar30);
                                    *(undefined4 *)(&stack0x000014a8 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a18 + lVar30);
                                    *(undefined4 *)(&stack0x000014ac + lVar30) =
                                         *(undefined4 *)(&stack0x00001a1c + lVar30);
                                    *(undefined4 *)(&stack0x00001490 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a00 + lVar30);
                                    *(undefined4 *)(&stack0x00001494 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a04 + lVar30);
                                    *(undefined4 *)(&stack0x00001498 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a08 + lVar30);
                                    *(undefined4 *)(&stack0x0000149c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a0c + lVar30);
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined1 **)(&stack0xfffffffffffffff8 + lVar30) =
                                         &stack0x00001490 + lVar30;
                                    *(undefined1 **)(&stack0xfffffffffffffff0 + lVar30) =
                                         &stack0x0000140c + lVar30;
                                    *(undefined8 *)(&stack0xffffffffffffffe8 + lVar30) =
                                         *(undefined8 *)(&stack0x00001a30 + lVar30);
                                    *(undefined8 *)(&stack0xffffffffffffffe0 + lVar30) = uVar37;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162afe
                                    ;
                                    FUN_1404ec110(&stack0x00001850 + lVar30,
                                                  &stack0x00001620 + lVar30,lVar42,
                                                  *(undefined8 *)(&stack0x00001a40 + lVar30));
                                    *(undefined8 *)(&stack0x000015f0 + lVar30) =
                                         *(undefined8 *)(&stack0x00001850 + lVar30);
                                    *(undefined8 *)(&stack0x000015f8 + lVar30) =
                                         *(undefined8 *)(&stack0x00001858 + lVar30);
                                    *(undefined8 *)(&stack0x00001600 + lVar30) =
                                         *(undefined8 *)(&stack0x00001860 + lVar30);
                                    *(undefined8 *)(&stack0x00001608 + lVar30) =
                                         *(undefined8 *)(&stack0x00001868 + lVar30);
                                    *(undefined2 *)(&stack0x00001610 + lVar30) =
                                         *(undefined2 *)(&stack0x00001870 + lVar30);
                                    *(undefined8 *)(&stack0x000015c0 + lVar30) =
                                         *(undefined8 *)(&stack0x00001874 + lVar30);
                                    *(undefined8 *)(&stack0x000015c8 + lVar30) =
                                         *(undefined8 *)(&stack0x0000187c + lVar30);
                                    *(undefined8 *)(&stack0x000015d0 + lVar30) =
                                         *(undefined8 *)(&stack0x00001884 + lVar30);
                                    *(undefined8 *)(&stack0x000015d8 + lVar30) =
                                         *(undefined8 *)(&stack0x0000188c + lVar30);
                                    *(undefined2 *)(&stack0x000015e0 + lVar30) =
                                         *(undefined2 *)(&stack0x00001894 + lVar30);
                                    cVar26 = (&stack0x00001896)[lVar30];
                                    bVar43 = (&stack0x00001897)[lVar30];
                                    pcVar33 = (char *)(ulonglong)bVar43;
                                    *(undefined4 *)(&stack0x00001a20 + lVar30) =
                                         *(undefined4 *)(&stack0x000018b8 + lVar30);
                                    *(undefined8 *)(&stack0x00001a10 + lVar30) =
                                         *(undefined8 *)(&stack0x000018a8 + lVar30);
                                    *(undefined8 *)(&stack0x00001a18 + lVar30) =
                                         *(undefined8 *)(&stack0x000018b0 + lVar30);
                                    *(undefined4 *)(&stack0x00001a00 + lVar30) =
                                         *(undefined4 *)(&stack0x00001898 + lVar30);
                                    *(undefined4 *)(&stack0x00001a04 + lVar30) =
                                         *(undefined4 *)(&stack0x0000189c + lVar30);
                                    *(undefined4 *)(&stack0x00001a08 + lVar30) =
                                         *(undefined4 *)(&stack0x000018a0 + lVar30);
                                    *(undefined4 *)(&stack0x00001a0c + lVar30) =
                                         *(undefined4 *)(&stack0x000018a4 + lVar30);
                                    *(int *)(&stack0x00001ac0 + lVar30) =
                                         (int)CONCAT71((uint7)(uint3)((uint)*(undefined4 *)
                                                                             (&stack0x000018b8 +
                                                                             lVar30) >> 8),1);
                                    if ((&stack0x00001872)[lVar30] == '\a')
                                    goto code_r0x000140162880;
                                    *(undefined2 *)(&stack0x00001840 + lVar30) =
                                         *(undefined2 *)(&stack0x00001610 + lVar30);
                                    *(undefined4 *)(&stack0x00001830 + lVar30) =
                                         *(undefined4 *)(&stack0x00001600 + lVar30);
                                    *(undefined4 *)(&stack0x00001834 + lVar30) =
                                         *(undefined4 *)(&stack0x00001604 + lVar30);
                                    *(undefined4 *)(&stack0x00001838 + lVar30) =
                                         *(undefined4 *)(&stack0x00001608 + lVar30);
                                    *(undefined4 *)(&stack0x0000183c + lVar30) =
                                         *(undefined4 *)(&stack0x0000160c + lVar30);
                                    *(undefined4 *)(&stack0x00001820 + lVar30) =
                                         *(undefined4 *)(&stack0x000015f0 + lVar30);
                                    *(undefined4 *)(&stack0x00001824 + lVar30) =
                                         *(undefined4 *)(&stack0x000015f4 + lVar30);
                                    *(undefined4 *)(&stack0x00001828 + lVar30) =
                                         *(undefined4 *)(&stack0x000015f8 + lVar30);
                                    *(undefined4 *)(&stack0x0000182c + lVar30) =
                                         *(undefined4 *)(&stack0x000015fc + lVar30);
                                    (&stack0x00001842)[lVar30] = (&stack0x00001872)[lVar30];
                                    (&stack0x00001843)[lVar30] = (&stack0x00001873)[lVar30];
                                    *(undefined4 *)(&stack0x00001ac0 + lVar30) = 1;
                                    if (cVar26 != '\a') goto code_r0x000140162bed;
code_r0x000140162930:
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162953
                                    ;
                                    func_0x000140bd52a0(&stack0x00001a50 + lVar30,0x100000001,9);
                                    *(undefined4 *)(&stack0x000014a0 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a60 + lVar30);
                                    *(undefined4 *)(&stack0x000014a4 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a64 + lVar30);
                                    *(undefined4 *)(&stack0x000014a8 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x000014ac + lVar30) =
                                         *(undefined4 *)(&stack0x00001a6c + lVar30);
                                    *(undefined4 *)(&stack0x00001490 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a50 + lVar30);
                                    *(undefined4 *)(&stack0x00001494 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a54 + lVar30);
                                    *(undefined4 *)(&stack0x00001498 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a58 + lVar30);
                                    *(undefined4 *)(&stack0x0000149c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a5c + lVar30);
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162989
                                    ;
                                    func_0x000140bd6ce0(&stack0x00001860 + lVar30,
                                                        &stack0x00001490 + lVar30);
                                    *(undefined **)(&stack0x00001850 + lVar30) = &UNK_14295d8a8;
                                    *(undefined8 *)(&stack0x00001858 + lVar30) = 3;
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_1401629bc
                                    ;
                                    uVar29 = FUN_140be94d0(&stack0x00001620 + lVar30,
                                                           &stack0x00001850 + lVar30);
                                    uVar28 = *(undefined4 *)(&stack0x00001a60 + lVar30);
                                    *(undefined4 *)(&stack0x000017f4 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a50 + lVar30);
                                    *(undefined4 *)(&stack0x000017f8 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a54 + lVar30);
                                    *(undefined4 *)(&stack0x000017fc + lVar30) =
                                         *(undefined4 *)(&stack0x00001a58 + lVar30);
                                    *(undefined4 *)(&stack0x00001800 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a5c + lVar30);
                                    *(undefined4 *)(&stack0x00001804 + lVar30) = uVar28;
                                    *(undefined4 *)(&stack0x00001808 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a64 + lVar30);
                                    *(undefined4 *)(&stack0x0000180c + lVar30) =
                                         *(undefined4 *)(&stack0x00001a68 + lVar30);
                                    *(undefined4 *)(&stack0x00001810 + lVar30) =
                                         *(undefined4 *)(&stack0x00001a6c + lVar30);
                                    *(undefined4 *)(&stack0x000017f0 + lVar30) = uVar29;
                                  }
                                  LOCK();
                                  bVar45 = cRam000000014362b050 == '\0';
                                  if (bVar45) {
                                    cRam000000014362b050 = '\x01';
                                  }
                                  UNLOCK();
                                  if (!bVar45) {
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163eef
                                    ;
                                    FUN_1428cd210(0x14362b050,uVar28,1000000000);
                                  }
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162c49;
                                  FUN_140c81c70();
                                  lVar31 = *(longlong *)(&stack0x000019f0 + lVar30);
                                  lVar40 = 0;
                                  lVar34 = lRam000000014362b060;
                                  do {
                                    if (lRam000000014362b068 * 0x98 - lVar40 == 0) {
                                      (&stack0x00001ad7)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140162ccb;
                                      FUN_140c81e10();
                                      LOCK();
                                      bVar45 = cRam000000014362b050 == '\x01';
                                      if (bVar45) {
                                        cRam000000014362b050 = '\0';
                                      }
                                      UNLOCK();
                                      bVar43 = 7;
                                      goto joined_r0x000140162cda;
                                    }
                                    lVar40 = lVar40 + 0x98;
                                    pcVar33 = (char *)(lVar34 + 0x96);
                                    lVar34 = lVar34 + 0x98;
                                  } while (*pcVar33 != '\x06');
                                  bVar43 = *(byte *)(lRam000000014362b060 + -0x4c + lVar40);
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162ca5;
                                  FUN_140c81e10();
                                  LOCK();
                                  bVar45 = cRam000000014362b050 == '\x01';
                                  if (bVar45) {
                                    cRam000000014362b050 = '\0';
                                  }
                                  UNLOCK();
joined_r0x000140162cda:
                                  if (!bVar45) {
                                    (&stack0x00001ad7)[lVar30] = 1;
                                    *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f09
                                    ;
                                    FUN_1428cdb90(0x14362b050,0);
                                  }
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  uVar41 = 0x4a142e09642375b9;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140162d03;
                                  lVar34 = FUN_1400dd6f0(lVar38,0x4a142e09642375b9,
                                                         0x43f6154cfa8c847f);
                                  if (((lVar34 != 0) && (uVar41 < *(ulonglong *)(lVar42 + 0x218)))
                                     && (uVar41 = *(ulonglong *)
                                                   (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                        uVar41 != 0)) {
                                    lVar34 = ~uVar41 * 0x60;
                                    if (*(longlong *)(*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34)
                                        != 0) {
                                      uVar37 = *(undefined8 *)
                                                (*(longlong *)(lVar42 + 0x1e0) + lVar34 + 0x10);
                                      if ((bVar43 == 7) ||
                                         ((((byte)(bVar43 - 2) == 3 || 4 < (byte)(bVar43 - 2)) &
                                          bVar43) == 0)) {
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140162e9c;
                                        uVar37 = func_0x0001408aea50(uVar37);
                                      }
                                      else {
                                        if (2 < *puRam000000014362c3a8) {
                                          (&stack0x00001ad7)[lVar30] = 1;
                                          *(undefined **)((longlong)&puStack_48 + lVar30) =
                                               &UNK_140162d97;
                                          uVar36 = func_0x00014067dd80();
                                          *(undefined8 *)(&stack0x00001490 + lVar30) = uVar36;
                                          *(undefined **)(&stack0x00001498 + lVar30) =
                                               &UNK_14003bad0;
                                          (&stack0x00001ad7)[lVar30] = 1;
                                          *(undefined **)((longlong)&puStack_48 + lVar30) =
                                               &UNK_140162dbf;
                                          puVar35 = (undefined4 *)
                                                    func_0x00014067dd80(&UNK_14295d800);
                                          uVar29 = *puVar35;
                                          uVar6 = puVar35[1];
                                          uVar7 = puVar35[2];
                                          uVar8 = puVar35[3];
                                          uVar28 = puVar35[4];
                                          *(undefined8 *)(&stack0x00001880 + lVar30) = 3;
                                          *(undefined **)(&stack0x00001888 + lVar30) =
                                               &UNK_14295d818;
                                          *(undefined8 *)(&stack0x00001890 + lVar30) = 0x17;
                                          *(undefined **)(&stack0x000018a0 + lVar30) =
                                               &UNK_14295d7f0;
                                          *(undefined8 *)(&stack0x000018a8 + lVar30) = 1;
                                          *(undefined1 **)(&stack0x000018b0 + lVar30) =
                                               &stack0x00001490 + lVar30;
                                          *(undefined8 *)(&stack0x000018b8 + lVar30) = 1;
                                          *(undefined8 *)(&stack0x000018c0 + lVar30) = 0;
                                          *(undefined8 *)(&stack0x00001850 + lVar30) = 0;
                                          *(undefined **)(&stack0x00001858 + lVar30) =
                                               &UNK_14295d818;
                                          *(undefined8 *)(&stack0x00001860 + lVar30) = 0x17;
                                          *(undefined8 *)(&stack0x00001868 + lVar30) = 0;
                                          *(undefined4 *)(&stack0x00001870 + lVar30) = uVar29;
                                          *(undefined4 *)(&stack0x00001874 + lVar30) = uVar6;
                                          *(undefined4 *)(&stack0x00001878 + lVar30) = uVar7;
                                          *(undefined4 *)(&stack0x0000187c + lVar30) = uVar8;
                                          *(undefined4 *)(&stack0x00001898 + lVar30) = 1;
                                          *(undefined4 *)(&stack0x0000189c + lVar30) = uVar28;
                                          (&stack0x00001ad7)[lVar30] = 1;
                                          *(undefined **)((longlong)&puStack_48 + lVar30) =
                                               &UNK_140162e7f;
                                          func_0x000140d4a330(&stack0x00001a90 + lVar30,
                                                              &stack0x00001850 + lVar30);
                                        }
                                        (&stack0x00001ad7)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140162e8e;
                                        uVar37 = func_0x00014067dd80(uVar37);
                                      }
                                      (&stack0x00001ad7)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140162eb2;
                                      FUN_140b00b20(&stack0x00001a50 + lVar30,uVar37);
                                      *(undefined8 *)(&stack0x000017e0 + lVar30) = uVar32;
                                      if ((bVar43 == 7) ||
                                         ((byte)(bVar43 - 2) != 3 && (byte)(bVar43 - 2) < 5)) {
                                        (&stack0x00001acd)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140162efa;
                                        FUN_140d7bd90(&stack0x00001850 + lVar30,
                                                      &stack0x00001a50 + lVar30);
                                        unaff_XMM6_Da = *(undefined4 *)(&stack0x00001a68 + lVar30);
                                        *(undefined8 *)(&stack0x00001a48 + lVar30) =
                                             *(undefined8 *)(&stack0x00001850 + lVar30);
                                        *(undefined8 *)(&stack0x00001a38 + lVar30) =
                                             *(undefined8 *)(&stack0x00001858 + lVar30);
                                        uVar32 = *(undefined8 *)(&stack0x00001860 + lVar30);
                                      }
                                      else {
                                        *(ulonglong *)(&stack0x00001a38 + lVar30) =
                                             (ulonglong)(bVar43 & 1);
                                      }
                                      (&stack0x00001ad6)[lVar30] = 1;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140162f53;
                                      FUN_1409ed860(&stack0x000011b4 + lVar30,
                                                    &stack0x00001620 + lVar30,lVar42,
                                                    &stack0x00001a00 + lVar30);
                                      if ((**(byte **)(&stack0x000017c0 + lVar30) & 1) != 0) {
                                        pbVar44 = *(byte **)(&stack0x000017c0 + lVar30) + 8;
                                      }
                                      uVar37 = *(undefined8 *)pbVar44;
                                      *(undefined4 *)(&stack0x00001870 + lVar30) =
                                           *(undefined4 *)(&stack0x00001a20 + lVar30);
                                      *(undefined8 *)(&stack0x00001860 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a10 + lVar30);
                                      *(undefined8 *)(&stack0x00001868 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a18 + lVar30);
                                      *(undefined8 *)(&stack0x00001850 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a00 + lVar30);
                                      *(undefined8 *)(&stack0x00001858 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a08 + lVar30);
                                      *(undefined8 *)(&stack0x00001490 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a48 + lVar30);
                                      *(undefined8 *)(&stack0x00001498 + lVar30) =
                                           *(undefined8 *)(&stack0x00001a38 + lVar30);
                                      *(undefined8 *)(&stack0x000014a0 + lVar30) = uVar32;
                                      *(undefined4 *)(&stack0x000014a8 + lVar30) = unaff_XMM6_Da;
                                      (&stack0x000014ac)[lVar30] = 0;
                                      uVar28 = *(undefined4 *)
                                                (*(longlong *)(&stack0x00001a30 + lVar30) + 0x9c);
                                      uVar29 = *(undefined4 *)
                                                (*(longlong *)(&stack0x00001a30 + lVar30) + 0xa4);
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined4 *)((longlong)auStackX_8 + lVar30) = uVar29;
                                      *(undefined4 *)(auStackX_10 + lVar30 + -0x10) = uVar28;
                                      *(undefined1 **)(&stack0xfffffffffffffff8 + lVar30) =
                                           &stack0x00001490 + lVar30;
                                      (&stack0xfffffffffffffff0)[lVar30] =
                                           (char)*(undefined4 *)(&stack0x00001ac0 + lVar30);
                                      *(undefined1 **)(&stack0xffffffffffffffe8 + lVar30) =
                                           &stack0x000011b4 + lVar30;
                                      *(undefined1 **)(&stack0xffffffffffffffe0 + lVar30) =
                                           &stack0x00001850 + lVar30;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140163030;
                                      FUN_14090d000(&stack0x00001a90 + lVar30,
                                                    &stack0x00001620 + lVar30,lVar42,uVar37);
                                      *(undefined8 *)(&stack0x00001858 + lVar30) = 2;
                                      *(undefined2 *)(&stack0x00001860 + lVar30) = 0x100;
                                      lVar34 = 0;
                                      while (lVar34 != 2) {
                                        *(longlong *)(&stack0x00001850 + lVar30) = lVar34 + 1;
                                        uVar2 = (&stack0x00001860)[lVar34 + lVar30];
                                        (&stack0x00001ad6)[lVar30] = 0;
                                        (&stack0x00001ad5)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_140163096;
                                        uVar32 = FUN_14031d140(&stack0x00001620 + lVar30,lVar42,
                                                               uVar2,&stack0x00001a90 + lVar30);
                                        (&stack0x00001ad6)[lVar30] = 0;
                                        (&stack0x00001ad5)[lVar30] = 1;
                                        *(undefined **)((longlong)&puStack_48 + lVar30) =
                                             &UNK_1401630ac;
                                        FUN_1404ebfb0(uVar32);
                                        lVar34 = lVar34 + 1;
                                      }
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_1401630d8;
                                      uVar32 = FUN_14031d140(&stack0x00001620 + lVar30,lVar42,2,
                                                             &stack0x00001a90 + lVar30);
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_1401630ee;
                                      FUN_1404ebfb0(uVar32);
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140163112;
                                      FUN_1408c9a30(&stack0x00001620 + lVar30,lVar42,
                                                    &stack0x00001a90 + lVar30);
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      (&stack0xffffffffffffffe8)[lVar30] =
                                           (char)*(undefined4 *)(&stack0x00001a88 + lVar30);
                                      *(undefined1 **)(&stack0xffffffffffffffe0 + lVar30) =
                                           &stack0x000017f0 + lVar30;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140163153;
                                      FUN_1408a6410(&stack0x00001620 + lVar30,lVar42,
                                                    &stack0x00001a90 + lVar30,
                                                    &stack0x00001820 + lVar30);
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      uVar41 = 0xc2b8e941c91a6428;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_14016317d;
                                      lVar34 = FUN_1400dd6f0(lVar38,0xc2b8e941c91a6428,
                                                             0x59f3c6d1d3da43a0);
                                      if (((lVar34 != 0) &&
                                          (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                         (uVar41 = *(ulonglong *)
                                                    (*(longlong *)(lVar42 + 0x210) + uVar41 * 8),
                                         uVar41 != 0)) {
                                        lVar34 = ~uVar41 * 0x60;
                                        if (*(longlong *)
                                             (*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) != 0) {
                                          iVar3 = **(int **)(*(longlong *)(lVar42 + 0x1e0) + lVar34
                                                            + 0x10);
                                          (&stack0x00001ad6)[lVar30] = 0;
                                          (&stack0x00001ad5)[lVar30] = 1;
                                          uVar41 = 0xa4c02661e67b048b;
                                          *(undefined **)((longlong)&puStack_48 + lVar30) =
                                               &UNK_1401631f9;
                                          lVar34 = FUN_1400dd6f0(lVar38,0xa4c02661e67b048b,
                                                                 0xeae761e6aa2a67ec);
                                          if (((lVar34 != 0) &&
                                              (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                             (uVar41 = *(ulonglong *)
                                                        (*(longlong *)(lVar42 + 0x210) + uVar41 * 8)
                                             , uVar41 != 0)) {
                                            lVar34 = ~uVar41 * 0x60;
                                            if (*(longlong *)
                                                 (*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34) !=
                                                0) {
                                              uVar2 = *(undefined1 *)
                                                       (*(longlong *)
                                                         (*(longlong *)(lVar42 + 0x1e0) + lVar34 +
                                                         0x10) + 0x20);
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              (&stack0xffffffffffffffe0)[lVar30] = uVar2;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_14016327c;
                                              bVar27 = FUN_1408a82f0(&stack0x00001620 + lVar30,
                                                                     lVar42,&stack0x00001a90 +
                                                                            lVar30,iVar3 == 0x13);
                                              if (*pcRam000000014362c098 == '\x01') {
                                                if ((byte)(bVar43 - 3) < 2) {
code_r0x0001401632b4:
                                                  (&stack0x00001ad6)[lVar30] = 0;
                                                  (&stack0x00001ad5)[lVar30] = 1;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401632d8;
                                                  FUN_1408a5f50(&stack0x00001620 + lVar30,lVar42,
                                                                &stack0x00001a90 + lVar30);
code_r0x0001401632d8:
                                                  (&stack0x00001ad6)[lVar30] = 0;
                                                  (&stack0x00001ad5)[lVar30] = 1;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401632ff;
                                                  uVar32 = FUN_14031d140(&stack0x00001620 + lVar30,
                                                                         lVar42,3,&stack0x00001a90 +
                                                                                  lVar30);
                                                  (&stack0x00001ad6)[lVar30] = 0;
                                                  (&stack0x00001ad5)[lVar30] = 1;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_140163315;
                                                  FUN_1404ebfb0(uVar32);
                                                }
                                              }
                                              else if (bVar43 == 3) {
                                                if (*(char *)(*(longlong *)
                                                               (&stack0x00001a30 + lVar30) + 0xf3)
                                                    == '\x01') goto code_r0x0001401632b4;
                                                goto code_r0x0001401632d8;
                                              }
                                              if ((bVar27 & iVar3 != 0x13) == 0) {
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_140163342;
                                                FUN_1408f6c00(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_140163366;
                                                FUN_1408b0560(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_14016338a;
                                                FUN_1408a5a70(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                                if ((&stack0x00001ac0)[lVar30] != '\0') {
                                                  (&stack0x00001ad6)[lVar30] = 0;
                                                  (&stack0x00001ad5)[lVar30] = 1;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401633be;
                                                  FUN_1409ccb20(&stack0x00001620 + lVar30,lVar42,
                                                                &stack0x00001a90 + lVar30,
                                                                *(undefined8 *)
                                                                 (&stack0x00001a40 + lVar30));
                                                  (&stack0x00001ad6)[lVar30] = 0;
                                                  (&stack0x00001ad5)[lVar30] = 1;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401633e9;
                                                  FUN_140837c40(&stack0x00001620 + lVar30,lVar42,
                                                                &stack0x00001a90 + lVar30,
                                                                *(undefined8 *)
                                                                 (&stack0x00001a40 + lVar30));
                                                }
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_14016340d;
                                                FUN_1409c4470(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_140163427;
                                                uVar28 = func_0x00014093fc00(*(undefined8 *)
                                                                              (&stack0x000017e0 +
                                                                              lVar30));
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_14016344e;
                                                FUN_1409c6700(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30,uVar28);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_140163472;
                                                FUN_1409d1710(&stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_140163499;
                                                uVar32 = FUN_14031d140(&stack0x00001620 + lVar30,
                                                                       lVar42,4,&stack0x00001a90 +
                                                                                lVar30);
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_1401634af;
                                                FUN_1404ebfb0(uVar32);
                                              }
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_1401634da;
                                              FUN_140837ff0(&stack0x00001620 + lVar30,lVar42,
                                                            &stack0x00001a90 + lVar30,
                                                            *(undefined8 *)
                                                             (&stack0x00001a40 + lVar30));
                                              if (*(longlong *)(&stack0x000019f8 + lVar30) != 0) {
                                                (&stack0x00001ad6)[lVar30] = 0;
                                                (&stack0x00001ad5)[lVar30] = 1;
                                                *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                     &UNK_14016350f;
                                                FUN_1400ddc80(*(undefined8 *)
                                                               (&stack0x000019f8 + lVar30),
                                                              &stack0x00001620 + lVar30,lVar42,
                                                              &stack0x00001a90 + lVar30);
                                              }
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_140163536;
                                              uVar32 = FUN_14031d140(&stack0x00001620 + lVar30,
                                                                     lVar42,6,&stack0x00001a90 +
                                                                              lVar30);
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_14016354c;
                                              FUN_1404ebfb0(uVar32);
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_140163570;
                                              FUN_140589c90(&stack0x00001620 + lVar30,lVar42,
                                                            &stack0x00001a90 + lVar30);
                                              *(undefined4 *)(&stack0x00001850 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001a90 + lVar30);
                                              *(undefined4 *)(&stack0x00001854 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001a94 + lVar30);
                                              *(undefined4 *)(&stack0x00001858 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001a98 + lVar30);
                                              *(undefined4 *)(&stack0x0000185c + lVar30) =
                                                   *(undefined4 *)(&stack0x00001a9c + lVar30);
                                              *(undefined4 *)(&stack0x00001870 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001ab0 + lVar30);
                                              *(undefined4 *)(&stack0x00001860 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001aa0 + lVar30);
                                              *(undefined4 *)(&stack0x00001864 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001aa4 + lVar30);
                                              *(undefined4 *)(&stack0x00001868 + lVar30) =
                                                   *(undefined4 *)(&stack0x00001aa8 + lVar30);
                                              *(undefined4 *)(&stack0x0000186c + lVar30) =
                                                   *(undefined4 *)(&stack0x00001aac + lVar30);
                                              uVar28 = *(undefined4 *)(&stack0x00001850 + lVar30);
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 1;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_1401635c3;
                                              FUN_140bd6380(&stack0x00001850 + lVar30,
                                                            &stack0x00001620 + lVar30,0x22);
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_1401635df;
                                              func_0x000142923c00(&stack0x00001850 + lVar30,
                                                                  &stack0x00001620 + lVar30,0x198);
                                              (&stack0x00001ad6)[lVar30] = 0;
                                              (&stack0x00001ad5)[lVar30] = 0;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_1401635fc;
                                              FUN_1409dda50(&stack0x00001490 + lVar30,
                                                            &stack0x00001850 + lVar30);
                                              uVar41 = 0xa68561b396784eaa;
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_140163618;
                                              lVar34 = FUN_1400dd6f0(lVar38,0xa68561b396784eaa,
                                                                     0xf5fd74e6faf4ed14);
                                              uVar32 = *(undefined8 *)(&stack0x000019f0 + lVar30);
                                              uVar37 = *(undefined8 *)(&stack0x000017c8 + lVar30);
                                              if (((lVar34 != 0) &&
                                                  (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                                 (uVar41 = *(ulonglong *)
                                                            (*(longlong *)(lVar42 + 0x210) +
                                                            uVar41 * 8), uVar41 != 0)) {
                                                lVar34 = ~uVar41 * 0x60;
                                                if (*(longlong *)
                                                     (*(longlong *)(lVar42 + 0x1e0) + 0x20 + lVar34)
                                                    != 0) {
                                                  lVar34 = *(longlong *)(lVar42 + 0x1e0) + lVar34;
                                                  uVar36 = *(undefined8 *)(lVar34 + 0x10);
                                                  *(undefined4 *)(lVar34 + 0x5c) =
                                                       *(undefined4 *)(lVar42 + 0x5b0);
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_14016368b;
                                                  FUN_14093ce30(uVar36,&stack0x00001490 + lVar30);
                                                  uVar41 = 0x8226cc786e679355;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401636a7;
                                                  lVar34 = FUN_1400dd6f0(lVar38,0x8226cc786e679355,
                                                                         0x65cfda6a55a841ac);
                                                  if (((lVar34 != 0) &&
                                                      (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                                     (uVar41 = *(ulonglong *)
                                                                (*(longlong *)(lVar42 + 0x210) +
                                                                uVar41 * 8), uVar41 != 0)) {
                                                    lVar34 = ~uVar41 * 0x60;
                                                    if (*(longlong *)
                                                         (*(longlong *)(lVar42 + 0x1e0) + 0x20 +
                                                         lVar34) != 0) {
                                                      lVar34 = *(longlong *)(lVar42 + 0x1e0) +
                                                               lVar34;
                                                      uVar36 = *(undefined8 *)(lVar34 + 0x10);
                                                      *(undefined4 *)(lVar34 + 0x5c) =
                                                           *(undefined4 *)(lVar42 + 0x5b0);
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_14016370c;
                                                      FUN_14091ebb0(uVar36,&stack0x00001490 + lVar30
                                                                   );
                                                      uVar41 = 0x662978458699a5ef;
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_140163728;
                                                      lVar38 = FUN_1400dd6f0(lVar38,
                                                  0x662978458699a5ef,0x3ec057c162419b46);
                                                  if (((lVar38 != 0) &&
                                                      (uVar41 < *(ulonglong *)(lVar42 + 0x218))) &&
                                                     (uVar41 = *(ulonglong *)
                                                                (*(longlong *)(lVar42 + 0x210) +
                                                                uVar41 * 8), uVar41 != 0)) {
                                                    lVar38 = ~uVar41 * 0x60;
                                                    if (*(longlong *)
                                                         (*(longlong *)(lVar42 + 0x1e0) + 0x20 +
                                                         lVar38) != 0) {
                                                      lVar38 = *(longlong *)(lVar42 + 0x1e0) +
                                                               lVar38;
                                                      uVar36 = *(undefined8 *)(lVar38 + 0x10);
                                                      *(undefined4 *)(lVar38 + 0x5c) =
                                                           *(undefined4 *)(lVar42 + 0x5b0);
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_14016378d;
                                                      FUN_140992220(uVar36,&stack0x00001490 + lVar30
                                                                   );
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_1401637a6;
                                                      func_0x000142923c00(&stack0x00001850 + lVar30,
                                                                          &stack0x00001490 + lVar30,
                                                                          0x110);
                                                      if (*(longlong *)(&stack0x00001a50 + lVar30)
                                                          != 0) {
                                                        *(undefined **)
                                                         ((longlong)&puStack_48 + lVar30) =
                                                             &UNK_1401637c4;
                                                        func_0x000140613c20(*(undefined8 *)
                                                                             (&stack0x00001a58 +
                                                                             lVar30),*(longlong *)
                                                                                      (&
                                                  stack0x00001a50 + lVar30),1);
                                                  }
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401637dd;
                                                  func_0x000142923c00(auStackX_10 + lVar30,
                                                                      &stack0x00001850 + lVar30,
                                                                      0x110);
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401637ec;
                                                  FUN_1404f7ca0(&stack0x00000120 + lVar30,
                                                                auStackX_10 + lVar30);
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_140163805;
                                                  func_0x000142923c00(&stack0x00001490 + lVar30,
                                                                      &stack0x00000120 + lVar30,0xa8
                                                                     );
                                                  *(undefined8 *)(&stack0x00001430 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001c8 + lVar30);
                                                  *(undefined8 *)(&stack0x00001438 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001d0 + lVar30);
                                                  *(undefined8 *)(&stack0x00001440 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001d8 + lVar30);
                                                  *(undefined8 *)(&stack0x00001448 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001e0 + lVar30);
                                                  *(undefined8 *)(&stack0x00001450 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001e8 + lVar30);
                                                  *(undefined8 *)(&stack0x00001458 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001f0 + lVar30);
                                                  *(undefined8 *)(&stack0x00001460 + lVar30) =
                                                       *(undefined8 *)(&stack0x000001f8 + lVar30);
                                                  *(undefined8 *)(&stack0x00001468 + lVar30) =
                                                       *(undefined8 *)(&stack0x00000200 + lVar30);
                                                  *(undefined8 *)(&stack0x00001470 + lVar30) =
                                                       *(undefined8 *)(&stack0x00000208 + lVar30);
                                                  *(undefined8 *)(&stack0x00001478 + lVar30) =
                                                       *(undefined8 *)(&stack0x00000210 + lVar30);
                                                  *(undefined8 *)(&stack0x00001480 + lVar30) =
                                                       *(undefined8 *)(&stack0x00000218 + lVar30);
                                                  *(undefined8 *)(&stack0x00001488 + lVar30) =
                                                       *(undefined8 *)(&stack0x00000220 + lVar30);
                                                  uVar36 = *(undefined8 *)(lVar31 + 0xe8);
                                                  *(undefined8 *)(&stack0x00000170 + lVar30) =
                                                       *(undefined8 *)(lVar31 + 0xe0);
                                                  *(undefined8 *)(&stack0x00000178 + lVar30) =
                                                       uVar36;
                                                  uVar36 = *(undefined8 *)(lVar31 + 0xd8);
                                                  *(undefined8 *)(&stack0x00000160 + lVar30) =
                                                       *(undefined8 *)(lVar31 + 0xd0);
                                                  *(undefined8 *)(&stack0x00000168 + lVar30) =
                                                       uVar36;
                                                  uVar36 = *(undefined8 *)(lVar31 + 0x90);
                                                  uVar17 = *(undefined8 *)(lVar31 + 0x98);
                                                  uVar18 = *(undefined8 *)(lVar31 + 0xa0);
                                                  uVar19 = *(undefined8 *)(lVar31 + 0xa8);
                                                  uVar20 = *(undefined8 *)(lVar31 + 0xb0);
                                                  uVar21 = *(undefined8 *)(lVar31 + 0xb8);
                                                  uVar22 = *(undefined8 *)(lVar31 + 200);
                                                  *(undefined8 *)(&stack0x00000150 + lVar30) =
                                                       *(undefined8 *)(lVar31 + 0xc0);
                                                  *(undefined8 *)(&stack0x00000158 + lVar30) =
                                                       uVar22;
                                                  *(undefined8 *)(&stack0x00000140 + lVar30) =
                                                       uVar20;
                                                  *(undefined8 *)(&stack0x00000148 + lVar30) =
                                                       uVar21;
                                                  *(undefined8 *)(&stack0x00000130 + lVar30) =
                                                       uVar18;
                                                  *(undefined8 *)(&stack0x00000138 + lVar30) =
                                                       uVar19;
                                                  *(undefined8 *)(&stack0x00000120 + lVar30) =
                                                       uVar36;
                                                  *(undefined8 *)(&stack0x00000128 + lVar30) =
                                                       uVar17;
                                                  uVar36 = *(undefined8 *)
                                                            (&stack0x00001438 + lVar30);
                                                  uVar29 = *(undefined4 *)
                                                            (&stack0x00001440 + lVar30);
                                                  uVar6 = *(undefined4 *)(&stack0x00001444 + lVar30)
                                                  ;
                                                  uVar7 = *(undefined4 *)(&stack0x00001448 + lVar30)
                                                  ;
                                                  uVar8 = *(undefined4 *)(&stack0x0000144c + lVar30)
                                                  ;
                                                  uVar9 = *(undefined4 *)(&stack0x00001450 + lVar30)
                                                  ;
                                                  uVar10 = *(undefined4 *)
                                                            (&stack0x00001454 + lVar30);
                                                  uVar11 = *(undefined4 *)
                                                            (&stack0x00001458 + lVar30);
                                                  uVar12 = *(undefined4 *)
                                                            (&stack0x0000145c + lVar30);
                                                  uVar13 = *(undefined4 *)
                                                            (&stack0x00001460 + lVar30);
                                                  uVar14 = *(undefined4 *)
                                                            (&stack0x00001464 + lVar30);
                                                  uVar15 = *(undefined4 *)
                                                            (&stack0x00001468 + lVar30);
                                                  uVar16 = *(undefined4 *)
                                                            (&stack0x0000146c + lVar30);
                                                  *(undefined8 *)(lVar31 + 0x90) =
                                                       *(undefined8 *)(&stack0x00001430 + lVar30);
                                                  *(undefined8 *)(lVar31 + 0x98) = uVar36;
                                                  *(undefined4 *)(lVar31 + 0xa0) = uVar29;
                                                  *(undefined4 *)(lVar31 + 0xa4) = uVar6;
                                                  *(undefined4 *)(lVar31 + 0xa8) = uVar7;
                                                  *(undefined4 *)(lVar31 + 0xac) = uVar8;
                                                  *(undefined4 *)(lVar31 + 0xb0) = uVar9;
                                                  *(undefined4 *)(lVar31 + 0xb4) = uVar10;
                                                  *(undefined4 *)(lVar31 + 0xb8) = uVar11;
                                                  *(undefined4 *)(lVar31 + 0xbc) = uVar12;
                                                  *(undefined4 *)(lVar31 + 0xc0) = uVar13;
                                                  *(undefined4 *)(lVar31 + 0xc4) = uVar14;
                                                  *(undefined4 *)(lVar31 + 200) = uVar15;
                                                  *(undefined4 *)(lVar31 + 0xcc) = uVar16;
                                                  uVar36 = *(undefined8 *)
                                                            (&stack0x00001478 + lVar30);
                                                  *(undefined8 *)(lVar31 + 0xd0) =
                                                       *(undefined8 *)(&stack0x00001470 + lVar30);
                                                  *(undefined8 *)(lVar31 + 0xd8) = uVar36;
                                                  uVar29 = *(undefined4 *)
                                                            (&stack0x00001484 + lVar30);
                                                  uVar6 = *(undefined4 *)(&stack0x00001488 + lVar30)
                                                  ;
                                                  uVar7 = *(undefined4 *)(&stack0x0000148c + lVar30)
                                                  ;
                                                  *(undefined4 *)(lVar31 + 0xe0) =
                                                       *(undefined4 *)(&stack0x00001480 + lVar30);
                                                  *(undefined4 *)(lVar31 + 0xe4) = uVar29;
                                                  *(undefined4 *)(lVar31 + 0xe8) = uVar6;
                                                  *(undefined4 *)(lVar31 + 0xec) = uVar7;
                                                  puVar4 = *(undefined8 **)
                                                            (&stack0x000017d0 + lVar30);
                                                  if (*(longlong *)(&stack0x00000120 + lVar30) != 0)
                                                  {
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_14016390b;
                                                    FUN_14004ceb0(&stack0x00000120 + lVar30,
                                                                  &stack0x00000140 + lVar30,0x68,
                                                                  0x10);
                                                    lVar38 = *(longlong *)
                                                              (&stack0x00000158 + lVar30);
                                                    if ((lVar38 != 0) &&
                                                       (lVar42 = lVar38 * 0x11 + 0x21, lVar42 != 0))
                                                    {
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_140163943;
                                                      func_0x000140613c20(*(longlong *)
                                                                           (&stack0x00000150 +
                                                                           lVar30) + lVar38 * -0x10
                                                                          + -0x10,lVar42,0x10);
                                                    }
                                                  }
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_14016395c;
                                                  func_0x000142923c00(&stack0x00001378 + lVar30,
                                                                      &stack0x00001490 + lVar30,0x90
                                                                     );
                                                  *(undefined8 *)(&stack0x00001aa0 + lVar30) =
                                                       *(undefined8 *)(&stack0x00001530 + lVar30);
                                                  *(undefined4 *)(&stack0x00001a90 + lVar30) =
                                                       *(undefined4 *)(&stack0x00001520 + lVar30);
                                                  *(undefined4 *)(&stack0x00001a94 + lVar30) =
                                                       *(undefined4 *)(&stack0x00001524 + lVar30);
                                                  *(undefined4 *)(&stack0x00001a98 + lVar30) =
                                                       *(undefined4 *)(&stack0x00001528 + lVar30);
                                                  *(undefined4 *)(&stack0x00001a9c + lVar30) =
                                                       *(undefined4 *)(&stack0x0000152c + lVar30);
                                                  *(undefined8 *)(&stack0x00000120 + lVar30) = 0;
                                                  *(undefined8 *)(&stack0x00000128 + lVar30) = 8;
                                                  *(undefined8 *)(&stack0x00000130 + lVar30) = 0;
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_1401639a0;
                                                  plVar39 = (longlong *)FUN_1400d9b20(0);
                                                  if (plVar39 != (longlong *)0x0) {
                                                    lVar38 = *plVar39;
                                                    lVar42 = plVar39[1];
                                                    *plVar39 = *plVar39 + 1;
                                                    *(undefined8 *)(&stack0x00001708 + lVar30) =
                                                         _UNK_1429b0030;
                                                    *(undefined8 *)(&stack0x00001710 + lVar30) =
                                                         _UNK_1429b0038;
                                                    *(undefined8 *)(&stack0x000016f8 + lVar30) =
                                                         _UNK_1429b0020;
                                                    *(undefined8 *)(&stack0x00001700 + lVar30) =
                                                         _UNK_1429b0028;
                                                    *(longlong *)(&stack0x00001718 + lVar30) =
                                                         lVar38;
                                                    *(longlong *)(&stack0x00001720 + lVar30) =
                                                         lVar42;
                                                    *(undefined8 *)(&stack0x00001620 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001628 + lVar30) = 2;
                                                    *(undefined8 *)(&stack0x00001630 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001638 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001640 + lVar30) = 1;
                                                    *(undefined8 *)(&stack0x00001648 + lVar30) = 0;
                                                    *(undefined4 *)(&stack0x00001650 + lVar30) =
                                                         *(undefined4 *)(&stack0x00000120 + lVar30);
                                                    *(undefined4 *)(&stack0x00001654 + lVar30) =
                                                         *(undefined4 *)(&stack0x00000124 + lVar30);
                                                    *(undefined4 *)(&stack0x00001658 + lVar30) =
                                                         *(undefined4 *)(&stack0x00000128 + lVar30);
                                                    *(undefined4 *)(&stack0x0000165c + lVar30) =
                                                         *(undefined4 *)(&stack0x0000012c + lVar30);
                                                    *(undefined8 *)(&stack0x00001660 + lVar30) =
                                                         *(undefined8 *)(&stack0x00000130 + lVar30);
                                                    *(undefined8 *)(&stack0x00001668 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001670 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x00001678 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001680 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001688 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x00001690 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x00001698 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016a0 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x000016a8 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016b0 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016b8 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x000016c0 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016c8 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016d0 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x000016d8 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016e0 + lVar30) = 0;
                                                    *(undefined8 *)(&stack0x000016e8 + lVar30) = 4;
                                                    *(undefined8 *)(&stack0x000016f0 + lVar30) = 0;
                                                    (&stack0x00001ad0)[lVar30] = 1;
                                                    (&stack0x00001acf)[lVar30] = 1;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163ace;
                                                    FUN_140530430(&stack0x00001668 + lVar30,
                                                                  &stack0x00001378 + lVar30,
                                                                  *(undefined8 *)
                                                                   (&stack0x00001a98 + lVar30),
                                                                  *(undefined8 *)
                                                                   (&stack0x00001aa0 + lVar30));
                                                    (&stack0x00001ad0)[lVar30] = 1;
                                                    (&stack0x00001acf)[lVar30] = 1;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163af9;
                                                    cVar26 = FUN_140be7ba0(&stack0x00001620 + lVar30
                                                                           ,&stack0x00001378 +
                                                                            lVar30,*(undefined8 *)
                                                                                    (&
                                                  stack0x000017d8 + lVar30),uVar37);
                                                  if (cVar26 == '\0') {
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163bc5;
                                                    func_0x000142923c00(&stack0x00001268 + lVar30,
                                                                        &stack0x00001378 + lVar30,
                                                                        0x90);
                                                    *(undefined8 *)(&stack0x00001a60 + lVar30) =
                                                         *(undefined8 *)(&stack0x00001aa0 + lVar30);
                                                    *(undefined4 *)(&stack0x00001a50 + lVar30) =
                                                         *(undefined4 *)(&stack0x00001a90 + lVar30);
                                                    *(undefined4 *)(&stack0x00001a54 + lVar30) =
                                                         *(undefined4 *)(&stack0x00001a94 + lVar30);
                                                    *(undefined4 *)(&stack0x00001a58 + lVar30) =
                                                         *(undefined4 *)(&stack0x00001a98 + lVar30);
                                                    *(undefined4 *)(&stack0x00001a5c + lVar30) =
                                                         *(undefined4 *)(&stack0x00001a9c + lVar30);
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163bfd;
                                                    func_0x000142923c00(&stack0x00001850 + lVar30,
                                                                        &stack0x00001620 + lVar30,
                                                                        0x108);
                                                    (&stack0x00001ad0)[lVar30] = 0;
                                                    (&stack0x00001acf)[lVar30] = 0;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163c24;
                                                    FUN_1402c4280(&stack0x00000120 + lVar30,
                                                                  &stack0x00001268 + lVar30,
                                                                  &stack0x00001a50 + lVar30,
                                                                  &stack0x00001850 + lVar30);
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163c39;
                                                    func_0x000142923c00(puVar4,&stack0x00000120 +
                                                                               lVar30,0x218);
                                                    *(undefined4 *)(puVar4 + 0x43) = uVar28;
                                                  }
                                                  else {
                                                    (&stack0x00001ad0)[lVar30] = 1;
                                                    (&stack0x00001acf)[lVar30] = 1;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163b1b;
                                                    FUN_140be9420(&stack0x00001378 + lVar30);
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163b30;
                                                    func_0x000142923c00(&stack0x00000120 + lVar30,
                                                                        uVar32,0x90);
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163b45;
                                                    func_0x000142923c00(uVar32,&stack0x00001378 +
                                                                               lVar30,0x90);
                                                    if (!SBORROW8(0,*(longlong *)
                                                                     (&stack0x00000120 + lVar30))) {
                                                      (&stack0x00001ad0)[lVar30] = 0;
                                                      (&stack0x00001acf)[lVar30] = 1;
                                                      *(undefined **)
                                                       ((longlong)&puStack_48 + lVar30) =
                                                           &UNK_140163b6a;
                                                      FUN_1405ef210(&stack0x00000120 + lVar30);
                                                    }
                                                    *puVar4 = 0x8000000000000000;
                                                    (&stack0x00001ad2)[lVar30] = 0;
                                                    (&stack0x00001ad1)[lVar30] = 1;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163b91;
                                                    FUN_1405fd1d0(&stack0x00001620 + lVar30);
                                                    (&stack0x00001acb)[lVar30] = 0;
                                                    *(undefined **)((longlong)&puStack_48 + lVar30)
                                                         = &UNK_140163ba4;
                                                    FUN_1405da820(&stack0x00001a90 + lVar30);
                                                  }
                                                  return puVar4;
                                                  }
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_140164098;
                                                  FUN_1428d80d0(&UNK_142932a10);
                                                  goto code_r0x0001401640c2;
                                                  }
                                                  }
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_140164046;
                                                  FUN_1428d9310(&UNK_14295d890);
                                                  goto code_r0x0001401640c2;
                                                  }
                                                  }
                                                  *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                       &UNK_140164035;
                                                  FUN_1428d9310(&UNK_14295d878);
                                                  goto code_r0x0001401640c2;
                                                }
                                              }
                                              *(undefined **)((longlong)&puStack_48 + lVar30) =
                                                   &UNK_140164024;
                                              FUN_1428d9310(&UNK_14295d860);
                                              goto code_r0x0001401640c2;
                                            }
                                          }
                                          (&stack0x00001ad6)[lVar30] = 0;
                                          (&stack0x00001ad5)[lVar30] = 1;
                                          *(undefined **)((longlong)&puStack_48 + lVar30) =
                                               &UNK_140164013;
                                          FUN_1428d9310(&UNK_14295d848);
                                          goto code_r0x0001401640c2;
                                        }
                                      }
                                      (&stack0x00001ad6)[lVar30] = 0;
                                      (&stack0x00001ad5)[lVar30] = 1;
                                      *(undefined **)((longlong)&puStack_48 + lVar30) =
                                           &UNK_140163ff4;
                                      FUN_1428d9310(&UNK_14295d830);
                                      goto code_r0x0001401640c2;
                                    }
                                  }
                                  (&stack0x00001ad7)[lVar30] = 1;
                                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163fd5;
                                  FUN_1428d9330(&UNK_14295d7a2,0x15,&UNK_14295d7b8);
                                  goto code_r0x0001401640c2;
                                }
                              }
                              (&stack0x00001ad7)[lVar30] = 1;
                              *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163fb1;
                              FUN_1428d9310(&UNK_14295d6d0);
                              goto code_r0x0001401640c2;
                            }
                          }
                          (&stack0x00001ad7)[lVar30] = 1;
                          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f99;
                          FUN_1428d9310(&UNK_14295d6b8);
                          goto code_r0x0001401640c2;
                        }
                      }
                      (&stack0x00001ad7)[lVar30] = 1;
                      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f81;
                      FUN_1428d9310(&UNK_14295d690);
                      goto code_r0x0001401640c2;
                    }
                  }
                  (&stack0x00001ad4)[lVar30] = 1;
                  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f69;
                  FUN_1428d9310(&UNK_14295d678);
                  goto code_r0x0001401640c2;
                }
              }
              (&stack0x00001ad4)[lVar30] = 1;
              *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f51;
              FUN_1428d9310(&UNK_14295d660);
              goto code_r0x0001401640c2;
            }
          }
          (&stack0x00001ad4)[lVar30] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f39;
          FUN_1428d9310(&UNK_14295d648);
          goto code_r0x0001401640c2;
        }
      }
      (&stack0x00001ad3)[lVar30] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140163f21;
      FUN_1428d9310(&UNK_14295d630);
      goto code_r0x0001401640c2;
    }
  }
  *(undefined **)((longlong)&puStack_48 + lVar30) = &UNK_140164075;
  FUN_1428d9310(&UNK_14295d5f0);
code_r0x0001401640c2:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}


