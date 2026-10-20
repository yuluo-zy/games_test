// name: create_plaster
// addr: 0x14075d460
// size: 1686
// signature: void __cdecl create_plaster(void)


void __cdecl system_wall_constructor::plaster_systems::create_plaster(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  longlong lVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  char extraout_AL;
  uint uVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *in_RCX;
  longlong *plVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong *in_R8;
  undefined *puVar17;
  ulonglong uVar18;
  longlong lVar19;
  longlong *in_R9;
  ushort uVar20;
  longlong lVar21;
  longlong *plVar22;
  longlong lVar23;
  byte bVar24;
  char cVar25;
  char cVar27;
  char cVar28;
  char cVar29;
  undefined1 auVar26 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 unaff_XMM6_Ba;
  char cVar36;
  undefined1 unaff_XMM6_Bb;
  char cVar37;
  undefined1 unaff_XMM6_Bc;
  char cVar38;
  undefined1 unaff_XMM6_Bd;
  char cVar39;
  undefined1 unaff_XMM6_Be;
  char cVar40;
  undefined1 unaff_XMM6_Bf;
  char cVar41;
  undefined1 unaff_XMM6_Bg;
  char cVar42;
  undefined1 unaff_XMM6_Bh;
  char cVar43;
  undefined1 unaff_XMM6_Bi;
  char cVar44;
  undefined1 unaff_XMM6_Bj;
  char cVar45;
  undefined1 unaff_XMM6_Bk;
  char cVar46;
  undefined1 unaff_XMM6_Bl;
  char cVar47;
  undefined1 unaff_XMM6_Bm;
  char cVar48;
  undefined1 unaff_XMM6_Bn;
  char cVar49;
  undefined1 unaff_XMM6_Bo;
  char cVar50;
  undefined1 unaff_XMM6_Bp;
  char cVar51;
  undefined8 *in_stack_00000038;
  longlong local_70;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar14 = 0x38;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar14 != 0) {
        if (uVar14 < 0x38) {
          if ((char)(&DAT_142a7cf10)[uVar14] < -0x40) {
LAB_14075d4e5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 != 0x38) goto LAB_14075d4e5;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar14 != 0xfffffffffffffffe)) {
        if (uVar14 + 2 < 0x38) {
          if ((char)(&UNK_142a7cf12)[uVar14] < -0x40) {
LAB_14075d53c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 + 2 != 0x38) goto LAB_14075d53c;
      }
    }
    puVar17 = &DAT_142a7d0ee;
    do {
      bVar24 = puVar17[-1];
      uVar10 = (uint)(char)bVar24;
      if ((char)bVar24 < '\0') {
        bVar3 = puVar17[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar17[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar17 + -4;
            puVar17 = puVar17 + -4;
            uVar10 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar17 = puVar17 + -3;
            uVar10 = bVar4 & 0xf;
          }
          uVar10 = bVar3 & 0x3f | uVar10 << 6;
        }
        else {
          puVar17 = puVar17 + -2;
          uVar10 = bVar3 & 0x1f;
        }
        uVar10 = (uint)(bVar24 & 0x3f) | uVar10 << 6;
        if (uVar10 != 0x5c) goto LAB_14075d604;
LAB_14075d615:
        if (puVar17 == &UNK_142a7d0b7) break;
        if (puVar17 + -0x142a7d0b7 < (undefined *)0x36) {
          if (-0x41 < (char)puVar17[1]) break;
        }
        else if (puVar17 + -0x142a7d0b7 == (undefined *)0x36) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar17 = puVar17 + -1;
      if (uVar10 == 0x5c) goto LAB_14075d615;
LAB_14075d604:
      if (uVar10 == 0x2f) goto LAB_14075d615;
    } while (puVar17 != &DAT_142a7d0b8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar13 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14075db0f:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar13 != 0) {
LAB_14075db1b:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar13 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar13 = *plVar13 + 1;
  }
  puVar5 = (ulonglong *)*in_stack_00000038;
  lVar16 = in_stack_00000038[1];
  uVar14 = *puVar5;
  uVar15 = 0;
  uVar11 = uVar14 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar11 = uVar15;
  }
  uVar18 = uVar14 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar18 = uVar15;
  }
  local_70 = uVar11 * 0x10 + *(longlong *)(lVar16 + 8);
  uVar14 = *(ulonglong *)(lVar16 + 0x10) - uVar11;
  if (*(ulonglong *)(lVar16 + 0x10) < uVar11) {
    local_70 = 8;
    uVar14 = uVar15;
  }
  lVar12 = uVar18 * 0x10 + *(longlong *)(lVar16 + 0x28);
  uVar11 = *(ulonglong *)(lVar16 + 0x30) - uVar18;
  if (*(ulonglong *)(lVar16 + 0x30) < uVar18) {
    lVar12 = 8;
    uVar11 = uVar15;
  }
  *puVar5 = (*(longlong *)(lVar16 + 0x40) - uVar11) - uVar14;
  lVar16 = uVar14 * 0x10 + local_70;
  lVar21 = uVar11 * 0x10 + lVar12;
  plVar13 = (longlong *)*in_R9;
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar36 = (char)iVar9;
  cVar37 = (char)((uint)iVar9 >> 8);
  cVar38 = (char)((uint)iVar9 >> 0x10);
  cVar39 = (char)((uint)iVar9 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar40 = (char)iVar9;
  cVar41 = (char)((uint)iVar9 >> 8);
  cVar42 = (char)((uint)iVar9 >> 0x10);
  cVar43 = (char)((uint)iVar9 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar44 = (char)iVar9;
  cVar45 = (char)((uint)iVar9 >> 8);
  cVar46 = (char)((uint)iVar9 >> 0x10);
  cVar47 = (char)((uint)iVar9 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                 CONCAT13(unaff_XMM6_Bp,
                          CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar48 = (char)iVar9;
  cVar49 = (char)((uint)iVar9 >> 8);
  cVar50 = (char)((uint)iVar9 >> 0x10);
  cVar51 = (char)((uint)iVar9 >> 0x18);
LAB_14075d758:
  if ((local_70 == 0) || (local_70 == lVar16)) {
    if ((lVar12 == 0) || (lVar12 == lVar21)) {
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_05 == 1) {
          plVar13 = extraout_RAX_05 + 1;
        }
        else if ((*extraout_RAX_05 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar13 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
        goto LAB_14075db0f;
        if (*plVar13 != 0) goto LAB_14075db1b;
        *plVar13 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar13 = *plVar13 + 1;
      }
      return;
    }
    local_70 = 0;
    lVar19 = lVar12 + 0x10;
    lVar23 = lVar12;
  }
  else {
    lVar19 = lVar12;
    lVar23 = local_70;
    local_70 = local_70 + 0x10;
  }
  lVar12 = lVar19;
  *puVar5 = *puVar5 + 1;
  plVar22 = (longlong *)*in_R8;
  if (plVar22[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar24 = (byte)(extraout_RAX_02 >> 0x39);
    auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
    auVar26 = pshuflw(auVar26,auVar26,0);
    lVar19 = 0;
    uVar14 = extraout_RAX_02;
    while( true ) {
      uVar14 = plVar22[1] & uVar14;
      pcVar2 = (char *)(*plVar22 + uVar14);
      cVar25 = auVar26[0];
      auVar33[0] = -(*pcVar2 == cVar25);
      cVar27 = auVar26[1];
      auVar33[1] = -(pcVar2[1] == cVar27);
      cVar28 = auVar26[2];
      auVar33[2] = -(pcVar2[2] == cVar28);
      cVar29 = auVar26[3];
      auVar33[3] = -(pcVar2[3] == cVar29);
      auVar33[4] = -(pcVar2[4] == cVar25);
      auVar33[5] = -(pcVar2[5] == cVar27);
      auVar33[6] = -(pcVar2[6] == cVar28);
      auVar33[7] = -(pcVar2[7] == cVar29);
      auVar33[8] = -(pcVar2[8] == cVar25);
      auVar33[9] = -(pcVar2[9] == cVar27);
      auVar33[10] = -(pcVar2[10] == cVar28);
      auVar33[0xb] = -(pcVar2[0xb] == cVar29);
      auVar33[0xc] = -(pcVar2[0xc] == cVar25);
      auVar33[0xd] = -(pcVar2[0xd] == cVar27);
      auVar33[0xe] = -(pcVar2[0xe] == cVar28);
      auVar33[0xf] = -(pcVar2[0xf] == cVar29);
      uVar20 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe | (ushort)(auVar33[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar20;
      if (uVar20 != 0) {
        lVar6 = *(longlong *)(lVar23 + 8);
        do {
          uVar7 = 0;
          for (uVar8 = uVar10; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          if (lVar6 == *(longlong *)(*plVar22 + -0x2a8 + (uVar7 + uVar14 & plVar22[1]) * -0x2a8)) {
            if (plVar13[3] == 0) goto LAB_14075d910;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            bVar24 = (byte)(extraout_RAX_03 >> 0x39);
            auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
            auVar26 = pshuflw(auVar26,auVar26,0);
            lVar19 = 0;
            uVar14 = extraout_RAX_03;
            goto LAB_14075d8aa;
          }
          uVar20 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar20);
        } while (uVar20 != 0);
      }
      auVar30[0] = -(*pcVar2 == cVar36);
      auVar30[1] = -(pcVar2[1] == cVar37);
      auVar30[2] = -(pcVar2[2] == cVar38);
      auVar30[3] = -(pcVar2[3] == cVar39);
      auVar30[4] = -(pcVar2[4] == cVar40);
      auVar30[5] = -(pcVar2[5] == cVar41);
      auVar30[6] = -(pcVar2[6] == cVar42);
      auVar30[7] = -(pcVar2[7] == cVar43);
      auVar30[8] = -(pcVar2[8] == cVar44);
      auVar30[9] = -(pcVar2[9] == cVar45);
      auVar30[10] = -(pcVar2[10] == cVar46);
      auVar30[0xb] = -(pcVar2[0xb] == cVar47);
      auVar30[0xc] = -(pcVar2[0xc] == cVar48);
      auVar30[0xd] = -(pcVar2[0xd] == cVar49);
      auVar30[0xe] = -(pcVar2[0xe] == cVar50);
      auVar30[0xf] = -(pcVar2[0xf] == cVar51);
      if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 || (SUB161(auVar30 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar30 >> 0x17,0) & 1) != 0) || (SUB161(auVar30 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar30 >> 0x27,0) & 1) != 0) || (SUB161(auVar30 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar30 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar30 >> 0x3f,0) & 1) != 0) || (SUB161(auVar30 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar30 >> 0x4f,0) & 1) != 0) || (SUB161(auVar30 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar30 >> 0x5f,0) & 1) != 0) || (SUB161(auVar30 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar30 >> 0x6f,0) & 1) != 0) || (SUB161(auVar30 >> 0x77,0) & 1) != 0) ||
          auVar30[0xf] < '\0') break;
      uVar14 = uVar14 + lVar19 + 0x10;
      lVar19 = lVar19 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_14075d8aa:
  uVar14 = plVar13[1] & uVar14;
  pcVar2 = (char *)(*plVar13 + uVar14);
  cVar25 = auVar26[0];
  auVar34[0] = -(*pcVar2 == cVar25);
  cVar27 = auVar26[1];
  auVar34[1] = -(pcVar2[1] == cVar27);
  cVar28 = auVar26[2];
  auVar34[2] = -(pcVar2[2] == cVar28);
  cVar29 = auVar26[3];
  auVar34[3] = -(pcVar2[3] == cVar29);
  auVar34[4] = -(pcVar2[4] == cVar25);
  auVar34[5] = -(pcVar2[5] == cVar27);
  auVar34[6] = -(pcVar2[6] == cVar28);
  auVar34[7] = -(pcVar2[7] == cVar29);
  auVar34[8] = -(pcVar2[8] == cVar25);
  auVar34[9] = -(pcVar2[9] == cVar27);
  auVar34[10] = -(pcVar2[10] == cVar28);
  auVar34[0xb] = -(pcVar2[0xb] == cVar29);
  auVar34[0xc] = -(pcVar2[0xc] == cVar25);
  auVar34[0xd] = -(pcVar2[0xd] == cVar27);
  auVar34[0xe] = -(pcVar2[0xe] == cVar28);
  auVar34[0xf] = -(pcVar2[0xf] == cVar29);
  uVar20 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
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
           (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf;
  uVar10 = (uint)uVar20;
  while (uVar20 != 0) {
    uVar7 = 0;
    for (uVar8 = uVar10; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar7 = uVar7 + 1;
    }
    if (lVar6 == *(longlong *)(*plVar13 + -0x68 + (uVar7 + uVar14 & plVar13[1]) * -0x68)) {
      country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized();
      country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
      plVar22 = (longlong *)*in_RCX;
      if (plVar22[3] == 0) goto LAB_14075da10;
      goto LAB_14075d959;
    }
    uVar20 = (ushort)(uVar10 - 1) & (ushort)uVar10;
    uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar20);
  }
  auVar31[0] = -(*pcVar2 == cVar36);
  auVar31[1] = -(pcVar2[1] == cVar37);
  auVar31[2] = -(pcVar2[2] == cVar38);
  auVar31[3] = -(pcVar2[3] == cVar39);
  auVar31[4] = -(pcVar2[4] == cVar40);
  auVar31[5] = -(pcVar2[5] == cVar41);
  auVar31[6] = -(pcVar2[6] == cVar42);
  auVar31[7] = -(pcVar2[7] == cVar43);
  auVar31[8] = -(pcVar2[8] == cVar44);
  auVar31[9] = -(pcVar2[9] == cVar45);
  auVar31[10] = -(pcVar2[10] == cVar46);
  auVar31[0xb] = -(pcVar2[0xb] == cVar47);
  auVar31[0xc] = -(pcVar2[0xc] == cVar48);
  auVar31[0xd] = -(pcVar2[0xd] == cVar49);
  auVar31[0xe] = -(pcVar2[0xe] == cVar50);
  auVar31[0xf] = -(pcVar2[0xf] == cVar51);
  if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar31 >> 0x17,0) & 1) != 0) || (SUB161(auVar31 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x37,0) & 1) != 0) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') goto LAB_14075d910;
  uVar14 = uVar14 + lVar19 + 0x10;
  lVar19 = lVar19 + 0x10;
  goto LAB_14075d8aa;
