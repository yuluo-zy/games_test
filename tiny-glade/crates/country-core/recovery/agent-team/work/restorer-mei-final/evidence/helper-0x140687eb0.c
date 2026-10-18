
void __cdecl toolbar::mei::core::process_input_action_handlers(void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  ulonglong *extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong uVar8;
  undefined8 uVar9;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong in_RCX;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  uint uVar13;
  ulonglong in_RDX;
  ulonglong *puVar14;
  longlong lVar15;
  longlong lVar16;
  undefined1 (*pauVar17) [16];
  ushort uVar18;
  undefined8 in_R8;
  ulonglong uVar19;
  ulonglong uVar20;
  byte bVar21;
  char cVar22;
  char cVar25;
  char cVar26;
  char cVar27;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  char unaff_XMM6_Ba;
  char unaff_XMM6_Bb;
  char unaff_XMM6_Bc;
  char unaff_XMM6_Bd;
  char unaff_XMM6_Be;
  char unaff_XMM6_Bf;
  char unaff_XMM6_Bg;
  char unaff_XMM6_Bh;
  char unaff_XMM6_Bi;
  char unaff_XMM6_Bj;
  char unaff_XMM6_Bk;
  char unaff_XMM6_Bl;
  char unaff_XMM6_Bm;
  char unaff_XMM6_Bn;
  char unaff_XMM6_Bo;
  char unaff_XMM6_Bp;
  ulonglong local_80;
  undefined8 local_70;
  undefined4 local_64;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_70 = in_RDX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = in_RDX;
  local_80 = 1;
  if (*(longlong *)(in_RCX + 0x1e8) == 0) {
    local_80 = 1;
  }
  else {
    lVar11 = *(longlong *)(in_RCX + 0x1d0);
    uVar20 = *(ulonglong *)(in_RCX + 0x1d8);
    lVar16 = -8;
    lVar15 = 0;
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    unaff_XMM6_Ba = (char)iVar4;
    unaff_XMM6_Bb = (char)((uint)iVar4 >> 8);
    unaff_XMM6_Bc = (char)((uint)iVar4 >> 0x10);
    unaff_XMM6_Bd = (char)((uint)iVar4 >> 0x18);
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    unaff_XMM6_Be = (char)iVar4;
    unaff_XMM6_Bf = (char)((uint)iVar4 >> 8);
    unaff_XMM6_Bg = (char)((uint)iVar4 >> 0x10);
    unaff_XMM6_Bh = (char)((uint)iVar4 >> 0x18);
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    unaff_XMM6_Bi = (char)iVar4;
    unaff_XMM6_Bj = (char)((uint)iVar4 >> 8);
    unaff_XMM6_Bk = (char)((uint)iVar4 >> 0x10);
    unaff_XMM6_Bl = (char)((uint)iVar4 >> 0x18);
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    unaff_XMM6_Bm = (char)iVar4;
    unaff_XMM6_Bn = (char)((uint)iVar4 >> 8);
    unaff_XMM6_Bo = (char)((uint)iVar4 >> 0x10);
    unaff_XMM6_Bp = (char)((uint)iVar4 >> 0x18);
LAB_140687f99:
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar21 = (byte)(extraout_RAX_00 >> 0x39);
    auVar23 = ZEXT216(CONCAT11(bVar21,bVar21));
    auVar23 = pshuflw(auVar23,auVar23,0);
    lVar10 = 0;
    uVar8 = extraout_RAX_00;
    while( true ) {
      uVar8 = uVar8 & uVar20;
      pcVar1 = (char *)(lVar11 + uVar8);
      cVar22 = auVar23[0];
      auVar31[0] = -(*pcVar1 == cVar22);
      cVar25 = auVar23[1];
      auVar31[1] = -(pcVar1[1] == cVar25);
      cVar26 = auVar23[2];
      auVar31[2] = -(pcVar1[2] == cVar26);
      cVar27 = auVar23[3];
      auVar31[3] = -(pcVar1[3] == cVar27);
      auVar31[4] = -(pcVar1[4] == cVar22);
      auVar31[5] = -(pcVar1[5] == cVar25);
      auVar31[6] = -(pcVar1[6] == cVar26);
      auVar31[7] = -(pcVar1[7] == cVar27);
      auVar31[8] = -(pcVar1[8] == cVar22);
      auVar31[9] = -(pcVar1[9] == cVar25);
      auVar31[10] = -(pcVar1[10] == cVar26);
      auVar31[0xb] = -(pcVar1[0xb] == cVar27);
      auVar31[0xc] = -(pcVar1[0xc] == cVar22);
      auVar31[0xd] = -(pcVar1[0xd] == cVar25);
      auVar31[0xe] = -(pcVar1[0xe] == cVar26);
      auVar31[0xf] = -(pcVar1[0xf] == cVar27);
      uVar18 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar18;
      while (uVar18 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar13; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        uVar19 = uVar2 + uVar8 & uVar20;
        if (in_RDX == *(ulonglong *)(lVar11 + -0x10 + uVar19 * -0x10)) {
          in_RDX = *(ulonglong *)(lVar11 + -8 + uVar19 * -0x10);
          if (local_80 == 1) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          extraout_RAX[local_80] = in_RDX;
          local_80 = local_80 + 1;
          lVar15 = lVar15 + 8;
          lVar16 = lVar16 + 8;
          local_70 = in_RDX;
          goto LAB_140687f99;
        }
        uVar18 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar18);
      }
      auVar28[0] = -(*pcVar1 == unaff_XMM6_Ba);
      auVar28[1] = -(pcVar1[1] == unaff_XMM6_Bb);
      auVar28[2] = -(pcVar1[2] == unaff_XMM6_Bc);
      auVar28[3] = -(pcVar1[3] == unaff_XMM6_Bd);
      auVar28[4] = -(pcVar1[4] == unaff_XMM6_Be);
      auVar28[5] = -(pcVar1[5] == unaff_XMM6_Bf);
      auVar28[6] = -(pcVar1[6] == unaff_XMM6_Bg);
      auVar28[7] = -(pcVar1[7] == unaff_XMM6_Bh);
      auVar28[8] = -(pcVar1[8] == unaff_XMM6_Bi);
      auVar28[9] = -(pcVar1[9] == unaff_XMM6_Bj);
      auVar28[10] = -(pcVar1[10] == unaff_XMM6_Bk);
      auVar28[0xb] = -(pcVar1[0xb] == unaff_XMM6_Bl);
      auVar28[0xc] = -(pcVar1[0xc] == unaff_XMM6_Bm);
      auVar28[0xd] = -(pcVar1[0xd] == unaff_XMM6_Bn);
      auVar28[0xe] = -(pcVar1[0xe] == unaff_XMM6_Bo);
      auVar28[0xf] = -(pcVar1[0xf] == unaff_XMM6_Bp);
      if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar28 >> 0x17,0) & 1) != 0) || (SUB161(auVar28 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar28 >> 0x27,0) & 1) != 0) || (SUB161(auVar28 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar28 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar28 >> 0x3f,0) & 1) != 0) || (SUB161(auVar28 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar28 >> 0x4f,0) & 1) != 0) || (SUB161(auVar28 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar28 >> 0x5f,0) & 1) != 0) || (SUB161(auVar28 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar28 >> 0x6f,0) & 1) != 0) || (SUB161(auVar28 >> 0x77,0) & 1) != 0) ||
          auVar28[0xf] < '\0') break;
      uVar8 = uVar8 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
    if (1 < local_80) {
      uVar20 = local_80 >> 1;
      if (local_80 < 8) {
        uVar8 = 0;
      }
      else {
        uVar8 = uVar20 & 0xfffffffffffffffc;
        pauVar17 = (undefined1 (*) [16])(lVar16 + (longlong)extraout_RAX);
        uVar19 = 0;
        do {
          auVar23 = *(undefined1 (*) [16])(extraout_RAX + uVar19);
          auVar31 = *(undefined1 (*) [16])(extraout_RAX + uVar19 + 2);
          auVar28 = pauVar17[-1];
          auVar24 = *pauVar17;
          auVar35._0_8_ = auVar24._8_8_;
          auVar35._8_4_ = auVar24._0_4_;
          auVar35._12_4_ = auVar24._4_4_;
          auVar32._0_8_ = auVar28._8_8_;
          auVar32._8_4_ = auVar28._0_4_;
          auVar32._12_4_ = auVar28._4_4_;
          *(undefined1 (*) [16])(extraout_RAX + uVar19) = auVar35;
          *(undefined1 (*) [16])(extraout_RAX + uVar19 + 2) = auVar32;
          auVar24._0_8_ = auVar23._8_8_;
          auVar24._8_4_ = auVar23._0_4_;
          auVar24._12_4_ = auVar23._4_4_;
          *pauVar17 = auVar24;
          auVar23._0_8_ = auVar31._8_8_;
          auVar23._8_4_ = auVar31._0_4_;
          auVar23._12_4_ = auVar31._4_4_;
          pauVar17[-1] = auVar23;
          uVar19 = uVar19 + 4;
          pauVar17 = pauVar17 + -2;
        } while ((uVar20 & 0xfffffffffffffffc) != uVar19);
        if (uVar20 == uVar8) goto LAB_140688112;
      }
      puVar14 = extraout_RAX + uVar8;
      lVar11 = -uVar8;
      do {
        uVar8 = *puVar14;
        *puVar14 = *(ulonglong *)((longlong)extraout_RAX + lVar11 * 8 + lVar15);
        *(ulonglong *)((longlong)extraout_RAX + lVar11 * 8 + lVar15) = uVar8;
        lVar11 = lVar11 + -1;
        puVar14 = puVar14 + 1;
      } while (-lVar11 != uVar20);
    }
  }
