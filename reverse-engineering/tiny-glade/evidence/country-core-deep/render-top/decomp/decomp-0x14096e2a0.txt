
undefined8 FUN_14096e2a0(undefined8 param_1,longlong param_2,ulonglong param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  undefined8 uVar12;
  ushort uVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  uint uVar17;
  ulonglong uVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 in_XMM1_Ba;
  undefined1 in_XMM1_Bb;
  undefined1 in_XMM1_Bc;
  undefined1 in_XMM1_Bd;
  undefined1 in_XMM1_Be;
  undefined1 in_XMM1_Bf;
  undefined1 in_XMM1_Bg;
  undefined1 in_XMM1_Bh;
  undefined1 in_XMM1_Bi;
  undefined1 in_XMM1_Bj;
  undefined1 in_XMM1_Bk;
  undefined1 in_XMM1_Bl;
  undefined1 in_XMM1_Bm;
  undefined1 in_XMM1_Bn;
  undefined1 in_XMM1_Bo;
  undefined1 in_XMM1_Bp;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined1 uStack_58;
  undefined7 uStack_57;
  longlong lStack_50;
  int aiStack_44 [3];
  
  aiStack_44[1] = -2;
  aiStack_44[2] = -1;
  lVar15 = *(longlong *)(param_2 + 0x138);
  if (*(longlong *)(lVar15 + 0xb0) != 0) {
    lVar4 = *(longlong *)(lVar15 + 0x98);
    bVar19 = (byte)(param_3 >> 0x39);
    auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
    auVar21 = pshuflw(auVar21,auVar21,0);
    lVar16 = 0;
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar10 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                    CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar11 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                    CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    uVar18 = param_3;
    while( true ) {
      uVar18 = *(ulonglong *)(lVar15 + 0xa0) & uVar18;
      auVar2 = *(undefined1 (*) [16])(lVar4 + uVar18);
      cVar20 = auVar21[0];
      auVar26[0] = -(auVar2[0] == cVar20);
      cVar22 = auVar21[1];
      auVar26[1] = -(auVar2[1] == cVar22);
      cVar23 = auVar21[2];
      auVar26[2] = -(auVar2[2] == cVar23);
      cVar24 = auVar21[3];
      auVar26[3] = -(auVar2[3] == cVar24);
      auVar26[4] = -(auVar2[4] == cVar20);
      auVar26[5] = -(auVar2[5] == cVar22);
      auVar26[6] = -(auVar2[6] == cVar23);
      auVar26[7] = -(auVar2[7] == cVar24);
      auVar26[8] = -(auVar2[8] == cVar20);
      auVar26[9] = -(auVar2[9] == cVar22);
      auVar26[10] = -(auVar2[10] == cVar23);
      auVar26[0xb] = -(auVar2[0xb] == cVar24);
      auVar26[0xc] = -(auVar2[0xc] == cVar20);
      auVar26[0xd] = -(auVar2[0xd] == cVar22);
      auVar26[0xe] = -(auVar2[0xe] == cVar23);
      auVar26[0xf] = -(auVar2[0xf] == cVar24);
      uVar13 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf;
      uVar17 = (uint)uVar13;
      while (uVar13 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar17; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        uVar14 = uVar6 + uVar18 & *(ulonglong *)(lVar15 + 0xa0);
        if (*(ulonglong *)(lVar4 + -0x10 + uVar14 * -0x10) == param_3) {
          lVar15 = uVar14 * -0x10;
          aiStack_44[0] = *(int *)(lVar4 + -8 + lVar15);
          uVar3 = *(undefined4 *)(lVar4 + -4 + lVar15);
          lStack_50 = param_2;
          if (aiStack_44[0] == 0) {
            uVar12 = param_4;
            uVar18 = FUN_140bef470(*(undefined8 *)(param_2 + 0x140),param_4,0);
            if ((uVar18 & 1) != 0) {
              func_0x000140bd6b00(&uStack_b0,param_4);
              param_2 = lStack_50;
              uStack_60 = uStack_a0;
              uStack_70 = uStack_b0;
              uStack_68 = uStack_a8;
              uStack_98 = CONCAT44(uStack_98._4_4_,uVar3);
              uStack_90 = 5;
              uStack_80 = 0;
              uStack_78 = 0xffffffffffffffff;
              uStack_58 = 0;
              lVar15 = *(longlong *)(lStack_50 + 0x40);
              uStack_88 = uVar12;
              if (lVar15 == *(longlong *)(lStack_50 + 0x30)) {
                FUN_140b5f9f0(lStack_50 + 0x30,&UNK_142ab6390);
              }
              lVar4 = *(longlong *)(param_2 + 0x38);
              *(ulonglong *)(lVar4 + 0x40 + lVar15 * 0x48) = CONCAT71(uStack_57,uStack_58);
              puVar1 = (undefined8 *)(lVar4 + 0x30 + lVar15 * 0x48);
              *puVar1 = uStack_68;
              puVar1[1] = uStack_60;
              puVar1 = (undefined8 *)(lVar4 + 0x20 + lVar15 * 0x48);
              *puVar1 = uStack_78;
              puVar1[1] = uStack_70;
              puVar1 = (undefined8 *)(lVar4 + 0x10 + lVar15 * 0x48);
              *puVar1 = uStack_88;
              puVar1[1] = uStack_80;
              puVar1 = (undefined8 *)(lVar4 + lVar15 * 0x48);
              *puVar1 = uStack_98;
              puVar1[1] = CONCAT44(uStack_8c,uStack_90);
              *(longlong *)(param_2 + 0x40) = lVar15 + 1;
              goto code_r0x00014096e45b;
            }
            FUN_1428d9310(&UNK_142ab6378);
          }
          else {
            uStack_98 = 0;
            FUN_1428a994f(0,aiStack_44,&UNK_142ab6240,&uStack_98,&UNK_142ab6360);
          }
                    /* WARNING: Does not return */
          pcVar5 = (code *)invalidInstructionException();
          (*pcVar5)();
        }
        uVar13 = (ushort)(uVar17 - 1) & (ushort)uVar17;
        uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar13);
      }
      auVar25[0] = -(auVar2[0] == (char)iVar8);
      auVar25[1] = -(auVar2[1] == (char)((uint)iVar8 >> 8));
      auVar25[2] = -(auVar2[2] == (char)((uint)iVar8 >> 0x10));
      auVar25[3] = -(auVar2[3] == (char)((uint)iVar8 >> 0x18));
      auVar25[4] = -(auVar2[4] == (char)iVar9);
      auVar25[5] = -(auVar2[5] == (char)((uint)iVar9 >> 8));
      auVar25[6] = -(auVar2[6] == (char)((uint)iVar9 >> 0x10));
      auVar25[7] = -(auVar2[7] == (char)((uint)iVar9 >> 0x18));
      auVar25[8] = -(auVar2[8] == (char)iVar10);
      auVar25[9] = -(auVar2[9] == (char)((uint)iVar10 >> 8));
      auVar25[10] = -(auVar2[10] == (char)((uint)iVar10 >> 0x10));
      auVar25[0xb] = -(auVar2[0xb] == (char)((uint)iVar10 >> 0x18));
      auVar25[0xc] = -(auVar2[0xc] == (char)iVar11);
      auVar25[0xd] = -(auVar2[0xd] == (char)((uint)iVar11 >> 8));
      auVar25[0xe] = -(auVar2[0xe] == (char)((uint)iVar11 >> 0x10));
      auVar25[0xf] = -(auVar2[0xf] == (char)((uint)iVar11 >> 0x18));
      if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') break;
      uVar18 = uVar18 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    }
  }
code_r0x00014096e45b:
  func_0x000142923c00(param_1,param_2,400);
  return param_1;
}