LAB_14075d910:
  plVar22 = (longlong *)*in_RCX;
  if (plVar22[3] != 0) {
LAB_14075d959:
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar24 = (byte)(extraout_RAX_04 >> 0x39);
    auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
    auVar26 = pshuflw(auVar26,auVar26,0);
    lVar19 = 0;
    uVar14 = extraout_RAX_04;
    while( true ) {
      uVar14 = plVar22[1] & uVar14;
      pcVar2 = (char *)(*plVar22 + uVar14);
      cVar25 = auVar26[0];
      auVar35[0] = -(*pcVar2 == cVar25);
      cVar27 = auVar26[1];
      auVar35[1] = -(pcVar2[1] == cVar27);
      cVar28 = auVar26[2];
      auVar35[2] = -(pcVar2[2] == cVar28);
      cVar29 = auVar26[3];
      auVar35[3] = -(pcVar2[3] == cVar29);
      auVar35[4] = -(pcVar2[4] == cVar25);
      auVar35[5] = -(pcVar2[5] == cVar27);
      auVar35[6] = -(pcVar2[6] == cVar28);
      auVar35[7] = -(pcVar2[7] == cVar29);
      auVar35[8] = -(pcVar2[8] == cVar25);
      auVar35[9] = -(pcVar2[9] == cVar27);
      auVar35[10] = -(pcVar2[10] == cVar28);
      auVar35[0xb] = -(pcVar2[0xb] == cVar29);
      auVar35[0xc] = -(pcVar2[0xc] == cVar25);
      auVar35[0xd] = -(pcVar2[0xd] == cVar27);
      auVar35[0xe] = -(pcVar2[0xe] == cVar28);
      auVar35[0xf] = -(pcVar2[0xf] == cVar29);
      uVar20 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar20;
      if (uVar20 != 0) {
        do {
          uVar7 = 0;
          for (uVar8 = uVar10; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          if (*(longlong *)(lVar23 + 8) ==
              *(longlong *)(*plVar22 + -0x20 + (uVar7 + uVar14 & plVar22[1]) * -0x20))
          goto LAB_14075d758;
          uVar20 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar20);
        } while (uVar20 != 0);
      }
      auVar32[0] = -(*pcVar2 == cVar36);
      auVar32[1] = -(pcVar2[1] == cVar37);
      auVar32[2] = -(pcVar2[2] == cVar38);
      auVar32[3] = -(pcVar2[3] == cVar39);
      auVar32[4] = -(pcVar2[4] == cVar40);
      auVar32[5] = -(pcVar2[5] == cVar41);
      auVar32[6] = -(pcVar2[6] == cVar42);
      auVar32[7] = -(pcVar2[7] == cVar43);
      auVar32[8] = -(pcVar2[8] == cVar44);
      auVar32[9] = -(pcVar2[9] == cVar45);
      auVar32[10] = -(pcVar2[10] == cVar46);
      auVar32[0xb] = -(pcVar2[0xb] == cVar47);
      auVar32[0xc] = -(pcVar2[0xc] == cVar48);
      auVar32[0xd] = -(pcVar2[0xd] == cVar49);
      auVar32[0xe] = -(pcVar2[0xe] == cVar50);
      auVar32[0xf] = -(pcVar2[0xf] == cVar51);
      if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 || (SUB161(auVar32 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar32 >> 0x17,0) & 1) != 0) || (SUB161(auVar32 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar32 >> 0x27,0) & 1) != 0) || (SUB161(auVar32 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar32 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar32 >> 0x3f,0) & 1) != 0) || (SUB161(auVar32 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar32 >> 0x4f,0) & 1) != 0) || (SUB161(auVar32 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar32 >> 0x5f,0) & 1) != 0) || (SUB161(auVar32 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar32 >> 0x6f,0) & 1) != 0) || (SUB161(auVar32 >> 0x77,0) & 1) != 0) ||
          auVar32[0xf] < '\0') break;
      uVar14 = uVar14 + lVar19 + 0x10;
      lVar19 = lVar19 + 0x10;
    }
  }
LAB_14075da10:
  generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple_<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>_>,country_core::resources::walls::wall_holes::impl_1::holes_no_duplicates::closure_env_0>,country_core::resources::walls::wall_holes::impl_1::holes_no_duplicates::closure_env_1>_>
            ();
  goto LAB_14075d758;
}

