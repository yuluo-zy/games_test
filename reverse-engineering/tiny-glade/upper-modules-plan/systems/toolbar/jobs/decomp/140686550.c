// 0x140686550 layout
// signature: void __cdecl layout(void)

void __cdecl toolbar::mei::core::NodeContext::layout(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  ushort uVar18;
  undefined8 *in_R8;
  ulonglong uVar19;
  longlong lVar20;
  uint uVar21;
  longlong lVar22;
  byte bVar23;
  char cVar24;
  char cVar26;
  char cVar27;
  char cVar28;
  undefined1 auVar25 [16];
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
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  
  lVar3 = *in_RDX;
  if (*(longlong *)(lVar3 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar4 = *(longlong *)(lVar3 + 0x1a0);
    bVar23 = (byte)(extraout_RAX >> 0x39);
    auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
    auVar25 = pshuflw(auVar25,auVar25,0);
    lVar20 = 0;
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar10 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                    CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    uVar19 = extraout_RAX;
    while( true ) {
      uVar19 = *(ulonglong *)(lVar3 + 0x1a8) & uVar19;
      pcVar1 = (char *)(lVar4 + uVar19);
      cVar24 = auVar25[0];
      auVar30[0] = -(*pcVar1 == cVar24);
      cVar26 = auVar25[1];
      auVar30[1] = -(pcVar1[1] == cVar26);
      cVar27 = auVar25[2];
      auVar30[2] = -(pcVar1[2] == cVar27);
      cVar28 = auVar25[3];
      auVar30[3] = -(pcVar1[3] == cVar28);
      auVar30[4] = -(pcVar1[4] == cVar24);
      auVar30[5] = -(pcVar1[5] == cVar26);
      auVar30[6] = -(pcVar1[6] == cVar27);
      auVar30[7] = -(pcVar1[7] == cVar28);
      auVar30[8] = -(pcVar1[8] == cVar24);
      auVar30[9] = -(pcVar1[9] == cVar26);
      auVar30[10] = -(pcVar1[10] == cVar27);
      auVar30[0xb] = -(pcVar1[0xb] == cVar28);
      auVar30[0xc] = -(pcVar1[0xc] == cVar24);
      auVar30[0xd] = -(pcVar1[0xd] == cVar26);
      auVar30[0xe] = -(pcVar1[0xe] == cVar27);
      auVar30[0xf] = -(pcVar1[0xf] == cVar28);
      uVar18 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf;
      uVar21 = (uint)uVar18;
      while (uVar18 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar21; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar22 = (uVar5 + uVar19 & *(ulonglong *)(lVar3 + 0x1a8)) * -0xc0;
        if (in_RDX[1] == *(longlong *)(lVar4 + -0xc0 + lVar22)) {
          *(undefined8 *)(lVar4 + -0x60 + lVar22) = in_R8[8];
          uVar11 = *in_R8;
          uVar12 = in_R8[1];
          uVar13 = in_R8[2];
          uVar14 = in_R8[3];
          uVar15 = in_R8[4];
          uVar16 = in_R8[5];
          uVar17 = in_R8[7];
          puVar2 = (undefined8 *)(lVar4 + -0x70 + lVar22);
          *puVar2 = in_R8[6];
          puVar2[1] = uVar17;
          puVar2 = (undefined8 *)(lVar4 + -0x80 + lVar22);
          *puVar2 = uVar15;
          puVar2[1] = uVar16;
          puVar2 = (undefined8 *)(lVar4 + -0x90 + lVar22);
          *puVar2 = uVar13;
          puVar2[1] = uVar14;
          puVar2 = (undefined8 *)(lVar4 + -0xa0 + lVar22);
          *puVar2 = uVar11;
          puVar2[1] = uVar12;
          in_RCX[2] = in_RDX[2];
          lVar3 = in_RDX[1];
          *in_RCX = *in_RDX;
          in_RCX[1] = lVar3;
          return;
        }
        uVar18 = (ushort)(uVar21 - 1) & (ushort)uVar21;
        uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar18);
      }
      auVar29[0] = -(*pcVar1 == (char)iVar7);
      auVar29[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
      auVar29[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
      auVar29[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
      auVar29[4] = -(pcVar1[4] == (char)iVar8);
      auVar29[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
      auVar29[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
      auVar29[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
      auVar29[8] = -(pcVar1[8] == (char)iVar9);
      auVar29[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
      auVar29[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
      auVar29[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
      auVar29[0xc] = -(pcVar1[0xc] == (char)iVar10);
      auVar29[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
      auVar29[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
      auVar29[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar29 >> 0x17,0) & 1) != 0) || (SUB161(auVar29 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar29 >> 0x27,0) & 1) != 0) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar29 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar29 >> 0x3f,0) & 1) != 0) || (SUB161(auVar29 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar29 >> 0x4f,0) & 1) != 0) || (SUB161(auVar29 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar29 >> 0x5f,0) & 1) != 0) || (SUB161(auVar29 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar29 >> 0x6f,0) & 1) != 0) || (SUB161(auVar29 >> 0x77,0) & 1) != 0) ||
          auVar29[0xf] < '\0') break;
      uVar19 = uVar19 + lVar20 + 0x10;
      lVar20 = lVar20 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

