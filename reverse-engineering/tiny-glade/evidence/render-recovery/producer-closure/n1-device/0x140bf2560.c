
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_140bf2560(undefined8 *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4)

{
  longlong lVar1;
  char *pcVar2;
  byte *pbVar3;
  uint *puVar4;
  char cVar5;
  longlong *plVar6;
  ulonglong *puVar7;
  code *pcVar8;
  undefined8 *puVar9;
  uint uVar10;
  uint uVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  undefined8 uVar27;
  undefined8 uVar28;
  undefined4 uVar29;
  undefined4 uVar30;
  undefined4 uVar31;
  undefined4 uVar32;
  undefined4 uVar33;
  undefined4 uVar34;
  undefined4 uVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  undefined4 uVar38;
  undefined4 uVar39;
  undefined8 uVar40;
  undefined8 uVar41;
  undefined8 uVar42;
  undefined8 uVar43;
  undefined8 uVar44;
  undefined8 uVar45;
  undefined8 uVar46;
  undefined8 uVar47;
  undefined8 uVar48;
  undefined8 uVar49;
  undefined8 uVar50;
  undefined8 uVar51;
  undefined8 uVar52;
  undefined8 uVar53;
  undefined8 uVar54;
  undefined8 *puVar55;
  short sVar56;
  undefined1 uVar57;
  ushort uVar58;
  undefined4 uVar59;
  int iVar60;
  byte extraout_var;
  longlong lVar61;
  undefined8 *puVar62;
  longlong lVar63;
  undefined4 *puVar64;
  ulonglong uVar65;
  undefined4 *puVar66;
  ulonglong uVar67;
  undefined1 *puVar68;
  undefined8 uVar69;
  undefined8 uVar70;
  ulonglong uVar71;
  undefined8 uVar72;
  undefined4 uVar73;
  uint uVar74;
  longlong lVar75;
  undefined1 *puVar76;
  undefined *puVar77;
  undefined8 uVar78;
  longlong lVar79;
  undefined1 *puVar80;
  ulonglong uVar81;
  longlong lVar82;
  longlong lVar83;
  undefined *puVar84;
  undefined8 uVar85;
  ulonglong uVar86;
  undefined1 *puVar87;
  byte bVar88;
  undefined1 auVar89 [16];
  undefined1 auVar90 [16];
  undefined8 unaff_XMM6_Qa;
  undefined8 unaff_XMM6_Qb;
  char cVar91;
  char cVar92;
  char cVar93;
  undefined4 unaff_XMM7_Da;
  char cVar94;
  undefined4 unaff_XMM7_Db;
  undefined4 unaff_XMM7_Dc;
  undefined4 unaff_XMM7_Dd;
  undefined8 unaff_XMM8_Qa;
  undefined8 unaff_XMM8_Qb;
  undefined1 auVar95 [16];
  undefined4 unaff_XMM9_Da;
  undefined4 unaff_XMM9_Db;
  undefined4 unaff_XMM9_Dc;
  undefined4 unaff_XMM9_Dd;
  undefined4 unaff_XMM10_Da;
  undefined4 unaff_XMM10_Db;
  undefined4 unaff_XMM10_Dc;
  undefined4 unaff_XMM10_Dd;
  undefined4 unaff_XMM11_Da;
  undefined4 unaff_XMM11_Db;
  undefined4 unaff_XMM11_Dc;
  undefined4 unaff_XMM11_Dd;
  undefined4 unaff_XMM12_Da;
  undefined4 unaff_XMM12_Db;
  undefined4 unaff_XMM12_Dc;
  undefined4 unaff_XMM12_Dd;
  undefined4 unaff_XMM13_Da;
  undefined4 unaff_XMM13_Db;
  undefined4 unaff_XMM13_Dc;
  undefined4 unaff_XMM13_Dd;
  undefined4 unaff_XMM14_Da;
  undefined4 unaff_XMM14_Db;
  undefined4 unaff_XMM14_Dc;
  undefined4 unaff_XMM14_Dd;
  undefined4 unaff_XMM15_Da;
  undefined4 unaff_XMM15_Db;
  undefined4 unaff_XMM15_Dc;
  undefined4 unaff_XMM15_Dd;
  undefined1 auStackX_8 [32];
  undefined *puStack_48;
  
  puStack_48 = &UNK_140bf2576;
  lVar61 = FUN_14289aa40();
  lVar61 = -lVar61;
  *(undefined4 *)(&stack0x000033a0 + lVar61) = unaff_XMM15_Da;
  *(undefined4 *)(&stack0x000033a4 + lVar61) = unaff_XMM15_Db;
  *(undefined4 *)(&stack0x000033a8 + lVar61) = unaff_XMM15_Dc;
  *(undefined4 *)(&stack0x000033ac + lVar61) = unaff_XMM15_Dd;
  *(undefined4 *)(&stack0x00003390 + lVar61) = unaff_XMM14_Da;
  *(undefined4 *)(&stack0x00003394 + lVar61) = unaff_XMM14_Db;
  *(undefined4 *)(&stack0x00003398 + lVar61) = unaff_XMM14_Dc;
  *(undefined4 *)(&stack0x0000339c + lVar61) = unaff_XMM14_Dd;
  *(undefined4 *)(&stack0x00003380 + lVar61) = unaff_XMM13_Da;
  *(undefined4 *)(&stack0x00003384 + lVar61) = unaff_XMM13_Db;
  *(undefined4 *)(&stack0x00003388 + lVar61) = unaff_XMM13_Dc;
  *(undefined4 *)(&stack0x0000338c + lVar61) = unaff_XMM13_Dd;
  *(undefined4 *)(&stack0x00003370 + lVar61) = unaff_XMM12_Da;
  *(undefined4 *)(&stack0x00003374 + lVar61) = unaff_XMM12_Db;
  *(undefined4 *)(&stack0x00003378 + lVar61) = unaff_XMM12_Dc;
  *(undefined4 *)(&stack0x0000337c + lVar61) = unaff_XMM12_Dd;
  *(undefined4 *)(&stack0x00003360 + lVar61) = unaff_XMM11_Da;
  *(undefined4 *)(&stack0x00003364 + lVar61) = unaff_XMM11_Db;
  *(undefined4 *)(&stack0x00003368 + lVar61) = unaff_XMM11_Dc;
  *(undefined4 *)(&stack0x0000336c + lVar61) = unaff_XMM11_Dd;
  *(undefined4 *)(&stack0x00003350 + lVar61) = unaff_XMM10_Da;
  *(undefined4 *)(&stack0x00003354 + lVar61) = unaff_XMM10_Db;
  *(undefined4 *)(&stack0x00003358 + lVar61) = unaff_XMM10_Dc;
  *(undefined4 *)(&stack0x0000335c + lVar61) = unaff_XMM10_Dd;
  *(undefined4 *)(&stack0x00003340 + lVar61) = unaff_XMM9_Da;
  *(undefined4 *)(&stack0x00003344 + lVar61) = unaff_XMM9_Db;
  *(undefined4 *)(&stack0x00003348 + lVar61) = unaff_XMM9_Dc;
  *(undefined4 *)(&stack0x0000334c + lVar61) = unaff_XMM9_Dd;
  *(undefined8 *)(&stack0x00003330 + lVar61) = unaff_XMM8_Qa;
  *(undefined8 *)(&stack0x00003338 + lVar61) = unaff_XMM8_Qb;
  *(undefined4 *)(&stack0x00003320 + lVar61) = unaff_XMM7_Da;
  *(undefined4 *)(&stack0x00003324 + lVar61) = unaff_XMM7_Db;
  *(undefined4 *)(&stack0x00003328 + lVar61) = unaff_XMM7_Dc;
  *(undefined4 *)(&stack0x0000332c + lVar61) = unaff_XMM7_Dd;
  *(undefined8 *)(&stack0x00003310 + lVar61) = unaff_XMM6_Qa;
  *(undefined8 *)(&stack0x00003318 + lVar61) = unaff_XMM6_Qb;
  *(undefined8 *)(&stack0x00003308 + lVar61) = 0xfffffffffffffffe;
  *(undefined8 *)(&stack0x00003288 + lVar61) = param_4;
  *(undefined8 *)(&stack0x000032a0 + lVar61) = param_3;
  lVar83 = *param_2;
  uVar74 = *(uint *)(lVar83 + 0x680);
  *(uint *)(&stack0x00001710 + lVar61) = uVar74;
  if (uVar74 < 0x2000) {
    *(undefined1 **)(&stack0x000005b0 + lVar61) = &stack0x00001710 + lVar61;
    *(code **)(&stack0x000005b8 + lVar61) = FUN_140d8d560;
    *(undefined **)(&stack0x000005c0 + lVar61) = &UNK_142b163c8;
    *(code **)(&stack0x000005c8 + lVar61) = FUN_140d8d6c0;
    *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b16420;
    *(undefined8 *)(&stack0x00000c58 + lVar61) = 2;
    *(undefined1 **)(&stack0x00000c60 + lVar61) = &stack0x000005b0 + lVar61;
    *(undefined8 *)(&stack0x00000c68 + lVar61) = 2;
    *(undefined8 *)(&stack0x00000c70 + lVar61) = 0;
    (&stack0x000032fa)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf269b;
    FUN_140d79f90(&stack0x000022f8 + lVar61,&stack0x00000c50 + lVar61);
    *(undefined **)(&stack0x00002310 + lVar61) = &UNK_142b16440;
    *(undefined8 *)(&stack0x00002318 + lVar61) = 0x1a;
    param_1[10] = *(undefined8 *)(&stack0x00002338 + lVar61);
    uVar70 = *(undefined8 *)(&stack0x000022f8 + lVar61);
    uVar69 = *(undefined8 *)(&stack0x00002300 + lVar61);
    uVar78 = *(undefined8 *)(&stack0x00002308 + lVar61);
    uVar85 = *(undefined8 *)(&stack0x00002310 + lVar61);
    uVar27 = *(undefined8 *)(&stack0x00002318 + lVar61);
    uVar28 = *(undefined8 *)(&stack0x00002320 + lVar61);
    uVar40 = *(undefined8 *)(&stack0x00002330 + lVar61);
    param_1[8] = *(undefined8 *)(&stack0x00002328 + lVar61);
    param_1[9] = uVar40;
    param_1[6] = uVar27;
    param_1[7] = uVar28;
    param_1[4] = uVar78;
    param_1[5] = uVar85;
    param_1[2] = uVar70;
    param_1[3] = uVar69;
    param_1[1] = 0xd;
code_r0x000140bf2b4b:
    *param_1 = 0x8000000000000000;
    goto code_r0x000140bf2b58;
  }
  *(longlong **)(&stack0x00003290 + lVar61) = param_2;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf270b;
  puVar62 = (undefined8 *)func_0x000140613c10(0x40,8);
  if (puVar62 == (undefined8 *)0x0) {
    (&stack0x000032fa)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5f29;
    FUN_1428d9000(8,0x40);
    goto code_r0x000140bf685d;
  }
  *puVar62 = &UNK_142b16726;
  puVar62[1] = &UNK_142b16737;
  puVar62[2] = &UNK_142b1674b;
  puVar62[3] = &UNK_142b1675f;
  puVar62[4] = &UNK_142b16773;
  puVar62[5] = &UNK_142b1678e;
  puVar62[6] = &UNK_142b167a7;
  puVar62[7] = &UNK_142b167d1;
  *(undefined8 *)(&stack0x00003180 + lVar61) = 8;
  *(undefined8 **)(&stack0x00003188 + lVar61) = puVar62;
  *(undefined8 *)(&stack0x00003190 + lVar61) = 8;
  *(undefined8 *)(&stack0x00002ee0 + lVar61) = *(undefined8 *)(&stack0x00003290 + lVar61);
  *(undefined1 **)(&stack0x00002ee8 + lVar61) = &stack0x00003180 + lVar61;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf27bd;
  uVar57 = FUN_140bf8fb0(&stack0x00002ee0 + lVar61,&UNK_142b167ee,0x1d);
  (&stack0x000032d6)[lVar61] = uVar57;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf27dc;
  uVar57 = FUN_140bf8fb0(&stack0x00002ee0 + lVar61,&UNK_142b1680b,0x15);
  (&stack0x000032bd)[lVar61] = uVar57;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf27fb;
  uVar57 = FUN_140bf8fb0(&stack0x00002ee0 + lVar61,&UNK_142b16820,0x1d);
  (&stack0x000032d7)[lVar61] = uVar57;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf281a;
  FUN_140bf8fb0(&stack0x00002ee0 + lVar61,&UNK_142b1683d);
  cVar5 = *(char *)(*(longlong *)(&stack0x00003288 + lVar61) + 3);
  *(undefined8 **)(&stack0x000032e8 + lVar61) = param_1;
  if (cVar5 == '\x01') {
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2847;
    uVar59 = FUN_140bf8fb0(&stack0x00002ee0 + lVar61,&UNK_142b1685a);
    *(undefined4 *)(&stack0x000032ac + lVar61) = uVar59;
  }
  else {
    *(undefined4 *)(&stack0x000032ac + lVar61) = 0;
  }
  *(longlong *)(&stack0x00003170 + lVar61) = lVar83 + 0x10;
  *(undefined8 *)(&stack0x00001710 + lVar61) = 0;
  *(undefined8 *)(&stack0x00001718 + lVar61) = 8;
  *(undefined8 *)(&stack0x00001720 + lVar61) = 0;
  puVar62 = *(undefined8 **)(&stack0x00003188 + lVar61);
  *(undefined8 **)(&stack0x000032d8 + lVar61) = puVar62 + *(longlong *)(&stack0x00003190 + lVar61);
  *(longlong *)(&stack0x000032e0 + lVar61) = lVar83 + 0xc0;
  *(longlong *)(&stack0x000032c0 + lVar61) = lVar83;
  if (puVar62 != *(undefined8 **)(&stack0x000032d8 + lVar61)) {
code_r0x000140bf290f:
    do {
      uVar70 = *puVar62;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf291a;
      func_0x000142924010(uVar70);
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf292d;
      FUN_140d79f70(&stack0x000005b0 + lVar61,uVar70);
      if (*(longlong *)(lVar83 + 0xb8) != 0) {
        lVar79 = *(longlong *)(&stack0x000005c0 + lVar61);
        *(undefined8 *)(&stack0x000032b0 + lVar61) = *(undefined8 *)(&stack0x000005b8 + lVar61);
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf295f;
        uVar71 = FUN_1405b1a70();
        lVar63 = *(longlong *)(lVar83 + 0xa0);
        uVar86 = *(ulonglong *)(lVar83 + 0xa8);
        bVar88 = (byte)(uVar71 >> 0x39);
        auVar89 = ZEXT216(CONCAT11(bVar88,bVar88));
        auVar89 = pshuflw(auVar89,auVar89,0);
        lVar75 = 0;
        while( true ) {
          uVar71 = uVar86 & uVar71;
          pcVar2 = (char *)(lVar63 + uVar71);
          cVar5 = *pcVar2;
          cVar12 = pcVar2[1];
          cVar13 = pcVar2[2];
          cVar14 = pcVar2[3];
          cVar15 = pcVar2[4];
          cVar16 = pcVar2[5];
          cVar17 = pcVar2[6];
          cVar18 = pcVar2[7];
          cVar19 = pcVar2[8];
          cVar20 = pcVar2[9];
          cVar21 = pcVar2[10];
          cVar22 = pcVar2[0xb];
          cVar23 = pcVar2[0xc];
          cVar24 = pcVar2[0xd];
          cVar25 = pcVar2[0xe];
          cVar26 = pcVar2[0xf];
          cVar91 = auVar89[0];
          auVar90[0] = -(cVar5 == cVar91);
          cVar92 = auVar89[1];
          auVar90[1] = -(cVar12 == cVar92);
          cVar93 = auVar89[2];
          auVar90[2] = -(cVar13 == cVar93);
          cVar94 = auVar89[3];
          auVar90[3] = -(cVar14 == cVar94);
          auVar90[4] = -(cVar15 == cVar91);
          auVar90[5] = -(cVar16 == cVar92);
          auVar90[6] = -(cVar17 == cVar93);
          auVar90[7] = -(cVar18 == cVar94);
          auVar90[8] = -(cVar19 == cVar91);
          auVar90[9] = -(cVar20 == cVar92);
          auVar90[10] = -(cVar21 == cVar93);
          auVar90[0xb] = -(cVar22 == cVar94);
          auVar90[0xc] = -(cVar23 == cVar91);
          auVar90[0xd] = -(cVar24 == cVar92);
          auVar90[0xe] = -(cVar25 == cVar93);
          auVar90[0xf] = -(cVar26 == cVar94);
          uVar58 = (ushort)(SUB161(auVar90 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar90 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar90 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar90 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar90 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar90 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar90 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar90 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar90 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar90 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar90 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar90 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar90 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar90 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar90 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar90[0xf] >> 7) << 0xf;
          uVar74 = (uint)uVar58;
          if (uVar58 != 0) {
            *(longlong *)(&stack0x00003298 + lVar61) = lVar75;
            do {
              uVar10 = 0;
              for (uVar11 = uVar74; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
                uVar10 = uVar10 + 1;
              }
              uVar65 = uVar10 + uVar71 & uVar86;
              if (*(longlong *)(lVar63 + -8 + uVar65 * -0x18) == lVar79) {
                uVar70 = *(undefined8 *)(lVar63 + uVar65 * -0x18 + -0x10);
                *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf29e0;
                iVar60 = func_0x000142923bf0(*(undefined8 *)(&stack0x000032b0 + lVar61),uVar70);
                if (iVar60 == 0) {
                  lVar83 = *(longlong *)(&stack0x000032c0 + lVar61);
                  if ((*(ulonglong *)(&stack0x000005b0 + lVar61) & 0x7fffffffffffffff) == 0) {
                    puVar62 = puVar62 + 1;
                    if (puVar62 == *(undefined8 **)(&stack0x000032d8 + lVar61))
                    goto code_r0x000140bf2ae1;
                    goto code_r0x000140bf290f;
                  }
                  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2ad0;
                  func_0x000140613c20(*(undefined8 *)(&stack0x000032b0 + lVar61));
                  puVar62 = puVar62 + 1;
                  if (puVar62 != *(undefined8 **)(&stack0x000032d8 + lVar61))
                  goto code_r0x000140bf290f;
                  goto code_r0x000140bf2ae1;
                }
              }
              uVar58 = (ushort)(uVar74 - 1) & (ushort)uVar74;
              uVar74 = CONCAT22((short)(uVar74 - 1 >> 0x10),uVar58);
              lVar75 = *(longlong *)(&stack0x00003298 + lVar61);
            } while (uVar58 != 0);
          }
          auVar95[0] = -(cVar5 == -1);
          auVar95[1] = -(cVar12 == -1);
          auVar95[2] = -(cVar13 == -1);
          auVar95[3] = -(cVar14 == -1);
          auVar95[4] = -(cVar15 == -1);
          auVar95[5] = -(cVar16 == -1);
          auVar95[6] = -(cVar17 == -1);
          auVar95[7] = -(cVar18 == -1);
          auVar95[8] = -(cVar19 == -1);
          auVar95[9] = -(cVar20 == -1);
          auVar95[10] = -(cVar21 == -1);
          auVar95[0xb] = -(cVar22 == -1);
          auVar95[0xc] = -(cVar23 == -1);
          auVar95[0xd] = -(cVar24 == -1);
          auVar95[0xe] = -(cVar25 == -1);
          auVar95[0xf] = -(cVar26 == -1);
          lVar83 = *(longlong *)(&stack0x000032c0 + lVar61);
          if ((((((((((((((((SUB161(auVar95 >> 7,0) & 1) != 0 || (SUB161(auVar95 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar95 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar95 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar95 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar95 >> 0x2f,0) & 1) != 0) || (SUB161(auVar95 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar95 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar95 >> 0x47,0) & 1) != 0) || (SUB161(auVar95 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar95 >> 0x57,0) & 1) != 0) || (SUB161(auVar95 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar95 >> 0x67,0) & 1) != 0) || (SUB161(auVar95 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar95 >> 0x77,0) & 1) != 0) || auVar95[0xf] < '\0') break;
          uVar71 = uVar71 + lVar75 + 0x10;
          lVar75 = lVar75 + 0x10;
        }
      }
      *(undefined8 *)(&stack0x00002300 + lVar61) = *(undefined8 *)(&stack0x000005c0 + lVar61);
      *(undefined8 *)(&stack0x000022f0 + lVar61) = *(undefined8 *)(&stack0x000005b0 + lVar61);
      *(undefined8 *)(&stack0x000022f8 + lVar61) = *(undefined8 *)(&stack0x000005b8 + lVar61);
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2a5f;
      FUN_140d7bed0(&stack0x00000c50 + lVar61,&stack0x000022f0 + lVar61);
      lVar79 = *(longlong *)(&stack0x00001720 + lVar61);
      if (lVar79 == *(longlong *)(&stack0x00001710 + lVar61)) {
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2a86;
        FUN_140562db0(&stack0x00001710 + lVar61,&UNK_142b16880);
      }
      lVar63 = *(longlong *)(&stack0x00001718 + lVar61);
      *(undefined8 *)(lVar63 + 0x10 + lVar79 * 0x18) = *(undefined8 *)(&stack0x00000c60 + lVar61);
      *(undefined1 (*) [16])(lVar63 + lVar79 * 0x18) =
           *(undefined1 (*) [16])(&stack0x00000c50 + lVar61);
      *(longlong *)(&stack0x00001720 + lVar61) = lVar79 + 1;
      puVar62 = puVar62 + 1;
    } while (puVar62 != *(undefined8 **)(&stack0x000032d8 + lVar61));
  }
code_r0x000140bf2ae1:
  lVar79 = *(longlong *)(&stack0x00001720 + lVar61);
  if (lVar79 != 0) {
    uVar70 = *(undefined8 *)(&stack0x00001710 + lVar61);
    uVar69 = *(undefined8 *)(&stack0x00001718 + lVar61);
    param_1 = *(undefined8 **)(&stack0x000032e8 + lVar61);
    if (*(longlong *)(&stack0x00003180 + lVar61) != 0) {
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2b28;
      func_0x000140613c20(*(undefined8 *)(&stack0x00003188 + lVar61),
                          *(longlong *)(&stack0x00003180 + lVar61) << 3,8);
    }
    param_1[2] = uVar70;
    param_1[3] = uVar69;
    param_1[4] = lVar79;
    param_1[1] = 0xb;
    *(undefined1 *)(param_1 + 5) = 0;
    *(char *)((longlong)param_1 + 0x2b) = (char)*(undefined4 *)(&stack0x000032ac + lVar61);
    goto code_r0x000140bf2b4b;
  }
  param_1 = *(undefined8 **)(&stack0x000032e8 + lVar61);
  if (*(longlong *)(&stack0x00001710 + lVar61) != 0) {
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2c03;
    func_0x000140613c20(*(undefined8 *)(&stack0x00001718 + lVar61),
                        *(longlong *)(&stack0x00001710 + lVar61) * 0x18);
  }
  puVar7 = puRam000000014362c3a8;
  lVar79 = *(longlong *)(&stack0x00003180 + lVar61);
  uVar70 = *(undefined8 *)(&stack0x00003188 + lVar61);
  uVar69 = *(undefined8 *)(&stack0x00003190 + lVar61);
  lVar63 = *(longlong *)(lVar83 + 0x38);
  lVar75 = *(longlong *)(lVar83 + 0x40);
  lVar82 = 0;
  do {
    if (lVar75 * 0x1c == lVar82) {
      param_1[1] = 0xe;
      *param_1 = 0x8000000000000000;
      goto code_r0x000140bf3dbc;
    }
    lVar1 = lVar82 + 0x1c;
    pbVar3 = (byte *)(lVar63 + lVar82);
    lVar82 = lVar1;
  } while ((*pbVar3 & 1) == 0);
  uVar78 = *(undefined8 *)(lVar63 + -0xc + lVar1);
  *(undefined8 *)(&stack0x00001700 + lVar61) = uVar78;
  puVar62 = (undefined8 *)(lVar63 + -0x1c + lVar1);
  uVar85 = *puVar62;
  uVar27 = puVar62[1];
  *(undefined8 *)(&stack0x000016f0 + lVar61) = uVar85;
  *(undefined8 *)(&stack0x000016f8 + lVar61) = uVar27;
  uVar59 = *(undefined4 *)(lVar63 + -4 + lVar1);
  *(undefined8 *)(&stack0x00002ed0 + lVar61) = uVar78;
  *(undefined8 *)(&stack0x00002ec0 + lVar61) = uVar85;
  *(undefined8 *)(&stack0x00002ec8 + lVar61) = uVar27;
  *(undefined8 *)(&stack0x00002d50 + lVar61) = uVar85;
  *(undefined8 *)(&stack0x00002d58 + lVar61) = uVar27;
  *(undefined8 *)(&stack0x00002d60 + lVar61) = uVar78;
  *(undefined4 *)(&stack0x00002d68 + lVar61) = uVar59;
  uVar86 = *puRam000000014362c3a8;
  *(longlong *)(&stack0x000032e0 + lVar61) = lVar79;
  *(undefined8 *)(&stack0x000032d8 + lVar61) = uVar70;
  if (4 < uVar86) {
    *(undefined1 **)(&stack0x000005b0 + lVar61) = &stack0x00002d50 + lVar61;
    *(code **)(&stack0x000005b8 + lVar61) = FUN_140bffb20;
    *(undefined8 *)(&stack0x00000c50 + lVar61) = 2;
    *(undefined8 *)(&stack0x00000c60 + lVar61) = 2;
    *(undefined8 *)(&stack0x00000c68 + lVar61) = 8;
    *(undefined8 *)(&stack0x00000c70 + lVar61) = 0;
    *(undefined8 *)(&stack0x00000c78 + lVar61) = 0x400000020;
    (&stack0x00000c80)[lVar61] = 3;
    (&stack0x000032ff)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2d20;
    puVar62 = (undefined8 *)func_0x00014067dd80(&UNK_142b16018);
    uVar70 = *puVar62;
    uVar78 = puVar62[1];
    uVar73 = *(undefined4 *)(puVar62 + 2);
    *(undefined8 *)(&stack0x00002320 + lVar61) = 5;
    *(undefined **)(&stack0x00002328 + lVar61) = &UNK_142b16030;
    *(undefined8 *)(&stack0x00002330 + lVar61) = 0x15;
    *(undefined **)(&stack0x00002340 + lVar61) = &UNK_142b16008;
    *(undefined8 *)(&stack0x00002348 + lVar61) = 1;
    *(undefined1 **)(&stack0x00002350 + lVar61) = &stack0x000005b0 + lVar61;
    *(undefined8 *)(&stack0x00002358 + lVar61) = 1;
    *(undefined1 **)(&stack0x00002360 + lVar61) = &stack0x00000c50 + lVar61;
    *(undefined8 *)(&stack0x00002368 + lVar61) = 1;
    *(undefined8 *)(&stack0x000022f0 + lVar61) = 0;
    *(undefined **)(&stack0x000022f8 + lVar61) = &UNK_142b16030;
    *(undefined8 *)(&stack0x00002300 + lVar61) = 0x15;
    *(undefined8 *)(&stack0x00002308 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002310 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00002318 + lVar61) = uVar78;
    *(undefined4 *)(&stack0x00002338 + lVar61) = 1;
    *(undefined4 *)(&stack0x0000233c + lVar61) = uVar73;
    (&stack0x000032ff)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2df0;
    func_0x000140d4a330(&stack0x00001710 + lVar61,&stack0x000022f0 + lVar61);
    lVar63 = *(longlong *)(lVar83 + 0x38);
    lVar75 = *(longlong *)(lVar83 + 0x40);
  }
  uVar73 = 0;
  lVar83 = 0;
  do {
    if (lVar75 * 0x1c == lVar83) goto code_r0x000140bf2e48;
    puVar4 = (uint *)(lVar63 + lVar83);
    lVar83 = lVar83 + 0x1c;
  } while ((*puVar4 & 3) != 2);
  puVar62 = (undefined8 *)(lVar63 + -0x1c + lVar83);
  uVar70 = *puVar62;
  uVar78 = puVar62[1];
  puVar62 = (undefined8 *)(lVar63 + -0x10 + lVar83);
  uVar85 = puVar62[1];
  *(undefined8 *)(&stack0x00002ff0 + lVar61) = *puVar62;
  *(undefined8 *)(&stack0x00002ff8 + lVar61) = uVar85;
  *(undefined8 *)(&stack0x00002fe4 + lVar61) = uVar70;
  *(undefined8 *)(&stack0x00002fec + lVar61) = uVar78;
  uVar73 = 1;
code_r0x000140bf2e48:
  *(undefined4 *)(&stack0x00002fe0 + lVar61) = uVar73;
  if (4 < *puVar7) {
    *(undefined1 **)(&stack0x000005b0 + lVar61) = &stack0x00002fe0 + lVar61;
    *(code **)(&stack0x000005b8 + lVar61) = FUN_140bede60;
    *(undefined8 *)(&stack0x00000c50 + lVar61) = 2;
    *(undefined8 *)(&stack0x00000c60 + lVar61) = 2;
    *(undefined8 *)(&stack0x00000c68 + lVar61) = 8;
    *(undefined8 *)(&stack0x00000c70 + lVar61) = 0;
    *(undefined8 *)(&stack0x00000c78 + lVar61) = 0x400000020;
    (&stack0x00000c80)[lVar61] = 3;
    (&stack0x000032ff)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2ece;
    puVar62 = (undefined8 *)func_0x00014067dd80(&UNK_142b16070);
    uVar70 = *puVar62;
    uVar78 = puVar62[1];
    uVar73 = *(undefined4 *)(puVar62 + 2);
    *(undefined8 *)(&stack0x00002320 + lVar61) = 5;
    *(undefined **)(&stack0x00002328 + lVar61) = &UNK_142b16030;
    *(undefined8 *)(&stack0x00002330 + lVar61) = 0x15;
    *(undefined **)(&stack0x00002340 + lVar61) = &UNK_142b16060;
    *(undefined8 *)(&stack0x00002348 + lVar61) = 1;
    *(undefined1 **)(&stack0x00002350 + lVar61) = &stack0x000005b0 + lVar61;
    *(undefined8 *)(&stack0x00002358 + lVar61) = 1;
    *(undefined1 **)(&stack0x00002360 + lVar61) = &stack0x00000c50 + lVar61;
    *(undefined8 *)(&stack0x00002368 + lVar61) = 1;
    *(undefined8 *)(&stack0x000022f0 + lVar61) = 0;
    *(undefined **)(&stack0x000022f8 + lVar61) = &UNK_142b16030;
    *(undefined8 *)(&stack0x00002300 + lVar61) = 0x15;
    *(undefined8 *)(&stack0x00002308 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002310 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00002318 + lVar61) = uVar78;
    *(undefined4 *)(&stack0x00002338 + lVar61) = 1;
    *(undefined4 *)(&stack0x0000233c + lVar61) = uVar73;
    (&stack0x000032ff)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2f9e;
    func_0x000140d4a330(&stack0x00001710 + lVar61,&stack0x000022f0 + lVar61);
  }
  (&stack0x000032ff)[lVar61] = 1;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2fb1;
  func_0x000140d4a3b0(&stack0x00003090 + lVar61);
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2fc7;
  puVar64 = (undefined4 *)func_0x000140613c10(0x28,8);
  if (puVar64 == (undefined4 *)0x0) {
    (&stack0x000032f3)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6739;
    FUN_1428d9000(8,0x28);
    goto code_r0x000140bf685d;
  }
  *(undefined4 *)(&stack0x000022fc + lVar61) = 0;
  *(undefined8 *)(&stack0x000022f4 + lVar61) = 0;
  *(undefined4 *)(&stack0x00000c50 + lVar61) = *(undefined4 *)(&stack0x000022f0 + lVar61);
  *(undefined8 *)(&stack0x00000c54 + lVar61) = *(undefined8 *)(&stack0x000022f4 + lVar61);
  *(undefined4 *)(&stack0x00000c5c + lVar61) = *(undefined4 *)(&stack0x000022fc + lVar61);
  puVar7 = *(ulonglong **)(&stack0x000030a0 + lVar61);
  uVar65 = puVar7[4] & 0xfffffffffffffffc;
  uVar86 = *puVar7;
  uVar71 = *puVar7;
  *(undefined4 **)(&stack0x000032b0 + lVar61) = puVar64;
  if (uVar65 - uVar71 < 4 || uVar65 < uVar86) {
code_r0x000140bf3044:
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf305b;
    puVar66 = (undefined4 *)FUN_1428d7780(&stack0x00003090 + lVar61,4,4);
    puVar64 = *(undefined4 **)(&stack0x000032b0 + lVar61);
    if (puVar66 == (undefined4 *)0x0) {
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6743;
      FUN_1428d79d0();
      goto code_r0x000140bf685d;
    }
  }
  else {
    puVar66 = (undefined4 *)(uVar65 - 4);
    puVar7[4] = (ulonglong)puVar66;
    if (puVar66 == (undefined4 *)0x0) goto code_r0x000140bf3044;
  }
  *puVar66 = 0x3f000000;
  uVar70 = *(undefined8 *)(&stack0x00000c58 + lVar61);
  *(undefined8 *)(puVar64 + 1) = *(undefined8 *)(&stack0x00000c50 + lVar61);
  *(undefined8 *)(puVar64 + 3) = uVar70;
  *puVar64 = 2;
  *(undefined4 *)(&stack0x00003298 + lVar61) = uVar59;
  puVar64[5] = uVar59;
  puVar64[6] = 1;
  *(undefined4 **)(puVar64 + 8) = puVar66;
  *(undefined8 *)(&stack0x00003270 + lVar61) = 1;
  *(undefined4 **)(&stack0x00003278 + lVar61) = puVar64;
  *(undefined8 *)(&stack0x00003280 + lVar61) = 1;
  *(undefined8 *)(&stack0x000030a8 + lVar61) = uVar69;
  if (*(int *)(&stack0x00002fe0 + lVar61) == 1) {
    uVar74 = *(uint *)(&stack0x00002fe8 + lVar61);
    uVar86 = (ulonglong)uVar74;
    uVar59 = *(undefined4 *)(&stack0x00002ffc + lVar61);
    uVar71 = uVar86 * 4;
    puVar7 = *(ulonglong **)(&stack0x000030a0 + lVar61);
    uVar65 = puVar7[4] & 0xfffffffffffffffc;
    if ((uVar65 < *puVar7) || (uVar65 - *puVar7 <= uVar71 && uVar71 - (uVar65 - *puVar7) != 0)) {
code_r0x000140bf3108:
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3123;
      uVar65 = FUN_1428d7780(&stack0x00003090 + lVar61,4,uVar71);
      if (uVar65 == 0) {
        (&stack0x00003307)[lVar61] = 1;
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6754;
        FUN_1428d79d0();
        goto code_r0x000140bf685d;
      }
    }
    else {
      uVar65 = uVar65 + uVar86 * -4;
      puVar7[4] = uVar65;
      if (uVar65 == 0) goto code_r0x000140bf3108;
    }
    uVar69 = _UNK_142925938;
    uVar70 = _UNK_142925930;
    if (uVar74 != 0) {
      if (uVar74 < 8) {
        uVar67 = 0;
      }
      else {
        uVar67 = (ulonglong)(uVar74 & 0xfffffff8);
        uVar81 = 0;
        do {
          *(undefined8 *)(uVar65 + uVar81) = uVar70;
          ((undefined8 *)(uVar65 + uVar81))[1] = uVar69;
          puVar62 = (undefined8 *)(uVar65 + 0x10 + uVar81);
          *puVar62 = uVar70;
          puVar62[1] = uVar69;
          uVar81 = uVar81 + 0x20;
        } while ((uVar71 & 0xffffffffffffffe0) != uVar81);
        if ((uVar74 & 0xfffffff8) == uVar74) goto code_r0x000140bf31b0;
      }
      do {
        *(undefined4 *)(uVar65 + uVar67 * 4) = 0x3f800000;
        uVar67 = uVar67 + 1;
      } while (uVar86 != uVar67);
    }
code_r0x000140bf31b0:
    *(undefined4 *)(&stack0x000022fc + lVar61) = 0;
    *(undefined8 *)(&stack0x000022f4 + lVar61) = 0;
    *(undefined4 *)(&stack0x00000c50 + lVar61) = *(undefined4 *)(&stack0x000022f0 + lVar61);
    *(undefined8 *)(&stack0x00000c54 + lVar61) = *(undefined8 *)(&stack0x000022f4 + lVar61);
    *(undefined4 *)(&stack0x00000c5c + lVar61) = *(undefined4 *)(&stack0x000022fc + lVar61);
    lVar83 = *(longlong *)(&stack0x00003280 + lVar61);
    if (lVar83 == *(longlong *)(&stack0x00003270 + lVar61)) {
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3215;
      FUN_140562ab0(&stack0x00003270 + lVar61,&UNK_142b16088);
    }
    lVar79 = *(longlong *)(&stack0x00003278 + lVar61);
    *(undefined4 *)(lVar79 + lVar83 * 0x28) = 2;
    uVar70 = *(undefined8 *)(&stack0x00000c58 + lVar61);
    puVar62 = (undefined8 *)(lVar79 + 4 + lVar83 * 0x28);
    *puVar62 = *(undefined8 *)(&stack0x00000c50 + lVar61);
    puVar62[1] = uVar70;
    *(undefined4 *)(lVar79 + 0x14 + lVar83 * 0x28) = uVar59;
    *(uint *)(lVar79 + 0x18 + lVar83 * 0x28) = uVar74;
    *(ulonglong *)(lVar79 + 0x20 + lVar83 * 0x28) = uVar65;
    lVar83 = lVar83 + 1;
    *(longlong *)(&stack0x00003280 + lVar61) = lVar83;
    *(undefined8 *)(&stack0x000032b0 + lVar61) = *(undefined8 *)(&stack0x00003278 + lVar61);
  }
  else {
    lVar83 = 1;
    uVar86 = 0;
  }
  bVar88 = *(byte *)(*(longlong *)(&stack0x00003288 + lVar61) + 2);
  (&stack0x00003110)[lVar61] = *(undefined1 *)(*(longlong *)(&stack0x00003288 + lVar61) + 4);
  lVar79 = *(longlong *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
  *(undefined4 *)(&stack0x00003180 + lVar61) = 0x33;
  *(undefined1 (*) [16])(&stack0x00003188 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00003198 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031a8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031b8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031c8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031d8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031e8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000031f8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00003208 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00003218 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00003228 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00003238 + lVar61) = (undefined1  [16])0x0;
  *(undefined4 *)(&stack0x00003248 + lVar61) = 0;
  *(undefined4 *)(&stack0x00002ee0 + lVar61) = 0x3b9f0020;
  *(undefined8 *)(&stack0x00002ee8 + lVar61) = 0;
  *(undefined4 *)(&stack0x00002ef0 + lVar61) = 0;
  *(undefined4 *)(&stack0x000022f0 + lVar61) = 0x3b9bb078;
  *(undefined1 (*) [16])(&stack0x000022f8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002308 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002318 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002328 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002338 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002348 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002358 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002368 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002378 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002388 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x00002398 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000023a8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000023b8 + lVar61) = (undefined1  [16])0x0;
  *(undefined1 (*) [16])(&stack0x000023c8 + lVar61) = (undefined1  [16])0x0;
  *(undefined4 *)(&stack0x000023d8 + lVar61) = 0;
  puVar80 = &stack0x00002300 + lVar61;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf33b7;
  func_0x000142923c00(&stack0x00001710 + lVar61,puVar80,0xe0);
  puVar68 = &stack0x00003180 + lVar61;
  do {
    puVar68 = *(undefined1 **)(puVar68 + 8);
  } while (puVar68 != (undefined1 *)0x0);
  puVar68 = &stack0x00003180 + lVar61;
  *(undefined1 **)(&stack0x000022f8 + lVar61) = puVar68;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf33ec;
  func_0x000142923c00(puVar80,&stack0x00001710 + lVar61,0xe0);
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3401;
  func_0x000142923c00(&stack0x00001710 + lVar61,puVar80,0xe0);
  puVar87 = &stack0x00002ee0 + lVar61;
  do {
    puVar76 = puVar87;
    puVar87 = *(undefined1 **)(puVar76 + 8);
  } while (puVar87 != (undefined1 *)0x0);
  *(undefined1 **)(puVar76 + 8) = puVar68;
  *(undefined1 **)(&stack0x000022f8 + lVar61) = &stack0x00002ee0 + lVar61;
  puVar87 = &stack0x00001710 + lVar61;
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3446;
  func_0x000142923c00(puVar80,puVar87,0xe0);
  uVar70 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0xd0);
  pcVar8 = *(code **)(lVar79 + 0xb0);
  *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3462;
  (*pcVar8)(uVar70,&stack0x000022f0 + lVar61);
  uVar69 = 4;
  puVar77 = &UNK_142b1645a;
  if (*(int *)(&stack0x00002300 + lVar61) == 0) {
    puVar84 = &UNK_142b1645e;
    uVar78 = 0x14;
code_r0x000140bf3ccf:
    lVar79 = *(longlong *)(&stack0x000032e0 + lVar61);
    uVar70 = *(undefined8 *)(&stack0x000032d8 + lVar61);
    param_1 = *(undefined8 **)(&stack0x000032e8 + lVar61);
    uVar86 = (ulonglong)puVar84 & 0xffffffff00000000;
    uVar85 = 0xc;
code_r0x000140bf3cf7:
    *(undefined8 *)(&stack0x000005d0 + lVar61) = *(undefined8 *)(&stack0x00000c70 + lVar61);
    uVar27 = *(undefined8 *)(&stack0x00000c50 + lVar61);
    uVar28 = *(undefined8 *)(&stack0x00000c58 + lVar61);
    uVar40 = *(undefined8 *)(&stack0x00000c60 + lVar61);
    uVar41 = *(undefined8 *)(&stack0x00000c68 + lVar61);
    *(undefined8 *)(&stack0x000005c0 + lVar61) = uVar40;
    *(undefined8 *)(&stack0x000005c8 + lVar61) = uVar41;
    *(undefined8 *)(&stack0x000005b0 + lVar61) = uVar27;
    *(undefined8 *)(&stack0x000005b8 + lVar61) = uVar28;
    param_1[10] = *(undefined8 *)(&stack0x00000c70 + lVar61);
    param_1[8] = uVar40;
    param_1[9] = uVar41;
    param_1[6] = uVar27;
    param_1[7] = uVar28;
    param_1[1] = uVar85;
    param_1[2] = puVar77;
    param_1[3] = uVar69;
    param_1[4] = (ulonglong)puVar84 & 0xffffffff | uVar86;
    param_1[5] = uVar78;
    *param_1 = 0x8000000000000000;
    lVar83 = *(longlong *)(&stack0x00003270 + lVar61);
  }
  else {
    if (*(int *)(&stack0x00002328 + lVar61) == 0) {
      puVar84 = &UNK_142b16472;
      uVar78 = 0x1c;
      goto code_r0x000140bf3ccf;
    }
    uVar78 = 0x13;
    if (*(int *)(&stack0x00002334 + lVar61) == 0) {
      puVar84 = &UNK_142b1648e;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x0000234c + lVar61) == 0) {
      puVar84 = &UNK_142b164a1;
      uVar78 = 0x12;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002358 + lVar61) == 0) {
      puVar84 = &UNK_142b164b3;
      uVar78 = 0x16;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002374 + lVar61) == 0) {
      puVar84 = &UNK_142b164c9;
      uVar78 = 0x25;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002380 + lVar61) == 0) {
      puVar84 = &UNK_142b164ee;
      uVar78 = 0x29;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x0000231c + lVar61) == 0) {
      puVar84 = &UNK_142b16517;
      uVar78 = 0xe;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002324 + lVar61) == 0) {
      puVar84 = &UNK_142b16525;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002368 + lVar61) == 0) {
      puVar84 = &UNK_142b16538;
      uVar78 = 0x1b;
      goto code_r0x000140bf3ccf;
    }
    iVar60 = *(int *)(&stack0x0000233c + lVar61);
    if (((bVar88 & 1) != 0) && (puVar84 = &UNK_142b16553, *(int *)(&stack0x00002364 + lVar61) == 0))
    {
code_r0x000140bf3519:
      uVar78 = 0x22;
      goto code_r0x000140bf3ccf;
    }
    uVar69 = 0x1e;
    puVar77 = &UNK_142b16575;
    if (*(int *)(&stack0x00003224 + lVar61) == 0) {
      puVar84 = &UNK_142b16593;
      uVar78 = 0x12;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x000031b4 + lVar61) == 0) {
      puVar84 = &UNK_142b165a5;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x000031fc + lVar61) == 0) {
      puVar84 = &UNK_142b165b8;
      goto code_r0x000140bf3519;
    }
    (&stack0x000032be)[lVar61] = *(int *)(&stack0x00003194 + lVar61) != 0;
    if (*(int *)(&stack0x000031e4 + lVar61) == 0) {
      puVar84 = &UNK_142b165da;
      uVar78 = 0x32;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x000031c8 + lVar61) == 0) {
      puVar84 = &UNK_142b1660c;
      uVar78 = 0x2f;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x000031f8 + lVar61) == 0) {
      puVar84 = &UNK_142b1663b;
      uVar78 = 0x2e;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00003200 + lVar61) == 0) {
      puVar84 = &UNK_142b16669;
      uVar78 = 0x2c;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00003204 + lVar61) == 0) {
      puVar84 = &UNK_142b16695;
      uVar78 = 0x18;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00003228 + lVar61) == 0) {
      puVar84 = &UNK_142b166ad;
      uVar78 = 0x15;
      goto code_r0x000140bf3ccf;
    }
    if (*(int *)(&stack0x00002ef0 + lVar61) == 0) {
      puVar84 = &UNK_142b166f6;
      uVar69 = 0x34;
      puVar77 = &UNK_142b166c2;
      goto code_r0x000140bf3519;
    }
    *(undefined4 *)(&stack0x00003180 + lVar61) = 0x33;
    *(undefined1 (*) [16])(&stack0x00003188 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00003198 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031a8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031b8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031c8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031d8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031e8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000031f8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00003208 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00003218 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00003228 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00003238 + lVar61) = (undefined1  [16])0x0;
    *(undefined4 *)(&stack0x00003248 + lVar61) = 0;
    *(undefined4 *)(&stack0x000030b0 + lVar61) = 0x3b9f0020;
    *(undefined8 *)(&stack0x000030b8 + lVar61) = 0;
    *(undefined4 *)(&stack0x000030c0 + lVar61) = 0;
    *(undefined4 *)(&stack0x00001710 + lVar61) = 0x3b9bb078;
    *(undefined1 (*) [16])(&stack0x00001718 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001728 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001738 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001748 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001758 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001768 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001778 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001788 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x00001798 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000017a8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000017b8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000017c8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000017d8 + lVar61) = (undefined1  [16])0x0;
    *(undefined1 (*) [16])(&stack0x000017e8 + lVar61) = (undefined1  [16])0x0;
    *(undefined4 *)(&stack0x000017f8 + lVar61) = 0;
    if (iVar60 != 0) {
      *(undefined4 *)(&stack0x0000175c + lVar61) = 1;
    }
    *(undefined4 *)(&stack0x00001720 + lVar61) = 1;
    *(undefined4 *)(&stack0x00001754 + lVar61) = 1;
    *(undefined4 *)(&stack0x0000176c + lVar61) = 1;
    *(undefined4 *)(&stack0x00001778 + lVar61) = 1;
    *(undefined4 *)(&stack0x00001794 + lVar61) = 1;
    *(undefined4 *)(&stack0x000017a0 + lVar61) = 1;
    *(undefined4 *)(&stack0x0000173c + lVar61) = 1;
    *(undefined8 *)(&stack0x00001744 + lVar61) = 0x100000001;
    *(undefined4 *)(&stack0x00001788 + lVar61) = 1;
    if ((bVar88 & 1) != 0) {
      *(undefined4 *)(&stack0x00001784 + lVar61) = 1;
    }
    *(undefined8 *)(&stack0x00003224 + lVar61) = 0x100000001;
    *(undefined4 *)(&stack0x000031b4 + lVar61) = 1;
    *(undefined4 *)(&stack0x000031e4 + lVar61) = 1;
    *(undefined4 *)(&stack0x000031c8 + lVar61) = 1;
    *(undefined8 *)(&stack0x000031f8 + lVar61) = _UNK_142958fe0;
    *(undefined8 *)(&stack0x00003200 + lVar61) = _UNK_142958fe8;
    if (*(int *)(&stack0x00003194 + lVar61) != 0) {
      *(undefined4 *)(&stack0x00003194 + lVar61) = 1;
    }
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3861;
    func_0x000142923c00(&stack0x000022f0 + lVar61,&stack0x00001720 + lVar61,0xe0);
    do {
      puVar68 = *(undefined1 **)(puVar68 + 8);
    } while (puVar68 != (undefined1 *)0x0);
    *(undefined1 **)(&stack0x00001718 + lVar61) = &stack0x00003180 + lVar61;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3894;
    func_0x000142923c00(&stack0x00001720 + lVar61,&stack0x000022f0 + lVar61,0xe0);
    *(undefined4 *)(&stack0x000030c0 + lVar61) = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf38b3;
    func_0x000142923c00(&stack0x000022f0 + lVar61,&stack0x00001720 + lVar61,0xe0);
    puVar80 = &stack0x000030b0 + lVar61;
    do {
      puVar68 = puVar80;
      puVar80 = *(undefined1 **)(puVar68 + 8);
    } while (puVar80 != (undefined1 *)0x0);
    *(undefined1 **)(puVar68 + 8) = &stack0x00003180 + lVar61;
    puVar80 = &stack0x000030b0 + lVar61;
    *(undefined1 **)(&stack0x00001718 + lVar61) = puVar80;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf38fb;
    func_0x000142923c00(&stack0x00001720 + lVar61,&stack0x000022f0 + lVar61,0xe0);
    *(undefined4 *)(&stack0x00002d70 + lVar61) = 0x3b9f5de1;
    *(undefined8 *)(&stack0x00002d78 + lVar61) = 0;
    *(uint *)(&stack0x00002d80 + lVar61) = ((byte)(&stack0x00003110)[lVar61] & 1) * 4 + 2;
    do {
      puVar87 = *(undefined1 **)(puVar87 + 8);
    } while (puVar87 != (undefined1 *)0x0);
    *(undefined4 *)(&stack0x00002ee0 + lVar61) = 3;
    *(undefined1 **)(&stack0x00002ee8 + lVar61) = &stack0x00001710 + lVar61;
    *(undefined4 *)(&stack0x00002ef0 + lVar61) = 0;
    *(int *)(&stack0x00002ef4 + lVar61) = (int)lVar83;
    *(undefined8 *)(&stack0x00002ef8 + lVar61) = *(undefined8 *)(&stack0x000032b0 + lVar61);
    *(undefined1 (*) [16])(&stack0x00002f00 + lVar61) = (undefined1  [16])0x0;
    *(int *)(&stack0x00002f10 + lVar61) = (int)*(undefined8 *)(&stack0x000030a8 + lVar61);
    *(undefined8 *)(&stack0x00002f18 + lVar61) = *(undefined8 *)(&stack0x000032d8 + lVar61);
    *(undefined8 *)(&stack0x00002f20 + lVar61) = 0;
    if ((&stack0x000032ac)[lVar61] != '\0') {
      *(undefined8 *)(&stack0x00002320 + lVar61) = *(undefined8 *)(&stack0x00002f20 + lVar61);
      *(undefined8 *)(&stack0x00002310 + lVar61) = *(undefined8 *)(&stack0x00002f10 + lVar61);
      *(undefined8 *)(&stack0x00002318 + lVar61) = *(undefined8 *)(&stack0x00002f18 + lVar61);
      *(undefined8 *)(&stack0x00002300 + lVar61) = *(undefined8 *)(&stack0x00002f00 + lVar61);
      *(undefined8 *)(&stack0x00002308 + lVar61) = *(undefined8 *)(&stack0x00002f08 + lVar61);
      *(undefined8 *)(&stack0x000022f0 + lVar61) = *(undefined8 *)(&stack0x00002ef0 + lVar61);
      *(undefined8 *)(&stack0x000022f8 + lVar61) = *(undefined8 *)(&stack0x00002ef8 + lVar61);
      puVar68 = &stack0x00002d70 + lVar61;
      do {
        puVar87 = puVar68;
        puVar68 = *(undefined1 **)(puVar87 + 8);
      } while (puVar68 != (undefined1 *)0x0);
      *(undefined1 **)(puVar87 + 8) = &stack0x00001710 + lVar61;
      *(undefined1 **)(&stack0x00002ee8 + lVar61) = &stack0x00002d70 + lVar61;
      *(undefined8 *)(&stack0x00002f20 + lVar61) = *(undefined8 *)(&stack0x00002320 + lVar61);
      *(undefined8 *)(&stack0x00002f10 + lVar61) = *(undefined8 *)(&stack0x00002310 + lVar61);
      *(undefined8 *)(&stack0x00002f18 + lVar61) = *(undefined8 *)(&stack0x00002318 + lVar61);
      *(undefined8 *)(&stack0x00002f00 + lVar61) = *(undefined8 *)(&stack0x00002300 + lVar61);
      *(undefined8 *)(&stack0x00002f08 + lVar61) = *(undefined8 *)(&stack0x00002308 + lVar61);
      *(undefined8 *)(&stack0x00002ef0 + lVar61) = *(undefined8 *)(&stack0x000022f0 + lVar61);
      *(undefined8 *)(&stack0x00002ef8 + lVar61) = *(undefined8 *)(&stack0x000022f8 + lVar61);
    }
    uVar70 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0xd0);
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3a54;
    FUN_140bed7f0(&stack0x000022f0 + lVar61,lVar79 + 0x40,uVar70,&stack0x00002ee0 + lVar61);
    *(longlong *)(&stack0x000032b0 + lVar61) = *(longlong *)(&stack0x000022f0 + lVar61);
    if (*(longlong *)(&stack0x000022f0 + lVar61) == 0) {
      puVar84 = (undefined *)(ulonglong)*(uint *)(&stack0x000022f8 + lVar61);
      uVar69 = 0xe;
      puVar77 = &UNK_142b16718;
      uVar85 = 0xf;
      uVar78 = 0;
      uVar86 = 0;
      lVar79 = *(longlong *)(&stack0x000032e0 + lVar61);
      uVar70 = *(undefined8 *)(&stack0x000032d8 + lVar61);
      param_1 = *(undefined8 **)(&stack0x000032e8 + lVar61);
      goto code_r0x000140bf3cf7;
    }
    (&stack0x000032bf)[lVar61] = iVar60 != 0;
    uVar70 = *(undefined8 *)(&stack0x000022f8 + lVar61);
    uVar69 = *(undefined8 *)(&stack0x00002300 + lVar61);
    uVar78 = *(undefined8 *)(&stack0x00002308 + lVar61);
    *(undefined8 *)(&stack0x00000c50 + lVar61) = *(undefined8 *)(&stack0x00002318 + lVar61);
    *(undefined8 *)(&stack0x00000c58 + lVar61) = *(undefined8 *)(&stack0x00002320 + lVar61);
    *(undefined8 *)(&stack0x00000c60 + lVar61) = *(undefined8 *)(&stack0x00002328 + lVar61);
    *(undefined8 *)(&stack0x00000c68 + lVar61) = *(undefined8 *)(&stack0x00002330 + lVar61);
    *(undefined8 *)(&stack0x00000c70 + lVar61) = *(undefined8 *)(&stack0x00002338 + lVar61);
    *(undefined8 *)(&stack0x00000c78 + lVar61) = *(undefined8 *)(&stack0x00002340 + lVar61);
    uVar85 = *(undefined8 *)(&stack0x00002310 + lVar61);
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3ad5;
    func_0x000142923c00(&stack0x00000c80 + lVar61,&stack0x00002348 + lVar61,0x578);
    puVar68 = &stack0x000005b0 + lVar61;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3af1;
    func_0x000142923c00(puVar68,&stack0x00000c50 + lVar61,0x5a8);
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3b03;
    func_0x000142923c00(auStackX_8 + lVar61,puVar68,0x5a8);
    *(undefined8 *)(&stack0x00001710 + lVar61) = *(undefined8 *)(&stack0x000032b0 + lVar61);
    *(undefined8 *)(&stack0x00002ea0 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00001718 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00002ea8 + lVar61) = uVar69;
    *(undefined8 *)(&stack0x00001720 + lVar61) = uVar69;
    *(undefined8 *)(&stack0x00002eb0 + lVar61) = uVar78;
    *(undefined8 *)(&stack0x00001728 + lVar61) = uVar78;
    *(undefined8 *)(&stack0x00002eb8 + lVar61) = uVar85;
    *(undefined8 *)(&stack0x00001730 + lVar61) = uVar85;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3b5e;
    func_0x000142923c00(&stack0x00001738 + lVar61,puVar68,0x5a8);
    if (4 < *puRam000000014362c3a8) {
      *(undefined **)(&stack0x000022f0 + lVar61) = &UNK_142b160b8;
      *(undefined8 *)(&stack0x000022f8 + lVar61) = 1;
      *(undefined8 *)(&stack0x00002300 + lVar61) = 8;
      *(undefined1 (*) [16])(&stack0x00002308 + lVar61) = (undefined1  [16])0x0;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3bb5;
      uVar70 = func_0x00014067dd80(&UNK_142b160c8);
      *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c58 + lVar61) = 0x15;
      *(undefined **)(&stack0x00000c60 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c68 + lVar61) = 0x15;
      *(undefined8 *)(&stack0x00000c70 + lVar61) = uVar70;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3c06;
      FUN_1404b1430(&stack0x000022f0 + lVar61,5,&stack0x00000c50 + lVar61);
    }
    lVar83 = *(longlong *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3c27;
    uVar71 = FUN_140c00530(*(undefined8 *)(&stack0x00003170 + lVar61));
    if (*(uint *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x224) < 2) {
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6797;
      FUN_1428d99e0(&UNK_142b160e0);
code_r0x000140bf685d:
                    /* WARNING: Does not return */
      pcVar8 = (code *)invalidInstructionException();
      (*pcVar8)();
    }
    uVar74 = *(uint *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x224) >> 1;
    if (uVar71 >> 0x20 == 0) {
      uVar65 = (uVar71 & 0xffffffff) / (ulonglong)uVar74;
      uVar71 = (uVar71 & 0xffffffff) % (ulonglong)uVar74;
    }
    else {
      uVar65 = uVar71 / uVar74;
      uVar71 = uVar71 % (ulonglong)uVar74;
    }
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3e53;
    uVar71 = FUN_140c00530(*(undefined8 *)(&stack0x00003170 + lVar61),uVar71);
    if (*(uint *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x224) < 2) {
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf67af;
      FUN_1428d99e0(&UNK_142b160f8);
      goto code_r0x000140bf685d;
    }
    uVar74 = *(uint *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x224) >> 1;
    if (uVar71 >> 0x20 == 0) {
      uVar67 = (uVar71 & 0xffffffff) / (ulonglong)uVar74;
      uVar71 = (uVar71 & 0xffffffff) % (ulonglong)uVar74;
    }
    else {
      uVar67 = uVar71 / uVar74;
      uVar71 = uVar71 % (ulonglong)uVar74;
    }
    uVar81 = uVar65 + 0x3fffff + (ulonglong)(uVar65 == 0) & 0xffffffffffc00000;
    uVar65 = uVar67 + 0x3fffff + (ulonglong)(uVar67 == 0) & 0xffffffffffc00000;
    if (3 < *puRam000000014362c3a8) {
      *(ulonglong *)(&stack0x00002ee0 + lVar61) = uVar81 >> 0x14;
      *(ulonglong *)(&stack0x00003180 + lVar61) = uVar65 >> 0x14;
      *(undefined1 **)(&stack0x000005b0 + lVar61) = &stack0x00002ee0 + lVar61;
      *(code **)(&stack0x000005b8 + lVar61) = FUN_140d8d6c0;
      *(undefined1 **)(&stack0x000005c0 + lVar61) = &stack0x00003180 + lVar61;
      *(code **)(&stack0x000005c8 + lVar61) = FUN_140d8d6c0;
      *(undefined **)(&stack0x000022f0 + lVar61) = &UNK_142b16140;
      *(undefined8 *)(&stack0x000022f8 + lVar61) = 3;
      *(undefined8 *)(&stack0x00002310 + lVar61) = 0;
      *(undefined1 **)(&stack0x00002300 + lVar61) = &stack0x000005b0 + lVar61;
      *(undefined8 *)(&stack0x00002308 + lVar61) = 2;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3f50;
      uVar70 = func_0x00014067dd80(&UNK_142b16170,uVar71);
      *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c58 + lVar61) = 0x15;
      *(undefined **)(&stack0x00000c60 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c68 + lVar61) = 0x15;
      *(undefined8 *)(&stack0x00000c70 + lVar61) = uVar70;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3fa1;
      FUN_1404b1430(&stack0x000022f0 + lVar61,4,&stack0x00000c50 + lVar61);
    }
    if (3 < *puRam000000014362c3a8) {
      *(longlong *)(&stack0x00003180 + lVar61) = *(longlong *)(&stack0x000032c0 + lVar61) + 0x410;
      *(code **)(&stack0x00003188 + lVar61) = FUN_1416d79a0;
      *(undefined8 *)(&stack0x000022f0 + lVar61) = 2;
      *(undefined8 *)(&stack0x00002300 + lVar61) = 2;
      *(undefined8 *)(&stack0x00002308 + lVar61) = 8;
      *(undefined8 *)(&stack0x00002310 + lVar61) = 0;
      *(undefined8 *)(&stack0x00002318 + lVar61) = 0x400000020;
      (&stack0x00002320)[lVar61] = 3;
      *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b161a8;
      *(undefined8 *)(&stack0x00000c58 + lVar61) = 1;
      *(undefined1 **)(&stack0x00000c70 + lVar61) = &stack0x000022f0 + lVar61;
      *(undefined8 *)(&stack0x00000c78 + lVar61) = 1;
      *(undefined1 **)(&stack0x00000c60 + lVar61) = &stack0x00003180 + lVar61;
      *(undefined8 *)(&stack0x00000c68 + lVar61) = 1;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4079;
      uVar70 = func_0x00014067dd80(&UNK_142b161b8);
      *(undefined **)(&stack0x000005b0 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x000005b8 + lVar61) = 0x15;
      *(undefined **)(&stack0x000005c0 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x000005c8 + lVar61) = 0x15;
      *(undefined8 *)(&stack0x000005d0 + lVar61) = uVar70;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf40ca;
      FUN_1404b1430(&stack0x00000c50 + lVar61,4,&stack0x000005b0 + lVar61);
    }
    if (3 < *puRam000000014362c3a8) {
      *(longlong *)(&stack0x00003180 + lVar61) = *(longlong *)(&stack0x000032c0 + lVar61) + 0x618;
      *(code **)(&stack0x00003188 + lVar61) = FUN_1418c83b0;
      *(undefined8 *)(&stack0x000022f0 + lVar61) = 2;
      *(undefined8 *)(&stack0x00002300 + lVar61) = 2;
      *(undefined8 *)(&stack0x00002308 + lVar61) = 8;
      *(undefined8 *)(&stack0x00002310 + lVar61) = 0;
      *(undefined8 *)(&stack0x00002318 + lVar61) = 0x400000020;
      (&stack0x00002320)[lVar61] = 3;
      *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b161e8;
      *(undefined8 *)(&stack0x00000c58 + lVar61) = 1;
      *(undefined1 **)(&stack0x00000c70 + lVar61) = &stack0x000022f0 + lVar61;
      *(undefined8 *)(&stack0x00000c78 + lVar61) = 1;
      *(undefined1 **)(&stack0x00000c60 + lVar61) = &stack0x00003180 + lVar61;
      *(undefined8 *)(&stack0x00000c68 + lVar61) = 1;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf41a2;
      uVar70 = func_0x00014067dd80(&UNK_142b161f8);
      *(undefined **)(&stack0x000005b0 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x000005b8 + lVar61) = 0x15;
      *(undefined **)(&stack0x000005c0 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x000005c8 + lVar61) = 0x15;
      *(undefined8 *)(&stack0x000005d0 + lVar61) = uVar70;
      (&stack0x00003307)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf41f3;
      FUN_1404b1430(&stack0x00000c50 + lVar61,4,&stack0x000005b0 + lVar61);
    }
    uVar70 = *(undefined8 *)(lVar83 + 0x48);
    *(undefined8 *)(&stack0x00003170 + lVar61) = *(undefined8 *)(lVar83 + 0x40);
    *(undefined8 *)(&stack0x00003178 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x58);
    *(undefined8 *)(&stack0x00003110 + lVar61) = *(undefined8 *)(lVar83 + 0x50);
    *(undefined8 *)(&stack0x00003118 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x68);
    *(undefined8 *)(&stack0x00003120 + lVar61) = *(undefined8 *)(lVar83 + 0x60);
    *(undefined8 *)(&stack0x00003128 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x78);
    *(undefined8 *)(&stack0x00003000 + lVar61) = *(undefined8 *)(lVar83 + 0x70);
    *(undefined8 *)(&stack0x00003008 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x88);
    *(undefined8 *)(&stack0x00001d30 + lVar61) = *(undefined8 *)(lVar83 + 0x80);
    *(undefined8 *)(&stack0x00001d38 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x98);
    *(undefined8 *)(&stack0x00001d40 + lVar61) = *(undefined8 *)(lVar83 + 0x90);
    *(undefined8 *)(&stack0x00001d48 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0xa8);
    *(undefined8 *)(&stack0x00001d50 + lVar61) = *(undefined8 *)(lVar83 + 0xa0);
    *(undefined8 *)(&stack0x00001d58 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0xb8);
    *(undefined8 *)(&stack0x00001d60 + lVar61) = *(undefined8 *)(lVar83 + 0xb0);
    *(undefined8 *)(&stack0x00001d68 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 200);
    *(undefined8 *)(&stack0x00001d70 + lVar61) = *(undefined8 *)(lVar83 + 0xc0);
    *(undefined8 *)(&stack0x00001d78 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0xd8);
    *(undefined8 *)(&stack0x00001d80 + lVar61) = *(undefined8 *)(lVar83 + 0xd0);
    *(undefined8 *)(&stack0x00001d88 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0xe8);
    *(undefined8 *)(&stack0x00001d90 + lVar61) = *(undefined8 *)(lVar83 + 0xe0);
    *(undefined8 *)(&stack0x00001d98 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0xf8);
    *(undefined8 *)(&stack0x00001da0 + lVar61) = *(undefined8 *)(lVar83 + 0xf0);
    *(undefined8 *)(&stack0x00001da8 + lVar61) = uVar70;
    uVar70 = *(undefined8 *)(lVar83 + 0x108);
    *(undefined8 *)(&stack0x00001db0 + lVar61) = *(undefined8 *)(lVar83 + 0x100);
    *(undefined8 *)(&stack0x00001db8 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00001dc0 + lVar61) = *(undefined8 *)(&stack0x00001710 + lVar61);
    *(undefined8 *)(&stack0x00001dc8 + lVar61) = *(undefined8 *)(&stack0x00001718 + lVar61);
    *(undefined8 *)(&stack0x00001dd0 + lVar61) = *(undefined8 *)(&stack0x00001720 + lVar61);
    *(undefined8 *)(&stack0x00001dd8 + lVar61) = *(undefined8 *)(&stack0x00001728 + lVar61);
    *(undefined8 *)(&stack0x00001de0 + lVar61) = *(undefined8 *)(&stack0x00001730 + lVar61);
    *(undefined8 *)(&stack0x00001de8 + lVar61) = *(undefined8 *)(&stack0x00001738 + lVar61);
    uVar70 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0xd0);
    (&stack0x00003307)[lVar61] = 1;
    uVar69 = *(undefined8 *)(&stack0x00001740 + lVar61);
    uVar78 = *(undefined8 *)(&stack0x00001748 + lVar61);
    uVar85 = *(undefined8 *)(&stack0x00001750 + lVar61);
    uVar27 = *(undefined8 *)(&stack0x00001758 + lVar61);
    uVar28 = *(undefined8 *)(&stack0x00001760 + lVar61);
    uVar40 = *(undefined8 *)(&stack0x00001768 + lVar61);
    uVar41 = *(undefined8 *)(&stack0x00001770 + lVar61);
    uVar42 = *(undefined8 *)(&stack0x00001778 + lVar61);
    uVar43 = *(undefined8 *)(&stack0x00001780 + lVar61);
    uVar44 = *(undefined8 *)(&stack0x00001788 + lVar61);
    uVar45 = *(undefined8 *)(&stack0x00001790 + lVar61);
    uVar46 = *(undefined8 *)(&stack0x00001798 + lVar61);
    uVar47 = *(undefined8 *)(&stack0x000017a0 + lVar61);
    uVar48 = *(undefined8 *)(&stack0x000017a8 + lVar61);
    uVar49 = *(undefined8 *)(&stack0x000017b0 + lVar61);
    uVar50 = *(undefined8 *)(&stack0x000017b8 + lVar61);
    uVar51 = *(undefined8 *)(&stack0x000017c0 + lVar61);
    uVar52 = *(undefined8 *)(&stack0x000017c8 + lVar61);
    uVar53 = *(undefined8 *)(&stack0x000017d0 + lVar61);
    uVar54 = *(undefined8 *)(&stack0x000017d8 + lVar61);
    *(undefined8 *)(&stack0x00001df0 + lVar61) = *(undefined8 *)(&stack0x000017e0 + lVar61);
    *(undefined8 *)(&stack0x00001df8 + lVar61) = *(undefined8 *)(&stack0x000017e8 + lVar61);
    *(undefined8 *)(&stack0x00001e00 + lVar61) = *(undefined8 *)(&stack0x000017f0 + lVar61);
    *(undefined8 *)(&stack0x00001e08 + lVar61) = *(undefined8 *)(&stack0x000017f8 + lVar61);
    *(undefined8 *)(&stack0x00001e10 + lVar61) = *(undefined8 *)(&stack0x00001800 + lVar61);
    *(undefined8 *)(&stack0x00001e18 + lVar61) = *(undefined8 *)(&stack0x00001808 + lVar61);
    *(undefined8 *)(&stack0x00001e20 + lVar61) = *(undefined8 *)(&stack0x00001810 + lVar61);
    *(undefined8 *)(&stack0x00001e28 + lVar61) = *(undefined8 *)(&stack0x00001818 + lVar61);
    *(undefined8 *)(&stack0x00001e30 + lVar61) = *(undefined8 *)(&stack0x00001820 + lVar61);
    *(undefined8 *)(&stack0x00001e38 + lVar61) = *(undefined8 *)(&stack0x00001828 + lVar61);
    *(undefined8 *)(&stack0x00001e40 + lVar61) = *(undefined8 *)(&stack0x00001830 + lVar61);
    *(undefined8 *)(&stack0x00001e48 + lVar61) = *(undefined8 *)(&stack0x00001838 + lVar61);
    *(undefined8 *)(&stack0x00001e50 + lVar61) = *(undefined8 *)(&stack0x00001840 + lVar61);
    *(undefined8 *)(&stack0x00001e58 + lVar61) = *(undefined8 *)(&stack0x00001848 + lVar61);
    *(undefined8 *)(&stack0x00001e60 + lVar61) = *(undefined8 *)(&stack0x00001850 + lVar61);
    *(undefined8 *)(&stack0x00001e68 + lVar61) = *(undefined8 *)(&stack0x00001858 + lVar61);
    *(undefined8 *)(&stack0x00001e70 + lVar61) = *(undefined8 *)(&stack0x00001860 + lVar61);
    *(undefined8 *)(&stack0x00001e78 + lVar61) = *(undefined8 *)(&stack0x00001868 + lVar61);
    *(undefined8 *)(&stack0x00001e80 + lVar61) = *(undefined8 *)(&stack0x00001870 + lVar61);
    *(undefined8 *)(&stack0x00001e88 + lVar61) = *(undefined8 *)(&stack0x00001878 + lVar61);
    *(undefined8 *)(&stack0x00001e90 + lVar61) = *(undefined8 *)(&stack0x00001880 + lVar61);
    *(undefined8 *)(&stack0x00001e98 + lVar61) = *(undefined8 *)(&stack0x00001888 + lVar61);
    *(undefined8 *)(&stack0x00001ea0 + lVar61) = *(undefined8 *)(&stack0x00001890 + lVar61);
    *(undefined8 *)(&stack0x00001ea8 + lVar61) = *(undefined8 *)(&stack0x00001898 + lVar61);
    *(undefined8 *)(&stack0x00001eb0 + lVar61) = *(undefined8 *)(&stack0x000018a0 + lVar61);
    *(undefined8 *)(&stack0x00001eb8 + lVar61) = *(undefined8 *)(&stack0x000018a8 + lVar61);
    *(undefined8 *)(&stack0x00001ec0 + lVar61) = *(undefined8 *)(&stack0x000018b0 + lVar61);
    *(undefined8 *)(&stack0x00001ec8 + lVar61) = *(undefined8 *)(&stack0x000018b8 + lVar61);
    *(undefined8 *)(&stack0x00001ed0 + lVar61) = *(undefined8 *)(&stack0x000018c0 + lVar61);
    *(undefined8 *)(&stack0x00001ed8 + lVar61) = *(undefined8 *)(&stack0x000018c8 + lVar61);
    *(undefined8 *)(&stack0x00001ee0 + lVar61) = *(undefined8 *)(&stack0x000018d0 + lVar61);
    *(undefined8 *)(&stack0x00001ee8 + lVar61) = *(undefined8 *)(&stack0x000018d8 + lVar61);
    *(undefined8 *)(&stack0x00001ef0 + lVar61) = *(undefined8 *)(&stack0x000018e0 + lVar61);
    *(undefined8 *)(&stack0x00001ef8 + lVar61) = *(undefined8 *)(&stack0x000018e8 + lVar61);
    *(undefined8 *)(&stack0x00001f00 + lVar61) = *(undefined8 *)(&stack0x000018f0 + lVar61);
    *(undefined8 *)(&stack0x00001f08 + lVar61) = *(undefined8 *)(&stack0x000018f8 + lVar61);
    *(undefined8 *)(&stack0x00001f10 + lVar61) = *(undefined8 *)(&stack0x00001900 + lVar61);
    *(undefined8 *)(&stack0x00001f18 + lVar61) = *(undefined8 *)(&stack0x00001908 + lVar61);
    *(undefined8 *)(&stack0x00001f20 + lVar61) = *(undefined8 *)(&stack0x00001910 + lVar61);
    *(undefined8 *)(&stack0x00001f28 + lVar61) = *(undefined8 *)(&stack0x00001918 + lVar61);
    *(undefined8 *)(&stack0x00001f30 + lVar61) = *(undefined8 *)(&stack0x00001920 + lVar61);
    *(undefined8 *)(&stack0x00001f38 + lVar61) = *(undefined8 *)(&stack0x00001928 + lVar61);
    *(undefined8 *)(&stack0x00001f40 + lVar61) = *(undefined8 *)(&stack0x00001930 + lVar61);
    *(undefined8 *)(&stack0x00001f48 + lVar61) = *(undefined8 *)(&stack0x00001938 + lVar61);
    *(undefined8 *)(&stack0x00001f50 + lVar61) = *(undefined8 *)(&stack0x00001940 + lVar61);
    *(undefined8 *)(&stack0x00001f58 + lVar61) = *(undefined8 *)(&stack0x00001948 + lVar61);
    *(undefined8 *)(&stack0x00001f60 + lVar61) = *(undefined8 *)(&stack0x00001950 + lVar61);
    *(undefined8 *)(&stack0x00001f68 + lVar61) = *(undefined8 *)(&stack0x00001958 + lVar61);
    *(undefined8 *)(&stack0x00001f70 + lVar61) = *(undefined8 *)(&stack0x00001960 + lVar61);
    *(undefined8 *)(&stack0x00001f78 + lVar61) = *(undefined8 *)(&stack0x00001968 + lVar61);
    *(undefined8 *)(&stack0x00001f80 + lVar61) = *(undefined8 *)(&stack0x00001970 + lVar61);
    *(undefined8 *)(&stack0x00001f88 + lVar61) = *(undefined8 *)(&stack0x00001978 + lVar61);
    *(undefined8 *)(&stack0x00001f90 + lVar61) = *(undefined8 *)(&stack0x00001980 + lVar61);
    *(undefined8 *)(&stack0x00001f98 + lVar61) = *(undefined8 *)(&stack0x00001988 + lVar61);
    *(undefined8 *)(&stack0x00001fa0 + lVar61) = *(undefined8 *)(&stack0x00001990 + lVar61);
    *(undefined8 *)(&stack0x00001fa8 + lVar61) = *(undefined8 *)(&stack0x00001998 + lVar61);
    *(undefined8 *)(&stack0x00001fb0 + lVar61) = *(undefined8 *)(&stack0x000019a0 + lVar61);
    *(undefined8 *)(&stack0x00001fb8 + lVar61) = *(undefined8 *)(&stack0x000019a8 + lVar61);
    *(undefined8 *)(&stack0x00001fc0 + lVar61) = *(undefined8 *)(&stack0x000019b0 + lVar61);
    *(undefined8 *)(&stack0x00001fc8 + lVar61) = *(undefined8 *)(&stack0x000019b8 + lVar61);
    *(undefined8 *)(&stack0x00001fd0 + lVar61) = *(undefined8 *)(&stack0x000019c0 + lVar61);
    *(undefined8 *)(&stack0x00001fd8 + lVar61) = *(undefined8 *)(&stack0x000019c8 + lVar61);
    *(undefined8 *)(&stack0x00001fe0 + lVar61) = *(undefined8 *)(&stack0x000019d0 + lVar61);
    *(undefined8 *)(&stack0x00001fe8 + lVar61) = *(undefined8 *)(&stack0x000019d8 + lVar61);
    *(undefined8 *)(&stack0x00001ff0 + lVar61) = *(undefined8 *)(&stack0x000019e0 + lVar61);
    *(undefined8 *)(&stack0x00001ff8 + lVar61) = *(undefined8 *)(&stack0x000019e8 + lVar61);
    *(undefined8 *)(&stack0x00002000 + lVar61) = *(undefined8 *)(&stack0x000019f0 + lVar61);
    *(undefined8 *)(&stack0x00002008 + lVar61) = *(undefined8 *)(&stack0x000019f8 + lVar61);
    *(undefined8 *)(&stack0x00002010 + lVar61) = *(undefined8 *)(&stack0x00001a00 + lVar61);
    *(undefined8 *)(&stack0x00002018 + lVar61) = *(undefined8 *)(&stack0x00001a08 + lVar61);
    *(undefined8 *)(&stack0x00002020 + lVar61) = *(undefined8 *)(&stack0x00001a10 + lVar61);
    *(undefined8 *)(&stack0x00002028 + lVar61) = *(undefined8 *)(&stack0x00001a18 + lVar61);
    *(undefined8 *)(&stack0x00002030 + lVar61) = *(undefined8 *)(&stack0x00001a20 + lVar61);
    *(undefined8 *)(&stack0x00002038 + lVar61) = *(undefined8 *)(&stack0x00001a28 + lVar61);
    *(undefined8 *)(&stack0x00002040 + lVar61) = *(undefined8 *)(&stack0x00001a30 + lVar61);
    *(undefined8 *)(&stack0x00002048 + lVar61) = *(undefined8 *)(&stack0x00001a38 + lVar61);
    *(undefined8 *)(&stack0x00002050 + lVar61) = *(undefined8 *)(&stack0x00001a40 + lVar61);
    *(undefined8 *)(&stack0x00002058 + lVar61) = *(undefined8 *)(&stack0x00001a48 + lVar61);
    *(undefined8 *)(&stack0x00002060 + lVar61) = *(undefined8 *)(&stack0x00001a50 + lVar61);
    *(undefined8 *)(&stack0x00002068 + lVar61) = *(undefined8 *)(&stack0x00001a58 + lVar61);
    *(undefined8 *)(&stack0x00002070 + lVar61) = *(undefined8 *)(&stack0x00001a60 + lVar61);
    *(undefined8 *)(&stack0x00002078 + lVar61) = *(undefined8 *)(&stack0x00001a68 + lVar61);
    *(undefined8 *)(&stack0x00002080 + lVar61) = *(undefined8 *)(&stack0x00001a70 + lVar61);
    *(undefined8 *)(&stack0x00002088 + lVar61) = *(undefined8 *)(&stack0x00001a78 + lVar61);
    *(undefined8 *)(&stack0x00002090 + lVar61) = *(undefined8 *)(&stack0x00001a80 + lVar61);
    *(undefined8 *)(&stack0x00002098 + lVar61) = *(undefined8 *)(&stack0x00001a88 + lVar61);
    *(undefined8 *)(&stack0x000020a0 + lVar61) = *(undefined8 *)(&stack0x00001a90 + lVar61);
    *(undefined8 *)(&stack0x000020a8 + lVar61) = *(undefined8 *)(&stack0x00001a98 + lVar61);
    *(undefined8 *)(&stack0x000020b0 + lVar61) = *(undefined8 *)(&stack0x00001aa0 + lVar61);
    *(undefined8 *)(&stack0x000020b8 + lVar61) = *(undefined8 *)(&stack0x00001aa8 + lVar61);
    *(undefined8 *)(&stack0x000020c0 + lVar61) = *(undefined8 *)(&stack0x00001ab0 + lVar61);
    *(undefined8 *)(&stack0x000020c8 + lVar61) = *(undefined8 *)(&stack0x00001ab8 + lVar61);
    *(undefined8 *)(&stack0x000020d0 + lVar61) = *(undefined8 *)(&stack0x00001ac0 + lVar61);
    *(undefined8 *)(&stack0x000020d8 + lVar61) = *(undefined8 *)(&stack0x00001ac8 + lVar61);
    *(undefined8 *)(&stack0x000020e0 + lVar61) = *(undefined8 *)(&stack0x00001ad0 + lVar61);
    *(undefined8 *)(&stack0x000020e8 + lVar61) = *(undefined8 *)(&stack0x00001ad8 + lVar61);
    *(undefined8 *)(&stack0x000020f0 + lVar61) = *(undefined8 *)(&stack0x00001ae0 + lVar61);
    *(undefined8 *)(&stack0x000020f8 + lVar61) = *(undefined8 *)(&stack0x00001ae8 + lVar61);
    *(undefined8 *)(&stack0x00002100 + lVar61) = *(undefined8 *)(&stack0x00001af0 + lVar61);
    *(undefined8 *)(&stack0x00002108 + lVar61) = *(undefined8 *)(&stack0x00001af8 + lVar61);
    *(undefined8 *)(&stack0x00002110 + lVar61) = *(undefined8 *)(&stack0x00001b00 + lVar61);
    *(undefined8 *)(&stack0x00002118 + lVar61) = *(undefined8 *)(&stack0x00001b08 + lVar61);
    *(undefined8 *)(&stack0x00002120 + lVar61) = *(undefined8 *)(&stack0x00001b10 + lVar61);
    *(undefined8 *)(&stack0x00002128 + lVar61) = *(undefined8 *)(&stack0x00001b18 + lVar61);
    *(undefined8 *)(&stack0x00002130 + lVar61) = *(undefined8 *)(&stack0x00001b20 + lVar61);
    *(undefined8 *)(&stack0x00002138 + lVar61) = *(undefined8 *)(&stack0x00001b28 + lVar61);
    *(undefined8 *)(&stack0x00002140 + lVar61) = *(undefined8 *)(&stack0x00001b30 + lVar61);
    *(undefined8 *)(&stack0x00002148 + lVar61) = *(undefined8 *)(&stack0x00001b38 + lVar61);
    *(undefined8 *)(&stack0x00002150 + lVar61) = *(undefined8 *)(&stack0x00001b40 + lVar61);
    *(undefined8 *)(&stack0x00002158 + lVar61) = *(undefined8 *)(&stack0x00001b48 + lVar61);
    *(undefined8 *)(&stack0x00002160 + lVar61) = *(undefined8 *)(&stack0x00001b50 + lVar61);
    *(undefined8 *)(&stack0x00002168 + lVar61) = *(undefined8 *)(&stack0x00001b58 + lVar61);
    *(undefined8 *)(&stack0x00002170 + lVar61) = *(undefined8 *)(&stack0x00001b60 + lVar61);
    *(undefined8 *)(&stack0x00002178 + lVar61) = *(undefined8 *)(&stack0x00001b68 + lVar61);
    *(undefined8 *)(&stack0x00002180 + lVar61) = *(undefined8 *)(&stack0x00001b70 + lVar61);
    *(undefined8 *)(&stack0x00002188 + lVar61) = *(undefined8 *)(&stack0x00001b78 + lVar61);
    *(undefined8 *)(&stack0x00002190 + lVar61) = *(undefined8 *)(&stack0x00001b80 + lVar61);
    *(undefined8 *)(&stack0x00002198 + lVar61) = *(undefined8 *)(&stack0x00001b88 + lVar61);
    *(undefined8 *)(&stack0x000021a0 + lVar61) = *(undefined8 *)(&stack0x00001b90 + lVar61);
    *(undefined8 *)(&stack0x000021a8 + lVar61) = *(undefined8 *)(&stack0x00001b98 + lVar61);
    *(undefined8 *)(&stack0x000021b0 + lVar61) = *(undefined8 *)(&stack0x00001ba0 + lVar61);
    *(undefined8 *)(&stack0x000021b8 + lVar61) = *(undefined8 *)(&stack0x00001ba8 + lVar61);
    *(undefined8 *)(&stack0x000021c0 + lVar61) = *(undefined8 *)(&stack0x00001bb0 + lVar61);
    *(undefined8 *)(&stack0x000021c8 + lVar61) = *(undefined8 *)(&stack0x00001bb8 + lVar61);
    *(undefined8 *)(&stack0x000021d0 + lVar61) = *(undefined8 *)(&stack0x00001bc0 + lVar61);
    *(undefined8 *)(&stack0x000021d8 + lVar61) = *(undefined8 *)(&stack0x00001bc8 + lVar61);
    *(undefined8 *)(&stack0x000021e0 + lVar61) = *(undefined8 *)(&stack0x00001bd0 + lVar61);
    *(undefined8 *)(&stack0x000021e8 + lVar61) = *(undefined8 *)(&stack0x00001bd8 + lVar61);
    *(undefined8 *)(&stack0x000021f0 + lVar61) = *(undefined8 *)(&stack0x00001be0 + lVar61);
    *(undefined8 *)(&stack0x000021f8 + lVar61) = *(undefined8 *)(&stack0x00001be8 + lVar61);
    *(undefined8 *)(&stack0x00002200 + lVar61) = *(undefined8 *)(&stack0x00001bf0 + lVar61);
    *(undefined8 *)(&stack0x00002208 + lVar61) = *(undefined8 *)(&stack0x00001bf8 + lVar61);
    *(undefined8 *)(&stack0x00002210 + lVar61) = *(undefined8 *)(&stack0x00001c00 + lVar61);
    *(undefined8 *)(&stack0x00002218 + lVar61) = *(undefined8 *)(&stack0x00001c08 + lVar61);
    *(undefined8 *)(&stack0x00002220 + lVar61) = *(undefined8 *)(&stack0x00001c10 + lVar61);
    *(undefined8 *)(&stack0x00002228 + lVar61) = *(undefined8 *)(&stack0x00001c18 + lVar61);
    *(undefined8 *)(&stack0x00002230 + lVar61) = *(undefined8 *)(&stack0x00001c20 + lVar61);
    *(undefined8 *)(&stack0x00002238 + lVar61) = *(undefined8 *)(&stack0x00001c28 + lVar61);
    *(undefined8 *)(&stack0x00002240 + lVar61) = *(undefined8 *)(&stack0x00001c30 + lVar61);
    *(undefined8 *)(&stack0x00002248 + lVar61) = *(undefined8 *)(&stack0x00001c38 + lVar61);
    *(undefined8 *)(&stack0x00002250 + lVar61) = *(undefined8 *)(&stack0x00001c40 + lVar61);
    *(undefined8 *)(&stack0x00002258 + lVar61) = *(undefined8 *)(&stack0x00001c48 + lVar61);
    *(undefined8 *)(&stack0x00002260 + lVar61) = *(undefined8 *)(&stack0x00001c50 + lVar61);
    *(undefined8 *)(&stack0x00002268 + lVar61) = *(undefined8 *)(&stack0x00001c58 + lVar61);
    *(undefined8 *)(&stack0x00002270 + lVar61) = *(undefined8 *)(&stack0x00001c60 + lVar61);
    *(undefined8 *)(&stack0x00002278 + lVar61) = *(undefined8 *)(&stack0x00001c68 + lVar61);
    *(undefined8 *)(&stack0x00002280 + lVar61) = *(undefined8 *)(&stack0x00001c70 + lVar61);
    *(undefined8 *)(&stack0x00002288 + lVar61) = *(undefined8 *)(&stack0x00001c78 + lVar61);
    *(undefined8 *)(&stack0x00002290 + lVar61) = *(undefined8 *)(&stack0x00001c80 + lVar61);
    *(undefined8 *)(&stack0x00002298 + lVar61) = *(undefined8 *)(&stack0x00001c88 + lVar61);
    *(undefined8 *)(&stack0x000022a0 + lVar61) = *(undefined8 *)(&stack0x00001c90 + lVar61);
    *(undefined8 *)(&stack0x000022a8 + lVar61) = *(undefined8 *)(&stack0x00001c98 + lVar61);
    *(undefined8 *)(&stack0x000022b0 + lVar61) = *(undefined8 *)(&stack0x00001ca0 + lVar61);
    *(undefined8 *)(&stack0x000022b8 + lVar61) = *(undefined8 *)(&stack0x00001ca8 + lVar61);
    *(undefined8 *)(&stack0x000022c0 + lVar61) = *(undefined8 *)(&stack0x00001cb0 + lVar61);
    *(undefined8 *)(&stack0x000022c8 + lVar61) = *(undefined8 *)(&stack0x00001cb8 + lVar61);
    *(undefined8 *)(&stack0x000022d0 + lVar61) = *(undefined8 *)(&stack0x00001cc0 + lVar61);
    *(undefined8 *)(&stack0x000022d8 + lVar61) = *(undefined8 *)(&stack0x00001cc8 + lVar61);
    *(undefined8 *)(&stack0x000022e0 + lVar61) = *(undefined8 *)(&stack0x00001cd0 + lVar61);
    *(undefined8 *)(&stack0x000022e8 + lVar61) = *(undefined8 *)(&stack0x00001cd8 + lVar61);
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4794;
    func_0x0001418c8490();
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf47a9;
    uVar72 = FUN_1418c84a0(uVar81);
    *(undefined8 *)(&stack0x000028d0 + lVar61) = *(undefined8 *)(&stack0x00003170 + lVar61);
    *(undefined8 *)(&stack0x000028d8 + lVar61) = *(undefined8 *)(&stack0x00003178 + lVar61);
    *(undefined8 *)(&stack0x000028e0 + lVar61) = *(undefined8 *)(&stack0x00003110 + lVar61);
    *(undefined8 *)(&stack0x000028e8 + lVar61) = *(undefined8 *)(&stack0x00003118 + lVar61);
    *(undefined8 *)(&stack0x000028f0 + lVar61) = *(undefined8 *)(&stack0x00003120 + lVar61);
    *(undefined8 *)(&stack0x000028f8 + lVar61) = *(undefined8 *)(&stack0x00003128 + lVar61);
    *(undefined8 *)(&stack0x00002900 + lVar61) = *(undefined8 *)(&stack0x00003000 + lVar61);
    *(undefined8 *)(&stack0x00002908 + lVar61) = *(undefined8 *)(&stack0x00003008 + lVar61);
    *(undefined8 *)(&stack0x00002910 + lVar61) = *(undefined8 *)(&stack0x00001d30 + lVar61);
    *(undefined8 *)(&stack0x00002918 + lVar61) = *(undefined8 *)(&stack0x00001d38 + lVar61);
    *(undefined8 *)(&stack0x00002920 + lVar61) = *(undefined8 *)(&stack0x00001d40 + lVar61);
    *(undefined8 *)(&stack0x00002928 + lVar61) = *(undefined8 *)(&stack0x00001d48 + lVar61);
    *(undefined8 *)(&stack0x00002930 + lVar61) = *(undefined8 *)(&stack0x00001d50 + lVar61);
    *(undefined8 *)(&stack0x00002938 + lVar61) = *(undefined8 *)(&stack0x00001d58 + lVar61);
    *(undefined8 *)(&stack0x00002940 + lVar61) = *(undefined8 *)(&stack0x00001d60 + lVar61);
    *(undefined8 *)(&stack0x00002948 + lVar61) = *(undefined8 *)(&stack0x00001d68 + lVar61);
    *(undefined8 *)(&stack0x00002950 + lVar61) = *(undefined8 *)(&stack0x00001d70 + lVar61);
    *(undefined8 *)(&stack0x00002958 + lVar61) = *(undefined8 *)(&stack0x00001d78 + lVar61);
    *(undefined8 *)(&stack0x00002960 + lVar61) = *(undefined8 *)(&stack0x00001d80 + lVar61);
    *(undefined8 *)(&stack0x00002968 + lVar61) = *(undefined8 *)(&stack0x00001d88 + lVar61);
    *(undefined8 *)(&stack0x00002970 + lVar61) = *(undefined8 *)(&stack0x00001d90 + lVar61);
    *(undefined8 *)(&stack0x00002978 + lVar61) = *(undefined8 *)(&stack0x00001d98 + lVar61);
    *(undefined8 *)(&stack0x00002980 + lVar61) = *(undefined8 *)(&stack0x00001da0 + lVar61);
    *(undefined8 *)(&stack0x00002988 + lVar61) = *(undefined8 *)(&stack0x00001da8 + lVar61);
    *(undefined8 *)(&stack0x00002990 + lVar61) = *(undefined8 *)(&stack0x00001db0 + lVar61);
    *(undefined8 *)(&stack0x00002998 + lVar61) = *(undefined8 *)(&stack0x00001db8 + lVar61);
    *(undefined8 *)(&stack0x00002300 + lVar61) = *(undefined8 *)(&stack0x00001dc0 + lVar61);
    *(undefined8 *)(&stack0x00002308 + lVar61) = *(undefined8 *)(&stack0x00001dc8 + lVar61);
    *(undefined8 *)(&stack0x00002310 + lVar61) = *(undefined8 *)(&stack0x00001dd0 + lVar61);
    *(undefined8 *)(&stack0x00002318 + lVar61) = *(undefined8 *)(&stack0x00001dd8 + lVar61);
    *(undefined8 *)(&stack0x00002320 + lVar61) = *(undefined8 *)(&stack0x00001de0 + lVar61);
    *(undefined8 *)(&stack0x00002328 + lVar61) = *(undefined8 *)(&stack0x00001de8 + lVar61);
    *(undefined8 *)(&stack0x00002330 + lVar61) = uVar69;
    *(undefined8 *)(&stack0x00002338 + lVar61) = uVar78;
    *(undefined8 *)(&stack0x00002340 + lVar61) = uVar85;
    *(undefined8 *)(&stack0x00002348 + lVar61) = uVar27;
    *(undefined8 *)(&stack0x00002350 + lVar61) = uVar28;
    *(undefined8 *)(&stack0x00002358 + lVar61) = uVar40;
    *(undefined8 *)(&stack0x00002360 + lVar61) = uVar41;
    *(undefined8 *)(&stack0x00002368 + lVar61) = uVar42;
    *(undefined8 *)(&stack0x00002370 + lVar61) = uVar43;
    *(undefined8 *)(&stack0x00002378 + lVar61) = uVar44;
    *(undefined8 *)(&stack0x00002380 + lVar61) = uVar45;
    *(undefined8 *)(&stack0x00002388 + lVar61) = uVar46;
    *(undefined8 *)(&stack0x00002390 + lVar61) = uVar47;
    *(undefined8 *)(&stack0x00002398 + lVar61) = uVar48;
    *(undefined8 *)(&stack0x000023a0 + lVar61) = uVar49;
    *(undefined8 *)(&stack0x000023a8 + lVar61) = uVar50;
    *(undefined8 *)(&stack0x000023b0 + lVar61) = uVar51;
    *(undefined8 *)(&stack0x000023b8 + lVar61) = uVar52;
    *(undefined8 *)(&stack0x000023c0 + lVar61) = uVar53;
    *(undefined8 *)(&stack0x000023c8 + lVar61) = uVar54;
    *(undefined8 *)(&stack0x000023d0 + lVar61) = *(undefined8 *)(&stack0x00001df0 + lVar61);
    *(undefined8 *)(&stack0x000023d8 + lVar61) = *(undefined8 *)(&stack0x00001df8 + lVar61);
    *(undefined8 *)(&stack0x000023e0 + lVar61) = *(undefined8 *)(&stack0x00001e00 + lVar61);
    *(undefined8 *)(&stack0x000023e8 + lVar61) = *(undefined8 *)(&stack0x00001e08 + lVar61);
    *(undefined8 *)(&stack0x000023f0 + lVar61) = *(undefined8 *)(&stack0x00001e10 + lVar61);
    *(undefined8 *)(&stack0x000023f8 + lVar61) = *(undefined8 *)(&stack0x00001e18 + lVar61);
    *(undefined8 *)(&stack0x00002400 + lVar61) = *(undefined8 *)(&stack0x00001e20 + lVar61);
    *(undefined8 *)(&stack0x00002408 + lVar61) = *(undefined8 *)(&stack0x00001e28 + lVar61);
    *(undefined8 *)(&stack0x00002410 + lVar61) = *(undefined8 *)(&stack0x00001e30 + lVar61);
    *(undefined8 *)(&stack0x00002418 + lVar61) = *(undefined8 *)(&stack0x00001e38 + lVar61);
    *(undefined8 *)(&stack0x00002420 + lVar61) = *(undefined8 *)(&stack0x00001e40 + lVar61);
    *(undefined8 *)(&stack0x00002428 + lVar61) = *(undefined8 *)(&stack0x00001e48 + lVar61);
    *(undefined8 *)(&stack0x00002430 + lVar61) = *(undefined8 *)(&stack0x00001e50 + lVar61);
    *(undefined8 *)(&stack0x00002438 + lVar61) = *(undefined8 *)(&stack0x00001e58 + lVar61);
    *(undefined8 *)(&stack0x00002440 + lVar61) = *(undefined8 *)(&stack0x00001e60 + lVar61);
    *(undefined8 *)(&stack0x00002448 + lVar61) = *(undefined8 *)(&stack0x00001e68 + lVar61);
    *(undefined8 *)(&stack0x00002450 + lVar61) = *(undefined8 *)(&stack0x00001e70 + lVar61);
    *(undefined8 *)(&stack0x00002458 + lVar61) = *(undefined8 *)(&stack0x00001e78 + lVar61);
    *(undefined8 *)(&stack0x00002460 + lVar61) = *(undefined8 *)(&stack0x00001e80 + lVar61);
    *(undefined8 *)(&stack0x00002468 + lVar61) = *(undefined8 *)(&stack0x00001e88 + lVar61);
    *(undefined8 *)(&stack0x00002470 + lVar61) = *(undefined8 *)(&stack0x00001e90 + lVar61);
    *(undefined8 *)(&stack0x00002478 + lVar61) = *(undefined8 *)(&stack0x00001e98 + lVar61);
    *(undefined8 *)(&stack0x00002480 + lVar61) = *(undefined8 *)(&stack0x00001ea0 + lVar61);
    *(undefined8 *)(&stack0x00002488 + lVar61) = *(undefined8 *)(&stack0x00001ea8 + lVar61);
    *(undefined8 *)(&stack0x00002490 + lVar61) = *(undefined8 *)(&stack0x00001eb0 + lVar61);
    *(undefined8 *)(&stack0x00002498 + lVar61) = *(undefined8 *)(&stack0x00001eb8 + lVar61);
    *(undefined8 *)(&stack0x000024a0 + lVar61) = *(undefined8 *)(&stack0x00001ec0 + lVar61);
    *(undefined8 *)(&stack0x000024a8 + lVar61) = *(undefined8 *)(&stack0x00001ec8 + lVar61);
    *(undefined8 *)(&stack0x000024b0 + lVar61) = *(undefined8 *)(&stack0x00001ed0 + lVar61);
    *(undefined8 *)(&stack0x000024b8 + lVar61) = *(undefined8 *)(&stack0x00001ed8 + lVar61);
    *(undefined8 *)(&stack0x000024c0 + lVar61) = *(undefined8 *)(&stack0x00001ee0 + lVar61);
    *(undefined8 *)(&stack0x000024c8 + lVar61) = *(undefined8 *)(&stack0x00001ee8 + lVar61);
    *(undefined8 *)(&stack0x000024d0 + lVar61) = *(undefined8 *)(&stack0x00001ef0 + lVar61);
    *(undefined8 *)(&stack0x000024d8 + lVar61) = *(undefined8 *)(&stack0x00001ef8 + lVar61);
    *(undefined8 *)(&stack0x000024e0 + lVar61) = *(undefined8 *)(&stack0x00001f00 + lVar61);
    *(undefined8 *)(&stack0x000024e8 + lVar61) = *(undefined8 *)(&stack0x00001f08 + lVar61);
    *(undefined8 *)(&stack0x000024f0 + lVar61) = *(undefined8 *)(&stack0x00001f10 + lVar61);
    *(undefined8 *)(&stack0x000024f8 + lVar61) = *(undefined8 *)(&stack0x00001f18 + lVar61);
    *(undefined8 *)(&stack0x00002500 + lVar61) = *(undefined8 *)(&stack0x00001f20 + lVar61);
    *(undefined8 *)(&stack0x00002508 + lVar61) = *(undefined8 *)(&stack0x00001f28 + lVar61);
    *(undefined8 *)(&stack0x00002510 + lVar61) = *(undefined8 *)(&stack0x00001f30 + lVar61);
    *(undefined8 *)(&stack0x00002518 + lVar61) = *(undefined8 *)(&stack0x00001f38 + lVar61);
    *(undefined8 *)(&stack0x00002520 + lVar61) = *(undefined8 *)(&stack0x00001f40 + lVar61);
    *(undefined8 *)(&stack0x00002528 + lVar61) = *(undefined8 *)(&stack0x00001f48 + lVar61);
    *(undefined8 *)(&stack0x00002530 + lVar61) = *(undefined8 *)(&stack0x00001f50 + lVar61);
    *(undefined8 *)(&stack0x00002538 + lVar61) = *(undefined8 *)(&stack0x00001f58 + lVar61);
    *(undefined8 *)(&stack0x00002540 + lVar61) = *(undefined8 *)(&stack0x00001f60 + lVar61);
    *(undefined8 *)(&stack0x00002548 + lVar61) = *(undefined8 *)(&stack0x00001f68 + lVar61);
    *(undefined8 *)(&stack0x00002550 + lVar61) = *(undefined8 *)(&stack0x00001f70 + lVar61);
    *(undefined8 *)(&stack0x00002558 + lVar61) = *(undefined8 *)(&stack0x00001f78 + lVar61);
    *(undefined8 *)(&stack0x00002560 + lVar61) = *(undefined8 *)(&stack0x00001f80 + lVar61);
    *(undefined8 *)(&stack0x00002568 + lVar61) = *(undefined8 *)(&stack0x00001f88 + lVar61);
    *(undefined8 *)(&stack0x00002570 + lVar61) = *(undefined8 *)(&stack0x00001f90 + lVar61);
    *(undefined8 *)(&stack0x00002578 + lVar61) = *(undefined8 *)(&stack0x00001f98 + lVar61);
    *(undefined8 *)(&stack0x00002580 + lVar61) = *(undefined8 *)(&stack0x00001fa0 + lVar61);
    *(undefined8 *)(&stack0x00002588 + lVar61) = *(undefined8 *)(&stack0x00001fa8 + lVar61);
    *(undefined8 *)(&stack0x00002590 + lVar61) = *(undefined8 *)(&stack0x00001fb0 + lVar61);
    *(undefined8 *)(&stack0x00002598 + lVar61) = *(undefined8 *)(&stack0x00001fb8 + lVar61);
    *(undefined8 *)(&stack0x000025a0 + lVar61) = *(undefined8 *)(&stack0x00001fc0 + lVar61);
    *(undefined8 *)(&stack0x000025a8 + lVar61) = *(undefined8 *)(&stack0x00001fc8 + lVar61);
    *(undefined8 *)(&stack0x000025b0 + lVar61) = *(undefined8 *)(&stack0x00001fd0 + lVar61);
    *(undefined8 *)(&stack0x000025b8 + lVar61) = *(undefined8 *)(&stack0x00001fd8 + lVar61);
    *(undefined8 *)(&stack0x000025c0 + lVar61) = *(undefined8 *)(&stack0x00001fe0 + lVar61);
    *(undefined8 *)(&stack0x000025c8 + lVar61) = *(undefined8 *)(&stack0x00001fe8 + lVar61);
    *(undefined8 *)(&stack0x000025d0 + lVar61) = *(undefined8 *)(&stack0x00001ff0 + lVar61);
    *(undefined8 *)(&stack0x000025d8 + lVar61) = *(undefined8 *)(&stack0x00001ff8 + lVar61);
    *(undefined8 *)(&stack0x000025e0 + lVar61) = *(undefined8 *)(&stack0x00002000 + lVar61);
    *(undefined8 *)(&stack0x000025e8 + lVar61) = *(undefined8 *)(&stack0x00002008 + lVar61);
    *(undefined8 *)(&stack0x000025f0 + lVar61) = *(undefined8 *)(&stack0x00002010 + lVar61);
    *(undefined8 *)(&stack0x000025f8 + lVar61) = *(undefined8 *)(&stack0x00002018 + lVar61);
    *(undefined8 *)(&stack0x00002600 + lVar61) = *(undefined8 *)(&stack0x00002020 + lVar61);
    *(undefined8 *)(&stack0x00002608 + lVar61) = *(undefined8 *)(&stack0x00002028 + lVar61);
    *(undefined8 *)(&stack0x00002610 + lVar61) = *(undefined8 *)(&stack0x00002030 + lVar61);
    *(undefined8 *)(&stack0x00002618 + lVar61) = *(undefined8 *)(&stack0x00002038 + lVar61);
    *(undefined8 *)(&stack0x00002620 + lVar61) = *(undefined8 *)(&stack0x00002040 + lVar61);
    *(undefined8 *)(&stack0x00002628 + lVar61) = *(undefined8 *)(&stack0x00002048 + lVar61);
    *(undefined8 *)(&stack0x00002630 + lVar61) = *(undefined8 *)(&stack0x00002050 + lVar61);
    *(undefined8 *)(&stack0x00002638 + lVar61) = *(undefined8 *)(&stack0x00002058 + lVar61);
    *(undefined8 *)(&stack0x00002640 + lVar61) = *(undefined8 *)(&stack0x00002060 + lVar61);
    *(undefined8 *)(&stack0x00002648 + lVar61) = *(undefined8 *)(&stack0x00002068 + lVar61);
    *(undefined8 *)(&stack0x00002650 + lVar61) = *(undefined8 *)(&stack0x00002070 + lVar61);
    *(undefined8 *)(&stack0x00002658 + lVar61) = *(undefined8 *)(&stack0x00002078 + lVar61);
    *(undefined8 *)(&stack0x00002660 + lVar61) = *(undefined8 *)(&stack0x00002080 + lVar61);
    *(undefined8 *)(&stack0x00002668 + lVar61) = *(undefined8 *)(&stack0x00002088 + lVar61);
    *(undefined8 *)(&stack0x00002670 + lVar61) = *(undefined8 *)(&stack0x00002090 + lVar61);
    *(undefined8 *)(&stack0x00002678 + lVar61) = *(undefined8 *)(&stack0x00002098 + lVar61);
    *(undefined8 *)(&stack0x00002680 + lVar61) = *(undefined8 *)(&stack0x000020a0 + lVar61);
    *(undefined8 *)(&stack0x00002688 + lVar61) = *(undefined8 *)(&stack0x000020a8 + lVar61);
    *(undefined8 *)(&stack0x00002690 + lVar61) = *(undefined8 *)(&stack0x000020b0 + lVar61);
    *(undefined8 *)(&stack0x00002698 + lVar61) = *(undefined8 *)(&stack0x000020b8 + lVar61);
    *(undefined8 *)(&stack0x000026a0 + lVar61) = *(undefined8 *)(&stack0x000020c0 + lVar61);
    *(undefined8 *)(&stack0x000026a8 + lVar61) = *(undefined8 *)(&stack0x000020c8 + lVar61);
    *(undefined8 *)(&stack0x000026b0 + lVar61) = *(undefined8 *)(&stack0x000020d0 + lVar61);
    *(undefined8 *)(&stack0x000026b8 + lVar61) = *(undefined8 *)(&stack0x000020d8 + lVar61);
    *(undefined8 *)(&stack0x000026c0 + lVar61) = *(undefined8 *)(&stack0x000020e0 + lVar61);
    *(undefined8 *)(&stack0x000026c8 + lVar61) = *(undefined8 *)(&stack0x000020e8 + lVar61);
    *(undefined8 *)(&stack0x000026d0 + lVar61) = *(undefined8 *)(&stack0x000020f0 + lVar61);
    *(undefined8 *)(&stack0x000026d8 + lVar61) = *(undefined8 *)(&stack0x000020f8 + lVar61);
    *(undefined8 *)(&stack0x000026e0 + lVar61) = *(undefined8 *)(&stack0x00002100 + lVar61);
    *(undefined8 *)(&stack0x000026e8 + lVar61) = *(undefined8 *)(&stack0x00002108 + lVar61);
    *(undefined8 *)(&stack0x000026f0 + lVar61) = *(undefined8 *)(&stack0x00002110 + lVar61);
    *(undefined8 *)(&stack0x000026f8 + lVar61) = *(undefined8 *)(&stack0x00002118 + lVar61);
    *(undefined8 *)(&stack0x00002700 + lVar61) = *(undefined8 *)(&stack0x00002120 + lVar61);
    *(undefined8 *)(&stack0x00002708 + lVar61) = *(undefined8 *)(&stack0x00002128 + lVar61);
    *(undefined8 *)(&stack0x00002710 + lVar61) = *(undefined8 *)(&stack0x00002130 + lVar61);
    *(undefined8 *)(&stack0x00002718 + lVar61) = *(undefined8 *)(&stack0x00002138 + lVar61);
    *(undefined8 *)(&stack0x00002720 + lVar61) = *(undefined8 *)(&stack0x00002140 + lVar61);
    *(undefined8 *)(&stack0x00002728 + lVar61) = *(undefined8 *)(&stack0x00002148 + lVar61);
    *(undefined8 *)(&stack0x00002730 + lVar61) = *(undefined8 *)(&stack0x00002150 + lVar61);
    *(undefined8 *)(&stack0x00002738 + lVar61) = *(undefined8 *)(&stack0x00002158 + lVar61);
    *(undefined8 *)(&stack0x00002740 + lVar61) = *(undefined8 *)(&stack0x00002160 + lVar61);
    *(undefined8 *)(&stack0x00002748 + lVar61) = *(undefined8 *)(&stack0x00002168 + lVar61);
    *(undefined8 *)(&stack0x00002750 + lVar61) = *(undefined8 *)(&stack0x00002170 + lVar61);
    *(undefined8 *)(&stack0x00002758 + lVar61) = *(undefined8 *)(&stack0x00002178 + lVar61);
    *(undefined8 *)(&stack0x00002760 + lVar61) = *(undefined8 *)(&stack0x00002180 + lVar61);
    *(undefined8 *)(&stack0x00002768 + lVar61) = *(undefined8 *)(&stack0x00002188 + lVar61);
    *(undefined8 *)(&stack0x00002770 + lVar61) = *(undefined8 *)(&stack0x00002190 + lVar61);
    *(undefined8 *)(&stack0x00002778 + lVar61) = *(undefined8 *)(&stack0x00002198 + lVar61);
    *(undefined8 *)(&stack0x00002780 + lVar61) = *(undefined8 *)(&stack0x000021a0 + lVar61);
    *(undefined8 *)(&stack0x00002788 + lVar61) = *(undefined8 *)(&stack0x000021a8 + lVar61);
    *(undefined8 *)(&stack0x00002790 + lVar61) = *(undefined8 *)(&stack0x000021b0 + lVar61);
    *(undefined8 *)(&stack0x00002798 + lVar61) = *(undefined8 *)(&stack0x000021b8 + lVar61);
    *(undefined8 *)(&stack0x000027a0 + lVar61) = *(undefined8 *)(&stack0x000021c0 + lVar61);
    *(undefined8 *)(&stack0x000027a8 + lVar61) = *(undefined8 *)(&stack0x000021c8 + lVar61);
    *(undefined8 *)(&stack0x000027b0 + lVar61) = *(undefined8 *)(&stack0x000021d0 + lVar61);
    *(undefined8 *)(&stack0x000027b8 + lVar61) = *(undefined8 *)(&stack0x000021d8 + lVar61);
    *(undefined8 *)(&stack0x000027c0 + lVar61) = *(undefined8 *)(&stack0x000021e0 + lVar61);
    *(undefined8 *)(&stack0x000027c8 + lVar61) = *(undefined8 *)(&stack0x000021e8 + lVar61);
    *(undefined8 *)(&stack0x000027d0 + lVar61) = *(undefined8 *)(&stack0x000021f0 + lVar61);
    *(undefined8 *)(&stack0x000027d8 + lVar61) = *(undefined8 *)(&stack0x000021f8 + lVar61);
    *(undefined8 *)(&stack0x000027e0 + lVar61) = *(undefined8 *)(&stack0x00002200 + lVar61);
    *(undefined8 *)(&stack0x000027e8 + lVar61) = *(undefined8 *)(&stack0x00002208 + lVar61);
    *(undefined8 *)(&stack0x000027f0 + lVar61) = *(undefined8 *)(&stack0x00002210 + lVar61);
    *(undefined8 *)(&stack0x000027f8 + lVar61) = *(undefined8 *)(&stack0x00002218 + lVar61);
    *(undefined8 *)(&stack0x00002800 + lVar61) = *(undefined8 *)(&stack0x00002220 + lVar61);
    *(undefined8 *)(&stack0x00002808 + lVar61) = *(undefined8 *)(&stack0x00002228 + lVar61);
    *(undefined8 *)(&stack0x00002810 + lVar61) = *(undefined8 *)(&stack0x00002230 + lVar61);
    *(undefined8 *)(&stack0x00002818 + lVar61) = *(undefined8 *)(&stack0x00002238 + lVar61);
    *(undefined8 *)(&stack0x00002820 + lVar61) = *(undefined8 *)(&stack0x00002240 + lVar61);
    *(undefined8 *)(&stack0x00002828 + lVar61) = *(undefined8 *)(&stack0x00002248 + lVar61);
    *(undefined8 *)(&stack0x00002830 + lVar61) = *(undefined8 *)(&stack0x00002250 + lVar61);
    *(undefined8 *)(&stack0x00002838 + lVar61) = *(undefined8 *)(&stack0x00002258 + lVar61);
    *(undefined8 *)(&stack0x00002840 + lVar61) = *(undefined8 *)(&stack0x00002260 + lVar61);
    *(undefined8 *)(&stack0x00002848 + lVar61) = *(undefined8 *)(&stack0x00002268 + lVar61);
    *(undefined8 *)(&stack0x00002850 + lVar61) = *(undefined8 *)(&stack0x00002270 + lVar61);
    *(undefined8 *)(&stack0x00002858 + lVar61) = *(undefined8 *)(&stack0x00002278 + lVar61);
    *(undefined8 *)(&stack0x00002860 + lVar61) = *(undefined8 *)(&stack0x00002280 + lVar61);
    *(undefined8 *)(&stack0x00002868 + lVar61) = *(undefined8 *)(&stack0x00002288 + lVar61);
    *(undefined8 *)(&stack0x00002870 + lVar61) = *(undefined8 *)(&stack0x00002290 + lVar61);
    *(undefined8 *)(&stack0x00002878 + lVar61) = *(undefined8 *)(&stack0x00002298 + lVar61);
    *(undefined8 *)(&stack0x00002880 + lVar61) = *(undefined8 *)(&stack0x000022a0 + lVar61);
    *(undefined8 *)(&stack0x00002888 + lVar61) = *(undefined8 *)(&stack0x000022a8 + lVar61);
    *(undefined8 *)(&stack0x00002890 + lVar61) = *(undefined8 *)(&stack0x000022b0 + lVar61);
    *(undefined8 *)(&stack0x00002898 + lVar61) = *(undefined8 *)(&stack0x000022b8 + lVar61);
    *(undefined8 *)(&stack0x000028a0 + lVar61) = *(undefined8 *)(&stack0x000022c0 + lVar61);
    *(undefined8 *)(&stack0x000028a8 + lVar61) = *(undefined8 *)(&stack0x000022c8 + lVar61);
    *(undefined8 *)(&stack0x000028b0 + lVar61) = *(undefined8 *)(&stack0x000022d0 + lVar61);
    *(undefined8 *)(&stack0x000028b8 + lVar61) = *(undefined8 *)(&stack0x000022d8 + lVar61);
    *(undefined8 *)(&stack0x000028c0 + lVar61) = *(undefined8 *)(&stack0x000022e0 + lVar61);
    *(undefined8 *)(&stack0x000028c8 + lVar61) = *(undefined8 *)(&stack0x000022e8 + lVar61);
    *(undefined8 *)(&stack0x000029a0 + lVar61) = uVar70;
    (&stack0x000029a8)[lVar61] = 1;
    *(undefined4 *)(&stack0x000029a9 + lVar61) = 0;
    (&stack0x000029ad)[lVar61] = extraout_var & 1;
    (&stack0x000029ae)[lVar61] = 1;
    *(undefined8 *)(&stack0x000022f0 + lVar61) = uVar72;
    *(ulonglong *)(&stack0x000022f8 + lVar61) = uVar65;
    uVar57 = *(undefined1 *)(*(longlong *)(&stack0x00003288 + lVar61) + 5);
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4d9e;
    FUN_140c0d980(&stack0x00000c50 + lVar61,&stack0x000022f0 + lVar61,uVar57);
    if (SBORROW8(0,*(longlong *)(&stack0x00000c50 + lVar61))) {
      *(undefined4 *)(&stack0x00003190 + lVar61) = *(undefined4 *)(&stack0x00000c68 + lVar61);
      *(undefined4 *)(&stack0x00003194 + lVar61) = *(undefined4 *)(&stack0x00000c6c + lVar61);
      *(undefined4 *)(&stack0x00003198 + lVar61) = *(undefined4 *)(&stack0x00000c70 + lVar61);
      *(undefined4 *)(&stack0x0000319c + lVar61) = *(undefined4 *)(&stack0x00000c74 + lVar61);
      *(undefined8 *)(&stack0x00003180 + lVar61) = *(undefined8 *)(&stack0x00000c58 + lVar61);
      *(undefined8 *)(&stack0x00003188 + lVar61) = *(undefined8 *)(&stack0x00000c60 + lVar61);
      *(undefined **)(&stack0xffffffffffffffe0 + lVar61) = &UNK_142b16228;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf67fd;
      FUN_1428d9760(&UNK_142b16210,0x16,&stack0x00003180 + lVar61,&UNK_142b15148);
      goto code_r0x000140bf685d;
    }
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4dc6;
    func_0x000142923c00(&stack0x000005b8 + lVar61,&stack0x00000c50 + lVar61,0x690);
    (&stack0x000005b0)[lVar61] = 0;
    if (4 < *puRam000000014362c3a8) {
      *(undefined **)(&stack0x000022f0 + lVar61) = &UNK_142b16260;
      *(undefined8 *)(&stack0x000022f8 + lVar61) = 1;
      *(undefined8 *)(&stack0x00002300 + lVar61) = 8;
      *(undefined1 (*) [16])(&stack0x00002308 + lVar61) = (undefined1  [16])0x0;
      (&stack0x00003306)[lVar61] = 1;
      (&stack0x00003305)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4e2b;
      uVar70 = func_0x00014067dd80(&UNK_142b16270);
      *(undefined **)(&stack0x00000c50 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c58 + lVar61) = 0x15;
      *(undefined **)(&stack0x00000c60 + lVar61) = &UNK_142b16030;
      *(undefined8 *)(&stack0x00000c68 + lVar61) = 0x15;
      *(undefined8 *)(&stack0x00000c70 + lVar61) = uVar70;
      (&stack0x00003306)[lVar61] = 1;
      (&stack0x00003305)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4e83;
      FUN_1404b1430(&stack0x000022f0 + lVar61,5,&stack0x00000c50 + lVar61);
    }
    uVar59 = *(undefined4 *)(&stack0x00003298 + lVar61);
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4ea2;
    (**(code **)(&stack0x00001718 + lVar61))
              (*(undefined8 *)(&stack0x00001cd8 + lVar61),uVar59,0,&stack0x000022f0 + lVar61);
    uVar70 = *(undefined8 *)(&stack0x000022f0 + lVar61);
    uVar69 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0xd0);
    uVar78 = *(undefined8 *)(*(longlong *)(&stack0x000032a0 + lVar61) + 0x40);
    pcVar8 = *(code **)(*(longlong *)(&stack0x000032a0 + lVar61) + 0x18);
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4ece;
    iVar60 = (*pcVar8)(uVar69,uVar59,uVar78,&stack0x000022f0 + lVar61);
    if (iVar60 != 0) {
      *(int *)(&stack0x000022f0 + lVar61) = iVar60;
      (&stack0x00003306)[lVar61] = 1;
      (&stack0x00003305)[lVar61] = 1;
      *(undefined **)(&stack0xffffffffffffffe0 + lVar61) = &UNK_142b16288;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf683e;
      FUN_1428d9760(&UNK_142b15188,0x2b,&stack0x000022f0 + lVar61,&UNK_142b15128);
      goto code_r0x000140bf685d;
    }
    iVar60 = *(int *)(&stack0x000022f0 + lVar61);
    *(undefined8 *)(&stack0x000022f0 + lVar61) = 0;
    *(undefined1 (*) [16])(&stack0x00002300 + lVar61) = (undefined1  [16])0x0;
    *(undefined8 *)(&stack0x000022f8 + lVar61) = 8;
    puVar68 = &stack0x00001710 + lVar61;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf4f0d;
    uVar69 = FUN_1418c8710(puVar68);
    *(undefined1 (*) [16])(&stack0x00002dc0 + lVar61) = (undefined1  [16])0x0;
    (&stack0x00002de0)[lVar61] = 0;
    *(undefined8 *)(&stack0x00002de8 + lVar61) = uVar70;
    *(undefined8 *)(&stack0x00002e00 + lVar61) = *(undefined8 *)(&stack0x00002ec0 + lVar61);
    *(undefined8 *)(&stack0x00002e08 + lVar61) = *(undefined8 *)(&stack0x00002ec8 + lVar61);
    *(undefined8 *)(&stack0x00002e10 + lVar61) = *(undefined8 *)(&stack0x00002ed0 + lVar61);
    *(undefined4 *)(&stack0x00002e18 + lVar61) = *(undefined4 *)(&stack0x00003298 + lVar61);
    *(undefined4 *)(&stack0x00002dd8 + lVar61) = 0;
    (&stack0x00002e1c)[lVar61] = iVar60 != 0;
    *(undefined8 *)(&stack0x00002d70 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002d78 + lVar61) = 8;
    *(undefined1 (*) [16])(&stack0x00002d80 + lVar61) = (undefined1  [16])0x0;
    *(undefined8 *)(&stack0x00002df0 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002d90 + lVar61) = *(undefined8 *)(&stack0x000022f0 + lVar61);
    *(undefined8 *)(&stack0x00002d98 + lVar61) = *(undefined8 *)(&stack0x000022f8 + lVar61);
    *(undefined4 *)(&stack0x00002da0 + lVar61) = *(undefined4 *)(&stack0x00002300 + lVar61);
    *(undefined4 *)(&stack0x00002da4 + lVar61) = *(undefined4 *)(&stack0x00002304 + lVar61);
    *(undefined4 *)(&stack0x00002da8 + lVar61) = *(undefined4 *)(&stack0x00002308 + lVar61);
    *(undefined4 *)(&stack0x00002dac + lVar61) = *(undefined4 *)(&stack0x0000230c + lVar61);
    *(undefined8 *)(&stack0x00002df8 + lVar61) = uVar69;
    *(undefined8 *)(&stack0x00002db0 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002db8 + lVar61) = 8;
    *(undefined8 *)(&stack0x00002dd0 + lVar61) = 0;
    *(undefined1 **)(&stack0x000022f0 + lVar61) = &stack0x00002fe0 + lVar61;
    *(undefined1 **)(&stack0x000022f8 + lVar61) = puVar68;
    *(undefined1 **)(&stack0x00002300 + lVar61) = &stack0x000032a0 + lVar61;
    *(undefined8 *)(&stack0x00002308 + lVar61) = *(undefined8 *)(&stack0x00003290 + lVar61);
    *(undefined8 *)(&stack0x00002310 + lVar61) = 0;
    *(ulonglong *)(&stack0x00002318 + lVar61) = uVar86;
    (&stack0x000032f9)[lVar61] = 1;
    (&stack0x000032f8)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5038;
    FUN_1418b8610(&stack0x00002e68 + lVar61,&stack0x000022f0 + lVar61,&UNK_142b15090);
    if ((&stack0x00002e1c)[lVar61] == '\0') {
      (&stack0x00003301)[lVar61] = 1;
      (&stack0x00003300)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf677f;
      FUN_1428d9430(&UNK_142b162a0,0x2d,&UNK_142b162d0);
      goto code_r0x000140bf685d;
    }
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf505b;
    puVar62 = (undefined8 *)func_0x000140613c10(0x4e0,8);
    if (puVar62 == (undefined8 *)0x0) {
      (&stack0x00003301)[lVar61] = 1;
      (&stack0x00003300)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf685d;
      FUN_1428d9000(8,0x4e0);
      goto code_r0x000140bf685d;
    }
    *(undefined4 *)(&stack0x00003084 + lVar61) = 0;
    *(undefined8 *)(&stack0x0000307c + lVar61) = 0;
    *(undefined8 *)(&stack0x00002e84 + lVar61) = 0;
    *(undefined4 *)(&stack0x00002e8c + lVar61) = 0;
    *(undefined8 *)(&stack0x00002f94 + lVar61) = 0;
    *(undefined4 *)(&stack0x00002f9c + lVar61) = 0;
    *(undefined8 *)(&stack0x000030e4 + lVar61) = 0;
    *(undefined4 *)(&stack0x000030ec + lVar61) = 0;
    *(undefined8 *)(&stack0x00002fb4 + lVar61) = 0;
    *(undefined4 *)(&stack0x00002fbc + lVar61) = 0;
    *(undefined8 *)(&stack0x000030b4 + lVar61) = 0;
    *(undefined4 *)(&stack0x000030bc + lVar61) = 0;
    *(undefined8 *)(&stack0x00003068 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003070 + lVar61) = 0;
    *(undefined8 *)(&stack0x00002ee4 + lVar61) = 0;
    *(undefined4 *)(&stack0x00002eec + lVar61) = 0;
    *(undefined8 *)(&stack0x00003058 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003060 + lVar61) = 0;
    *(undefined4 *)(&stack0x0000318c + lVar61) = 0;
    *(undefined8 *)(&stack0x00003184 + lVar61) = 0;
    *(undefined8 *)(&stack0x00003048 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003050 + lVar61) = 0;
    *(undefined4 *)(&stack0x00000c5c + lVar61) = 0;
    *(undefined8 *)(&stack0x00000c54 + lVar61) = 0;
    *(undefined8 *)(&stack0x00003038 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003040 + lVar61) = 0;
    *(undefined4 *)(&stack0x0000316c + lVar61) = 0;
    *(undefined8 *)(&stack0x00003164 + lVar61) = 0;
    *(undefined8 *)(&stack0x00003028 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003030 + lVar61) = 0;
    *(undefined4 *)(&stack0x0000315c + lVar61) = 0;
    *(undefined8 *)(&stack0x00003154 + lVar61) = 0;
    *(undefined8 *)(&stack0x00003018 + lVar61) = 0;
    *(undefined4 *)(&stack0x00003020 + lVar61) = 0;
    *(undefined4 *)(&stack0x0000314c + lVar61) = 0;
    *(undefined8 *)(&stack0x00003144 + lVar61) = 0;
    *(undefined8 *)(&stack0x000022f0 + lVar61) = 0;
    *(undefined4 *)(&stack0x000022f8 + lVar61) = 0;
    *(undefined4 *)(&stack0x0000313c + lVar61) = 0;
    *(undefined8 *)(&stack0x00003134 + lVar61) = 0;
    *puVar62 = &UNK_142b16b42;
    puVar62[1] = 0xb;
    *(undefined4 *)(puVar62 + 2) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003080 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x14) = *(undefined8 *)(&stack0x00003078 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x1c) = uVar70;
    uVar85 = _UNK_142b14e88;
    uVar78 = _UNK_142b14e80;
    *(undefined8 *)((longlong)puVar62 + 0x24) = _UNK_142b14e80;
    *(undefined8 *)((longlong)puVar62 + 0x2c) = uVar85;
    uVar28 = _UNK_142b14e98;
    uVar27 = _UNK_142b14e90;
    *(undefined8 *)((longlong)puVar62 + 0x34) = _UNK_142b14e90;
    *(undefined8 *)((longlong)puVar62 + 0x3c) = uVar28;
    *(undefined4 *)((longlong)puVar62 + 0x44) = 0x40800000;
    puVar62[10] = 0x447a000000000000;
    puVar62[9] = 0;
    puVar62[0xb] = 0;
    puVar62[0xc] = &UNK_142b16b4d;
    puVar62[0xd] = 0xb;
    *(undefined4 *)(puVar62 + 0xe) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00002e88 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x74) = *(undefined8 *)(&stack0x00002e80 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x7c) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x84) = 0x100000001;
    *(undefined4 *)((longlong)puVar62 + 0x8c) = 1;
    puVar62[0x12] = 0;
    puVar62[0x13] = 0;
    puVar62[0x14] = 0x4080000000000001;
    puVar62[0x16] = 0x447a000000000000;
    puVar62[0x15] = 0;
    puVar62[0x17] = 0;
    puVar62[0x18] = &UNK_142b16b58;
    puVar62[0x19] = 0xb;
    *(undefined4 *)(puVar62 + 0x1a) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00002f98 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0xd4) = *(undefined8 *)(&stack0x00002f90 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0xdc) = uVar70;
    uVar70 = _UNK_142b14ea8;
    *(undefined8 *)((longlong)puVar62 + 0xe4) = _UNK_142b14ea0;
    *(undefined8 *)((longlong)puVar62 + 0xec) = uVar70;
    uVar69 = _UNK_142aebae0;
    *(undefined8 *)((longlong)puVar62 + 0xf4) = _UNK_142aebae0;
    *(undefined8 *)((longlong)puVar62 + 0xfc) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x104) = 0x40800000;
    puVar62[0x22] = 0x447a000000000000;
    puVar62[0x21] = 0;
    puVar62[0x23] = 0;
    puVar62[0x24] = &UNK_142b16b63;
    puVar62[0x25] = 0xb;
    *(undefined4 *)(puVar62 + 0x26) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x000030e8 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x134) = *(undefined8 *)(&stack0x000030e0 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x13c) = uVar70;
    uVar41 = _UNK_142b14eb8;
    uVar40 = _UNK_142b14eb0;
    *(undefined8 *)((longlong)puVar62 + 0x144) = _UNK_142b14eb0;
    *(undefined8 *)((longlong)puVar62 + 0x14c) = uVar41;
    *(undefined8 *)((longlong)puVar62 + 0x154) = uVar27;
    *(undefined8 *)((longlong)puVar62 + 0x15c) = uVar28;
    *(undefined4 *)((longlong)puVar62 + 0x164) = 0x40800000;
    puVar62[0x2e] = 0x447a000000000000;
    puVar62[0x2d] = 0;
    puVar62[0x2f] = 0;
    puVar62[0x30] = &UNK_142b16b6e;
    puVar62[0x31] = 0xb;
    *(undefined4 *)(puVar62 + 0x32) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00002fb8 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x194) = *(undefined8 *)(&stack0x00002fb0 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x19c) = uVar70;
    uVar28 = _UNK_142b14ec8;
    uVar27 = _UNK_142b14ec0;
    *(undefined8 *)((longlong)puVar62 + 0x1a4) = _UNK_142b14ec0;
    *(undefined8 *)((longlong)puVar62 + 0x1ac) = uVar28;
    uVar70 = _UNK_142b14ed8;
    *(undefined8 *)((longlong)puVar62 + 0x1b4) = _UNK_142b14ed0;
    *(undefined8 *)((longlong)puVar62 + 0x1bc) = uVar70;
    *(undefined4 *)((longlong)puVar62 + 0x1c4) = 0x40800000;
    puVar62[0x3a] = 0x447a000000000000;
    puVar62[0x39] = 0;
    puVar62[0x3b] = 2;
    puVar62[0x3c] = &UNK_142b16b79;
    puVar62[0x3d] = 0xf;
    *(undefined4 *)(puVar62 + 0x3e) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x000030b8 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 500) = *(undefined8 *)(&stack0x000030b0 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x1fc) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x204) = uVar78;
    *(undefined8 *)((longlong)puVar62 + 0x20c) = uVar85;
    *(undefined8 *)((longlong)puVar62 + 0x214) = uVar69;
    *(undefined8 *)((longlong)puVar62 + 0x21c) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x224) = 0x3f800000;
    *(undefined4 *)(puVar62 + 0x46) = *(undefined4 *)(&stack0x00003070 + lVar61);
    puVar62[0x45] = *(undefined8 *)(&stack0x00003068 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x234) = 0x447a0000;
    puVar62[0x47] = 0;
    puVar62[0x48] = &UNK_142b16b88;
    puVar62[0x49] = 0xf;
    *(undefined4 *)(puVar62 + 0x4a) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00002ee8 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x254) = *(undefined8 *)(&stack0x00002ee0 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x25c) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x264) = 0x100000001;
    *(undefined4 *)((longlong)puVar62 + 0x26c) = 1;
    puVar62[0x4e] = 0;
    puVar62[0x4f] = 0;
    puVar62[0x50] = 0x3f80000000000000;
    puVar62[0x51] = *(undefined8 *)(&stack0x00003058 + lVar61);
    *(undefined4 *)(puVar62 + 0x52) = *(undefined4 *)(&stack0x00003060 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x294) = 0x447a0000;
    puVar62[0x53] = 0;
    puVar62[0x54] = &UNK_142b16b97;
    puVar62[0x55] = 0xf;
    *(undefined4 *)(puVar62 + 0x56) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003188 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x2b4) = *(undefined8 *)(&stack0x00003180 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 700) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x2c4) = uVar40;
    *(undefined8 *)((longlong)puVar62 + 0x2cc) = uVar41;
    *(undefined8 *)((longlong)puVar62 + 0x2d4) = uVar69;
    *(undefined8 *)((longlong)puVar62 + 0x2dc) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x2e4) = 0x3f800000;
    *(undefined4 *)(puVar62 + 0x5e) = *(undefined4 *)(&stack0x00003050 + lVar61);
    puVar62[0x5d] = *(undefined8 *)(&stack0x00003048 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x2f4) = 0x447a0000;
    puVar62[0x5f] = 0;
    puVar62[0x60] = &UNK_142b16ba6;
    puVar62[0x61] = 0xf;
    *(undefined4 *)(puVar62 + 0x62) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00000c58 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x314) = *(undefined8 *)(&stack0x00000c50 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x31c) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x324) = uVar27;
    *(undefined8 *)((longlong)puVar62 + 0x32c) = uVar28;
    uVar27 = _UNK_142b14ee0;
    *(undefined8 *)((longlong)puVar62 + 0x334) = _UNK_142b14ee0;
    *(undefined8 *)((longlong)puVar62 + 0x33c) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x344) = 0x3f800000;
    *(undefined4 *)(puVar62 + 0x6a) = *(undefined4 *)(&stack0x00003040 + lVar61);
    puVar62[0x69] = *(undefined8 *)(&stack0x00003038 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x354) = 0x447a0000;
    puVar62[0x6b] = 2;
    puVar62[0x6c] = &UNK_142b16bb5;
    puVar62[0x6d] = 0xf;
    *(undefined4 *)(puVar62 + 0x6e) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003168 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x374) = *(undefined8 *)(&stack0x00003160 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x37c) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 900) = uVar78;
    *(undefined8 *)((longlong)puVar62 + 0x38c) = uVar85;
    *(undefined8 *)((longlong)puVar62 + 0x394) = uVar69;
    *(undefined8 *)((longlong)puVar62 + 0x39c) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x3a4) = 0x3f800000;
    *(undefined4 *)(puVar62 + 0x76) = *(undefined4 *)(&stack0x00003030 + lVar61);
    puVar62[0x75] = *(undefined8 *)(&stack0x00003028 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x3b4) = 0;
    puVar62[0x77] = 0;
    puVar62[0x78] = &UNK_142b16bc4;
    puVar62[0x79] = 0xe;
    *(undefined4 *)(puVar62 + 0x7a) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003158 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x3d4) = *(undefined8 *)(&stack0x00003150 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x3dc) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x3e4) = 0x100000001;
    *(undefined4 *)((longlong)puVar62 + 0x3ec) = 1;
    puVar62[0x7e] = 0;
    puVar62[0x7f] = 0;
    puVar62[0x80] = 0x3f80000000000000;
    puVar62[0x81] = *(undefined8 *)(&stack0x00003018 + lVar61);
    *(undefined4 *)(puVar62 + 0x82) = *(undefined4 *)(&stack0x00003020 + lVar61);
    *(undefined4 *)((longlong)puVar62 + 0x414) = 0x447a0000;
    puVar62[0x83] = 0;
    puVar62[0x84] = &UNK_142b16bd2;
    puVar62[0x85] = 0x16;
    *(undefined4 *)(puVar62 + 0x86) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003148 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x434) = *(undefined8 *)(&stack0x00003140 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x43c) = uVar70;
    uVar70 = _UNK_142aebf78;
    *(undefined8 *)((longlong)puVar62 + 0x444) = _UNK_142aebf70;
    *(undefined8 *)((longlong)puVar62 + 0x44c) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x454) = uVar27;
    *(undefined8 *)((longlong)puVar62 + 0x45c) = 0;
    *(undefined4 *)((longlong)puVar62 + 0x464) = 0x3f800000;
    *(undefined4 *)(puVar62 + 0x8e) = *(undefined4 *)(&stack0x000022f8 + lVar61);
    puVar62[0x8d] = *(undefined8 *)(&stack0x000022f0 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x474) = 0x400000000;
    *(undefined4 *)((longlong)puVar62 + 0x47c) = 0;
    puVar62[0x90] = &UNK_142b16be8;
    puVar62[0x91] = 0x15;
    *(undefined4 *)(puVar62 + 0x92) = 0x1f;
    uVar70 = *(undefined8 *)(&stack0x00003138 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x494) = *(undefined8 *)(&stack0x00003130 + lVar61);
    *(undefined8 *)((longlong)puVar62 + 0x49c) = uVar70;
    uVar70 = _UNK_142b14ef8;
    *(undefined8 *)((longlong)puVar62 + 0x4a4) = _UNK_142b14ef0;
    *(undefined8 *)((longlong)puVar62 + 0x4ac) = uVar70;
    *(undefined8 *)((longlong)puVar62 + 0x4b4) = uVar27;
    *(undefined8 *)((longlong)puVar62 + 0x4bc) = 0;
    *(undefined8 *)((longlong)puVar62 + 0x4c4) = 0x13f800000;
    *(undefined4 *)((longlong)puVar62 + 0x4cc) = 3;
    puVar62[0x9a] = 0;
    puVar62[0x9b] = 4;
    *(undefined8 **)(&stack0x000022f0 + lVar61) = puVar62;
    *(undefined8 **)(&stack0x000022f8 + lVar61) = puVar62;
    *(undefined8 *)(&stack0x00002300 + lVar61) = 0xd;
    *(undefined8 **)(&stack0x00002308 + lVar61) = puVar62 + 0x9c;
    *(undefined1 **)(&stack0x00002310 + lVar61) = puVar68;
    (&stack0x00003301)[lVar61] = 1;
    (&stack0x00003300)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5875;
    FUN_1418c2970(&stack0x00001d00 + lVar61,&stack0x000022f0 + lVar61);
    if ((&stack0x000032d6)[lVar61] == '\0') {
      *(undefined8 *)(&stack0x00001ce0 + lVar61) = 0;
    }
    else {
      lVar79 = *(longlong *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
      (&stack0x00003304)[lVar61] = 1;
      (&stack0x00003303)[lVar61] = 1;
      (&stack0x00003302)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf58b8;
      FUN_1418c8e60(&stack0x00001ce0 + lVar61,lVar79 + 0x40,&stack0x00001710 + lVar61);
    }
    lVar79 = *(longlong *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
    if (*(char *)(lVar79 + 0x110) == '\0') {
      *(undefined8 *)(&stack0x00002e20 + lVar61) = 0;
    }
    else {
      (&stack0x00003304)[lVar61] = 1;
      (&stack0x00003303)[lVar61] = 1;
      (&stack0x00003302)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf590c;
      FUN_1418c89a0(&stack0x000022f0 + lVar61,lVar79 + 0x40,&stack0x00001710 + lVar61);
      *(undefined8 *)(&stack0x00002e60 + lVar61) = *(undefined8 *)(&stack0x00002330 + lVar61);
      *(undefined8 *)(&stack0x00002e50 + lVar61) = *(undefined8 *)(&stack0x00002320 + lVar61);
      *(undefined8 *)(&stack0x00002e58 + lVar61) = *(undefined8 *)(&stack0x00002328 + lVar61);
      *(undefined8 *)(&stack0x00002e40 + lVar61) = *(undefined8 *)(&stack0x00002310 + lVar61);
      *(undefined8 *)(&stack0x00002e48 + lVar61) = *(undefined8 *)(&stack0x00002318 + lVar61);
      *(undefined8 *)(&stack0x00002e30 + lVar61) = *(undefined8 *)(&stack0x00002300 + lVar61);
      *(undefined8 *)(&stack0x00002e38 + lVar61) = *(undefined8 *)(&stack0x00002308 + lVar61);
      *(undefined8 *)(&stack0x00002e20 + lVar61) = *(undefined8 *)(&stack0x000022f0 + lVar61);
      *(undefined8 *)(&stack0x00002e28 + lVar61) = *(undefined8 *)(&stack0x000022f8 + lVar61);
      lVar79 = *(longlong *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
    }
    (&stack0x00003304)[lVar61] = 1;
    (&stack0x00003303)[lVar61] = 1;
    (&stack0x00003302)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf599b;
    FUN_1418c9060(&stack0x000016a8 + lVar61,lVar79 + 0x40,&stack0x00001710 + lVar61);
    if ((&stack0x000032d7)[lVar61] == '\0') {
      uVar59 = 0;
    }
    else {
      *(undefined4 *)(&stack0x000030b0 + lVar61) = 0x3b9e38e8;
      *(undefined1 (*) [16])(&stack0x000030b8 + lVar61) = (undefined1  [16])0x0;
      *(undefined8 *)(&stack0x000030c8 + lVar61) = 0;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf59d9;
      func_0x000142923c20(&stack0x00002304 + lVar61,0,0x110);
      *(undefined4 *)(&stack0x00000d34 + lVar61) = 0;
      *(undefined8 *)(&stack0x00000d2c + lVar61) = 0;
      *(undefined1 (*) [16])(&stack0x00000c50 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000c60 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000c6c + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000c80 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000c90 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000ca0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000cb0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000cc0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000cd0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000ce0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000cf0 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000d00 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000d10 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00000d1c + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00003220 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00003214 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00003204 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031f4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031e4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031d4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031c4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031b4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x000031a4 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00003194 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00003184 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00002fb4 + lVar61) = (undefined1  [16])0x0;
      *(undefined8 *)(&stack0x00002fc4 + lVar61) = 0;
      *(undefined1 (*) [16])(&stack0x000022f0 + lVar61) = (undefined1  [16])0x0;
      *(undefined4 *)(&stack0x00002300 + lVar61) = 0;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5af4;
      func_0x000142923c00(&stack0x00002418 + lVar61,&stack0x00000c50 + lVar61,0xe8);
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5b0d;
      func_0x000142923c00(&stack0x00000c50 + lVar61,&stack0x000022f0 + lVar61,0x210);
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5b26;
      func_0x000142923c00(&stack0x00002ee0 + lVar61,&stack0x00003180 + lVar61,0xb0);
      *(undefined8 *)(&stack0x000030e0 + lVar61) = *(undefined8 *)(&stack0x00002fb0 + lVar61);
      *(undefined8 *)(&stack0x000030e8 + lVar61) = *(undefined8 *)(&stack0x00002fb8 + lVar61);
      *(undefined4 *)(&stack0x000030ec + lVar61) = *(undefined4 *)(&stack0x00002fbc + lVar61);
      *(undefined4 *)(&stack0x000030f0 + lVar61) = *(undefined4 *)(&stack0x00002fc0 + lVar61);
      *(undefined4 *)(&stack0x000030f4 + lVar61) = *(undefined4 *)(&stack0x00002fc4 + lVar61);
      *(undefined4 *)(&stack0x000030f8 + lVar61) = *(undefined4 *)(&stack0x00002fc8 + lVar61);
      do {
        puVar80 = *(undefined1 **)(puVar80 + 8);
      } while (puVar80 != (undefined1 *)0x0);
      *(undefined4 *)(&stack0x000022f0 + lVar61) = 0x3b9bb079;
      *(undefined1 **)(&stack0x000022f8 + lVar61) = &stack0x000030b0 + lVar61;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5b80;
      func_0x000142923c00(&stack0x00002300 + lVar61,&stack0x00000c50 + lVar61,0x210);
      *(undefined1 (*) [16])(&stack0x00002510 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00002520 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00002530 + lVar61) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(&stack0x00002540 + lVar61) = (undefined1  [16])0x0;
      *(undefined4 *)(&stack0x00002550 + lVar61) = 0;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5bc7;
      func_0x000142923c00(&stack0x00002554 + lVar61,&stack0x00002ee0 + lVar61,0xb0);
      *(undefined8 *)(&stack0x00002604 + lVar61) = *(undefined8 *)(&stack0x000030e0 + lVar61);
      *(undefined8 *)(&stack0x0000260c + lVar61) = *(undefined8 *)(&stack0x000030e8 + lVar61);
      *(undefined8 *)(&stack0x00002610 + lVar61) = *(undefined8 *)(&stack0x000030ec + lVar61);
      *(undefined8 *)(&stack0x00002618 + lVar61) = *(undefined8 *)(&stack0x000030f4 + lVar61);
      *(undefined1 (*) [16])(&stack0x00002620 + lVar61) = (undefined1  [16])0x0;
      *(undefined4 *)(&stack0x00002630 + lVar61) = 0;
      uVar70 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0xd0);
      pcVar8 = *(code **)(lVar83 + 0xb8);
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5c12;
      (*pcVar8)(uVar70,&stack0x000022f0 + lVar61);
      *(undefined4 *)(&stack0x00003290 + lVar61) = *(undefined4 *)(&stack0x000030c0 + lVar61);
      uVar59 = 1;
    }
    sVar56 = sRam0000000143634e58;
    LOCK();
    sRam0000000143634e58 = sRam0000000143634e58 + 1;
    UNLOCK();
    (&stack0x00003304)[lVar61] = 1;
    (&stack0x00003303)[lVar61] = 1;
    (&stack0x00003302)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5c57;
    FUN_1418c08e0(&stack0x00000c50 + lVar61,sVar56);
    *(undefined8 *)(&stack0x00002ee0 + lVar61) = 4;
    *(undefined4 *)(&stack0x00002ee8 + lVar61) = 2;
    (&stack0x00002eee)[lVar61] = 4;
    *(undefined2 *)(&stack0x00002eec + lVar61) = 0;
    puVar80 = &stack0x00002e20 + lVar61;
    if (*(longlong *)(&stack0x00002e20 + lVar61) == 0) {
      puVar80 = (undefined1 *)0x0;
    }
    (&stack0x000032f7)[lVar61] = 1;
    (&stack0x000032f6)[lVar61] = 1;
    (&stack0x000032f5)[lVar61] = 1;
    (&stack0x000032f4)[lVar61] = 1;
    *(undefined **)(&stack0xfffffffffffffff0 + lVar61) = &UNK_142b162e8;
    *(undefined1 **)(&stack0xffffffffffffffe8 + lVar61) = &stack0x00002ee0 + lVar61;
    *(undefined1 **)(&stack0xffffffffffffffe0 + lVar61) = &stack0x000005b0 + lVar61;
    *(undefined8 *)(&stack0xfffffffffffffff8 + lVar61) = 0x15;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5cf4;
    FUN_140bee200(&stack0x000022f0 + lVar61,&stack0x00001710 + lVar61,puVar80,
                  &stack0x00000c50 + lVar61);
    lVar83 = *(longlong *)(&stack0x000022f0 + lVar61);
    *(undefined8 *)(&stack0x00003180 + lVar61) = *(undefined8 *)(&stack0x000022f8 + lVar61);
    *(undefined8 *)(&stack0x00003188 + lVar61) = *(undefined8 *)(&stack0x00002300 + lVar61);
    *(undefined8 *)(&stack0x00003190 + lVar61) = *(undefined8 *)(&stack0x00002308 + lVar61);
    *(undefined8 *)(&stack0x00003198 + lVar61) = *(undefined8 *)(&stack0x00002310 + lVar61);
    *(undefined8 *)(&stack0x000031a0 + lVar61) = *(undefined8 *)(&stack0x00002318 + lVar61);
    *(undefined8 *)(&stack0x000031a8 + lVar61) = *(undefined8 *)(&stack0x00002320 + lVar61);
    *(undefined8 *)(&stack0x000031b0 + lVar61) = *(undefined8 *)(&stack0x00002328 + lVar61);
    *(undefined8 *)(&stack0x000031b8 + lVar61) = *(undefined8 *)(&stack0x00002330 + lVar61);
    if (lVar83 == 0xb) {
      *(undefined4 *)(&stack0x00003100 + lVar61) = *(undefined4 *)(&stack0x000031a0 + lVar61);
      *(undefined4 *)(&stack0x00003104 + lVar61) = *(undefined4 *)(&stack0x000031a4 + lVar61);
      *(undefined4 *)(&stack0x00003108 + lVar61) = *(undefined4 *)(&stack0x000031a8 + lVar61);
      *(undefined4 *)(&stack0x0000310c + lVar61) = *(undefined4 *)(&stack0x000031ac + lVar61);
      *(undefined4 *)(&stack0x000030f0 + lVar61) = *(undefined4 *)(&stack0x00003190 + lVar61);
      *(undefined4 *)(&stack0x000030f4 + lVar61) = *(undefined4 *)(&stack0x00003194 + lVar61);
      *(undefined4 *)(&stack0x000030f8 + lVar61) = *(undefined4 *)(&stack0x00003198 + lVar61);
      *(undefined4 *)(&stack0x000030fc + lVar61) = *(undefined4 *)(&stack0x0000319c + lVar61);
      *(undefined8 *)(&stack0x000030e0 + lVar61) = *(undefined8 *)(&stack0x00003180 + lVar61);
      *(undefined8 *)(&stack0x000030e8 + lVar61) = *(undefined8 *)(&stack0x00003188 + lVar61);
      *(undefined4 *)(&stack0x000022f0 + lVar61) = 0x11;
      *(undefined8 *)(&stack0x000022f8 + lVar61) = 0;
      *(undefined4 *)(&stack0x00002300 + lVar61) = 0;
      *(undefined1 (*) [16])(&stack0x00002308 + lVar61) = (undefined1  [16])0x0;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5db2;
      iVar60 = (**(code **)(&stack0x00001870 + lVar61))
                         (*(undefined8 *)(&stack0x00001cd8 + lVar61),&stack0x000022f0 + lVar61,0,
                          &stack0x00003180 + lVar61);
      if (iVar60 == 0) {
        *(undefined8 *)(&stack0x00003110 + lVar61) = *(undefined8 *)(&stack0x00003180 + lVar61);
        if (4 < *puRam000000014362c3a8) {
          *(undefined **)(&stack0x000022f0 + lVar61) = &UNK_142b16320;
          *(undefined8 *)(&stack0x000022f8 + lVar61) = 1;
          *(undefined8 *)(&stack0x00002300 + lVar61) = 8;
          *(undefined1 (*) [16])(&stack0x00002308 + lVar61) = (undefined1  [16])0x0;
          (&stack0x000032fe)[lVar61] = 1;
          (&stack0x000032fd)[lVar61] = 1;
          (&stack0x000032fc)[lVar61] = 1;
          (&stack0x000032fb)[lVar61] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5fa8;
          uVar70 = func_0x00014067dd80(&UNK_142b16330);
          *(undefined **)(&stack0x00003180 + lVar61) = &UNK_142b16030;
          *(undefined8 *)(&stack0x00003188 + lVar61) = 0x15;
          *(undefined **)(&stack0x00003190 + lVar61) = &UNK_142b16030;
          *(undefined8 *)(&stack0x00003198 + lVar61) = 0x15;
          *(undefined8 *)(&stack0x000031a0 + lVar61) = uVar70;
          (&stack0x000032fe)[lVar61] = 1;
          (&stack0x000032fd)[lVar61] = 1;
          (&stack0x000032fc)[lVar61] = 1;
          (&stack0x000032fb)[lVar61] = 1;
          *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf600e;
          FUN_1404b1430(&stack0x000022f0 + lVar61,5,&stack0x00003180 + lVar61);
        }
        (&stack0x000032fe)[lVar61] = 1;
        (&stack0x000032fd)[lVar61] = 1;
        (&stack0x000032fc)[lVar61] = 1;
        (&stack0x000032fb)[lVar61] = 1;
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6045;
        FUN_1418b7510(&stack0x00003078 + lVar61,0,uVar86 + 1,&UNK_142b15090);
        (&stack0x000032d5)[lVar61] = 1;
        (&stack0x000032d4)[lVar61] = 1;
        (&stack0x000032d3)[lVar61] = 1;
        (&stack0x000032d2)[lVar61] = 1;
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6074;
        FUN_140c10f20(&stack0x00002e80 + lVar61,&stack0x00001710 + lVar61);
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf608d;
        func_0x000142923c00(&stack0x000022f0 + lVar61,&stack0x00000c50 + lVar61,0xa58);
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6092;
        uVar70 = FUN_140bedd50();
        *(undefined8 *)(&stack0x00003170 + lVar61) = uVar70;
        *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf60b2;
        func_0x000142923c00(*(undefined8 *)(&stack0x00003170 + lVar61),&stack0x000022f0 + lVar61,
                            0xa58);
        *(undefined8 *)(&stack0x00003160 + lVar61) = *(undefined8 *)(&stack0x000032a0 + lVar61);
        plVar6 = *(longlong **)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x98);
        LOCK();
        lVar83 = *plVar6;
        *plVar6 = *plVar6 + 1;
        UNLOCK();
        if (*plVar6 != 0 && SCARRY8(lVar83,1) == *plVar6 < 0) {
          *(longlong **)(&stack0x00003150 + lVar61) = plVar6;
          plVar6 = *(longlong **)(&stack0x000032c0 + lVar61);
          LOCK();
          lVar83 = *plVar6;
          *plVar6 = *plVar6 + 1;
          UNLOCK();
          if (*plVar6 != 0 && SCARRY8(lVar83,1) == *plVar6 < 0) {
            *(undefined8 *)(&stack0x00003140 + lVar61) = *(undefined8 *)(&stack0x000032c0 + lVar61);
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6117;
            func_0x000142923c00(&stack0x00003198 + lVar61,&stack0x00002d70 + lVar61,0xb0);
            *(undefined8 *)(&stack0x00003248 + lVar61) = *(undefined8 *)(&stack0x00002e68 + lVar61);
            *(undefined8 *)(&stack0x00003250 + lVar61) = *(undefined8 *)(&stack0x00002e70 + lVar61);
            *(undefined8 *)(&stack0x00003258 + lVar61) = *(undefined8 *)(&stack0x00002e78 + lVar61);
            (&stack0x00003180)[lVar61] = 0;
            *(undefined8 *)(&stack0x00003188 + lVar61) = 1;
            *(undefined8 *)(&stack0x00003190 + lVar61) = 0;
            *(undefined8 *)(&stack0x00003260 + lVar61) = 1;
            *(undefined8 *)(&stack0x00002300 + lVar61) = *(undefined8 *)(&stack0x00003088 + lVar61);
            *(undefined8 *)(&stack0x000022f0 + lVar61) = *(undefined8 *)(&stack0x00003078 + lVar61);
            *(undefined8 *)(&stack0x000022f8 + lVar61) = *(undefined8 *)(&stack0x00003080 + lVar61);
            (&stack0x000032d1)[lVar61] = 1;
            (&stack0x000032d0)[lVar61] = 1;
            puVar77 = &UNK_142b16348;
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf619a;
            uVar70 = FUN_1418c9520(&stack0x000022f0 + lVar61);
            *(undefined8 *)(&stack0x00003000 + lVar61) = uVar70;
            *(undefined **)(&stack0x00003120 + lVar61) = puVar77;
            puVar80 = &stack0x000005b0 + lVar61;
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf61c4;
            func_0x000142923c00(&stack0x000022f0 + lVar61,puVar80,0x698);
            (&stack0x000032cf)[lVar61] = 1;
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf61d3;
            uVar70 = FUN_140bedc90(&stack0x000022f0 + lVar61);
            *(undefined8 *)(&stack0x00003130 + lVar61) = uVar70;
            *(undefined8 *)(&stack0x00002fd0 + lVar61) = *(undefined8 *)(&stack0x00001d20 + lVar61);
            *(undefined8 *)(&stack0x00002fd8 + lVar61) = *(undefined8 *)(&stack0x00001d28 + lVar61);
            *(undefined8 *)(&stack0x00002fc0 + lVar61) = *(undefined8 *)(&stack0x00001d10 + lVar61);
            *(undefined8 *)(&stack0x00002fc8 + lVar61) = *(undefined8 *)(&stack0x00001d18 + lVar61);
            *(undefined8 *)(&stack0x00002fb0 + lVar61) = *(undefined8 *)(&stack0x00001d00 + lVar61);
            *(undefined8 *)(&stack0x00002fb8 + lVar61) = *(undefined8 *)(&stack0x00001d08 + lVar61);
            *(undefined4 *)(&stack0x000030d0 + lVar61) = *(undefined4 *)(&stack0x00003100 + lVar61);
            *(undefined4 *)(&stack0x000030d4 + lVar61) = *(undefined4 *)(&stack0x00003104 + lVar61);
            *(undefined4 *)(&stack0x000030d8 + lVar61) = *(undefined4 *)(&stack0x00003108 + lVar61);
            *(undefined4 *)(&stack0x000030dc + lVar61) = *(undefined4 *)(&stack0x0000310c + lVar61);
            *(undefined4 *)(&stack0x000030c0 + lVar61) = *(undefined4 *)(&stack0x000030f0 + lVar61);
            *(undefined4 *)(&stack0x000030c4 + lVar61) = *(undefined4 *)(&stack0x000030f4 + lVar61);
            *(undefined4 *)(&stack0x000030c8 + lVar61) = *(undefined4 *)(&stack0x000030f8 + lVar61);
            *(undefined4 *)(&stack0x000030cc + lVar61) = *(undefined4 *)(&stack0x000030fc + lVar61);
            *(undefined8 *)(&stack0x000030b0 + lVar61) = *(undefined8 *)(&stack0x000030e0 + lVar61);
            *(undefined8 *)(&stack0x000030b8 + lVar61) = *(undefined8 *)(&stack0x000030e8 + lVar61);
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf623c;
            uVar70 = FUN_1418c2e90(&UNK_142b15658);
            uVar48 = _UNK_142b157a8;
            uVar47 = _UNK_142b157a0;
            uVar46 = _UNK_142b15798;
            uVar45 = _UNK_142b15790;
            (&stack0x00002ee0)[lVar61] = 0;
            *(undefined8 *)(&stack0x00002ee8 + lVar61) = _UNK_142b15790;
            *(undefined8 *)(&stack0x00002ef0 + lVar61) = _UNK_142b15798;
            *(undefined8 *)(&stack0x00002ef8 + lVar61) = _UNK_142b157a0;
            *(undefined8 *)(&stack0x00002f00 + lVar61) = _UNK_142b157a8;
            *(undefined8 *)(&stack0x00002f08 + lVar61) = uVar70;
            *(undefined1 **)(&stack0x00002f10 + lVar61) = puVar80;
            *(undefined4 *)(&stack0x00002f18 + lVar61) = 0;
            uVar70 = *(undefined8 *)(*(longlong *)(&stack0x000032c0 + lVar61) + 0x40);
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6292;
            FUN_140bed4c0(&stack0x000022f0 + lVar61);
            (&stack0x00002f90)[lVar61] = 0;
            *(undefined8 *)(&stack0x00002f98 + lVar61) = *(undefined8 *)(&stack0x000022f0 + lVar61);
            *(undefined8 *)(&stack0x00002fa0 + lVar61) = *(undefined8 *)(&stack0x000022f8 + lVar61);
            *(undefined8 *)(&stack0x00002fa8 + lVar61) = *(undefined8 *)(&stack0x00002300 + lVar61);
            (&stack0x000022f0)[lVar61] = 0;
            *(undefined8 *)(&stack0x000022f8 + lVar61) = 4;
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf62d5;
            uVar69 = FUN_1418c2e90(&UNK_142b15658);
            *(undefined8 *)(&stack0x000032c0 + lVar61) = uVar69;
            uVar69 = *(undefined8 *)(&stack0x00002e80 + lVar61);
            uVar78 = *(undefined8 *)(&stack0x00002e88 + lVar61);
            uVar73 = *(undefined4 *)(&stack0x00002e94 + lVar61);
            uVar29 = *(undefined4 *)(&stack0x00002e98 + lVar61);
            uVar30 = *(undefined4 *)(&stack0x00002e9c + lVar61);
            puVar62 = *(undefined8 **)(&stack0x000032e8 + lVar61);
            *(undefined4 *)(puVar62 + 0xe1) = *(undefined4 *)(&stack0x00002e90 + lVar61);
            *(undefined4 *)((longlong)puVar62 + 0x70c) = uVar73;
            *(undefined4 *)(puVar62 + 0xe2) = uVar29;
            *(undefined4 *)((longlong)puVar62 + 0x714) = uVar30;
            puVar62[0xdf] = uVar69;
            puVar62[0xe0] = uVar78;
            puVar62[0x151] = 0;
            puVar62[0x152] = 0;
            uVar73 = *(undefined4 *)(*(longlong *)(&stack0x00003288 + lVar61) + 6);
            puVar62[0x154] = 0;
            puVar62[0x155] = 0;
            puVar62[0x156] = 0;
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf633e;
            func_0x000142923c00(puVar62 + 0x14,auStackX_8 + lVar61,0x5a8);
            uVar69 = *(undefined8 *)(&stack0x00003160 + lVar61);
            uVar78 = *(undefined8 *)(&stack0x00003140 + lVar61);
            uVar85 = *(undefined8 *)(&stack0x00003150 + lVar61);
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf636c;
            func_0x000142923c00(puVar62 + 0xe3,&stack0x00003180 + lVar61,0xe8);
            *(undefined8 *)((longlong)puVar62 + 0xacc) = *(undefined8 *)(&stack0x00002ed0 + lVar61);
            uVar27 = *(undefined8 *)(&stack0x00002ec8 + lVar61);
            *(undefined8 *)((longlong)puVar62 + 0xabc) = *(undefined8 *)(&stack0x00002ec0 + lVar61);
            *(undefined8 *)((longlong)puVar62 + 0xac4) = uVar27;
            uVar27 = *(undefined8 *)(&stack0x00002fe0 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00002fe8 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00002ff8 + lVar61);
            puVar62[6] = *(undefined8 *)(&stack0x00002ff0 + lVar61);
            puVar62[7] = uVar40;
            puVar62[4] = uVar27;
            puVar62[5] = uVar28;
            uVar27 = *(undefined8 *)(&stack0x00002fb0 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00002fb8 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00002fc0 + lVar61);
            uVar41 = *(undefined8 *)(&stack0x00002fc8 + lVar61);
            uVar42 = *(undefined8 *)(&stack0x00002fd8 + lVar61);
            puVar62[0xd4] = *(undefined8 *)(&stack0x00002fd0 + lVar61);
            puVar62[0xd5] = uVar42;
            puVar62[0xd2] = uVar40;
            puVar62[0xd3] = uVar41;
            puVar62[0xd0] = uVar27;
            puVar62[0xd1] = uVar28;
            uVar27 = *(undefined8 *)(&stack0x000030e0 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x000030e8 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x000030f0 + lVar61);
            uVar41 = *(undefined8 *)(&stack0x000030f8 + lVar61);
            uVar42 = *(undefined8 *)(&stack0x00003108 + lVar61);
            puVar62[0xd] = *(undefined8 *)(&stack0x00003100 + lVar61);
            puVar62[0xe] = uVar42;
            puVar62[0xb] = uVar40;
            puVar62[0xc] = uVar41;
            puVar62[9] = uVar27;
            puVar62[10] = uVar28;
            uVar27 = *(undefined8 *)(&stack0x00002ee8 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00002ef0 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00002ef8 + lVar61);
            uVar41 = *(undefined8 *)(&stack0x00002f00 + lVar61);
            uVar42 = *(undefined8 *)(&stack0x00002f08 + lVar61);
            uVar43 = *(undefined8 *)(&stack0x00002f10 + lVar61);
            uVar44 = *(undefined8 *)(&stack0x00002f18 + lVar61);
            puVar62[0x102] = *(undefined8 *)(&stack0x00002ee0 + lVar61);
            puVar62[0x103] = uVar27;
            puVar62[0x104] = uVar28;
            puVar62[0x105] = uVar40;
            puVar62[0x106] = uVar41;
            puVar62[0x107] = uVar42;
            puVar62[0x108] = uVar43;
            puVar62[0x109] = uVar44;
            puVar62[0xde] = *(undefined8 *)(&stack0x000016e8 + lVar61);
            uVar27 = *(undefined8 *)(&stack0x000016b0 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x000016b8 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x000016c0 + lVar61);
            uVar41 = *(undefined8 *)(&stack0x000016c8 + lVar61);
            uVar42 = *(undefined8 *)(&stack0x000016d0 + lVar61);
            uVar43 = *(undefined8 *)(&stack0x000016d8 + lVar61);
            uVar44 = *(undefined8 *)(&stack0x000016e0 + lVar61);
            puVar62[0xd6] = *(undefined8 *)(&stack0x000016a8 + lVar61);
            puVar62[0xd7] = uVar27;
            puVar62[0xd8] = uVar28;
            puVar62[0xd9] = uVar40;
            puVar62[0xda] = uVar41;
            puVar62[0xdb] = uVar42;
            puVar62[0xdc] = uVar43;
            puVar62[0xdd] = uVar44;
            uVar27 = *(undefined8 *)(&stack0x00001ce8 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00001cf0 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00001cf8 + lVar61);
            puVar62[0x10a] = *(undefined8 *)(&stack0x00001ce0 + lVar61);
            puVar62[0x10b] = uVar27;
            puVar62[0x10c] = uVar28;
            puVar62[0x10d] = uVar40;
            puVar62[0x116] = *(undefined8 *)(&stack0x00002e60 + lVar61);
            uVar27 = *(undefined8 *)(&stack0x00002e20 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00002e28 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00002e30 + lVar61);
            uVar41 = *(undefined8 *)(&stack0x00002e38 + lVar61);
            uVar42 = *(undefined8 *)(&stack0x00002e40 + lVar61);
            uVar43 = *(undefined8 *)(&stack0x00002e48 + lVar61);
            uVar44 = *(undefined8 *)(&stack0x00002e58 + lVar61);
            puVar62[0x114] = *(undefined8 *)(&stack0x00002e50 + lVar61);
            puVar62[0x115] = uVar44;
            puVar62[0x112] = uVar42;
            puVar62[0x113] = uVar43;
            puVar62[0x110] = uVar40;
            puVar62[0x111] = uVar41;
            puVar62[0x10e] = uVar27;
            puVar62[0x10f] = uVar28;
            uVar27 = *(undefined8 *)(&stack0x00002f98 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x00002fa0 + lVar61);
            uVar40 = *(undefined8 *)(&stack0x00002fa8 + lVar61);
            puVar62[0x136] = *(undefined8 *)(&stack0x00002f90 + lVar61);
            puVar62[0x137] = uVar27;
            puVar62[0x138] = uVar28;
            puVar62[0x139] = uVar40;
            *(undefined1 *)(puVar62 + 0x13a) = 0;
            puVar62[0x13b] = 0;
            puVar62[0x13c] = 8;
            puVar62[0x13d] = 0;
            uVar27 = *(undefined8 *)(&stack0x00002348 + lVar61);
            puVar62[0x148] = *(undefined8 *)(&stack0x00002340 + lVar61);
            puVar62[0x149] = uVar27;
            uVar27 = *(undefined8 *)(&stack0x00002338 + lVar61);
            puVar62[0x146] = *(undefined8 *)(&stack0x00002330 + lVar61);
            puVar62[0x147] = uVar27;
            uVar27 = *(undefined8 *)(&stack0x000022f0 + lVar61);
            uVar28 = *(undefined8 *)(&stack0x000022f8 + lVar61);
            uVar29 = *(undefined4 *)(&stack0x00002300 + lVar61);
            uVar30 = *(undefined4 *)(&stack0x00002304 + lVar61);
            uVar31 = *(undefined4 *)(&stack0x00002308 + lVar61);
            uVar32 = *(undefined4 *)(&stack0x0000230c + lVar61);
            uVar33 = *(undefined4 *)(&stack0x00002310 + lVar61);
            uVar34 = *(undefined4 *)(&stack0x00002314 + lVar61);
            uVar35 = *(undefined4 *)(&stack0x00002318 + lVar61);
            uVar36 = *(undefined4 *)(&stack0x0000231c + lVar61);
            uVar37 = *(undefined4 *)(&stack0x00002324 + lVar61);
            uVar38 = *(undefined4 *)(&stack0x00002328 + lVar61);
            uVar39 = *(undefined4 *)(&stack0x0000232c + lVar61);
            *(undefined4 *)(puVar62 + 0x144) = *(undefined4 *)(&stack0x00002320 + lVar61);
            *(undefined4 *)((longlong)puVar62 + 0xa24) = uVar37;
            *(undefined4 *)(puVar62 + 0x145) = uVar38;
            *(undefined4 *)((longlong)puVar62 + 0xa2c) = uVar39;
            *(undefined4 *)(puVar62 + 0x142) = uVar33;
            *(undefined4 *)((longlong)puVar62 + 0xa14) = uVar34;
            *(undefined4 *)(puVar62 + 0x143) = uVar35;
            *(undefined4 *)((longlong)puVar62 + 0xa1c) = uVar36;
            puVar62[0x13e] = uVar27;
            puVar62[0x13f] = uVar28;
            *(undefined4 *)(puVar62 + 0x140) = uVar29;
            *(undefined4 *)((longlong)puVar62 + 0xa04) = uVar30;
            *(undefined4 *)(puVar62 + 0x141) = uVar31;
            *(undefined4 *)((longlong)puVar62 + 0xa0c) = uVar32;
            *puVar62 = *(undefined8 *)(&stack0x000032e0 + lVar61);
            puVar62[1] = *(undefined8 *)(&stack0x000032d8 + lVar61);
            puVar62[2] = *(undefined8 *)(&stack0x000030a8 + lVar61);
            *(undefined1 *)(puVar62 + 3) = 0;
            *(undefined1 *)((longlong)puVar62 + 0x19) = (&stack0x000032d6)[lVar61];
            *(undefined1 *)((longlong)puVar62 + 0x1a) = (&stack0x000032bd)[lVar61];
            *(char *)((longlong)puVar62 + 0x1b) = (char)*(undefined4 *)(&stack0x000032ac + lVar61);
            *(undefined1 *)((longlong)puVar62 + 0x1c) = (&stack0x000032d7)[lVar61];
            *(undefined4 *)(puVar62 + 8) = uVar59;
            *(undefined4 *)((longlong)puVar62 + 0x44) = *(undefined4 *)(&stack0x00003290 + lVar61);
            puVar62[0xf] = *(undefined8 *)(&stack0x000032b0 + lVar61);
            puVar62[0x10] = *(undefined8 *)(&stack0x00002ea0 + lVar61);
            puVar62[0x11] = *(undefined8 *)(&stack0x00002ea8 + lVar61);
            puVar62[0x12] = *(undefined8 *)(&stack0x00002eb0 + lVar61);
            puVar62[0x13] = *(undefined8 *)(&stack0x00002eb8 + lVar61);
            puVar62[0xc9] = *(undefined8 *)(&stack0x00003170 + lVar61);
            puVar62[0xca] = uVar69;
            puVar62[0xcb] = uVar78;
            puVar62[0xcc] = uVar85;
            puVar62[0xcd] = *(undefined8 *)(&stack0x00003000 + lVar61);
            puVar62[0xce] = *(undefined8 *)(&stack0x00003120 + lVar61);
            puVar62[0xcf] = *(undefined8 *)(&stack0x00003130 + lVar61);
            puVar62[0x100] = 0;
            puVar62[0x101] = *(undefined8 *)(&stack0x00003110 + lVar61);
            puVar62[0x117] = 0;
            puVar62[0x14a] = 0;
            puVar62[0x14d] = uVar47;
            puVar62[0x14e] = uVar48;
            puVar62[0x14b] = uVar45;
            puVar62[0x14c] = uVar46;
            puVar62[0x14f] = *(undefined8 *)(&stack0x000032c0 + lVar61);
            puVar62[0x150] = uVar70;
            *(undefined4 *)(puVar62 + 0x157) = uVar73;
            *(undefined4 *)((longlong)puVar62 + 0xad4) = *(undefined4 *)(&stack0x00003298 + lVar61);
            *(undefined1 *)(puVar62 + 0x15b) = (&stack0x000032be)[lVar61];
            *(undefined1 *)((longlong)puVar62 + 0xad9) = (&stack0x000032bf)[lVar61];
            *(undefined2 *)((longlong)puVar62 + 0xade) = *(undefined2 *)(&stack0x000022f4 + lVar61);
            *(undefined4 *)((longlong)puVar62 + 0xada) = *(undefined4 *)(&stack0x000022f0 + lVar61);
            if (*(longlong *)(&stack0x00003270 + lVar61) != 0) {
              *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf670b;
              func_0x000140613c20(*(undefined8 *)(&stack0x00003278 + lVar61),
                                  *(longlong *)(&stack0x00003270 + lVar61) * 0x28,8);
            }
            *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf6717;
            FUN_1405e6620(&stack0x00003090 + lVar61);
            return *(undefined8 **)(&stack0x000032e8 + lVar61);
          }
        }
        goto code_r0x000140bf685d;
      }
      puVar62 = *(undefined8 **)(&stack0x000032e8 + lVar61);
      puVar62[1] = 0xf;
      puVar62[2] = &UNK_142b16360;
      puVar62[3] = 0x15;
      *(int *)(puVar62 + 4) = iVar60;
      *puVar62 = 0x8000000000000000;
      (&stack0x000032f7)[lVar61] = 1;
      (&stack0x000032f6)[lVar61] = 1;
      (&stack0x000032f5)[lVar61] = 1;
      (&stack0x000032f4)[lVar61] = 1;
      *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5e14;
      FUN_140be24b0(&stack0x000030e8 + lVar61);
    }
    else {
      uVar70 = *(undefined8 *)(&stack0x00002338 + lVar61);
      uVar69 = *(undefined8 *)(&stack0x00003180 + lVar61);
      uVar78 = *(undefined8 *)(&stack0x00003188 + lVar61);
      uVar85 = *(undefined8 *)(&stack0x00003190 + lVar61);
      uVar27 = *(undefined8 *)(&stack0x00003198 + lVar61);
      uVar28 = *(undefined8 *)(&stack0x000031a0 + lVar61);
      uVar40 = *(undefined8 *)(&stack0x000031a8 + lVar61);
      uVar41 = *(undefined8 *)(&stack0x000031b8 + lVar61);
      puVar62 = *(undefined8 **)(&stack0x000032e8 + lVar61);
      puVar62[8] = *(undefined8 *)(&stack0x000031b0 + lVar61);
      puVar62[9] = uVar41;
      puVar62[6] = uVar28;
      puVar62[7] = uVar40;
      puVar62[4] = uVar85;
      puVar62[5] = uVar27;
      puVar62[2] = uVar69;
      puVar62[3] = uVar78;
      puVar62[1] = lVar83;
      puVar62[10] = uVar70;
      *puVar62 = 0x8000000000000000;
    }
    (&stack0x00003304)[lVar61] = 1;
    (&stack0x00003303)[lVar61] = 1;
    (&stack0x00003302)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5e88;
    FUN_140be0d50(&stack0x00000c50 + lVar61);
    (&stack0x00003301)[lVar61] = 1;
    (&stack0x00003300)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5ea2;
    func_0x00014014a360(&stack0x00001d00 + lVar61);
    (&stack0x000032f9)[lVar61] = 1;
    (&stack0x000032f8)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5ebc;
    FUN_140be0b70(&stack0x00002e68 + lVar61);
    (&stack0x00003306)[lVar61] = 1;
    (&stack0x00003305)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5ed6;
    FUN_140bdf360(&stack0x00002d70 + lVar61);
    (&stack0x00003307)[lVar61] = 1;
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf5ee9;
    FUN_140bde330(&stack0x000005b0 + lVar61);
    lVar79 = *(longlong *)(&stack0x000032e0 + lVar61);
    uVar70 = *(undefined8 *)(&stack0x000032d8 + lVar61);
    param_1 = *(undefined8 **)(&stack0x000032e8 + lVar61);
    lVar83 = *(longlong *)(&stack0x00003270 + lVar61);
  }
  if (lVar83 != 0) {
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3d7f;
    func_0x000140613c20(*(undefined8 *)(&stack0x00003278 + lVar61),lVar83 * 0x28,8);
  }
  puVar55 = puRam000000014362c3e0;
  puVar62 = *(undefined8 **)(&stack0x000030a0 + lVar61);
  while (puVar62 != puVar55) {
    puVar9 = (undefined8 *)puVar62[3];
    uVar69 = *puVar62;
    uVar78 = puVar62[1];
    uVar85 = puVar62[2];
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3db4;
    func_0x000140613c20(uVar69,uVar85,uVar78);
    puVar62 = puVar9;
  }
code_r0x000140bf3dbc:
  if (lVar79 != 0) {
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf3dda;
    func_0x000140613c20(uVar70,lVar79 << 3,8);
  }
code_r0x000140bf2b58:
  plVar6 = *(longlong **)(&stack0x000032a0 + lVar61);
  LOCK();
  *plVar6 = *plVar6 + -1;
  UNLOCK();
  if (*plVar6 == 0) {
    *(undefined **)((longlong)&puStack_48 + lVar61) = &UNK_140bf2b71;
    func_0x000140b5d090(&stack0x000032a0 + lVar61);
  }
  return param_1;
}

