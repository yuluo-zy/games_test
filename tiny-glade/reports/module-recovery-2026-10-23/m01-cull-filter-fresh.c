
void __cdecl _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17h111590a7f0aff7d0E(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong in_RDX;
  uint uVar11;
  longlong lVar12;
  undefined8 *in_R8;
  ulonglong uVar13;
  uint uVar14;
  ushort uVar15;
  longlong lVar16;
  undefined1 (*pauVar17) [16];
  char cVar18;
  byte bVar19;
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
  
  uVar11 = (uint)*(ushort *)(in_RCX + 3);
  lVar16 = *in_RCX;
  pauVar17 = (undefined1 (*) [16])in_RCX[1];
  puVar2 = (undefined8 *)*in_R8;
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  do {
    if ((short)uVar11 == 0) {
      if (in_RDX == 0) {
        return;
      }
      do {
        auVar20 = *pauVar17;
        lVar16 = lVar16 + -0x980;
        pauVar17 = pauVar17 + 1;
        uVar11 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
                                (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar20[0xf] >> 7) << 0xf);
      } while (uVar11 == 0xffff);
      uVar11 = ~uVar11;
      *in_RCX = lVar16;
      in_RCX[1] = (longlong)pauVar17;
    }
    uVar14 = 0;
    for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar14 = uVar14 + 1;
    }
    uVar11 = uVar11 - 1 & uVar11;
    *(short *)(in_RCX + 3) = (short)uVar11;
    lVar3 = *(longlong *)(lVar16 + -0x98 + (ulonglong)uVar14 * -0x98);
    plVar4 = *(longlong **)*puVar2;
    if (plVar4[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar19 = (byte)(extraout_RAX >> 0x39);
      auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
      auVar20 = pshuflw(auVar20,auVar20,0);
      lVar12 = 0;
      uVar13 = extraout_RAX;
      while( true ) {
        uVar13 = plVar4[1] & uVar13;
        pcVar1 = (char *)(*plVar4 + uVar13);
        cVar18 = auVar20[0];
        auVar25[0] = -(*pcVar1 == cVar18);
        cVar21 = auVar20[1];
        auVar25[1] = -(pcVar1[1] == cVar21);
        cVar22 = auVar20[2];
        auVar25[2] = -(pcVar1[2] == cVar22);
        cVar23 = auVar20[3];
        auVar25[3] = -(pcVar1[3] == cVar23);
        auVar25[4] = -(pcVar1[4] == cVar18);
        auVar25[5] = -(pcVar1[5] == cVar21);
        auVar25[6] = -(pcVar1[6] == cVar22);
        auVar25[7] = -(pcVar1[7] == cVar23);
        auVar25[8] = -(pcVar1[8] == cVar18);
        auVar25[9] = -(pcVar1[9] == cVar21);
        auVar25[10] = -(pcVar1[10] == cVar22);
        auVar25[0xb] = -(pcVar1[0xb] == cVar23);
        auVar25[0xc] = -(pcVar1[0xc] == cVar18);
        auVar25[0xd] = -(pcVar1[0xd] == cVar21);
        auVar25[0xe] = -(pcVar1[0xe] == cVar22);
        auVar25[0xf] = -(pcVar1[0xf] == cVar23);
        uVar15 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf
        ;
        uVar14 = (uint)uVar15;
        while (uVar15 != 0) {
          uVar6 = 0;
          for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar6 = uVar6 + 1;
          }
          if (lVar3 == *(longlong *)(*plVar4 + -0x2a8 + (uVar6 + uVar13 & plVar4[1]) * -0x2a8)) {
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            goto LAB_1412642be;
          }
          uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
          uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
        }
        auVar24[0] = -(*pcVar1 == (char)iVar7);
        auVar24[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
        auVar24[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
        auVar24[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
        auVar24[4] = -(pcVar1[4] == (char)iVar8);
        auVar24[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
        auVar24[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
        auVar24[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
        auVar24[8] = -(pcVar1[8] == (char)iVar9);
        auVar24[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
        auVar24[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
        auVar24[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
        auVar24[0xc] = -(pcVar1[0xc] == (char)iVar10);
        auVar24[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
        auVar24[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
        auVar24[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
        if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x1f,0) & 1) != 0) || (SUB161(auVar24 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') break;
        uVar13 = uVar13 + lVar12 + 0x10;
        lVar12 = lVar12 + 0x10;
      }
    }
LAB_1412642be:
    in_RDX = in_RDX + -1;
  } while( true );
}


