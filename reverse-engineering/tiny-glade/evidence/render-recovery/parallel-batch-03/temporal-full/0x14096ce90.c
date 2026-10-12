
void FUN_14096ce90(longlong *param_1,longlong param_2,undefined8 param_3,int *param_4)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  uint uVar3;
  uint uVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  uint uVar8;
  ushort uVar9;
  longlong lVar10;
  longlong lVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined8 auStack_1e0 [3];
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  uint uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
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
  undefined4 uStack_168;
  undefined4 uStack_164;
  longlong *plStack_160;
  code *pcStack_158;
  longlong lStack_150;
  code *pcStack_148;
  undefined6 uStack_140;
  undefined2 uStack_13a;
  undefined6 uStack_138;
  undefined2 uStack_132;
  undefined6 uStack_130;
  char cStack_12a;
  undefined1 uStack_129;
  char cStack_102;
  undefined *puStack_100;
  undefined *puStack_f8;
  longlong **pplStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  longlong *plStack_d8;
  code *pcStack_d0;
  undefined *puStack_c8;
  code *pcStack_c0;
  undefined8 uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  longlong **pplStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  longlong *plStack_80;
  code *pcStack_78;
  longlong lStack_70;
  code *pcStack_68;
  undefined6 uStack_60;
  undefined2 uStack_5a;
  undefined6 uStack_58;
  undefined2 uStack_52;
  undefined6 uStack_50;
  char cStack_4a;
  undefined1 uStack_49;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lStack_48 = FUN_140c0f650(param_3);
  if (*(longlong *)(param_2 + 200) != 0) {
    uVar5 = FUN_1412e8120(param_2 + 0xd0,&lStack_48);
    lVar11 = *(longlong *)(param_2 + 0xb0);
    bVar12 = (byte)(uVar5 >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar7 = 0;
    while( true ) {
      uVar5 = *(ulonglong *)(param_2 + 0xb8) & uVar5;
      auVar1 = *(undefined1 (*) [16])(lVar11 + uVar5);
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
      uVar9 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
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
      uVar8 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        lVar10 = (uVar3 + uVar5 & *(ulonglong *)(param_2 + 0xb8)) * -0x68;
        if (lStack_48 == *(longlong *)(lVar11 + -0x68 + lVar10)) {
          if (*(int *)(lVar11 + -0x60 + lVar10) == 2) {
            lVar11 = lVar11 + lVar10;
            if (*(char *)(lVar11 + -2) != '\a') {
              if (*(longlong *)(lVar11 + -0x14) == *(longlong *)(param_4 + 3) &&
                  param_4[5] == *(int *)(lVar11 + -0xc)) {
                if ((((*(char *)(lVar11 + -2) == *(char *)((longlong)param_4 + 0x1e)) &&
                     (*(int *)(lVar11 + -0x20) == *param_4)) &&
                    (*(int *)(lVar11 + -0x1c) == param_4[1])) &&
                   (((*(int *)(lVar11 + -0x18) == param_4[2] &&
                     (*(char *)(lVar11 + -4) == (char)param_4[7])) &&
                    ((*(int *)(lVar11 + -8) == param_4[6] &&
                     (*(char *)(lVar11 + -3) == *(char *)((longlong)param_4 + 0x1d))))))) {
                  FUN_140bd60a0(&uStack_184,lVar11 + -0x38,param_2,*(undefined1 *)(lVar11 + -0x5c));
                  *(undefined4 *)(lVar11 + -0x60) = 0;
                  *(undefined4 *)(lVar11 + -0x5c) = uStack_184;
                  *(ulonglong *)(lVar11 + -0x48) = CONCAT44(uStack_16c,uStack_170);
                  *(ulonglong *)(lVar11 + -0x40) = CONCAT44(uStack_164,uStack_168);
                  *(ulonglong *)(lVar11 + -0x58) = CONCAT44(uStack_17c,uStack_180);
                  *(ulonglong *)(lVar11 + -0x50) = CONCAT44(uStack_174,uStack_178);
                  *param_1 = CONCAT44(uStack_180,uStack_184);
                  param_1[1] = CONCAT44(uStack_178,uStack_17c);
                  param_1[2] = CONCAT44(uStack_170,uStack_174);
                  param_1[3] = CONCAT44(uStack_168,uStack_16c);
                  *(undefined4 *)(param_1 + 4) = uStack_164;
                  *(undefined1 *)((longlong)param_1 + 0x24) = 1;
                  return;
                }
                puStack_100 = (undefined *)0x0;
                FUN_1428a998a(0,lVar11 + -0x20,param_4,&puStack_100,&UNK_142ab5e28);
                pcVar2 = (code *)swi(3);
                (*pcVar2)();
                return;
              }
              if (4 < *puRam000000014362c3a8) {
                lStack_150 = lVar11 + -0x14;
                pcStack_158 = FUN_140c0f770;
                pcStack_148 = FUN_14157bd40;
                uStack_140 = SUB86(param_4 + 3,0);
                uStack_13a = (undefined2)((ulonglong)(param_4 + 3) >> 0x30);
                uStack_138 = 0x14157bd40;
                uStack_132 = 0;
                plStack_160 = &lStack_48;
                puVar6 = (undefined8 *)func_0x00014067dd80(&UNK_142ab5df8);
                uStack_e0 = *puVar6;
                plStack_d8 = (longlong *)puVar6[1];
                pcStack_d0 = (code *)0x5;
                puStack_c8 = &UNK_142ab5e10;
                pcStack_c0 = (code *)0x16;
                puStack_b0 = &UNK_142ab5dc8;
                uStack_a8 = 3;
                pplStack_a0 = &plStack_160;
                uStack_98 = 3;
                uStack_90 = 0;
                puStack_100 = (undefined *)0x0;
                puStack_f8 = &UNK_142ab5e10;
                pplStack_f0 = (longlong **)0x16;
                uStack_e8 = 0;
                uStack_b8 = CONCAT44(*(undefined4 *)(puVar6 + 2),1);
                func_0x000140d4a330(&plStack_80,&puStack_100);
              }
              goto code_r0x00014096d182;
            }
            puStack_100 = &UNK_142ab5e68;
            puStack_f8 = (undefined *)0x2;
          }
          else {
            puStack_100 = &UNK_142ab5d78;
            puStack_f8 = (undefined *)0x1;
          }
          pcStack_78 = FUN_140c0f770;
          pplStack_f0 = &plStack_80;
          uStack_e8 = 1;
          uStack_e0 = 0;
          plStack_80 = &lStack_48;
          FUN_140d79f90(&plStack_160,&puStack_100);
          lVar11 = FUN_1428a83e0(&plStack_160);
          *param_1 = lVar11;
          *(undefined1 *)((longlong)param_1 + 0x24) = 2;
          return;
        }
        uVar9 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar9);
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
      uVar5 = uVar5 + lVar7 + 0x10;
      lVar7 = lVar7 + 0x10;
    }
  }
