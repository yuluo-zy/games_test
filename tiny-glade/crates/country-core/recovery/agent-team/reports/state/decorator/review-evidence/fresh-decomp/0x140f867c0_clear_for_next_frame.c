
void __cdecl country_core::resources::walls::wall_holes::WallHoles::clear_for_next_frame(void)

{
  longlong *plVar1;
  char *pcVar2;
  void *_Src;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  uint uVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 (*pauVar17) [16];
  undefined8 *in_RDX;
  undefined1 (*pauVar18) [16];
  ulonglong uVar19;
  uint uVar20;
  longlong lVar21;
  byte bVar22;
  undefined1 (*pauVar23) [16];
  ulonglong uVar24;
  uint uVar25;
  longlong lVar26;
  longlong lVar27;
  ulonglong uVar28;
  ushort uVar29;
  ulonglong uVar30;
  byte bVar31;
  char cVar32;
  char cVar34;
  char cVar35;
  char cVar36;
  undefined1 auVar33 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
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
  
  lVar15 = in_RDX[3];
  if (lVar15 != 0) {
    pauVar18 = (undefined1 (*) [16])*in_RDX;
    pauVar23 = pauVar18 + 1;
    auVar33 = *pauVar18;
    uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar25 == 0) {
        do {
          auVar33 = *pauVar23;
          uVar29 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
          pauVar18 = pauVar18 + -0x28;
          pauVar23 = pauVar23 + 1;
        } while (uVar29 == 0xffff);
        uVar20 = ~(uint)uVar29;
        uVar25 = ~(uint)uVar29 & -(uint)uVar29 - 2;
      }
      else {
        uVar20 = uVar25;
        uVar25 = uVar25 - 1 & uVar25;
      }
      uVar14 = 0;
      for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      *(undefined8 *)((longlong)pauVar18 + (ulonglong)uVar14 * -0x28 + -0x10) = 0;
      lVar15 = lVar15 + -1;
    } while (lVar15 != 0);
  }
  lVar15 = in_RCX[3];
  if (lVar15 != 0) {
    pauVar18 = (undefined1 (*) [16])*in_RCX;
    pauVar23 = pauVar18 + 1;
    auVar33 = *pauVar18;
    uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_140f868e6:
    do {
      if ((short)uVar25 == 0) {
        do {
          auVar33 = *pauVar23;
          uVar29 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
          pauVar18 = pauVar18 + -0x68;
          pauVar23 = pauVar23 + 1;
        } while (uVar29 == 0xffff);
        uVar20 = ~(uint)uVar29;
        uVar25 = uVar20 & -(uint)uVar29 - 2;
      }
      else {
        uVar20 = uVar25;
        uVar25 = uVar25 - 1 & uVar25;
      }
      uVar14 = 0;
      for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      lVar15 = lVar15 + -1;
      lVar16 = (ulonglong)uVar14 * -0x68;
      lVar21 = *(longlong *)((longlong)pauVar18 + lVar16 + -0x68);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      pauVar17 = (undefined1 (*) [16])*in_RDX;
      uVar19 = in_RDX[1];
      uVar24 = uVar19 & extraout_RAX;
      bVar22 = (byte)(extraout_RAX >> 0x38);
      bVar31 = bVar22 >> 1;
      auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
      auVar33 = pshuflw(auVar33,auVar33,0);
      lVar26 = 0;
      uVar28 = uVar24;
      while( true ) {
        pcVar2 = *pauVar17 + uVar28;
        cVar32 = auVar33[0];
        auVar38[0] = -(*pcVar2 == cVar32);
        cVar34 = auVar33[1];
        auVar38[1] = -(pcVar2[1] == cVar34);
        cVar35 = auVar33[2];
        auVar38[2] = -(pcVar2[2] == cVar35);
        cVar36 = auVar33[3];
        auVar38[3] = -(pcVar2[3] == cVar36);
        auVar38[4] = -(pcVar2[4] == cVar32);
        auVar38[5] = -(pcVar2[5] == cVar34);
        auVar38[6] = -(pcVar2[6] == cVar35);
        auVar38[7] = -(pcVar2[7] == cVar36);
        auVar38[8] = -(pcVar2[8] == cVar32);
        auVar38[9] = -(pcVar2[9] == cVar34);
        auVar38[10] = -(pcVar2[10] == cVar35);
        auVar38[0xb] = -(pcVar2[0xb] == cVar36);
        auVar38[0xc] = -(pcVar2[0xc] == cVar32);
        auVar38[0xd] = -(pcVar2[0xd] == cVar34);
        auVar38[0xe] = -(pcVar2[0xe] == cVar35);
        auVar38[0xf] = -(pcVar2[0xf] == cVar36);
        uVar29 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(auVar38[0xf] >> 7) << 0xf
        ;
        uVar20 = (uint)uVar29;
        while (uVar29 != 0) {
          uVar14 = 0;
          for (uVar3 = uVar20; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
            uVar14 = uVar14 + 1;
          }
          uVar30 = uVar14 + uVar28 & uVar19;
          if (*(longlong *)((longlong)pauVar17 + uVar30 * -0x28 + -0x28) == lVar21) {
            lVar26 = (longlong)pauVar17 + uVar30 * -0x28;
            lVar21 = *(longlong *)((longlong)pauVar18 + lVar16 + -0x60);
            goto joined_r0x000140f869fb;
          }
          uVar29 = (ushort)(uVar20 - 1) & (ushort)uVar20;
          uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar29);
        }
        auVar37[0] = -(*pcVar2 == (char)iVar4);
        auVar37[1] = -(pcVar2[1] == (char)((uint)iVar4 >> 8));
        auVar37[2] = -(pcVar2[2] == (char)((uint)iVar4 >> 0x10));
        auVar37[3] = -(pcVar2[3] == (char)((uint)iVar4 >> 0x18));
        auVar37[4] = -(pcVar2[4] == (char)iVar5);
        auVar37[5] = -(pcVar2[5] == (char)((uint)iVar5 >> 8));
        auVar37[6] = -(pcVar2[6] == (char)((uint)iVar5 >> 0x10));
        auVar37[7] = -(pcVar2[7] == (char)((uint)iVar5 >> 0x18));
        auVar37[8] = -(pcVar2[8] == (char)iVar6);
        auVar37[9] = -(pcVar2[9] == (char)((uint)iVar6 >> 8));
        auVar37[10] = -(pcVar2[10] == (char)((uint)iVar6 >> 0x10));
        auVar37[0xb] = -(pcVar2[0xb] == (char)((uint)iVar6 >> 0x18));
        auVar37[0xc] = -(pcVar2[0xc] == (char)iVar7);
        auVar37[0xd] = -(pcVar2[0xd] == (char)((uint)iVar7 >> 8));
        auVar37[0xe] = -(pcVar2[0xe] == (char)((uint)iVar7 >> 0x10));
        auVar37[0xf] = -(pcVar2[0xf] == (char)((uint)iVar7 >> 0x18));
        if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x1f,0) & 1) != 0) || (SUB161(auVar37 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar37 >> 0x37,0) & 1) != 0) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') break;
        lVar27 = uVar28 + lVar26;
        lVar26 = lVar26 + 0x10;
        uVar28 = lVar27 + 0x10U & uVar19;
      }
      if (in_RDX[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::wall_holes::FinalizedWallHoles>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::wall_holes::FinalizedWallHoles,std::hash::random::RandomState>_>
                  ();
        pauVar17 = (undefined1 (*) [16])*in_RDX;
        uVar19 = in_RDX[1];
        uVar24 = extraout_RAX & uVar19;
        auVar33 = *(undefined1 (*) [16])(*pauVar17 + uVar24);
        uVar20 = (uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
        if (uVar20 == 0) goto LAB_140f86c55;
      }
      else {
        auVar33 = *(undefined1 (*) [16])(*pauVar17 + uVar24);
        uVar29 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
        uVar20 = (uint)uVar29;
        if (uVar29 == 0) {
LAB_140f86c55:
          lVar26 = 0x10;
          do {
            uVar24 = uVar24 + lVar26 & uVar19;
            auVar33 = *(undefined1 (*) [16])(*pauVar17 + uVar24);
            lVar26 = lVar26 + 0x10;
            uVar20 = (uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                    (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                    (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                    (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                    (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                    (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                    (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                    (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                    (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                    (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                    (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                    (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                    (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                    (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
          } while (uVar20 == 0);
        }
      }
      uVar14 = 0;
      for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      uVar28 = uVar14 + uVar24 & uVar19;
      bVar31 = (*pauVar17)[uVar28];
      if (-1 < (char)bVar31) {
        auVar33 = *pauVar17;
        uVar20 = 0;
        for (uVar14 = (uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar33[0xf] >> 7) << 0xf); (uVar14 & 1) == 0;
            uVar14 = uVar14 >> 1 | 0x80000000) {
          uVar20 = uVar20 + 1;
        }
        uVar28 = (ulonglong)uVar20;
        bVar31 = (*pauVar17)[uVar28];
      }
      bVar22 = bVar22 >> 1;
      (*pauVar17)[uVar28] = bVar22;
      pauVar17[1][uVar28 - 0x10 & uVar19] = bVar22;
      lVar26 = (longlong)pauVar17 + uVar28 * -0x28;
      in_RDX[2] = in_RDX[2] - (ulonglong)(bVar31 & 1);
      *(longlong *)((longlong)pauVar17 + uVar28 * -0x28 + -0x28) = lVar21;
      *(undefined8 *)((longlong)pauVar17 + uVar28 * -0x28 + -0x20) = 0;
      *(undefined8 *)((longlong)pauVar17 + uVar28 * -0x28 + -0x18) = 4;
      *(undefined8 *)((longlong)pauVar17 + uVar28 * -0x28 + -0x10) = 0;
      *(undefined4 *)((longlong)pauVar17 + uVar28 * -0x28 + -8) = 0;
      *(undefined1 *)((longlong)pauVar17 + uVar28 * -0x28 + -4) = 0;
      in_RDX[3] = in_RDX[3] + 1;
      lVar21 = *(longlong *)((longlong)pauVar18 + lVar16 + -0x60);
joined_r0x000140f869fb:
      if (lVar21 == 0) {
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x48) = 0;
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x30) = 0;
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x20) = 0;
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x18) = 0;
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x10) = 0;
        *(undefined8 *)((longlong)pauVar18 + lVar16 + -8) = 0;
        if (lVar15 == 0) break;
        goto LAB_140f868e6;
      }
      *(undefined4 *)(lVar26 + -8) = *(undefined4 *)((longlong)pauVar18 + lVar16 + -0x40);
      *(undefined1 *)(lVar26 + -4) = *(undefined1 *)((longlong)pauVar18 + lVar16 + -0x3c);
      _Src = *(void **)((longlong)pauVar18 + lVar16 + -0x50);
      uVar19 = *(ulonglong *)((longlong)pauVar18 + lVar16 + -0x48);
      *(undefined8 *)(lVar26 + -0x10) = 0;
      if (*(ulonglong *)(lVar26 + -0x20) < uVar19) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar21 = *(longlong *)(lVar26 + -0x10);
      }
      else {
        lVar21 = 0;
      }
      plVar1 = (longlong *)((longlong)pauVar18 + lVar16 + -0x58);
      memcpy((void *)(lVar21 * 0x14 + *(longlong *)(lVar26 + -0x18)),_Src,uVar19 * 0x14);
      *(ulonglong *)(lVar26 + -0x10) = lVar21 + uVar19;
      lVar21 = *plVar1;
      uVar8 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x50);
      uVar9 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x48);
      uVar10 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x40);
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x58) = 0;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x50) = 4;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x48) = 0;
      *(undefined4 *)((longlong)pauVar18 + lVar16 + -0x40) = 0;
      *(undefined1 *)((longlong)pauVar18 + lVar16 + -0x3c) = 0;
      lVar26 = *(longlong *)((longlong)pauVar18 + lVar16 + -0x38);
      uVar11 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x30);
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x38) = 0;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x30) = 4;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x28) = 0;
      uVar12 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x20);
      uVar13 = *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x18);
      if (*plVar1 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x60) = 0;
      *plVar1 = lVar26;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x50) = uVar11;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x48) = 0;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x30) = uVar9;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x28) = uVar10;
      *(longlong *)((longlong)pauVar18 + lVar16 + -0x40) = lVar21;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x38) = uVar8;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x20) = 0;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x18) = 0;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -0x10) = uVar12;
      *(undefined8 *)((longlong)pauVar18 + lVar16 + -8) = uVar13;
    } while (lVar15 != 0);
  }
  hashbrown::map::HashMap::
  retain<country_core::resources::walls::WallId,country_core::resources::walls::wall_holes::FinalizedWallHoles,std::hash::random::RandomState,alloc::alloc::Global,country_core::resources::walls::wall_holes::impl_0::clear_for_next_frame::closure_env_0>
            ();
  return;
}

