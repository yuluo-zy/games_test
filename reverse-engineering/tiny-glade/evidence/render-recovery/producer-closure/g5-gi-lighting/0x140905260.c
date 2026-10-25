
void FUN_140905260(undefined1 *param_1,longlong *param_2,ulonglong param_3,undefined4 param_4,
                  undefined8 *param_5)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  undefined4 uVar7;
  char cVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  undefined *puVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  ushort uVar15;
  ulonglong uVar16;
  longlong lVar17;
  byte bVar18;
  char cVar20;
  char cVar21;
  char cVar22;
  undefined1 auVar19 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  int iStack_d4;
  undefined8 uStack_d0;
  ulonglong uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined8 uStack_54;
  
  FUN_140c10e10(param_4);
  lVar17 = param_2[0x27];
  if (*(longlong *)(lVar17 + 0xb0) == 0) {
code_r0x00014090533e:
    uVar9 = func_0x000140c87650(param_3);
    if (*(char *)((longlong)param_2 + 0x136) != '\b') {
      FUN_14096bf50(param_2 + 0x22,lVar17 + 0xe8,uVar9,param_1,param_2 + 0x29);
    }
    return;
  }
  lVar3 = *(longlong *)(lVar17 + 0x98);
  bVar18 = (byte)(param_3 >> 0x39);
  auVar19 = ZEXT216(CONCAT11(bVar18,bVar18));
  auVar19 = pshuflw(auVar19,auVar19,0);
  lVar13 = 0;
  uVar10 = param_3;
  do {
    uVar10 = *(ulonglong *)(lVar17 + 0xa0) & uVar10;
    auVar26 = *(undefined1 (*) [16])(lVar3 + uVar10);
    cVar8 = auVar19[0];
    auVar25[0] = -(auVar26[0] == cVar8);
    cVar20 = auVar19[1];
    auVar25[1] = -(auVar26[1] == cVar20);
    cVar21 = auVar19[2];
    auVar25[2] = -(auVar26[2] == cVar21);
    cVar22 = auVar19[3];
    auVar25[3] = -(auVar26[3] == cVar22);
    auVar25[4] = -(auVar26[4] == cVar8);
    auVar25[5] = -(auVar26[5] == cVar20);
    auVar25[6] = -(auVar26[6] == cVar21);
    auVar25[7] = -(auVar26[7] == cVar22);
    auVar25[8] = -(auVar26[8] == cVar8);
    auVar25[9] = -(auVar26[9] == cVar20);
    auVar25[10] = -(auVar26[10] == cVar21);
    auVar25[0xb] = -(auVar26[0xb] == cVar22);
    auVar25[0xc] = -(auVar26[0xc] == cVar8);
    auVar25[0xd] = -(auVar26[0xd] == cVar20);
    auVar25[0xe] = -(auVar26[0xe] == cVar21);
    auVar25[0xf] = -(auVar26[0xf] == cVar22);
    uVar15 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
    uVar14 = (uint)uVar15;
    while (uVar15 != 0) {
      uVar5 = 0;
      for (uVar6 = uVar14; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      uVar16 = uVar5 + uVar10 & *(ulonglong *)(lVar17 + 0xa0);
      if (*(ulonglong *)(lVar3 + -0x10 + uVar16 * -0x10) == param_3) {
        lVar17 = uVar16 * -0x10;
        iStack_d4 = *(int *)(lVar3 + -8 + lVar17);
        if (iStack_d4 != 0) {
          uStack_d0 = 0;
          FUN_1428a994f(0,&iStack_d4,&UNK_142aaa718,&uStack_d0,&UNK_142aaa760);
          goto code_r0x00014090569a;
        }
        uStack_d8 = *(undefined4 *)(lVar3 + -4 + lVar17);
        FUN_14093cf30(&uStack_d0,param_2[0x2a],*param_1);
        uVar9 = func_0x000140bd5790((longlong)&uStack_d0 + 4);
        uVar16 = uStack_d0;
        uVar10 = param_2[2];
        if (0xff < uVar10) goto code_r0x0001409056a6;
        uVar7 = (undefined4)uStack_d0;
        cVar8 = FUN_140c10e10(param_4);
        if (cVar8 == '\0') {
          func_0x00014003bb80(param_4);
          FUN_1409dca90(param_2 + 0xc,uVar16 & 0xffffffff,param_4,1);
          uStack_dc = uVar7;
          uStack_88 = *param_5;
          uStack_80 = param_5[1];
          uStack_78 = param_5[2];
          uStack_70 = param_5[3];
          uStack_68 = param_5[4];
          uStack_54 = *(undefined8 *)((longlong)param_5 + 0x34);
          uStack_58 = (undefined4)((ulonglong)*(undefined8 *)((longlong)param_5 + 0x2c) >> 0x20);
          uStack_60 = (undefined4)param_5[5];
          uStack_5c = (undefined4)((ulonglong)param_5[5] >> 0x20);
          func_0x000140bd6c00(&uStack_d0,&uStack_dc,&uStack_88);
          lVar17 = param_2[2];
          if (lVar17 == *param_2) {
            puVar11 = &UNK_142aaa778;
            goto code_r0x0001409054e5;
          }
        }
        else {
          func_0x00014003bb80(param_4);
          FUN_1409dca90(param_2 + 0xc,uVar16 & 0xffffffff,param_4,0);
          uStack_dc = uVar7;
          uStack_88 = *param_5;
          uStack_80 = param_5[1];
          uStack_78 = param_5[2];
          uStack_70 = param_5[3];
          uStack_68 = param_5[4];
          uStack_54 = *(undefined8 *)((longlong)param_5 + 0x34);
          uStack_58 = (undefined4)((ulonglong)*(undefined8 *)((longlong)param_5 + 0x2c) >> 0x20);
          uStack_60 = (undefined4)param_5[5];
          uStack_5c = (undefined4)((ulonglong)param_5[5] >> 0x20);
          func_0x000140bd6c30(&uStack_d0,&uStack_dc,&uStack_88);
          lVar17 = param_2[2];
          if (lVar17 == *param_2) {
            puVar11 = &UNK_142aaa790;
code_r0x0001409054e5:
            FUN_140b60150(param_2,puVar11);
          }
        }
        uVar7 = uStack_d8;
        lVar3 = param_2[1];
        lVar13 = lVar17 * 0x44;
        *(undefined4 *)(lVar3 + 0x40 + lVar13) = uStack_90;
        puVar2 = (undefined8 *)(lVar3 + 0x30 + lVar13);
        *puVar2 = uStack_a0;
        puVar2[1] = uStack_98;
        puVar2 = (undefined8 *)(lVar3 + 0x20 + lVar13);
        *puVar2 = uStack_b0;
        puVar2[1] = uStack_a8;
        puVar2 = (undefined8 *)(lVar3 + 0x10 + lVar13);
        *puVar2 = uStack_c0;
        puVar2[1] = uStack_b8;
        *(ulonglong *)(lVar3 + lVar13) = uStack_d0;
        ((ulonglong *)(lVar3 + lVar13))[1] = uStack_c8;
        param_2[2] = lVar17 + 1;
        lVar17 = param_2[5];
        if (lVar17 == param_2[3]) {
          FUN_140562e70(param_2 + 3,&UNK_142aaa7a8);
        }
        lVar3 = param_2[4];
        *(undefined4 *)(lVar3 + lVar17 * 8) = uVar7;
        *(char *)(lVar3 + 4 + lVar17 * 8) = (char)uVar10;
        param_2[5] = lVar17 + 1;
        lVar17 = param_2[0x27];
        uVar10 = func_0x000140c87650(param_3);
        if (*(char *)((longlong)param_2 + 0x136) == '\b') {
          return;
        }
        if (*(longlong *)(lVar17 + 0xe8) == 0) {
          return;
        }
        if (*(longlong *)(lVar17 + 0x130) == 0) {
          return;
        }
        lVar3 = *(longlong *)(lVar17 + 0x118);
        bVar18 = (byte)(uVar10 >> 0x39);
        auVar19 = ZEXT216(CONCAT11(bVar18,bVar18));
        auVar19 = pshuflw(auVar19,auVar19,0);
        lVar13 = 0;
        uVar16 = uVar10;
        do {
          uVar16 = *(ulonglong *)(lVar17 + 0x120) & uVar16;
          pcVar1 = (char *)(lVar3 + uVar16);
          cVar8 = auVar19[0];
          auVar26[0] = -(*pcVar1 == cVar8);
          cVar20 = auVar19[1];
          auVar26[1] = -(pcVar1[1] == cVar20);
          cVar21 = auVar19[2];
          auVar26[2] = -(pcVar1[2] == cVar21);
          cVar22 = auVar19[3];
          auVar26[3] = -(pcVar1[3] == cVar22);
          auVar26[4] = -(pcVar1[4] == cVar8);
          auVar26[5] = -(pcVar1[5] == cVar20);
          auVar26[6] = -(pcVar1[6] == cVar21);
          auVar26[7] = -(pcVar1[7] == cVar22);
          auVar26[8] = -(pcVar1[8] == cVar8);
          auVar26[9] = -(pcVar1[9] == cVar20);
          auVar26[10] = -(pcVar1[10] == cVar21);
          auVar26[0xb] = -(pcVar1[0xb] == cVar22);
          auVar26[0xc] = -(pcVar1[0xc] == cVar8);
          auVar26[0xd] = -(pcVar1[0xd] == cVar20);
          auVar26[0xe] = -(pcVar1[0xe] == cVar21);
          auVar26[0xf] = -(pcVar1[0xf] == cVar22);
          uVar15 = (ushort)(SUB161(auVar26 >> 7,0) & 1) |
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
          uVar14 = (uint)uVar15;
          while (uVar15 != 0) {
            uVar5 = 0;
            for (uVar6 = uVar14; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
              uVar5 = uVar5 + 1;
            }
            uVar12 = uVar5 + uVar16 & *(ulonglong *)(lVar17 + 0x120);
            if (*(ulonglong *)(lVar3 + -0x10 + uVar12 * -0x10) == uVar10) {
              uVar10 = (ulonglong)*(uint *)(lVar3 + -8 + uVar12 * -0x10);
              if (uVar10 <= (ulonglong)param_2[0x23]) {
                *(undefined8 *)(param_2[0x22] + uVar10) = uVar9;
                return;
              }
code_r0x00014090569a:
              FUN_1428d9830();
code_r0x0001409056a6:
              FUN_1428d9760(&UNK_142aaa288,0x2b,&uStack_d0,&UNK_142aaa268,&UNK_142aaa540);
              pcVar4 = (code *)swi(3);
              (*pcVar4)();
              return;
            }
            uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
            uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
          }
          auVar24[0] = -(*pcVar1 == -1);
          auVar24[1] = -(pcVar1[1] == -1);
          auVar24[2] = -(pcVar1[2] == -1);
          auVar24[3] = -(pcVar1[3] == -1);
          auVar24[4] = -(pcVar1[4] == -1);
          auVar24[5] = -(pcVar1[5] == -1);
          auVar24[6] = -(pcVar1[6] == -1);
          auVar24[7] = -(pcVar1[7] == -1);
          auVar24[8] = -(pcVar1[8] == -1);
          auVar24[9] = -(pcVar1[9] == -1);
          auVar24[10] = -(pcVar1[10] == -1);
          auVar24[0xb] = -(pcVar1[0xb] == -1);
          auVar24[0xc] = -(pcVar1[0xc] == -1);
          auVar24[0xd] = -(pcVar1[0xd] == -1);
          auVar24[0xe] = -(pcVar1[0xe] == -1);
          auVar24[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar24 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar24 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x2f,0) & 1) != 0) || (SUB161(auVar24 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') {
            return;
          }
          uVar16 = uVar16 + lVar13 + 0x10;
          lVar13 = lVar13 + 0x10;
        } while( true );
      }
      uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
      uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
    }
    auVar23[0] = -(auVar26[0] == -1);
    auVar23[1] = -(auVar26[1] == -1);
    auVar23[2] = -(auVar26[2] == -1);
    auVar23[3] = -(auVar26[3] == -1);
    auVar23[4] = -(auVar26[4] == -1);
    auVar23[5] = -(auVar26[5] == -1);
    auVar23[6] = -(auVar26[6] == -1);
    auVar23[7] = -(auVar26[7] == -1);
    auVar23[8] = -(auVar26[8] == -1);
    auVar23[9] = -(auVar26[9] == -1);
    auVar23[10] = -(auVar26[10] == -1);
    auVar23[0xb] = -(auVar26[0xb] == -1);
    auVar23[0xc] = -(auVar26[0xc] == -1);
    auVar23[0xd] = -(auVar26[0xd] == -1);
    auVar23[0xe] = -(auVar26[0xe] == -1);
    auVar23[0xf] = -(auVar26[0xf] == -1);
    if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar23 >> 0x37,0) & 1) != 0) || (SUB161(auVar23 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar23 >> 0x47,0) & 1) != 0) || (SUB161(auVar23 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar23 >> 0x57,0) & 1) != 0) || (SUB161(auVar23 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar23 >> 0x67,0) & 1) != 0) || (SUB161(auVar23 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar23 >> 0x77,0) & 1) != 0) || auVar23[0xf] < '\0') goto code_r0x00014090533e;
    uVar10 = uVar10 + lVar13 + 0x10;
    lVar13 = lVar13 + 0x10;
  } while( true );
}

