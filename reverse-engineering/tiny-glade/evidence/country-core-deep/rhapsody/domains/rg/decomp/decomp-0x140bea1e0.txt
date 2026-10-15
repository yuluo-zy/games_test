
undefined8 FUN_140bea1e0(longlong *param_1,longlong param_2)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  ulonglong uVar4;
  ushort uVar5;
  longlong lVar6;
  uint uVar7;
  byte bVar8;
  char cVar9;
  char cVar11;
  char cVar12;
  char cVar13;
  undefined1 auVar10 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  longlong lStack_28;
  
  if (param_1[3] != 0) {
    lStack_28 = param_2;
    uVar4 = FUN_1405b0d70(param_1 + 4,&lStack_28);
    bVar8 = (byte)(uVar4 >> 0x39);
    auVar10 = ZEXT216(CONCAT11(bVar8,bVar8));
    auVar10 = pshuflw(auVar10,auVar10,0);
    lVar6 = 0;
    while( true ) {
      uVar4 = param_1[1] & uVar4;
      pcVar1 = (char *)(*param_1 + uVar4);
      cVar9 = auVar10[0];
      auVar15[0] = -(*pcVar1 == cVar9);
      cVar11 = auVar10[1];
      auVar15[1] = -(pcVar1[1] == cVar11);
      cVar12 = auVar10[2];
      auVar15[2] = -(pcVar1[2] == cVar12);
      cVar13 = auVar10[3];
      auVar15[3] = -(pcVar1[3] == cVar13);
      auVar15[4] = -(pcVar1[4] == cVar9);
      auVar15[5] = -(pcVar1[5] == cVar11);
      auVar15[6] = -(pcVar1[6] == cVar12);
      auVar15[7] = -(pcVar1[7] == cVar13);
      auVar15[8] = -(pcVar1[8] == cVar9);
      auVar15[9] = -(pcVar1[9] == cVar11);
      auVar15[10] = -(pcVar1[10] == cVar12);
      auVar15[0xb] = -(pcVar1[0xb] == cVar13);
      auVar15[0xc] = -(pcVar1[0xc] == cVar9);
      auVar15[0xd] = -(pcVar1[0xd] == cVar11);
      auVar15[0xe] = -(pcVar1[0xe] == cVar12);
      auVar15[0xf] = -(pcVar1[0xf] == cVar13);
      uVar5 = (ushort)(SUB161(auVar15 >> 7,0) & 1) | (ushort)(SUB161(auVar15 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar15 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar15 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar15 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar15 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar15 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar15 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar15 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar15 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar15 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar15 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar15 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar15 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar15 >> 0x77,0) & 1) << 0xe | (ushort)(auVar15[0xf] >> 7) << 0xf;
      uVar7 = (uint)uVar5;
      while (uVar5 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar7; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        if (*(longlong *)(*param_1 + -0x10 + (uVar2 + uVar4 & param_1[1]) * -0x10) == param_2) {
          return 1;
        }
        uVar5 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar5);
      }
      auVar14[0] = -(*pcVar1 == -1);
      auVar14[1] = -(pcVar1[1] == -1);
      auVar14[2] = -(pcVar1[2] == -1);
      auVar14[3] = -(pcVar1[3] == -1);
      auVar14[4] = -(pcVar1[4] == -1);
      auVar14[5] = -(pcVar1[5] == -1);
      auVar14[6] = -(pcVar1[6] == -1);
      auVar14[7] = -(pcVar1[7] == -1);
      auVar14[8] = -(pcVar1[8] == -1);
      auVar14[9] = -(pcVar1[9] == -1);
      auVar14[10] = -(pcVar1[10] == -1);
      auVar14[0xb] = -(pcVar1[0xb] == -1);
      auVar14[0xc] = -(pcVar1[0xc] == -1);
      auVar14[0xd] = -(pcVar1[0xd] == -1);
      auVar14[0xe] = -(pcVar1[0xe] == -1);
      auVar14[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar14 >> 7,0) & 1) != 0 || (SUB161(auVar14 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar14 >> 0x17,0) & 1) != 0) || (SUB161(auVar14 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar14 >> 0x27,0) & 1) != 0) || (SUB161(auVar14 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar14 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar14 >> 0x3f,0) & 1) != 0) || (SUB161(auVar14 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar14 >> 0x4f,0) & 1) != 0) || (SUB161(auVar14 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar14 >> 0x5f,0) & 1) != 0) || (SUB161(auVar14 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar14 >> 0x6f,0) & 1) != 0) || (SUB161(auVar14 >> 0x77,0) & 1) != 0) ||
          auVar14[0xf] < '\0') break;
      uVar4 = uVar4 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
  return 0;
}

