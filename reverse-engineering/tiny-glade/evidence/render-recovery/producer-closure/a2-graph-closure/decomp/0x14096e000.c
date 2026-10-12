
undefined8
FUN_14096e000(undefined8 param_1,longlong *param_2,ulonglong param_3,longlong param_4,
             undefined4 param_5)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  longlong lVar9;
  uint uVar10;
  ulonglong uVar11;
  ushort uVar12;
  ulonglong uVar13;
  longlong lVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_58;
  int iStack_54;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar14 = param_2[0x27];
  if (*(longlong *)(lVar14 + 0xb0) != 0) {
    lVar5 = *(longlong *)(lVar14 + 0x98);
    bVar15 = (byte)(param_3 >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar9 = 0;
    uVar11 = param_3;
    while( true ) {
      uVar11 = *(ulonglong *)(lVar14 + 0xa0) & uVar11;
      auVar2 = *(undefined1 (*) [16])(lVar5 + uVar11);
      cVar16 = auVar17[0];
      auVar22[0] = -(auVar2[0] == cVar16);
      cVar18 = auVar17[1];
      auVar22[1] = -(auVar2[1] == cVar18);
      cVar19 = auVar17[2];
      auVar22[2] = -(auVar2[2] == cVar19);
      cVar20 = auVar17[3];
      auVar22[3] = -(auVar2[3] == cVar20);
      auVar22[4] = -(auVar2[4] == cVar16);
      auVar22[5] = -(auVar2[5] == cVar18);
      auVar22[6] = -(auVar2[6] == cVar19);
      auVar22[7] = -(auVar2[7] == cVar20);
      auVar22[8] = -(auVar2[8] == cVar16);
      auVar22[9] = -(auVar2[9] == cVar18);
      auVar22[10] = -(auVar2[10] == cVar19);
      auVar22[0xb] = -(auVar2[0xb] == cVar20);
      auVar22[0xc] = -(auVar2[0xc] == cVar16);
      auVar22[0xd] = -(auVar2[0xd] == cVar18);
      auVar22[0xe] = -(auVar2[0xe] == cVar19);
      auVar22[0xf] = -(auVar2[0xf] == cVar20);
      uVar12 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar7 = 0;
        for (uVar8 = uVar10; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        uVar13 = uVar7 + uVar11 & *(ulonglong *)(lVar14 + 0xa0);
        if (*(ulonglong *)(lVar5 + -0x10 + uVar13 * -0x10) == param_3) {
          lVar14 = uVar13 * -0x10;
          iStack_54 = *(int *)(lVar5 + -8 + lVar14);
          uVar3 = *(undefined4 *)(lVar5 + -4 + lVar14);
          plStack_50 = param_2;
          if (iStack_54 == 0) {
            func_0x00014003bb80(0x37);
            uVar4 = *(undefined4 *)(param_4 + 0x10);
            FUN_1409dca90(plStack_50 + 0xc,uVar4,0x37,param_5);
            uVar11 = plStack_50[2];
            uStack_58 = uVar4;
            if (uVar11 < 0x100) {
              func_0x000140bd6c60(&uStack_a0,&uStack_58);
              lVar14 = plStack_50[2];
              if (lVar14 == *plStack_50) {
                FUN_140b60150(plStack_50,&UNK_142ab6330);
              }
              lVar5 = plStack_50[1];
              lVar9 = lVar14 * 0x44;
              *(undefined4 *)(lVar5 + 0x40 + lVar9) = uStack_60;
              puVar1 = (undefined8 *)(lVar5 + 0x30 + lVar9);
              *puVar1 = uStack_70;
              puVar1[1] = uStack_68;
              puVar1 = (undefined8 *)(lVar5 + 0x20 + lVar9);
              *puVar1 = uStack_80;
              puVar1[1] = uStack_78;
              puVar1 = (undefined8 *)(lVar5 + 0x10 + lVar9);
              *puVar1 = uStack_90;
              puVar1[1] = uStack_88;
              *(undefined8 *)(lVar5 + lVar9) = uStack_a0;
              ((undefined8 *)(lVar5 + lVar9))[1] = uStack_98;
              plStack_50[2] = lVar14 + 1;
              lVar14 = plStack_50[5];
              if (lVar14 == plStack_50[3]) {
                FUN_140562e70(plStack_50 + 3,&UNK_142ab6348);
              }
              lVar5 = plStack_50[4];
              *(undefined4 *)(lVar5 + lVar14 * 8) = uVar3;
              *(char *)(lVar5 + 4 + lVar14 * 8) = (char)uVar11;
              plStack_50[5] = lVar14 + 1;
              param_2 = plStack_50;
              goto code_r0x00014096e1e5;
            }
            FUN_1428d9760(&UNK_142ab5f00,0x2b,&uStack_a0,&UNK_142ab5f30,&UNK_142ab6140);
          }
          else {
            uStack_a0 = 0;
            FUN_1428a994f(0,&iStack_54,&UNK_142ab6240,&uStack_a0,&UNK_142ab6318);
          }
                    /* WARNING: Does not return */
          pcVar6 = (code *)invalidInstructionException();
          (*pcVar6)();
        }
        uVar12 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar12);
      }
      auVar21[0] = -(auVar2[0] == -1);
      auVar21[1] = -(auVar2[1] == -1);
      auVar21[2] = -(auVar2[2] == -1);
      auVar21[3] = -(auVar2[3] == -1);
      auVar21[4] = -(auVar2[4] == -1);
      auVar21[5] = -(auVar2[5] == -1);
      auVar21[6] = -(auVar2[6] == -1);
      auVar21[7] = -(auVar2[7] == -1);
      auVar21[8] = -(auVar2[8] == -1);
      auVar21[9] = -(auVar2[9] == -1);
      auVar21[10] = -(auVar2[10] == -1);
      auVar21[0xb] = -(auVar2[0xb] == -1);
      auVar21[0xc] = -(auVar2[0xc] == -1);
      auVar21[0xd] = -(auVar2[0xd] == -1);
      auVar21[0xe] = -(auVar2[0xe] == -1);
      auVar21[0xf] = -(auVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      uVar11 = uVar11 + lVar9 + 0x10;
      lVar9 = lVar9 + 0x10;
    }
  }
code_r0x00014096e1e5:
  func_0x000142923c00(param_1,param_2,400);
  return param_1;
}

