
void __cdecl system_clutter::autoclutter::AutoClutterState::mark_dirty(void)

{
  longlong *plVar1;
  char *pcVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong lVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  ulonglong extraout_RAX;
  longlong in_RCX;
  ulonglong in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  ushort uVar15;
  ulonglong uVar16;
  longlong *plVar17;
  byte bVar18;
  char cVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  
  uVar12 = in_RDX & 0xffffffff;
  if (*(ulonglong *)(in_RCX + 0x10) <= uVar12) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar3 = *(longlong *)(*(longlong *)(in_RCX + 8) + 0x10 + uVar12 * 0x18);
  if (lVar3 == 0) {
    return;
  }
  plVar4 = *(longlong **)(*(longlong *)(in_RCX + 8) + 8 + uVar12 * 0x18);
  iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  plVar17 = plVar4;
LAB_140ee494d:
  if (*(longlong *)(in_RCX + 0x78) != 0) {
    plVar1 = plVar17 + 1;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar5 = *(longlong *)(in_RCX + 0x60);
    bVar18 = (byte)(extraout_RAX >> 0x39);
    auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
    auVar20 = pshuflw(auVar20,auVar20,0);
    lVar13 = 0;
    uVar12 = extraout_RAX;
    while( true ) {
      uVar12 = *(ulonglong *)(in_RCX + 0x68) & uVar12;
      pcVar2 = (char *)(lVar5 + uVar12);
      cVar19 = auVar20[0];
      auVar25[0] = -(*pcVar2 == cVar19);
      cVar21 = auVar20[1];
      auVar25[1] = -(pcVar2[1] == cVar21);
      cVar22 = auVar20[2];
      auVar25[2] = -(pcVar2[2] == cVar22);
      cVar23 = auVar20[3];
      auVar25[3] = -(pcVar2[3] == cVar23);
      auVar25[4] = -(pcVar2[4] == cVar19);
      auVar25[5] = -(pcVar2[5] == cVar21);
      auVar25[6] = -(pcVar2[6] == cVar22);
      auVar25[7] = -(pcVar2[7] == cVar23);
      auVar25[8] = -(pcVar2[8] == cVar19);
      auVar25[9] = -(pcVar2[9] == cVar21);
      auVar25[10] = -(pcVar2[10] == cVar22);
      auVar25[0xb] = -(pcVar2[0xb] == cVar23);
      auVar25[0xc] = -(pcVar2[0xc] == cVar19);
      auVar25[0xd] = -(pcVar2[0xd] == cVar21);
      auVar25[0xe] = -(pcVar2[0xe] == cVar22);
      auVar25[0xf] = -(pcVar2[0xf] == cVar23);
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
      if (uVar15 != 0) {
        do {
          uVar6 = 0;
          for (uVar7 = uVar14; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
            uVar6 = uVar6 + 1;
          }
          uVar16 = uVar6 + uVar12 & *(ulonglong *)(in_RCX + 0x68);
          if (*plVar17 == *(longlong *)(lVar5 + -0x48 + uVar16 * -0x48)) {
            uVar15 = *(ushort *)(lVar5 + -6 + uVar16 * -0x48);
            if (uVar15 < 0xfffe) {
              lVar13 = *(longlong *)(in_RCX + 0x40);
              if (lVar13 == *(longlong *)(in_RCX + 0x30)) {
                alloc::raw_vec::RawVec::grow_one<u16,alloc::alloc::Global>();
              }
              *(ushort *)(*(longlong *)(in_RCX + 0x38) + lVar13 * 2) = uVar15;
              *(longlong *)(in_RCX + 0x40) = lVar13 + 1;
            }
            *(undefined2 *)(uVar16 * -0x48 + lVar5 + -6) = 0xffff;
            plVar17 = plVar1;
            if (plVar1 == plVar4 + lVar3) {
              return;
            }
            goto LAB_140ee494d;
          }
          uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
          uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
        } while (uVar15 != 0);
      }
      auVar24[0] = -(*pcVar2 == (char)iVar8);
      auVar24[1] = -(pcVar2[1] == (char)((uint)iVar8 >> 8));
      auVar24[2] = -(pcVar2[2] == (char)((uint)iVar8 >> 0x10));
      auVar24[3] = -(pcVar2[3] == (char)((uint)iVar8 >> 0x18));
      auVar24[4] = -(pcVar2[4] == (char)iVar9);
      auVar24[5] = -(pcVar2[5] == (char)((uint)iVar9 >> 8));
      auVar24[6] = -(pcVar2[6] == (char)((uint)iVar9 >> 0x10));
      auVar24[7] = -(pcVar2[7] == (char)((uint)iVar9 >> 0x18));
      auVar24[8] = -(pcVar2[8] == (char)iVar10);
      auVar24[9] = -(pcVar2[9] == (char)((uint)iVar10 >> 8));
      auVar24[10] = -(pcVar2[10] == (char)((uint)iVar10 >> 0x10));
      auVar24[0xb] = -(pcVar2[0xb] == (char)((uint)iVar10 >> 0x18));
      auVar24[0xc] = -(pcVar2[0xc] == (char)iVar11);
      auVar24[0xd] = -(pcVar2[0xd] == (char)((uint)iVar11 >> 8));
      auVar24[0xe] = -(pcVar2[0xe] == (char)((uint)iVar11 >> 0x10));
      auVar24[0xf] = -(pcVar2[0xf] == (char)((uint)iVar11 >> 0x18));
      if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar24 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar24 >> 0x3f,0) & 1) != 0) || (SUB161(auVar24 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar24 >> 0x4f,0) & 1) != 0) || (SUB161(auVar24 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar24 >> 0x5f,0) & 1) != 0) || (SUB161(auVar24 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar24 >> 0x6f,0) & 1) != 0) || (SUB161(auVar24 >> 0x77,0) & 1) != 0) ||
          auVar24[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

