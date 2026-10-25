
void FUN_14096f090(longlong *param_1,longlong *param_2,ulonglong param_3,undefined4 param_4,
                  longlong *param_5)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  undefined4 uVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  char cVar9;
  undefined8 uVar10;
  ulonglong uVar11;
  undefined *puVar12;
  ulonglong uVar13;
  longlong lVar14;
  uint uVar15;
  ushort uVar16;
  ulonglong uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  int iStack_dc;
  undefined *puStack_d8;
  undefined8 uStack_d0;
  longlong ***ppplStack_c8;
  longlong lStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  longlong *plStack_90;
  longlong **pplStack_88;
  code *pcStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined8 uStack_54;
  
  plStack_90 = param_1;
  cVar9 = FUN_140c10e10(param_4);
  if (cVar9 != '\0') {
code_r0x00014096f5b3:
    pplStack_88 = &plStack_90;
    pcStack_80 = FUN_14096d700;
    puStack_d8 = &UNK_142ab6470;
    uStack_d0 = 2;
    uStack_b8 = 0;
    ppplStack_c8 = &pplStack_88;
    lStack_c0 = 1;
    FUN_1428d9390(&puStack_d8,&UNK_142ab6490);
code_r0x00014096f617:
    FUN_1428d9760(&UNK_142ab5f00,0x2b,&puStack_d8,&UNK_142ab5f30,&UNK_142ab6140);
    pcVar6 = (code *)swi(3);
    (*pcVar6)();
    return;
  }
  lVar18 = param_2[0x27];
  if (*(longlong *)(lVar18 + 0xb0) == 0) {
code_r0x00014096f18e:
    uVar11 = func_0x000140c87650(param_3);
    if (((*(char *)((longlong)param_2 + 0x136) == '\b') || (*(longlong *)(lVar18 + 0xe8) == 0)) ||
       (*(longlong *)(lVar18 + 0x130) == 0)) {
      return;
    }
    lVar5 = *(longlong *)(lVar18 + 0x118);
    bVar19 = (byte)(uVar11 >> 0x39);
    auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
    auVar20 = pshuflw(auVar20,auVar20,0);
    lVar14 = 0;
    uVar17 = uVar11;
    do {
      uVar17 = *(ulonglong *)(lVar18 + 0x120) & uVar17;
      auVar29 = *(undefined1 (*) [16])(lVar5 + uVar17);
      cVar9 = auVar20[0];
      auVar28[0] = -(auVar29[0] == cVar9);
      cVar21 = auVar20[1];
      auVar28[1] = -(auVar29[1] == cVar21);
      cVar22 = auVar20[2];
      auVar28[2] = -(auVar29[2] == cVar22);
      cVar23 = auVar20[3];
      auVar28[3] = -(auVar29[3] == cVar23);
      auVar28[4] = -(auVar29[4] == cVar9);
      auVar28[5] = -(auVar29[5] == cVar21);
      auVar28[6] = -(auVar29[6] == cVar22);
      auVar28[7] = -(auVar29[7] == cVar23);
      auVar28[8] = -(auVar29[8] == cVar9);
      auVar28[9] = -(auVar29[9] == cVar21);
      auVar28[10] = -(auVar29[10] == cVar22);
      auVar28[0xb] = -(auVar29[0xb] == cVar23);
      auVar28[0xc] = -(auVar29[0xc] == cVar9);
      auVar28[0xd] = -(auVar29[0xd] == cVar21);
      auVar28[0xe] = -(auVar29[0xe] == cVar22);
      auVar28[0xf] = -(auVar29[0xf] == cVar23);
      uVar16 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe | (ushort)(auVar28[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar16;
      while (uVar16 != 0) {
        uVar7 = 0;
        for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        uVar13 = uVar7 + uVar17 & *(ulonglong *)(lVar18 + 0x120);
        if (*(ulonglong *)(lVar5 + -0x10 + uVar13 * -0x10) == uVar11) {
          param_3 = (ulonglong)*(uint *)(lVar5 + -8 + uVar13 * -0x10);
          uVar10 = func_0x000140bd5790(*param_1 + 4);
          if (param_3 <= (ulonglong)param_2[0x23]) {
            *(undefined8 *)(param_2[0x22] + param_3) = uVar10;
            return;
          }
code_r0x00014096f598:
          FUN_1428d9830(param_3);
code_r0x00014096f5a7:
          FUN_1428d9830();
          goto code_r0x00014096f5b3;
        }
        uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
      }
      auVar25[0] = -(auVar29[0] == -1);
      auVar25[1] = -(auVar29[1] == -1);
      auVar25[2] = -(auVar29[2] == -1);
      auVar25[3] = -(auVar29[3] == -1);
      auVar25[4] = -(auVar29[4] == -1);
      auVar25[5] = -(auVar29[5] == -1);
      auVar25[6] = -(auVar29[6] == -1);
      auVar25[7] = -(auVar29[7] == -1);
      auVar25[8] = -(auVar29[8] == -1);
      auVar25[9] = -(auVar29[9] == -1);
      auVar25[10] = -(auVar29[10] == -1);
      auVar25[0xb] = -(auVar29[0xb] == -1);
      auVar25[0xc] = -(auVar29[0xc] == -1);
      auVar25[0xd] = -(auVar29[0xd] == -1);
      auVar25[0xe] = -(auVar29[0xe] == -1);
      auVar25[0xf] = -(auVar29[0xf] == -1);
      if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') {
        return;
      }
      uVar17 = uVar17 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    } while( true );
  }
  lVar5 = *(longlong *)(lVar18 + 0x98);
  bVar19 = (byte)(param_3 >> 0x39);
  auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar14 = 0;
  uVar11 = param_3;
  do {
    uVar11 = *(ulonglong *)(lVar18 + 0xa0) & uVar11;
    auVar29 = *(undefined1 (*) [16])(lVar5 + uVar11);
    cVar9 = auVar20[0];
    auVar27[0] = -(auVar29[0] == cVar9);
    cVar21 = auVar20[1];
    auVar27[1] = -(auVar29[1] == cVar21);
    cVar22 = auVar20[2];
    auVar27[2] = -(auVar29[2] == cVar22);
    cVar23 = auVar20[3];
    auVar27[3] = -(auVar29[3] == cVar23);
    auVar27[4] = -(auVar29[4] == cVar9);
    auVar27[5] = -(auVar29[5] == cVar21);
    auVar27[6] = -(auVar29[6] == cVar22);
    auVar27[7] = -(auVar29[7] == cVar23);
    auVar27[8] = -(auVar29[8] == cVar9);
    auVar27[9] = -(auVar29[9] == cVar21);
    auVar27[10] = -(auVar29[10] == cVar22);
    auVar27[0xb] = -(auVar29[0xb] == cVar23);
    auVar27[0xc] = -(auVar29[0xc] == cVar9);
    auVar27[0xd] = -(auVar29[0xd] == cVar21);
    auVar27[0xe] = -(auVar29[0xe] == cVar22);
    auVar27[0xf] = -(auVar29[0xf] == cVar23);
    uVar16 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf;
    uVar15 = (uint)uVar16;
    while (uVar16 != 0) {
      uVar7 = 0;
      for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      uVar17 = uVar7 + uVar11 & *(ulonglong *)(lVar18 + 0xa0);
      if (*(ulonglong *)(lVar5 + -0x10 + uVar17 * -0x10) == param_3) {
        lVar18 = uVar17 * -0x10;
        iStack_dc = *(int *)(lVar5 + -8 + lVar18);
        if (iStack_dc != 0) {
          puStack_d8 = (undefined *)0x0;
          FUN_1428a994f(0,&iStack_dc,&UNK_142ab6240,&puStack_d8,&UNK_142ab63d8);
          goto code_r0x00014096f598;
        }
        uVar4 = *(undefined4 *)*param_1;
        uStack_e0 = *(undefined4 *)(lVar5 + -4 + lVar18);
        uVar10 = func_0x000140bd5790((undefined4 *)*param_1 + 1);
        uVar11 = param_2[2];
        if (0xff < uVar11) goto code_r0x00014096f617;
        cVar9 = FUN_140c10e10(param_4);
        uStack_e4 = uVar4;
        if (cVar9 == '\0') {
          func_0x00014003bb80(param_4);
          FUN_1409dca90(param_2 + 0xc,uVar4,param_4,1);
          pplStack_88 = (longlong **)*param_5;
          pcStack_80 = (code *)param_5[1];
          lStack_78 = param_5[2];
          lStack_70 = param_5[3];
          lStack_68 = param_5[4];
          uStack_54 = *(undefined8 *)((longlong)param_5 + 0x34);
          uStack_58 = (undefined4)((ulonglong)*(undefined8 *)((longlong)param_5 + 0x2c) >> 0x20);
          uStack_60 = (undefined4)param_5[5];
          uStack_5c = (undefined4)((ulonglong)param_5[5] >> 0x20);
          func_0x000140bd6c00(&puStack_d8,&uStack_e4,&pplStack_88);
          lVar18 = param_2[2];
          if (lVar18 == *param_2) {
            puVar12 = &UNK_142ab63f0;
            goto code_r0x00014096f3b0;
          }
        }
        else {
          func_0x00014003bb80(param_4);
          FUN_1409dca90(param_2 + 0xc,uVar4,param_4,0);
          pplStack_88 = (longlong **)*param_5;
          pcStack_80 = (code *)param_5[1];
          lStack_78 = param_5[2];
          lStack_70 = param_5[3];
          lStack_68 = param_5[4];
          uStack_54 = *(undefined8 *)((longlong)param_5 + 0x34);
          uStack_58 = (undefined4)((ulonglong)*(undefined8 *)((longlong)param_5 + 0x2c) >> 0x20);
          uStack_60 = (undefined4)param_5[5];
          uStack_5c = (undefined4)((ulonglong)param_5[5] >> 0x20);
          func_0x000140bd6c30(&puStack_d8,&uStack_e4,&pplStack_88);
          lVar18 = param_2[2];
          if (lVar18 == *param_2) {
            puVar12 = &UNK_142ab6408;
code_r0x00014096f3b0:
            FUN_140b60150(param_2,puVar12);
          }
        }
        uVar4 = uStack_e0;
        lVar5 = param_2[1];
        lVar14 = lVar18 * 0x44;
        *(undefined4 *)(lVar5 + 0x40 + lVar14) = uStack_98;
        puVar2 = (undefined8 *)(lVar5 + 0x30 + lVar14);
        *puVar2 = uStack_a8;
        puVar2[1] = uStack_a0;
        puVar2 = (undefined8 *)(lVar5 + 0x20 + lVar14);
        *puVar2 = uStack_b8;
        puVar2[1] = uStack_b0;
        plVar3 = (longlong *)(lVar5 + 0x10 + lVar14);
        *plVar3 = (longlong)ppplStack_c8;
        plVar3[1] = lStack_c0;
        *(undefined8 *)(lVar5 + lVar14) = puStack_d8;
        ((undefined8 *)(lVar5 + lVar14))[1] = uStack_d0;
        param_2[2] = lVar18 + 1;
        lVar18 = param_2[5];
        if (lVar18 == param_2[3]) {
          FUN_140562e70(param_2 + 3,&UNK_142ab6420);
        }
        lVar5 = param_2[4];
        *(undefined4 *)(lVar5 + lVar18 * 8) = uVar4;
        *(char *)(lVar5 + 4 + lVar18 * 8) = (char)uVar11;
        param_2[5] = lVar18 + 1;
        lVar18 = param_2[0x27];
        uVar11 = func_0x000140c87650(param_3);
        if (*(char *)((longlong)param_2 + 0x136) == '\b') {
          return;
        }
        if (*(longlong *)(lVar18 + 0xe8) == 0) {
          return;
        }
        if (*(longlong *)(lVar18 + 0x130) == 0) {
          return;
        }
        lVar5 = *(longlong *)(lVar18 + 0x118);
        bVar19 = (byte)(uVar11 >> 0x39);
        auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
        auVar20 = pshuflw(auVar20,auVar20,0);
        lVar14 = 0;
        uVar17 = uVar11;
        goto code_r0x00014096f4a5;
      }
      uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
      uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
    }
    auVar24[0] = -(auVar29[0] == -1);
    auVar24[1] = -(auVar29[1] == -1);
    auVar24[2] = -(auVar29[2] == -1);
    auVar24[3] = -(auVar29[3] == -1);
    auVar24[4] = -(auVar29[4] == -1);
    auVar24[5] = -(auVar29[5] == -1);
    auVar24[6] = -(auVar29[6] == -1);
    auVar24[7] = -(auVar29[7] == -1);
    auVar24[8] = -(auVar29[8] == -1);
    auVar24[9] = -(auVar29[9] == -1);
    auVar24[10] = -(auVar29[10] == -1);
    auVar24[0xb] = -(auVar29[0xb] == -1);
    auVar24[0xc] = -(auVar29[0xc] == -1);
    auVar24[0xd] = -(auVar29[0xd] == -1);
    auVar24[0xe] = -(auVar29[0xe] == -1);
    auVar24[0xf] = -(auVar29[0xf] == -1);
    if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') goto code_r0x00014096f18e;
    uVar11 = uVar11 + lVar14 + 0x10;
    lVar14 = lVar14 + 0x10;
  } while( true );
code_r0x00014096f4a5:
  uVar17 = *(ulonglong *)(lVar18 + 0x120) & uVar17;
  pcVar1 = (char *)(lVar5 + uVar17);
  cVar9 = auVar20[0];
  auVar29[0] = -(*pcVar1 == cVar9);
  cVar21 = auVar20[1];
  auVar29[1] = -(pcVar1[1] == cVar21);
  cVar22 = auVar20[2];
  auVar29[2] = -(pcVar1[2] == cVar22);
  cVar23 = auVar20[3];
  auVar29[3] = -(pcVar1[3] == cVar23);
  auVar29[4] = -(pcVar1[4] == cVar9);
  auVar29[5] = -(pcVar1[5] == cVar21);
  auVar29[6] = -(pcVar1[6] == cVar22);
  auVar29[7] = -(pcVar1[7] == cVar23);
  auVar29[8] = -(pcVar1[8] == cVar9);
  auVar29[9] = -(pcVar1[9] == cVar21);
  auVar29[10] = -(pcVar1[10] == cVar22);
  auVar29[0xb] = -(pcVar1[0xb] == cVar23);
  auVar29[0xc] = -(pcVar1[0xc] == cVar9);
  auVar29[0xd] = -(pcVar1[0xd] == cVar21);
  auVar29[0xe] = -(pcVar1[0xe] == cVar22);
  auVar29[0xf] = -(pcVar1[0xf] == cVar23);
  uVar16 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
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
           (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
  uVar15 = (uint)uVar16;
  while (uVar16 != 0) {
    uVar7 = 0;
    for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar7 = uVar7 + 1;
    }
    uVar13 = uVar7 + uVar17 & *(ulonglong *)(lVar18 + 0x120);
    if (*(ulonglong *)(lVar5 + -0x10 + uVar13 * -0x10) == uVar11) {
      uVar11 = (ulonglong)*(uint *)(lVar5 + -8 + uVar13 * -0x10);
      if (uVar11 <= (ulonglong)param_2[0x23]) {
        *(undefined8 *)(param_2[0x22] + uVar11) = uVar10;
        return;
      }
      goto code_r0x00014096f5a7;
    }
    uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
    uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
  }
  auVar26[0] = -(*pcVar1 == -1);
  auVar26[1] = -(pcVar1[1] == -1);
  auVar26[2] = -(pcVar1[2] == -1);
  auVar26[3] = -(pcVar1[3] == -1);
  auVar26[4] = -(pcVar1[4] == -1);
  auVar26[5] = -(pcVar1[5] == -1);
  auVar26[6] = -(pcVar1[6] == -1);
  auVar26[7] = -(pcVar1[7] == -1);
  auVar26[8] = -(pcVar1[8] == -1);
  auVar26[9] = -(pcVar1[9] == -1);
  auVar26[10] = -(pcVar1[10] == -1);
  auVar26[0xb] = -(pcVar1[0xb] == -1);
  auVar26[0xc] = -(pcVar1[0xc] == -1);
  auVar26[0xd] = -(pcVar1[0xd] == -1);
  auVar26[0xe] = -(pcVar1[0xe] == -1);
  auVar26[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar26 >> 0x37,0) & 1) != 0) || (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar26 >> 0x47,0) & 1) != 0) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') {
    return;
  }
  uVar17 = uVar17 + lVar14 + 0x10;
  lVar14 = lVar14 + 0x10;
  goto code_r0x00014096f4a5;
}

