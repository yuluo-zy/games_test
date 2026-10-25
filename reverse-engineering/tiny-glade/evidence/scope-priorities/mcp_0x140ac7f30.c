
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140ac7f30(undefined8 param_1,longlong param_2,longlong *param_3,undefined8 param_4)

{
  undefined1 auVar1 [16];
  longlong lVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  longlong lVar8;
  uint uVar9;
  ushort uVar10;
  longlong lVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  longlong alStack_40 [4];
  
  if (param_3[3] != 0) {
    alStack_40[0] = param_2;
    uVar6 = FUN_1405b0d70(param_3 + 4,alStack_40);
    lVar2 = *param_3;
    bVar12 = (byte)(uVar6 >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar8 = 0;
    while( true ) {
      uVar6 = param_3[1] & uVar6;
      auVar1 = *(undefined1 (*) [16])(lVar2 + uVar6);
      cVar13 = auVar14[0];
      auVar19[0] = -(auVar1[0] == cVar13);
      cVar15 = auVar14[1];
      auVar19[1] = -(auVar1[1] == cVar15);
      cVar16 = auVar14[2];
      auVar19[2] = -(auVar1[2] == cVar16);
      cVar17 = auVar14[3];
      auVar19[3] = -(auVar1[3] == cVar17);
      auVar19[4] = -(auVar1[4] == cVar13);
      auVar19[5] = -(auVar1[5] == cVar15);
      auVar19[6] = -(auVar1[6] == cVar16);
      auVar19[7] = -(auVar1[7] == cVar17);
      auVar19[8] = -(auVar1[8] == cVar13);
      auVar19[9] = -(auVar1[9] == cVar15);
      auVar19[10] = -(auVar1[10] == cVar16);
      auVar19[0xb] = -(auVar1[0xb] == cVar17);
      auVar19[0xc] = -(auVar1[0xc] == cVar13);
      auVar19[0xd] = -(auVar1[0xd] == cVar15);
      auVar19[0xe] = -(auVar1[0xe] == cVar16);
      auVar19[0xf] = -(auVar1[0xf] == cVar17);
      uVar10 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar9 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar11 = (uVar4 + uVar6 & param_3[1]) * -0x2a8;
        if (*(longlong *)(lVar2 + -0x2a8 + lVar11) == param_2) {
          if ((1 < *(ulonglong *)(lVar2 + -0x290 + lVar11)) &&
             (0.0 < *(float *)(lVar2 + -0x270 + lVar11))) {
            FUN_140a923a0(param_3,param_2,param_4);
            return 1;
          }
          FUN_1428d9430(&UNK_142ada193,0x32,&UNK_142adfa58);
          pcVar3 = (code *)swi(3);
          uVar7 = (*pcVar3)();
          return uVar7;
        }
        uVar10 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar10);
      }
      auVar18[0] = -(auVar1[0] == -1);
      auVar18[1] = -(auVar1[1] == -1);
      auVar18[2] = -(auVar1[2] == -1);
      auVar18[3] = -(auVar1[3] == -1);
      auVar18[4] = -(auVar1[4] == -1);
      auVar18[5] = -(auVar1[5] == -1);
      auVar18[6] = -(auVar1[6] == -1);
      auVar18[7] = -(auVar1[7] == -1);
      auVar18[8] = -(auVar1[8] == -1);
      auVar18[9] = -(auVar1[9] == -1);
      auVar18[10] = -(auVar1[10] == -1);
      auVar18[0xb] = -(auVar1[0xb] == -1);
      auVar18[0xc] = -(auVar1[0xc] == -1);
      auVar18[0xd] = -(auVar1[0xd] == -1);
      auVar18[0xe] = -(auVar1[0xe] == -1);
      auVar18[0xf] = -(auVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      uVar6 = uVar6 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
  return 0;
}

