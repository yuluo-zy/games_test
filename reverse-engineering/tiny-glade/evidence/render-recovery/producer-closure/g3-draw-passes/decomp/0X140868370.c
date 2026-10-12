// FUN_140868370  0x140868370

void FUN_140868370(undefined1 (*param_1) [16])

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  char cVar3;
  undefined8 uVar4;
  ushort uVar5;
  char cVar6;
  ulonglong uVar7;
  byte bVar8;
  uint uVar9;
  undefined1 (**ppauVar10) [16];
  uint uVar11;
  ulonglong uVar12;
  undefined8 *puVar13;
  longlong lVar14;
  undefined1 (*pauVar15) [16];
  byte bVar16;
  undefined1 (*pauVar17) [16];
  undefined1 (*pauVar18) [16];
  int iVar19;
  undefined8 uStack_5f0;
  undefined4 uStack_5e8;
  undefined4 uStack_5e4;
  undefined8 uStack_5e0;
  undefined1 (*pauStack_568) [16];
  undefined8 uStack_560;
  undefined8 uStack_558;
  undefined1 auStack_550 [16];
  undefined8 uStack_540;
  undefined8 uStack_538;
  undefined8 uStack_530;
  undefined4 uStack_528;
  undefined4 uStack_524;
  undefined4 uStack_520;
  undefined4 uStack_51c;
  undefined4 uStack_518;
  undefined4 uStack_514;
  undefined4 uStack_510;
  undefined4 uStack_50c;
  undefined4 uStack_508;
  undefined4 uStack_504;
  undefined4 uStack_500;
  undefined4 uStack_4fc;
  undefined8 uStack_4f8;
  undefined8 uStack_4f0;
  int iStack_4e8;
  undefined4 uStack_4e4;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined4 uStack_4d8;
  undefined4 uStack_4d4;
  undefined8 uStack_4d0;
  undefined8 uStack_4c8;
  undefined8 uStack_4c0;
  undefined4 uStack_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  undefined4 uStack_4ac;
  undefined8 uStack_4a8;
  undefined8 uStack_4a0;
  undefined8 uStack_498;
  undefined8 uStack_490;
  undefined8 uStack_488;
  undefined8 uStack_480;
  undefined8 uStack_478;
  undefined8 uStack_470;
  undefined8 uStack_468;
  undefined8 uStack_460;
  undefined8 uStack_458;
  undefined1 auStack_450 [488];
  undefined1 (*pauStack_268) [16];
  undefined8 *puStack_260;
  undefined1 (*pauStack_250) [16];
  undefined8 *puStack_248;
  undefined8 uStack_240;
  undefined4 uStack_238;
  char cStack_234;
  undefined2 uStack_233;
  undefined1 auStack_231 [17];
  longlong lStack_220;
  longlong lStack_218;
  longlong lStack_1f0;
  longlong lStack_1e8;
  uint uStack_1c0;
  undefined8 uStack_1bc;
  undefined8 uStack_1b4;
  char cStack_1ac;
  undefined2 uStack_1ab;
  undefined1 uStack_1a9;
  undefined8 uStack_1a8;
  undefined8 *puStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  int iStack_a8;
  undefined1 (*pauStack_a4) [16];
  undefined8 *puStack_9c;
  undefined1 uStack_94;
  undefined1 (*pauStack_88) [16];
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined1 (*pauStack_58) [16];
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  param_1[0x3e][4] = 1;
  pauVar18 = param_1 + 0x20;
  uVar11 = *(uint *)param_1[0x3e];
  pauStack_88 = pauVar18;
  pauStack_58 = param_1;
  if ((ulonglong)uVar11 == 0) {
    iVar19 = 0;
  }
  else {
    iVar19 = 0;
    lVar14 = 0;
    do {
      uStack_49 = 1;
      func_0x000140be6780(&pauStack_250,*(undefined4 *)(pauVar18[5] + 4));
      uStack_49 = 1;
      func_0x000140be67c0(&uStack_128,&pauStack_250,pauVar18[5][0xc]);
      uStack_128 = (undefined1 (*) [16])
                   CONCAT44(*(undefined4 *)(pauVar18[5] + 8),(undefined4)uStack_128);
      uStack_5e8 = (undefined4)uStack_120;
      uStack_5f0 = uStack_128;
      cVar6 = uStack_120._4_1_;
      uStack_1a8._0_3_ = (undefined3)((ulonglong)uStack_120 >> 0x28);
      if (uStack_120._4_1_ == '\0') break;
      if (lVar14 == 0x80) {
        uStack_49 = 1;
        FUN_1428fa090(&UNK_142c77358);
      }
      pauVar18 = pauVar18 + 6;
      *(undefined4 *)((longlong)&uStack_560 + lVar14) = uStack_5e8;
      *(undefined8 *)((longlong)&pauStack_568 + lVar14) = uStack_5f0;
      *(char *)((longlong)&uStack_560 + lVar14 + 4) = cVar6;
      *(undefined1 *)((longlong)&uStack_560 + lVar14 + 7) = uStack_1a8._2_1_;
      *(undefined2 *)((longlong)&uStack_560 + lVar14 + 5) = (undefined2)uStack_1a8;
      lVar14 = lVar14 + 0x10;
      iVar19 = iVar19 + 1;
    } while (pauVar18 != param_1 + (ulonglong)uVar11 * 6 + 0x20);
  }
  uStack_1a8 = pauStack_568;
  puStack_1a0 = uStack_560;
  uStack_198 = (undefined4)uStack_558;
  uStack_194 = uStack_558._4_4_;
  uStack_190 = auStack_550._0_4_;
  uStack_18c = auStack_550._4_4_;
  uStack_188 = auStack_550._8_4_;
  uStack_184 = auStack_550._12_4_;
  uStack_180 = (undefined4)uStack_540;
  uStack_17c = uStack_540._4_4_;
  uStack_178 = (undefined4)uStack_538;
  uStack_174 = uStack_538._4_4_;
  uStack_170 = (undefined4)uStack_530;
  uStack_16c = uStack_530._4_4_;
  uStack_168 = CONCAT44(uStack_524,uStack_528);
  uStack_160 = CONCAT44(uStack_51c,uStack_520);
  uStack_158 = CONCAT44(uStack_514,uStack_518);
  uStack_150 = CONCAT44(uStack_50c,uStack_510);
  uStack_148 = CONCAT44(uStack_504,uStack_508);
  uStack_140 = CONCAT44(uStack_4fc,uStack_500);
  uStack_138 = uStack_4f8;
  uStack_130 = uStack_4f0;
  uStack_94 = 3;
  if (*(int *)pauStack_58[0xe] != 2) {
    uStack_49 = 1;
    func_0x000140be6780(&pauStack_568,*(undefined4 *)(pauStack_58[0x12] + 8));
    uStack_49 = 1;
    func_0x000140be67c0(&pauStack_250,&pauStack_568,pauStack_58[0x12][0xd]);
    if (*(int *)(pauStack_58[0x11] + 0xc) != 0) {
      pauStack_568 = pauStack_250;
      uStack_560 = puStack_248;
      uStack_49 = 1;
      func_0x000140be67a0(&pauStack_250,&pauStack_568);
    }
    uStack_94 = pauStack_58[0x12][0xc];
    pauStack_268 = pauStack_250;
    puStack_260 = puStack_248;
  }
  uStack_128 = uStack_1a8;
  uStack_120 = puStack_1a0;
  uStack_118 = CONCAT44(uStack_194,uStack_198);
  uStack_110 = CONCAT44(uStack_18c,uStack_190);
  uStack_108 = CONCAT44(uStack_184,uStack_188);
  uStack_100 = CONCAT44(uStack_17c,uStack_180);
  uStack_f8 = CONCAT44(uStack_174,uStack_178);
  uStack_f0 = CONCAT44(uStack_16c,uStack_170);
  uStack_e8 = uStack_168;
  uStack_e0 = uStack_160;
  uStack_d8 = uStack_158;
  uStack_d0 = uStack_150;
  uStack_c8 = uStack_148;
  uStack_c0 = uStack_140;
  uStack_b8 = uStack_138;
  uStack_b0 = uStack_130;
  pauStack_a4 = pauStack_268;
  puStack_9c = puStack_260;
  uStack_49 = 1;
  iStack_a8 = iVar19;
  FUN_141524830(&pauStack_568,*(undefined8 *)(pauStack_58[0x1e] + 8),&uStack_128);
  puVar13 = uStack_560;
  pauVar15 = pauStack_568;
  cVar6 = uStack_4c8._4_1_;
  if (uStack_4c8._4_1_ == '\x04') {
    puVar13 = (undefined8 *)(pauStack_568[-2] + 8);
    uStack_49 = 1;
    func_0x000140bd6b00(&uStack_78,puVar13);
    while( true ) {
      pauVar15 = pauStack_58;
      pauVar17 = pauStack_58 + 0xe;
      func_0x000142923c00(&pauStack_250,pauStack_58,0x90);
      uStack_e8 = *(undefined8 *)pauVar15[0xd];
      uStack_e0 = *(undefined8 *)(pauVar15[0xd] + 8);
      uStack_128 = *(undefined1 (**) [16])pauVar15[9];
      uStack_120 = *(undefined8 **)(pauVar15[9] + 8);
      uStack_118 = *(undefined8 *)pauVar15[10];
      uStack_110 = *(undefined8 *)(pauVar15[10] + 8);
      uStack_108 = *(undefined8 *)pauVar15[0xb];
      uStack_100 = *(undefined8 *)(pauVar15[0xb] + 8);
      uStack_f8 = *(undefined8 *)pauVar15[0xc];
      uStack_f0 = *(undefined8 *)(pauVar15[0xc] + 8);
      uStack_478 = uStack_68;
      uStack_488 = uStack_78;
      uStack_480 = uStack_70;
      uStack_4d8 = *(undefined4 *)pauVar15[0x14];
      uStack_4d4 = *(undefined4 *)(pauVar15[0x14] + 4);
      uStack_4d0 = *(undefined8 *)(pauVar15[0x14] + 8);
      uStack_4c8 = *(undefined8 *)pauVar15[0x15];
      uStack_4c0 = *(undefined8 *)(pauVar15[0x15] + 8);
      uStack_4b8 = *(undefined4 *)pauVar15[0x16];
      uStack_4b4 = *(undefined4 *)(pauVar15[0x16] + 4);
      uStack_4b0 = *(undefined4 *)(pauVar15[0x16] + 8);
      uStack_4ac = *(undefined4 *)(pauVar15[0x16] + 0xc);
      iStack_4e8 = (int)*(undefined8 *)pauVar15[0x13];
      uStack_4e4 = (undefined4)((ulonglong)*(undefined8 *)pauVar15[0x13] >> 0x20);
      uStack_4e0 = (undefined4)*(undefined8 *)(pauVar15[0x13] + 8);
      uStack_4dc = (undefined4)((ulonglong)*(undefined8 *)(pauVar15[0x13] + 8) >> 0x20);
      uStack_4a8 = *(undefined8 *)pauVar15[0x17];
      uStack_4a0 = *(undefined8 *)(pauVar15[0x17] + 8);
      uStack_498 = *(undefined8 *)pauVar15[0x18];
      uStack_490 = *(undefined8 *)(pauVar15[0x18] + 8);
      uStack_470 = *(undefined8 *)pauVar15[0x19];
      uStack_468 = *(undefined8 *)(pauVar15[0x19] + 8);
      uStack_460 = *(undefined8 *)pauVar15[0x1a];
      func_0x000142923c00(auStack_450,pauStack_88,0x1e4);
      uStack_538 = *(undefined8 *)*pauVar17;
      uStack_530 = *(undefined8 *)(pauVar15[0xe] + 8);
      uStack_528 = *(undefined4 *)pauVar15[0xf];
      uStack_524 = *(undefined4 *)(pauVar15[0xf] + 4);
      uStack_520 = *(undefined4 *)(pauVar15[0xf] + 8);
      uStack_51c = *(undefined4 *)(pauVar15[0xf] + 0xc);
      uStack_518 = *(undefined4 *)pauVar15[0x10];
      uStack_514 = *(undefined4 *)(pauVar15[0x10] + 4);
      uStack_510 = *(undefined4 *)(pauVar15[0x10] + 8);
      uStack_50c = *(undefined4 *)(pauVar15[0x10] + 0xc);
      uStack_508 = *(undefined4 *)pauVar15[0x11];
      uStack_504 = *(undefined4 *)(pauVar15[0x11] + 4);
      uStack_500 = *(undefined4 *)(pauVar15[0x11] + 8);
      uStack_4fc = *(undefined4 *)(pauVar15[0x11] + 0xc);
      uStack_4f8 = *(undefined8 *)pauVar15[0x12];
      uStack_4f0 = *(undefined8 *)(pauVar15[0x12] + 8);
      uStack_458 = *(undefined8 *)(pauVar15[0x1f] + 8);
      uStack_558 = *(undefined8 *)pauVar15[1];
      pauStack_568 = *(undefined1 (**) [16])*pauVar15;
      uStack_560 = *(undefined8 **)(*pauVar15 + 8);
      auStack_550 = *(undefined1 (*) [16])(pauVar15[1] + 8);
      uStack_540 = *(undefined8 *)(pauVar15[2] + 8);
      FUN_1409dcd10(&uStack_128,&pauStack_568);
      if ((lStack_218 != 0) && (lVar14 = lStack_218 * 0x11 + 0x21, lVar14 != 0)) {
        func_0x000140613c20(lStack_220 + lStack_218 * -0x10 + -0x10,lVar14,0x10);
      }
      pauVar17 = pauStack_58;
      if (lStack_1e8 != 0) {
        uVar7 = (lStack_1e8 + 1) * 0x18 + 0xfU & 0xfffffffffffffff0;
        lVar14 = lStack_1e8 + uVar7 + 0x11;
        if (lVar14 != 0) {
          func_0x000140613c20(lStack_1f0 - uVar7,lVar14,0x10);
        }
      }
      if ((pauVar17[0x3e][4] != '\0') ||
         (((*puRam000000014362c420 & 0x7fffffffffffffff) != 0 &&
          (cVar6 = FUN_1428d86b0(), cVar6 == '\0')))) break;
      pauStack_568 = (undefined1 (*) [16])&UNK_142a98958;
      uStack_560 = (undefined8 *)0x1;
      uStack_558 = 8;
      auStack_550 = (undefined1  [16])0x0;
      ppauVar10 = &pauStack_568;
      uVar7 = FUN_1428d9390(ppauVar10,&UNK_142a989a8);
code_r0x000140868b49:
      lVar14 = 0x10;
      do {
        ppauVar10 = (undefined1 (**) [16])((longlong)ppauVar10 + lVar14 & uVar7);
        auVar2 = *(undefined1 (*) [16])(*pauVar15 + (longlong)ppauVar10);
        lVar14 = lVar14 + 0x10;
        uVar11 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
        pauVar17 = pauVar18;
      } while (uVar11 == 0);
code_r0x000140868872:
      uVar9 = 0;
      for (; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar12 = (ulonglong)uVar9 + (longlong)ppauVar10 & uVar7;
      bVar8 = (*pauVar15)[uVar12];
      if (-1 < (char)bVar8) {
        auVar2 = *pauVar15;
        uVar11 = 0;
        for (uVar9 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                    (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                    (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                    (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                    (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                    (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                    (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                    (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                    (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                    (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                    (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                    (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                    (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                    (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar2[0xf] >> 7) << 0xf); (uVar9 & 1) == 0;
            uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar11 = uVar11 + 1;
        }
        uVar12 = (ulonglong)uVar11;
        bVar8 = (*pauVar15)[uVar12];
      }
      pauVar18 = (undefined1 (*) [16])0x0;
      bVar16 = (byte)((ulonglong)pauVar17 >> 0x39);
      (*pauVar15)[uVar12] = bVar16;
      pauVar15[1][uVar12 - 0x10 & uVar7] = bVar16;
      puVar13[2] = puVar13[2] - (ulonglong)(bVar8 & 1);
      func_0x000142923c00(pauVar15 + uVar12 * -0xb + -0xb,&pauStack_568,0xb0);
      puVar13[3] = puVar13[3] + 1;
      uStack_4a = 0;
      func_0x000140bd6b00(&uStack_78,pauVar15[uVar12 * -0xb + -2] + 8);
    }
    return;
  }
  func_0x000142923c00(&uStack_240,&uStack_558,0x94);
  uStack_1ab = uStack_4c8._5_2_;
  uStack_1a9 = uStack_4c8._7_1_;
  pauStack_250 = pauVar15;
  puStack_248 = puVar13;
  cStack_1ac = cVar6;
  uStack_80 = *(undefined8 *)pauStack_58[0x1e];
  if ((ulonglong)uStack_1c0 == 0) {
    iVar19 = 0;
  }
  else {
    lVar14 = 0;
    iVar19 = 0;
    puVar13 = &uStack_240;
    do {
      cVar3 = *(char *)((longlong)puVar13 + 0xc);
      if (cVar3 == '\0') break;
      if (lVar14 == 0x80) {
        uStack_4a = 1;
        FUN_1428fa090(&UNK_142c77358);
      }
      uVar4 = *puVar13;
      puVar1 = puVar13 + 2;
      *(undefined4 *)((longlong)&uStack_560 + lVar14) = *(undefined4 *)(puVar13 + 1);
      *(undefined8 *)((longlong)&pauStack_568 + lVar14) = uVar4;
      *(char *)((longlong)&uStack_560 + lVar14 + 4) = cVar3;
      *(undefined1 *)((longlong)&uStack_560 + lVar14 + 7) = *(undefined1 *)((longlong)puVar13 + 0xf)
      ;
      *(undefined2 *)((longlong)&uStack_560 + lVar14 + 5) = *(undefined2 *)((longlong)puVar13 + 0xd)
      ;
      lVar14 = lVar14 + 0x10;
      iVar19 = iVar19 + 1;
      puVar13 = puVar1;
    } while (puVar1 != &uStack_240 + (ulonglong)uStack_1c0 * 2);
  }
  iStack_4e8 = iVar19;
  func_0x000142923c00(&uStack_5f0,&pauStack_568,0x84);
  if (cVar6 != '\x03') {
    uStack_78 = uStack_1bc;
    uStack_70 = uStack_1b4;
  }
  func_0x000142923c00(&pauStack_568,&uStack_5f0,0x84);
  uStack_4e4 = (undefined4)uStack_78;
  uStack_4e0 = (undefined4)((ulonglong)uStack_78 >> 0x20);
  uStack_4dc = (undefined4)uStack_70;
  uStack_4d8 = (undefined4)((ulonglong)uStack_70 >> 0x20);
  uStack_4d4 = CONCAT31(uStack_4d4._1_3_,cVar6);
  uStack_4a = 1;
  FUN_140bfc0d0(&uStack_5f0,uStack_80,&pauStack_568);
  puVar13 = puStack_248;
  pauVar18 = pauStack_250;
  func_0x000142923c00(&pauStack_568,&uStack_240,0x98);
  uStack_4c0 = uStack_5e0;
  uStack_4c8 = CONCAT44(uStack_5e4,uStack_5e8);
  uStack_4d0 = uStack_5f0;
  pauVar15 = (undefined1 (*) [16])*puVar13;
  uVar7 = puVar13[1];
  ppauVar10 = (undefined1 (**) [16])(uVar7 & (ulonglong)pauVar18);
  auVar2 = *(undefined1 (*) [16])(*pauVar15 + (longlong)ppauVar10);
  uVar5 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
          (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3
          | (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
          (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 | (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6
          | (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
          (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9
          | (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
          (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
          (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
          (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
          (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
  uVar11 = (uint)uVar5;
  pauVar17 = pauVar18;
  if (uVar5 == 0) goto code_r0x000140868b49;
  goto code_r0x000140868872;
}

