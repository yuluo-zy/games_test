// ADDRESS: 0x140a07680
// SIZE: 423
// SIGNATURE: void __cdecl add_color_self(void)

void __cdecl country_core::render::mesh::Mesh::add_color_self(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong *plVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  uint uVar17;
  ulonglong uVar18;
  longlong lVar19;
  ushort uVar20;
  longlong lVar21;
  byte bVar22;
  char cVar23;
  char cVar25;
  char cVar26;
  char cVar27;
  undefined1 auVar24 [16];
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
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  
  if (in_RDX[10] != 0) {
    core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<str_>_>();
    lVar2 = in_RDX[7];
    bVar22 = (byte)(extraout_RAX >> 0x39);
    auVar24 = ZEXT216(CONCAT11(bVar22,bVar22));
    auVar24 = pshuflw(auVar24,auVar24,0);
    lVar19 = 0;
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    uVar18 = extraout_RAX;
    while( true ) {
      uVar18 = in_RDX[8] & uVar18;
      pcVar1 = (char *)(lVar2 + uVar18);
      cVar23 = auVar24[0];
      auVar29[0] = -(*pcVar1 == cVar23);
      cVar25 = auVar24[1];
      auVar29[1] = -(pcVar1[1] == cVar25);
      cVar26 = auVar24[2];
      auVar29[2] = -(pcVar1[2] == cVar26);
      cVar27 = auVar24[3];
      auVar29[3] = -(pcVar1[3] == cVar27);
      auVar29[4] = -(pcVar1[4] == cVar23);
      auVar29[5] = -(pcVar1[5] == cVar25);
      auVar29[6] = -(pcVar1[6] == cVar26);
      auVar29[7] = -(pcVar1[7] == cVar27);
      auVar29[8] = -(pcVar1[8] == cVar23);
      auVar29[9] = -(pcVar1[9] == cVar25);
      auVar29[10] = -(pcVar1[10] == cVar26);
      auVar29[0xb] = -(pcVar1[0xb] == cVar27);
      auVar29[0xc] = -(pcVar1[0xc] == cVar23);
      auVar29[0xd] = -(pcVar1[0xd] == cVar25);
      auVar29[0xe] = -(pcVar1[0xe] == cVar26);
      auVar29[0xf] = -(pcVar1[0xf] == cVar27);
      uVar20 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
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
      uVar17 = (uint)uVar20;
      while (uVar20 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar17; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar21 = (uVar4 + uVar18 & in_RDX[8]) * -0x38;
        if ((*(longlong *)(lVar2 + -0x28 + lVar21) == 0xf) &&
           (plVar3 = *(longlong **)(lVar2 + lVar21 + -0x30),
           *(longlong *)((longlong)plVar3 + 7) == 0x6e6f697469736f50 &&
           *plVar3 == 0x505f786574726556)) {
          alloc::vec::from_elem<array_<f32,3>_>();
          set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
          in_RCX[0xe] = in_RDX[0xe];
          uVar10 = in_RDX[0xd];
          in_RCX[0xc] = in_RDX[0xc];
          in_RCX[0xd] = uVar10;
          uVar10 = in_RDX[0xb];
          in_RCX[10] = in_RDX[10];
          in_RCX[0xb] = uVar10;
          uVar10 = in_RDX[9];
          in_RCX[8] = in_RDX[8];
          in_RCX[9] = uVar10;
          uVar10 = *in_RDX;
          uVar11 = in_RDX[1];
          uVar12 = in_RDX[2];
          uVar13 = in_RDX[3];
          uVar14 = in_RDX[4];
          uVar15 = in_RDX[5];
          uVar16 = in_RDX[7];
          in_RCX[6] = in_RDX[6];
          in_RCX[7] = uVar16;
          in_RCX[4] = uVar14;
          in_RCX[5] = uVar15;
          in_RCX[2] = uVar12;
          in_RCX[3] = uVar13;
          *in_RCX = uVar10;
          in_RCX[1] = uVar11;
          return;
        }
        uVar20 = (ushort)(uVar17 - 1) & (ushort)uVar17;
        uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar20);
      }
      auVar28[0] = -(*pcVar1 == (char)iVar6);
      auVar28[1] = -(pcVar1[1] == (char)((uint)iVar6 >> 8));
      auVar28[2] = -(pcVar1[2] == (char)((uint)iVar6 >> 0x10));
      auVar28[3] = -(pcVar1[3] == (char)((uint)iVar6 >> 0x18));
      auVar28[4] = -(pcVar1[4] == (char)iVar7);
      auVar28[5] = -(pcVar1[5] == (char)((uint)iVar7 >> 8));
      auVar28[6] = -(pcVar1[6] == (char)((uint)iVar7 >> 0x10));
      auVar28[7] = -(pcVar1[7] == (char)((uint)iVar7 >> 0x18));
      auVar28[8] = -(pcVar1[8] == (char)iVar8);
      auVar28[9] = -(pcVar1[9] == (char)((uint)iVar8 >> 8));
      auVar28[10] = -(pcVar1[10] == (char)((uint)iVar8 >> 0x10));
      auVar28[0xb] = -(pcVar1[0xb] == (char)((uint)iVar8 >> 0x18));
      auVar28[0xc] = -(pcVar1[0xc] == (char)iVar9);
      auVar28[0xd] = -(pcVar1[0xd] == (char)((uint)iVar9 >> 8));
      auVar28[0xe] = -(pcVar1[0xe] == (char)((uint)iVar9 >> 0x10));
      auVar28[0xf] = -(pcVar1[0xf] == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar28 >> 0x17,0) & 1) != 0) || (SUB161(auVar28 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar28 >> 0x27,0) & 1) != 0) || (SUB161(auVar28 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar28 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar28 >> 0x3f,0) & 1) != 0) || (SUB161(auVar28 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar28 >> 0x4f,0) & 1) != 0) || (SUB161(auVar28 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar28 >> 0x5f,0) & 1) != 0) || (SUB161(auVar28 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar28 >> 0x6f,0) & 1) != 0) || (SUB161(auVar28 >> 0x77,0) & 1) != 0) ||
          auVar28[0xf] < '\0') break;
      uVar18 = uVar18 + lVar19 + 0x10;
      lVar19 = lVar19 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

