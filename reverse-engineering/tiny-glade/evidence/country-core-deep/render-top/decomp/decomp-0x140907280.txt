
void FUN_140907280(longlong param_1,longlong param_2)

{
  undefined1 auVar1 [16];
  longlong *plVar2;
  longlong lVar3;
  undefined8 uVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  uint uVar11;
  ushort uVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auStack_248 [248];
  undefined1 auStack_150 [248];
  undefined1 auStack_58 [24];
  longlong lStack_40;
  longlong lStack_38;
  undefined1 uStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  plVar2 = *(longlong **)(*(longlong *)(param_1 + 0xe0) + 0x1c8);
  lStack_40 = param_2;
  lStack_38 = param_1;
  if (plVar2[3] != 0) {
    uVar8 = FUN_1405b0d70(plVar2 + 4,&lStack_40);
    lVar3 = *plVar2;
    bVar13 = (byte)(uVar8 >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar10 = 0;
    while( true ) {
      uVar8 = plVar2[1] & uVar8;
      auVar1 = *(undefined1 (*) [16])(lVar3 + uVar8);
      cVar14 = auVar15[0];
      auVar20[0] = -(auVar1[0] == cVar14);
      cVar16 = auVar15[1];
      auVar20[1] = -(auVar1[1] == cVar16);
      cVar17 = auVar15[2];
      auVar20[2] = -(auVar1[2] == cVar17);
      cVar18 = auVar15[3];
      auVar20[3] = -(auVar1[3] == cVar18);
      auVar20[4] = -(auVar1[4] == cVar14);
      auVar20[5] = -(auVar1[5] == cVar16);
      auVar20[6] = -(auVar1[6] == cVar17);
      auVar20[7] = -(auVar1[7] == cVar18);
      auVar20[8] = -(auVar1[8] == cVar14);
      auVar20[9] = -(auVar1[9] == cVar16);
      auVar20[10] = -(auVar1[10] == cVar17);
      auVar20[0xb] = -(auVar1[0xb] == cVar18);
      auVar20[0xc] = -(auVar1[0xc] == cVar14);
      auVar20[0xd] = -(auVar1[0xd] == cVar16);
      auVar20[0xe] = -(auVar1[0xe] == cVar17);
      auVar20[0xf] = -(auVar1[0xf] == cVar18);
      uVar12 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar11; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        lVar9 = (uVar6 + uVar8 & plVar2[1]) * -0x38;
        if (*(longlong *)(lVar3 + -0x38 + lVar9) == param_2) {
          uStack_29 = 1;
          func_0x000140bd6b00(auStack_58,lVar3 + lVar9 + -0x30);
          uVar4 = *(undefined8 *)(lVar3 + lVar9 + -8);
          func_0x000142923c00(auStack_248,lStack_38,0xf8);
          FUN_140906400(auStack_150,auStack_248,0x30e6778252531a83,auStack_58);
          FUN_140907490(auStack_150,uVar4);
          uStack_29 = 0;
          FUN_140be24b0(auStack_58);
          return;
        }
        uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
      }
      auVar19[0] = -(auVar1[0] == -1);
      auVar19[1] = -(auVar1[1] == -1);
      auVar19[2] = -(auVar1[2] == -1);
      auVar19[3] = -(auVar1[3] == -1);
      auVar19[4] = -(auVar1[4] == -1);
      auVar19[5] = -(auVar1[5] == -1);
      auVar19[6] = -(auVar1[6] == -1);
      auVar19[7] = -(auVar1[7] == -1);
      auVar19[8] = -(auVar1[8] == -1);
      auVar19[9] = -(auVar1[9] == -1);
      auVar19[10] = -(auVar1[10] == -1);
      auVar19[0xb] = -(auVar1[0xb] == -1);
      auVar19[0xc] = -(auVar1[0xc] == -1);
      auVar19[0xd] = -(auVar1[0xd] == -1);
      auVar19[0xe] = -(auVar1[0xe] == -1);
      auVar19[0xf] = -(auVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      uVar8 = uVar8 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
  uStack_29 = 1;
  FUN_1428d9330(&UNK_142aaaa18,0x45,&UNK_142aaaa60);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

