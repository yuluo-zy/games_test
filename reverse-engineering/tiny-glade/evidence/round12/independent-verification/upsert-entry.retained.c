uint * FUN_140b40b50(uint *param_1,undefined8 *param_2,longlong param_3,byte *param_4,
                    undefined8 *param_5)
{
  undefined1 (*pauVar1) [16];
  ulonglong uVar2;
  code *pcVar3;
  ushort uVar4;
  longlong *plVar5;
  undefined8 uVar6;
  char cVar7;
  uint uVar8;
  uint uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  byte bVar12;
  int iVar13;
  longlong lVar14;
  uint uVar15;
  longlong *plVar16;
  uint uVar17;
  byte bVar18;
  ulonglong uVar19;
  longlong lVar20;
  undefined1 auVar21 [16];
  undefined1 auStack_2c8 [4];
  longlong lStack_2c4;
  undefined8 uStack_2bc;
  undefined8 uStack_2a4;
  undefined8 uStack_29c;
  undefined8 uStack_218;
  undefined8 uStack_210;
  longlong lStack_208;
  undefined8 uStack_200;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  float fStack_1c0;
  float fStack_1bc;
  float fStack_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  float fStack_1a8;
  undefined2 uStack_194;
  undefined1 uStack_192;
  undefined1 uStack_189;
  undefined8 uStack_188;
  undefined4 uStack_180;
  float fStack_178;
  undefined8 uStack_174;
  undefined4 uStack_16c;
  float fStack_168;
  float fStack_164;
  float fStack_160;
  float fStack_15c;
  longlong lStack_158;
  undefined8 uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  ulonglong uStack_138;
  undefined8 uStack_130;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  float fStack_110;
  float fStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  float fStack_f8;
  undefined2 uStack_e4;
  undefined1 uStack_e2;
  undefined1 uStack_d9;
  undefined4 uStack_9c;
  byte *pbStack_98;
  undefined2 uStack_8c;
  undefined1 uStack_8a;
  undefined2 uStack_89;
  undefined1 uStack_87;
  byte bStack_86;
  undefined1 uStack_85;
  undefined1 uStack_84;
  undefined1 uStack_83;
  undefined1 uStack_82;
  undefined1 uStack_81;
  undefined8 uStack_80;
  uStack_80 = 0xfffffffffffffffe;
  bVar18 = (&UNK_142aef03c)[*param_4];
  pbStack_98 = param_4 + 0x18;
  lVar20 = *(longlong *)(param_4 + 0x18);
  uStack_180 = *(undefined4 *)(param_5 + 1);
  uStack_188 = *param_5;
  uStack_218 = param_5[2];
  uStack_210 = param_5[3];
  uStack_83 = 1;
  func_0x000140c91b60(&lStack_158,&uStack_188,&uStack_218);
  uStack_83 = 1;
  FUN_140c91b80(&fStack_178,lVar20 + 0x60,&lStack_158);
  plVar16 = *(longlong **)(lVar20 + 0x30);
  uVar11 = *(undefined8 *)(lVar20 + 0x38);
  LOCK();
  lVar20 = *plVar16;
  *plVar16 = *plVar16 + 1;
  UNLOCK();
  if (*plVar16 == 0 || SCARRY8(lVar20,1) != *plVar16 < 0) {
code_r0x000140b41484:
                    /* WARNING: Does not return */
    pcVar3 = (code *)invalidInstructionException();
    (*pcVar3)();
  }
  uStack_83 = 1;
  func_0x000140b92fa0(&lStack_158,plVar16,uVar11);
  uStack_e2 = 1;
  lStack_2c4 = lStack_158;
  uStack_2bc = uStack_150;
  uStack_2a4 = uStack_138;
  uVar11 = uStack_2a4;
  uStack_29c = uStack_130;
  uVar6 = uStack_29c;
  uStack_1c8 = uStack_118;
  uStack_1c4 = uStack_114;
  uStack_194 = uStack_e4;
  uStack_192 = 1;
  uStack_189 = uStack_d9;
  uStack_2a4._0_4_ = (undefined4)uStack_138;
  uStack_2a4._4_4_ = (undefined4)(uStack_138 >> 0x20);
  uStack_29c._0_4_ = (undefined4)uStack_130;
  uStack_29c._4_4_ = (undefined4)((ulonglong)uStack_130 >> 0x20);
  lStack_208 = lStack_158;
  uStack_200 = uStack_150;
  uStack_1f8 = uStack_148;
  uStack_1f4 = uStack_144;
  uStack_1f0 = uStack_140;
  uStack_1ec = uStack_13c;
  uStack_1e8 = (undefined4)uStack_2a4;
  uStack_1e4 = uStack_2a4._4_4_;
  uStack_1e0 = (undefined4)uStack_29c;
  uStack_1dc = uStack_29c._4_4_;
  uStack_1d8 = uStack_128;
  uStack_1d4 = uStack_124;
  uStack_1d0 = uStack_120;
  uStack_1cc = uStack_11c;
  fStack_1c0 = fStack_168;
  fStack_1bc = fStack_164;
  fStack_1b8 = fStack_160;
  fStack_1b4 = fStack_15c;
  fStack_1a8 = uStack_174._4_4_;
  fStack_1b0 = fStack_178;
  fStack_1ac = (float)uStack_174;
  puVar10 = (undefined8 *)*param_2;
  uStack_2a4 = uVar11;
  uStack_29c = uVar6;
  bStack_86 = bVar18;
  if (puVar10 == (undefined8 *)0x0) {
    lVar20 = param_2[1];
    uVar8 = *(uint *)(lVar20 + -0x10);
    uVar17 = *(uint *)(lVar20 + -0xc);
    lVar14 = param_3 + (ulonglong)*(byte *)(lVar20 + -8) * 0x390 + 0x20;
    uStack_81 = 1;
    uStack_82 = 1;
    puVar10 = (undefined8 *)func_0x000140b93940(lVar14,uVar8,uVar17);
    if (puVar10 == (undefined8 *)0x0) {
      uStack_81 = 1;
      uStack_82 = 1;
      FUN_1428d9330(&UNK_142aa3d50,0x42,&UNK_142aa3d98);
      goto code_r0x000140b41484;
    }
    if ((*(float *)((longlong)puVar10 + 0x44) == fStack_178) &&
       (!NAN(*(float *)((longlong)puVar10 + 0x44)) && !NAN(fStack_178))) {
      if ((*(float *)(puVar10 + 9) == (float)uStack_174) &&
         (!NAN(*(float *)(puVar10 + 9)) && !NAN((float)uStack_174))) {
        if (((*(float *)((longlong)puVar10 + 0x4c) == uStack_174._4_4_) &&
            (!NAN(*(float *)((longlong)puVar10 + 0x4c)) && !NAN(uStack_174._4_4_))) &&
           (auVar21._4_4_ = -(uint)(*(float *)(puVar10 + 7) == fStack_164),
           auVar21._0_4_ = -(uint)(*(float *)((longlong)puVar10 + 0x34) == fStack_168),
           auVar21._8_4_ = -(uint)(*(float *)((longlong)puVar10 + 0x3c) == fStack_160),
           auVar21._12_4_ = -(uint)(*(float *)(puVar10 + 8) == fStack_15c),
           iVar13 = movmskps((int)lVar14,auVar21), iVar13 == 0xf)) {
          uStack_81 = 1;
          uStack_82 = 1;
          uVar11 = FUN_1408cdbd0(param_3,(uint)((ulonglong)*puVar10 >> 0x20) | 1);
          uStack_81 = 1;
          uStack_82 = 1;
          cVar7 = func_0x0001408cea30(uVar11,param_4);
          if (cVar7 != '\0') {
            param_1[2] = *(uint *)(lVar20 + -8);
            *(undefined8 *)param_1 = *(undefined8 *)(lVar20 + -0x10);
            param_1[3] = 2;
            *(undefined8 *)(param_1 + 4) = *(undefined8 *)(lVar20 + -0x10);
            uStack_83 = 1;
            FUN_1405f9e40(&lStack_208);
            plVar16 = *(longlong **)pbStack_98;
            LOCK();
            *plVar16 = *plVar16 + -1;
            UNLOCK();
            if (*plVar16 != 0) {
              return param_1;
            }
            FUN_140b5c620();
            return param_1;
          }
        }
      }
    }
    uStack_81 = 1;
    uStack_82 = 1;
    FUN_1408d9590(&lStack_158,param_3,lVar20 + -0x18);
    plVar16 = (longlong *)CONCAT44(uStack_11c,uStack_120);
    LOCK();
    *plVar16 = *plVar16 + -1;
    UNLOCK();
    if (*plVar16 == 0) {
      uStack_81 = 1;
      uStack_82 = 1;
      FUN_140b5c620(&uStack_120);
    }
    lStack_158 = *(longlong *)param_4;
    uStack_150 = *(undefined8 *)(param_4 + 8);
    uStack_148 = *(undefined4 *)(param_4 + 0x10);
    uStack_144 = *(undefined4 *)(param_4 + 0x14);
    uStack_140 = *(undefined4 *)(param_4 + 0x18);
    uStack_13c = *(undefined4 *)(param_4 + 0x1c);
    uStack_81 = 1;
    uStack_82 = 0;
    plVar16 = &lStack_158;
    uVar9 = FUN_141558ea0(param_3);
    *(uint *)(lVar20 + -0x18) = uVar9;
    *(int *)(lVar20 + -0x14) = (int)plVar16;
    uStack_148 = uStack_1f8;
    uStack_144 = uStack_1f4;
    uStack_140 = uStack_1f0;
    uStack_13c = uStack_1ec;
    lStack_158 = lStack_208;
    uStack_150 = uStack_200;
    uStack_128 = uStack_1d8;
    uStack_124 = uStack_1d4;
    uStack_120 = uStack_1d0;
    uStack_11c = uStack_1cc;
    uStack_118 = uStack_1c8;
    uStack_114 = uStack_1c4;
    fStack_110 = fStack_1c0;
    fStack_10c = fStack_1bc;
    uStack_108 = CONCAT44(fStack_1b4,fStack_1b8);
    uStack_100 = CONCAT44(fStack_1ac,fStack_1b0);
    fStack_f8 = fStack_1a8;
    uStack_e4 = uStack_194;
    uStack_e2 = uStack_192;
    uStack_d9 = uStack_189;
    uStack_138 = (ulonglong)plVar16 & 0xffffffff | (ulonglong)uVar9 << 0x20;
    uStack_130 = 0;
    FUN_140b93000(auStack_2c8,&lStack_158);
    plVar16 = (longlong *)CONCAT44(uStack_144,uStack_148);
    LOCK();
    *plVar16 = *plVar16 + -1;
    UNLOCK();
    if (*plVar16 == 0) {
      FUN_14064f9d0(&uStack_148);
    }
    if (1 < (uint)(float)lStack_158) {
      func_0x000140613c20(uStack_150,0x2c,4);
    }
    lVar14 = (ulonglong)bVar18 * 0x390;
    *(undefined1 *)(param_3 + 0x3a8 + lVar14) = 1;
    uStack_84 = 1;
    FUN_140b92f60(&lStack_158,auStack_2c8);
    lVar14 = param_3 + 0x20 + lVar14;
    fStack_178 = (float)lStack_158;
    uStack_174 = uStack_150;
    uStack_16c = uStack_144;
    uStack_84 = 1;
    FUN_140ade8e0(&fStack_178,lVar14 + 0x188);
    plVar16 = &lStack_158;
    func_0x000142923c00(plVar16,auStack_2c8,0xb0);
    uStack_84 = 0;
    uVar9 = FUN_1415529b0(lVar14);
    uVar15 = (uint)plVar16;
    *(uint *)(lVar20 + -0x10) = uVar9;
    *(uint *)(lVar20 + -0xc) = uVar15;
    *(byte *)(lVar20 + -8) = bStack_86;
    *(undefined1 *)(lVar20 + -5) = uStack_87;
    *(undefined2 *)(lVar20 + -7) = uStack_89;
    uStack_150 = CONCAT17(uStack_87,CONCAT25(uStack_89,CONCAT14(bStack_86,uVar15)));
    lStack_158 = (ulonglong)uVar9 << 0x20;
    uStack_84 = 0;
    FUN_141453c00(param_3 + 0x3cb0,&lStack_158);
    *param_1 = uVar9;
    param_1[1] = uVar15;
    *(byte *)(param_1 + 2) = bStack_86;
    *(undefined2 *)((longlong)param_1 + 9) = uStack_89;
    *(undefined1 *)((longlong)param_1 + 0xb) = uStack_87;
    param_1[3] = 1;
    param_1[4] = uVar8;
    param_1[5] = uVar17;
  }
  else {
    lStack_158 = *(longlong *)param_4;
    uStack_150 = *(undefined8 *)(param_4 + 8);
    uStack_148 = *(undefined4 *)(param_4 + 0x10);
    uStack_144 = *(undefined4 *)(param_4 + 0x14);
    uStack_140 = *(undefined4 *)(param_4 + 0x18);
    uStack_13c = *(undefined4 *)(param_4 + 0x1c);
    uStack_81 = 1;
    uStack_82 = 0;
    plVar16 = &lStack_158;
    uVar8 = FUN_141558ea0(param_3);
    uStack_148 = uStack_1f8;
    uStack_144 = uStack_1f4;
    uStack_140 = uStack_1f0;
    uStack_13c = uStack_1ec;
    lStack_158 = lStack_208;
    uStack_150 = uStack_200;
    uStack_128 = uStack_1d8;
    uStack_124 = uStack_1d4;
    uStack_120 = uStack_1d0;
    uStack_11c = uStack_1cc;
    uStack_118 = uStack_1c8;
    uStack_114 = uStack_1c4;
    fStack_110 = fStack_1c0;
    fStack_10c = fStack_1bc;
    uStack_108 = CONCAT44(fStack_1b4,fStack_1b8);
    uStack_100 = CONCAT44(fStack_1ac,fStack_1b0);
    fStack_f8 = fStack_1a8;
    uStack_e4 = uStack_194;
    uStack_e2 = uStack_192;
    uStack_d9 = uStack_189;
    uStack_138 = (ulonglong)plVar16 & 0xffffffff | (ulonglong)uVar8 << 0x20;
    uStack_130 = 0;
    FUN_140b93000(auStack_2c8,&lStack_158);
    plVar5 = (longlong *)CONCAT44(uStack_144,uStack_148);
    LOCK();
    *plVar5 = *plVar5 + -1;
    UNLOCK();
    if (*plVar5 == 0) {
      FUN_14064f9d0(&uStack_148);
    }
    uStack_9c = (int)plVar16;
    if (1 < (uint)(float)lStack_158) {
      func_0x000140613c20(uStack_150,0x2c,4);
    }
    lVar20 = (ulonglong)bVar18 * 0x390;
    *(undefined1 *)(param_3 + 0x3a8 + lVar20) = 1;
    uStack_85 = 1;
    FUN_140b92f60(&lStack_158,auStack_2c8);
    lVar20 = lVar20 + param_3;
    fStack_178 = (float)lStack_158;
    uStack_174 = uStack_150;
    uStack_16c = uStack_144;
    uStack_85 = 1;
    FUN_140ade8e0(&fStack_178,lVar20 + 0x1a8);
    plVar16 = &lStack_158;
    func_0x000142923c00(plVar16,auStack_2c8,0xb0);
    uStack_85 = 0;
    uVar9 = FUN_1415529b0(lVar20 + 0x20);
    pauVar1 = (undefined1 (*) [16])*puVar10;
    uVar2 = puVar10[1];
    uVar19 = uVar2 & param_2[2];
    auVar21 = *(undefined1 (*) [16])(*pauVar1 + uVar19);
    uVar4 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
            (ushort)(byte)(auVar21[0xf] >> 7) << 0xf;
    uVar17 = (uint)uVar4;
    if (uVar4 == 0) {
      lVar20 = 0x10;
      do {
        uVar19 = uVar19 + lVar20 & uVar2;
        auVar21 = *(undefined1 (*) [16])(*pauVar1 + uVar19);
        lVar20 = lVar20 + 0x10;
        uVar17 = (uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar21[0xf] >> 7) << 0xf);
      } while (uVar17 == 0);
    }
    uVar11 = param_2[1];
    uVar15 = 0;
    for (; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
      uVar15 = uVar15 + 1;
    }
    uVar19 = uVar15 + uVar19 & uVar2;
    bVar18 = (*pauVar1)[uVar19];
    if (-1 < (char)bVar18) {
      auVar21 = *pauVar1;
      uVar17 = 0;
      for (uVar15 = (uint)(ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar21[0xf] >> 7) << 0xf); (uVar15 & 1) == 0;
          uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar17 = uVar17 + 1;
      }
      uVar19 = (ulonglong)uVar17;
      bVar18 = (*pauVar1)[uVar19];
    }
    bVar12 = (byte)((ulonglong)param_2[2] >> 0x39);
    (*pauVar1)[uVar19] = bVar12;
    pauVar1[1][uVar19 - 0x10 & uVar2] = bVar12;
    puVar10[2] = puVar10[2] - (ulonglong)(bVar18 & 1);
    *(undefined8 *)pauVar1[uVar19 * -2 + -2] = uVar11;
    *(uint *)(pauVar1[uVar19 * -2 + -2] + 8) = uVar8;
    *(undefined4 *)(pauVar1[uVar19 * -2 + -2] + 0xc) = uStack_9c;
    *(uint *)pauVar1[uVar19 * -2 + -1] = uVar9;
    uVar8 = (uint)plVar16;
    *(uint *)(pauVar1[uVar19 * -2 + -1] + 4) = uVar8;
    pauVar1[uVar19 * -2 + -1][8] = bStack_86;
    pauVar1[uVar19 * -2 + -1][0xb] = uStack_8a;
    *(undefined2 *)(pauVar1[uVar19 * -2 + -1] + 9) = uStack_8c;
    puVar10[3] = puVar10[3] + 1;
    uStack_150 = CONCAT17(uStack_8a,CONCAT25(uStack_8c,CONCAT14(bStack_86,uVar8)));
    lStack_158 = (ulonglong)uVar9 << 0x20;
    uStack_85 = 0;
    FUN_141453c00(param_3 + 0x3cb0,&lStack_158);
    *param_1 = uVar9;
    param_1[1] = uVar8;
    *(byte *)(param_1 + 2) = bStack_86;
    *(undefined2 *)((longlong)param_1 + 9) = uStack_8c;
    *(undefined1 *)((longlong)param_1 + 0xb) = uStack_8a;
    param_1[3] = 0;
  }
  return param_1;
}