code_r0x00014096d182:
  auStack_1e0[0] = 0;
  uStack_1c8 = *(undefined8 *)param_4;
  uStack_1c0 = *(undefined8 *)(param_4 + 2);
  uStack_1b8 = *(undefined8 *)(param_4 + 4);
  uStack_1b0 = *(undefined8 *)(param_4 + 6);
  FUN_1416a71f0(&puStack_100,*(undefined8 *)(param_2 + 0xa8),lStack_48,
                *(undefined8 *)(lStack_48 + -8),&uStack_1c8,auStack_1e0);
  FUN_1415a53f0(&plStack_160,&puStack_100,param_4);
  if (cStack_12a == '\a') {
    *param_1 = (longlong)plStack_160;
    *(undefined1 *)((longlong)param_1 + 0x24) = 2;
  }
  else {
    uStack_58 = uStack_138;
    uStack_52 = uStack_132;
    uStack_50 = uStack_130;
    pcStack_68 = pcStack_148;
    uStack_60 = uStack_140;
    uStack_5a = uStack_13a;
    pcStack_78 = pcStack_158;
    lStack_70 = lStack_150;
    plStack_80 = plStack_160;
    cStack_4a = cStack_12a;
    uStack_49 = uStack_129;
    uStack_39 = 1;
    FUN_140bd60a0(&uStack_1a8,&plStack_80,param_2,0);
    uStack_a8 = CONCAT17(uStack_49,CONCAT16(cStack_4a,uStack_50));
    uStack_b8 = CONCAT26(uStack_5a,uStack_60);
    puStack_b0 = (undefined *)CONCAT26(uStack_52,uStack_58);
    puStack_c8 = (undefined *)lStack_70;
    pcStack_c0 = pcStack_68;
    plStack_d8 = plStack_80;
    pcStack_d0 = pcStack_78;
    puStack_100 = (undefined *)((ulonglong)uStack_1a8 << 0x20);
    uStack_39 = 0;
    FUN_14152ac30(&plStack_160,param_2 + 0xb0,lStack_48,&puStack_100);
    if ((int)plStack_160 != 3) {
      if (cStack_102 != '\a') {
        func_0x00014003bb80(&uStack_138);
      }
      uStack_39 = 0;
      FUN_140be24b0(&uStack_138);
    }
    *(undefined4 *)(param_1 + 4) = uStack_188;
    param_1[2] = CONCAT44(uStack_194,uStack_198);
    param_1[3] = CONCAT44(uStack_18c,uStack_190);
    *param_1 = CONCAT44(uStack_1a4,uStack_1a8);
    param_1[1] = CONCAT44(uStack_19c,uStack_1a0);
    *(undefined1 *)((longlong)param_1 + 0x24) = 0;
  }
  return;
}