LAB_140688112:
  lVar11 = *(longlong *)(in_RCX + 0x1b8);
  lVar15 = *(longlong *)(in_RCX + 0x1a0);
  uVar20 = *(ulonglong *)(in_RCX + 0x1a8);
  uVar8 = 0;
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
  for (puVar14 = extraout_RAX; puVar14 != extraout_RAX + local_80; puVar14 = puVar14 + 1) {
    uVar8 = uVar8 + 1;
    uVar19 = *puVar14;
    if (lVar11 != 0) {
      ::core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar21 = (byte)(extraout_RAX_01 >> 0x39);
      auVar23 = ZEXT216(CONCAT11(bVar21,bVar21));
      auVar23 = pshuflw(auVar23,auVar23,0);
      lVar16 = 0;
      uVar12 = extraout_RAX_01;
      while( true ) {
        uVar12 = uVar20 & uVar12;
        auVar31 = *(undefined1 (*) [16])(lVar15 + uVar12);
        cVar22 = auVar23[0];
        auVar33[0] = -(auVar31[0] == cVar22);
        cVar25 = auVar23[1];
        auVar33[1] = -(auVar31[1] == cVar25);
        cVar26 = auVar23[2];
        auVar33[2] = -(auVar31[2] == cVar26);
        cVar27 = auVar23[3];
        auVar33[3] = -(auVar31[3] == cVar27);
        auVar33[4] = -(auVar31[4] == cVar22);
        auVar33[5] = -(auVar31[5] == cVar25);
        auVar33[6] = -(auVar31[6] == cVar26);
        auVar33[7] = -(auVar31[7] == cVar27);
        auVar33[8] = -(auVar31[8] == cVar22);
        auVar33[9] = -(auVar31[9] == cVar25);
        auVar33[10] = -(auVar31[10] == cVar26);
        auVar33[0xb] = -(auVar31[0xb] == cVar27);
        auVar33[0xc] = -(auVar31[0xc] == cVar22);
        auVar33[0xd] = -(auVar31[0xd] == cVar25);
        auVar33[0xe] = -(auVar31[0xe] == cVar26);
        auVar33[0xf] = -(auVar31[0xf] == cVar27);
        uVar18 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe | (ushort)(auVar33[0xf] >> 7) << 0xf
        ;
        uVar13 = (uint)uVar18;
        while (uVar18 != 0) {
          uVar2 = 0;
          for (uVar3 = uVar13; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
            uVar2 = uVar2 + 1;
          }
          lVar10 = (uVar2 + uVar12 & uVar20) * -0xc0;
          if (uVar19 == *(ulonglong *)(lVar15 + -0xc0 + lVar10)) {
            lVar16 = *(longlong *)(lVar15 + -0x38 + lVar10);
            if (lVar16 == 0) goto LAB_140688260;
            local_70 = local_70 & 0xffffffffffff0000;
            uVar9 = (**(code **)(*(longlong *)(lVar10 + lVar15 + -0x30) + 0x50))
                              (lVar16,in_R8,&local_70);
            if ((char)uVar9 == '\0') goto LAB_140688260;
            if (local_80 < uVar8) {
                    // WARNING: Subroutine does not return
              ::core::slice::index::slice_end_index_len_fail();
            }
            local_64 = (undefined4)CONCAT71((int7)((ulonglong)uVar9 >> 8),1);
            local_80 = uVar8;
            goto LAB_140688271;
          }
          uVar18 = (ushort)(uVar13 - 1) & (ushort)uVar13;
          uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar18);
        }
        auVar29[0] = -(auVar31[0] == (char)iVar4);
        auVar29[1] = -(auVar31[1] == (char)((uint)iVar4 >> 8));
        auVar29[2] = -(auVar31[2] == (char)((uint)iVar4 >> 0x10));
        auVar29[3] = -(auVar31[3] == (char)((uint)iVar4 >> 0x18));
        auVar29[4] = -(auVar31[4] == (char)iVar5);
        auVar29[5] = -(auVar31[5] == (char)((uint)iVar5 >> 8));
        auVar29[6] = -(auVar31[6] == (char)((uint)iVar5 >> 0x10));
        auVar29[7] = -(auVar31[7] == (char)((uint)iVar5 >> 0x18));
        auVar29[8] = -(auVar31[8] == (char)iVar6);
        auVar29[9] = -(auVar31[9] == (char)((uint)iVar6 >> 8));
        auVar29[10] = -(auVar31[10] == (char)((uint)iVar6 >> 0x10));
        auVar29[0xb] = -(auVar31[0xb] == (char)((uint)iVar6 >> 0x18));
        auVar29[0xc] = -(auVar31[0xc] == (char)iVar7);
        auVar29[0xd] = -(auVar31[0xd] == (char)((uint)iVar7 >> 8));
        auVar29[0xe] = -(auVar31[0xe] == (char)((uint)iVar7 >> 0x10));
        auVar29[0xf] = -(auVar31[0xf] == (char)((uint)iVar7 >> 0x18));
        if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x1f,0) & 1) != 0) || (SUB161(auVar29 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x37,0) & 1) != 0) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
        uVar12 = uVar12 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
