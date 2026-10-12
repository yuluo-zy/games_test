
undefined8 *
FUN_14096c310(undefined8 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             longlong *param_5)

{
  undefined1 auVar1 [16];
  undefined8 uVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  longlong lVar9;
  uint uVar10;
  ushort uVar11;
  longlong lVar12;
  longlong lVar13;
  byte bVar14;
  char cVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  longlong lStack_1b8;
  longlong lStack_1b0;
  int iStack_1a8;
  undefined4 uStack_1a4;
  undefined8 uStack_1a0;
  undefined6 uStack_190;
  undefined8 uStack_18a;
  char cStack_182;
  undefined1 uStack_181;
  undefined1 auStack_180 [54];
  char cStack_14a;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  longlong *plStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined1 uStack_92;
  longlong *plStack_90;
  code *pcStack_88;
  undefined6 uStack_78;
  undefined2 uStack_72;
  undefined6 uStack_70;
  char cStack_6a;
  undefined1 uStack_69;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_68 = func_0x000140c0f640(param_3,param_4);
  if (*(longlong *)(param_2 + 200) != 0) {
    uVar6 = FUN_1412e8120(param_2 + 0xd0,&lStack_68);
    lVar13 = *(longlong *)(param_2 + 0xb0);
    bVar14 = (byte)(uVar6 >> 0x39);
    auVar16 = ZEXT216(CONCAT11(bVar14,bVar14));
    auVar16 = pshuflw(auVar16,auVar16,0);
    lVar9 = 0;
    while( true ) {
      uVar6 = *(ulonglong *)(param_2 + 0xb8) & uVar6;
      auVar1 = *(undefined1 (*) [16])(lVar13 + uVar6);
      cVar15 = auVar16[0];
      auVar21[0] = -(auVar1[0] == cVar15);
      cVar17 = auVar16[1];
      auVar21[1] = -(auVar1[1] == cVar17);
      cVar18 = auVar16[2];
      auVar21[2] = -(auVar1[2] == cVar18);
      cVar19 = auVar16[3];
      auVar21[3] = -(auVar1[3] == cVar19);
      auVar21[4] = -(auVar1[4] == cVar15);
      auVar21[5] = -(auVar1[5] == cVar17);
      auVar21[6] = -(auVar1[6] == cVar18);
      auVar21[7] = -(auVar1[7] == cVar19);
      auVar21[8] = -(auVar1[8] == cVar15);
      auVar21[9] = -(auVar1[9] == cVar17);
      auVar21[10] = -(auVar1[10] == cVar18);
      auVar21[0xb] = -(auVar1[0xb] == cVar19);
      auVar21[0xc] = -(auVar1[0xc] == cVar15);
      auVar21[0xd] = -(auVar1[0xd] == cVar17);
      auVar21[0xe] = -(auVar1[0xe] == cVar18);
      auVar21[0xf] = -(auVar1[0xf] == cVar19);
      uVar11 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar10; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar12 = (uVar4 + uVar6 & *(ulonglong *)(param_2 + 0xb8)) * -0x68;
        if (lStack_68 == *(longlong *)(lVar13 + -0x68 + lVar12)) {
          if (*(int *)(lVar13 + -0x60 + lVar12) == 2) {
            lVar13 = lVar13 + lVar12;
            if (*(char *)(lVar13 + -2) == '\a') {
              FUN_140bd4fc0(&uStack_108,lVar13 + -0x38,param_2,*(undefined1 *)(lVar13 + -0x5c));
              uStack_130 = (undefined4)uStack_108;
              uStack_12c = (undefined4)((ulonglong)uStack_108 >> 0x20);
              uStack_128 = (undefined4)uStack_100;
              uStack_124 = (undefined4)((ulonglong)uStack_100 >> 0x20);
              *(undefined4 *)(lVar13 + -0x60) = 1;
              *(ulonglong *)(lVar13 + -0x5c) = CONCAT44(uStack_130,uStack_134);
              *(ulonglong *)(lVar13 + -0x54) = CONCAT44(uStack_128,uStack_12c);
              *(undefined4 *)(lVar13 + -0x4c) = uStack_124;
              *(undefined4 *)(lVar13 + -0x48) = uStack_f8;
              param_1[2] = CONCAT44(uStack_f4,uStack_f8);
              *param_1 = uStack_108;
              param_1[1] = uStack_100;
              *(undefined1 *)(param_1 + 3) = 1;
              return param_1;
            }
            uStack_f0 = 0x42ab5d30;
            uStack_e8 = 2;
          }
          else {
            uStack_f0 = 0x42ab5d78;
            uStack_e8 = 1;
          }
          uStack_e4 = 0;
          uStack_ec = 1;
          pcStack_88 = FUN_140c0f770;
          uStack_e0 = &plStack_90;
          uStack_d8 = 1;
          uStack_d0 = 0;
          plStack_90 = &lStack_68;
          FUN_140d79f90(&iStack_1a8,&uStack_f0);
          uVar7 = FUN_1428a83e0(&iStack_1a8);
          *param_1 = uVar7;
          *(undefined1 *)(param_1 + 3) = 2;
          return param_1;
        }
        uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
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
      if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar20 >> 0x3f,0) & 1) != 0) || (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar20 >> 0x5f,0) & 1) != 0) || (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0) ||
          auVar20[0xf] < '\0') break;
      uVar6 = uVar6 + lVar9 + 0x10;
      lVar9 = lVar9 + 0x10;
    }
  }
  lVar9 = lStack_68;
  lVar13 = *param_5;
  if (lVar13 < 0) {
    lStack_58 = 0;
code_r0x00014096c76c:
    FUN_1428d8fe3(lStack_58,lVar13,&UNK_142ab5d88);
    pcVar3 = (code *)swi(3);
    puVar8 = (undefined8 *)(*pcVar3)();
    return puVar8;
  }
  uVar7 = *(undefined8 *)(param_2 + 0xa8);
  uVar2 = *(undefined8 *)(lStack_68 + -8);
  if (lVar13 == 0) {
    lStack_60 = 1;
  }
  else {
    lStack_58 = 1;
    lStack_60 = func_0x000140613c40(lVar13,1);
    if (lStack_60 == 0) goto code_r0x00014096c76c;
  }
  lStack_1b8 = *param_5;
  lStack_1b0 = param_5[1];
  lStack_58 = lVar13;
  FUN_1416a2dc0(&uStack_f0,uVar7,lVar9,uVar2,&lStack_1b8,lStack_60,lVar13);
  FUN_1415a5210(&iStack_1a8,&uStack_f0,param_5);
  plStack_90 = (longlong *)CONCAT44(uStack_1a4,iStack_1a8);
  if (cStack_182 == '\x05') {
    *param_1 = plStack_90;
    *(undefined1 *)(param_1 + 3) = 2;
    if (lStack_58 != 0) {
      func_0x000140613c20(lStack_60,lStack_58,1);
    }
  }
  else {
    uStack_78 = uStack_190;
    uStack_72 = (undefined2)uStack_18a;
    uStack_70 = (undefined6)((ulonglong)uStack_18a >> 0x10);
    pcStack_88 = (code *)uStack_1a0;
    cStack_6a = cStack_182;
    uStack_69 = uStack_181;
    if (lStack_58 != 0) {
      func_0x000140613c20(lStack_60,lStack_58,1);
    }
    uStack_49 = 1;
    FUN_140bd4fc0(&uStack_120,&plStack_90,param_2,0);
    uStack_a8 = CONCAT17(uStack_69,CONCAT16(cStack_6a,uStack_70));
    uStack_b0 = CONCAT26(uStack_72,uStack_78);
    plStack_c8 = plStack_90;
    uStack_c0 = pcStack_88;
    uStack_144 = (undefined4)uStack_120;
    uStack_140 = (undefined4)((ulonglong)uStack_120 >> 0x20);
    uStack_13c = (undefined4)uStack_118;
    uStack_138 = (undefined4)((ulonglong)uStack_118 >> 0x20);
    uStack_92 = 7;
    uStack_f0 = 1;
    uStack_e8 = uStack_144;
    uStack_e4 = uStack_140;
    uStack_e0._0_4_ = uStack_13c;
    uStack_e0._4_4_ = uStack_138;
    uStack_d8 = CONCAT44(uStack_d8._4_4_,uStack_110);
    uStack_49 = 0;
    FUN_14152ac30(&iStack_1a8,param_2 + 0xb0,lStack_68,&uStack_f0);
    if (iStack_1a8 != 3) {
      if (cStack_14a != '\a') {
        func_0x00014003bb80(auStack_180);
      }
      uStack_49 = 0;
      FUN_140be24b0(auStack_180);
    }
    param_1[2] = CONCAT44(uStack_10c,uStack_110);
    *param_1 = uStack_120;
    param_1[1] = uStack_118;
    *(undefined1 *)(param_1 + 3) = 0;
  }
  return param_1;
}