LAB_140688260:
  }
  local_64 = 0;
LAB_140688271:
  puVar14 = extraout_RAX + local_80;
  do {
    if (extraout_RAX == puVar14) {
LAB_14068839e:
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
    uVar8 = puVar14[-1];
    if (lVar11 != 0) {
      ::core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar21 = (byte)(extraout_RAX_02 >> 0x39);
      auVar23 = ZEXT216(CONCAT11(bVar21,bVar21));
      auVar23 = pshuflw(auVar23,auVar23,0);
      lVar16 = 0;
      uVar19 = extraout_RAX_02;
      while( true ) {
        uVar19 = uVar20 & uVar19;
        auVar31 = *(undefined1 (*) [16])(lVar15 + uVar19);
        cVar22 = auVar23[0];
        auVar34[0] = -(auVar31[0] == cVar22);
        cVar25 = auVar23[1];
        auVar34[1] = -(auVar31[1] == cVar25);
        cVar26 = auVar23[2];
        auVar34[2] = -(auVar31[2] == cVar26);
        cVar27 = auVar23[3];
        auVar34[3] = -(auVar31[3] == cVar27);
        auVar34[4] = -(auVar31[4] == cVar22);
        auVar34[5] = -(auVar31[5] == cVar25);
        auVar34[6] = -(auVar31[6] == cVar26);
        auVar34[7] = -(auVar31[7] == cVar27);
        auVar34[8] = -(auVar31[8] == cVar22);
        auVar34[9] = -(auVar31[9] == cVar25);
        auVar34[10] = -(auVar31[10] == cVar26);
        auVar34[0xb] = -(auVar31[0xb] == cVar27);
        auVar34[0xc] = -(auVar31[0xc] == cVar22);
        auVar34[0xd] = -(auVar31[0xd] == cVar25);
        auVar34[0xe] = -(auVar31[0xe] == cVar26);
        auVar34[0xf] = -(auVar31[0xf] == cVar27);
        uVar18 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf
        ;
        uVar13 = (uint)uVar18;
        while (uVar18 != 0) {
          uVar2 = 0;
          for (uVar3 = uVar13; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
            uVar2 = uVar2 + 1;
          }
          lVar10 = (uVar2 + uVar19 & uVar20) * -0xc0;
          if (uVar8 == *(ulonglong *)(lVar15 + -0xc0 + lVar10)) {
            lVar16 = *(longlong *)(lVar15 + -0x38 + lVar10);
            if (lVar16 == 0) goto LAB_140688380;
            local_70._0_2_ = CONCAT11((char)local_64,1);
            uVar9 = (**(code **)(*(longlong *)(lVar10 + lVar15 + -0x30) + 0x50))
                              (lVar16,in_R8,&local_70);
            if ((char)uVar9 == '\0') goto LAB_140688380;
            local_64 = (undefined4)CONCAT71((int7)((ulonglong)uVar9 >> 8),1);
            goto LAB_14068839e;
          }
          uVar18 = (ushort)(uVar13 - 1) & (ushort)uVar13;
          uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar18);
        }
        auVar30[0] = -(auVar31[0] == -1);
        auVar30[1] = -(auVar31[1] == -1);
        auVar30[2] = -(auVar31[2] == -1);
        auVar30[3] = -(auVar31[3] == -1);
        auVar30[4] = -(auVar31[4] == -1);
        auVar30[5] = -(auVar31[5] == -1);
        auVar30[6] = -(auVar31[6] == -1);
        auVar30[7] = -(auVar31[7] == -1);
        auVar30[8] = -(auVar31[8] == -1);
        auVar30[9] = -(auVar31[9] == -1);
        auVar30[10] = -(auVar31[10] == -1);
        auVar30[0xb] = -(auVar31[0xb] == -1);
        auVar30[0xc] = -(auVar31[0xc] == -1);
        auVar30[0xd] = -(auVar31[0xd] == -1);
        auVar30[0xe] = -(auVar31[0xe] == -1);
        auVar30[0xf] = -(auVar31[0xf] == -1);
        if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 || (SUB161(auVar30 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x1f,0) & 1) != 0) || (SUB161(auVar30 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x37,0) & 1) != 0) || (SUB161(auVar30 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x47,0) & 1) != 0) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') break;
        uVar19 = uVar19 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
LAB_140688380:
    puVar14 = puVar14 + -1;
  } while( true );
}

