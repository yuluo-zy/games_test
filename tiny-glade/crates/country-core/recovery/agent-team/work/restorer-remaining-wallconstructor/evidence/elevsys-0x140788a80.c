
// WARNING: Removing unreachable block (ram,0x00014078922f)
// WARNING: Removing unreachable block (ram,0x00014078928d)
// WARNING: Removing unreachable block (ram,0x00014078929d)

void __cdecl
system_wall_constructor::construct_elevation_supports::construct_rectangle_columns(void)

{
  byte *pbVar1;
  longlong lVar2;
  char *pcVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  longlong lVar10;
  undefined4 *puVar11;
  undefined8 *puVar12;
  undefined4 *puVar13;
  uint uVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  char extraout_AL;
  uint uVar22;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined1 (*pauVar23) [16];
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  undefined8 *in_RCX;
  longlong *plVar24;
  uint uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong *in_R8;
  undefined *puVar29;
  longlong lVar30;
  ushort uVar31;
  ulonglong uVar32;
  longlong lVar33;
  ulonglong uVar34;
  byte bVar35;
  undefined1 (*pauVar36) [16];
  ulonglong uVar37;
  char cVar38;
  byte bVar39;
  char cVar41;
  char cVar42;
  char cVar43;
  undefined1 auVar40 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar48;
  undefined1 unaff_XMM6_Bc;
  char cVar49;
  undefined1 unaff_XMM6_Bd;
  char cVar50;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar51;
  undefined1 unaff_XMM6_Bg;
  char cVar52;
  undefined1 unaff_XMM6_Bh;
  char cVar53;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar54;
  undefined1 unaff_XMM6_Bk;
  char cVar55;
  undefined1 unaff_XMM6_Bl;
  char cVar56;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar57;
  undefined1 unaff_XMM6_Bo;
  char cVar58;
  undefined1 unaff_XMM6_Bp;
  char cVar59;
  undefined8 *in_stack_00000038;
  longlong in_stack_00000040;
  byte local_194;
  longlong local_88;
  longlong local_80;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar26 = 0x52;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar26 != 0) {
        if (uVar26 < 0x52) {
          if ((char)(&DAT_142a80560)[uVar26] < -0x40) {
LAB_140788b1a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 != 0x52) goto LAB_140788b1a;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar26 != 0xfffffffffffffffe)) {
        if (uVar26 + 2 < 0x52) {
          if ((char)(&UNK_142a80562)[uVar26] < -0x40) {
LAB_140788b71:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 + 2 != 0x52) goto LAB_140788b71;
      }
    }
    puVar29 = &DAT_142a806c7;
    do {
      bVar35 = puVar29[-1];
      uVar22 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar39 = puVar29[-2];
        if ((char)bVar39 < -0x40) {
          bVar6 = puVar29[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar29 + -4;
            puVar29 = puVar29 + -4;
            uVar22 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar29 = puVar29 + -3;
            uVar22 = bVar6 & 0xf;
          }
          uVar22 = bVar39 & 0x3f | uVar22 << 6;
        }
        else {
          puVar29 = puVar29 + -2;
          uVar22 = bVar39 & 0x1f;
        }
        uVar22 = (uint)(bVar35 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_140788c34;
LAB_140788c46:
        if (puVar29 == &UNK_142a8067f) break;
        if (puVar29 + -0x142a8067f < (undefined *)0x47) {
          if (-0x41 < (char)puVar29[1]) break;
        }
        else if (puVar29 + -0x142a8067f == (undefined *)0x47) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar29 = puVar29 + -1;
      if (uVar22 == 0x5c) goto LAB_140788c46;
LAB_140788c34:
      if (uVar22 == 0x2f) goto LAB_140788c46;
    } while (puVar29 != &DAT_142a80680);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar24 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_1407894fc:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar24 != 0) {
LAB_140789508:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar24 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar24 = *plVar24 + 1;
  }
  puVar9 = (ulonglong *)*in_RCX;
  lVar10 = in_RCX[1];
  uVar26 = *puVar9;
  uVar27 = 0;
  uVar34 = uVar26 - *(ulonglong *)(lVar10 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar10 + 0x18)) {
    uVar34 = uVar27;
  }
  uVar37 = uVar26 - *(ulonglong *)(lVar10 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar10 + 0x38)) {
    uVar37 = uVar27;
  }
  local_88 = uVar34 * 0x28 + *(longlong *)(lVar10 + 8);
  uVar26 = *(ulonglong *)(lVar10 + 0x10);
  uVar32 = uVar26 - uVar34;
  if (uVar26 < uVar34) {
    uVar32 = uVar27;
  }
  if (uVar26 < uVar34) {
    local_88 = 8;
  }
  local_80 = uVar37 * 0x28 + *(longlong *)(lVar10 + 0x28);
  uVar26 = *(ulonglong *)(lVar10 + 0x30) - uVar37;
  if (*(ulonglong *)(lVar10 + 0x30) < uVar37) {
    local_80 = 8;
    uVar26 = uVar27;
  }
  *puVar9 = (*(longlong *)(lVar10 + 0x40) - uVar26) - uVar32;
  lVar10 = local_88 + uVar32 * 0x28;
  lVar2 = local_80 + uVar26 * 0x28;
  plVar24 = (longlong *)*in_R8;
  uVar7 = *(undefined4 *)(in_stack_00000040 + 0x1c);
  puVar11 = *(undefined4 **)(in_stack_00000040 + 0x10);
  uVar8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar12 = (undefined8 *)*in_stack_00000038;
  puVar13 = (undefined4 *)in_stack_00000038[2];
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar48 = (char)((uint)iVar15 >> 8);
  cVar49 = (char)((uint)iVar15 >> 0x10);
  cVar50 = (char)((uint)iVar15 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar51 = (char)((uint)iVar16 >> 8);
  cVar52 = (char)((uint)iVar16 >> 0x10);
  cVar53 = (char)((uint)iVar16 >> 0x18);
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar54 = (char)((uint)iVar17 >> 8);
  cVar55 = (char)((uint)iVar17 >> 0x10);
  cVar56 = (char)((uint)iVar17 >> 0x18);
  iVar18 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar57 = (char)((uint)iVar18 >> 8);
  cVar58 = (char)((uint)iVar18 >> 0x10);
  cVar59 = (char)((uint)iVar18 >> 0x18);
LAB_140788e6a:
  if ((local_88 == 0) || (local_88 == lVar10)) {
    if ((local_80 == 0) || (local_80 == lVar2)) {
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_06 == 1) {
          plVar24 = extraout_RAX_06 + 1;
        }
        else if ((*extraout_RAX_06 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar24 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0))
        goto LAB_1407894fc;
        if (*plVar24 != 0) goto LAB_140789508;
        *plVar24 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar24 = *plVar24 + 1;
      }
      return;
    }
    local_88 = 0;
    lVar28 = local_80;
    local_80 = local_80 + 0x28;
  }
  else {
    lVar28 = local_88;
    local_88 = local_88 + 0x28;
  }
  *puVar9 = *puVar9 + 1;
  lVar28 = *(longlong *)(lVar28 + 0x18);
  if (plVar24[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar35 = (byte)(extraout_RAX_02 >> 0x39);
    auVar40 = ZEXT216(CONCAT11(bVar35,bVar35));
    auVar40 = pshuflw(auVar40,auVar40,0);
    lVar30 = 0;
    uVar26 = extraout_RAX_02;
    while( true ) {
      uVar26 = plVar24[1] & uVar26;
      pcVar3 = (char *)(*plVar24 + uVar26);
      cVar38 = auVar40[0];
      auVar46[0] = -(*pcVar3 == cVar38);
      cVar41 = auVar40[1];
      auVar46[1] = -(pcVar3[1] == cVar41);
      cVar42 = auVar40[2];
      auVar46[2] = -(pcVar3[2] == cVar42);
      cVar43 = auVar40[3];
      auVar46[3] = -(pcVar3[3] == cVar43);
      auVar46[4] = -(pcVar3[4] == cVar38);
      auVar46[5] = -(pcVar3[5] == cVar41);
      auVar46[6] = -(pcVar3[6] == cVar42);
      auVar46[7] = -(pcVar3[7] == cVar43);
      auVar46[8] = -(pcVar3[8] == cVar38);
      auVar46[9] = -(pcVar3[9] == cVar41);
      auVar46[10] = -(pcVar3[10] == cVar42);
      auVar46[0xb] = -(pcVar3[0xb] == cVar43);
      auVar46[0xc] = -(pcVar3[0xc] == cVar38);
      auVar46[0xd] = -(pcVar3[0xd] == cVar41);
      auVar46[0xe] = -(pcVar3[0xe] == cVar42);
      auVar46[0xf] = -(pcVar3[0xf] == cVar43);
      uVar31 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe | (ushort)(auVar46[0xf] >> 7) << 0xf;
      uVar22 = (uint)uVar31;
      while (uVar31 != 0) {
        uVar25 = 0;
        for (uVar14 = uVar22; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
          uVar25 = uVar25 + 1;
        }
        if (lVar28 == *(longlong *)(*plVar24 + -0x2a8 + (uVar25 + uVar26 & plVar24[1]) * -0x2a8)) {
          country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
          if ((local_194 & 1) == 0) goto LAB_140788e6a;
          *puVar11 = uVar7;
          country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage::
          clear_and_get_mut();
          rectangle_bottom_layer::construct_rect_bottom_layer();
          rectangle_pillars::construct_rectangle_pillars();
          *puVar13 = uVar8;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          pauVar23 = (undefined1 (*) [16])*puVar12;
          uVar26 = puVar12[1];
          uVar27 = uVar26 & extraout_RAX_04;
          bVar35 = (byte)(extraout_RAX_04 >> 0x38);
          bVar39 = bVar35 >> 1;
          auVar40 = ZEXT216(CONCAT11(bVar39,bVar39));
          auVar40 = pshuflw(auVar40,auVar40,0);
          lVar30 = 0;
          uVar34 = uVar27;
          goto LAB_1407890b8;
        }
        uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
        uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
      }
      auVar44[0] = -(*pcVar3 == (char)iVar15);
      auVar44[1] = -(pcVar3[1] == cVar48);
      auVar44[2] = -(pcVar3[2] == cVar49);
      auVar44[3] = -(pcVar3[3] == cVar50);
      auVar44[4] = -(pcVar3[4] == (char)iVar16);
      auVar44[5] = -(pcVar3[5] == cVar51);
      auVar44[6] = -(pcVar3[6] == cVar52);
      auVar44[7] = -(pcVar3[7] == cVar53);
      auVar44[8] = -(pcVar3[8] == (char)iVar17);
      auVar44[9] = -(pcVar3[9] == cVar54);
      auVar44[10] = -(pcVar3[10] == cVar55);
      auVar44[0xb] = -(pcVar3[0xb] == cVar56);
      auVar44[0xc] = -(pcVar3[0xc] == (char)iVar18);
      auVar44[0xd] = -(pcVar3[0xd] == cVar57);
      auVar44[0xe] = -(pcVar3[0xe] == cVar58);
      auVar44[0xf] = -(pcVar3[0xf] == cVar59);
      if ((((((((((((((((SUB161(auVar44 >> 7,0) & 1) != 0 || (SUB161(auVar44 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar44 >> 0x17,0) & 1) != 0) || (SUB161(auVar44 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar44 >> 0x27,0) & 1) != 0) || (SUB161(auVar44 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar44 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar44 >> 0x3f,0) & 1) != 0) || (SUB161(auVar44 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar44 >> 0x4f,0) & 1) != 0) || (SUB161(auVar44 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar44 >> 0x5f,0) & 1) != 0) || (SUB161(auVar44 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar44 >> 0x6f,0) & 1) != 0) || (SUB161(auVar44 >> 0x77,0) & 1) != 0) ||
          auVar44[0xf] < '\0') break;
      uVar26 = uVar26 + lVar30 + 0x10;
      lVar30 = lVar30 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_1407890b8:
  pcVar3 = *pauVar23 + uVar34;
  cVar38 = auVar40[0];
  auVar47[0] = -(*pcVar3 == cVar38);
  cVar41 = auVar40[1];
  auVar47[1] = -(pcVar3[1] == cVar41);
  cVar42 = auVar40[2];
  auVar47[2] = -(pcVar3[2] == cVar42);
  cVar43 = auVar40[3];
  auVar47[3] = -(pcVar3[3] == cVar43);
  auVar47[4] = -(pcVar3[4] == cVar38);
  auVar47[5] = -(pcVar3[5] == cVar41);
  auVar47[6] = -(pcVar3[6] == cVar42);
  auVar47[7] = -(pcVar3[7] == cVar43);
  auVar47[8] = -(pcVar3[8] == cVar38);
  auVar47[9] = -(pcVar3[9] == cVar41);
  auVar47[10] = -(pcVar3[10] == cVar42);
  auVar47[0xb] = -(pcVar3[0xb] == cVar43);
  auVar47[0xc] = -(pcVar3[0xc] == cVar38);
  auVar47[0xd] = -(pcVar3[0xd] == cVar41);
  auVar47[0xe] = -(pcVar3[0xe] == cVar42);
  auVar47[0xf] = -(pcVar3[0xf] == cVar43);
  uVar31 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf;
  uVar22 = (uint)uVar31;
  while (uVar31 != 0) {
    uVar25 = 0;
    for (uVar14 = uVar22; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
      uVar25 = uVar25 + 1;
    }
    uVar37 = uVar25 + uVar34 & uVar26;
    if (*(longlong *)pauVar23[uVar37 * -4 + -4] == lVar28) {
      pauVar36 = pauVar23 + uVar37 * -4;
      goto LAB_1407892df;
    }
    uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
    uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
  }
  auVar45[0] = -(*pcVar3 == (char)iVar15);
  auVar45[1] = -(pcVar3[1] == cVar48);
  auVar45[2] = -(pcVar3[2] == cVar49);
  auVar45[3] = -(pcVar3[3] == cVar50);
  auVar45[4] = -(pcVar3[4] == (char)iVar16);
  auVar45[5] = -(pcVar3[5] == cVar51);
  auVar45[6] = -(pcVar3[6] == cVar52);
  auVar45[7] = -(pcVar3[7] == cVar53);
  auVar45[8] = -(pcVar3[8] == (char)iVar17);
  auVar45[9] = -(pcVar3[9] == cVar54);
  auVar45[10] = -(pcVar3[10] == cVar55);
  auVar45[0xb] = -(pcVar3[0xb] == cVar56);
  auVar45[0xc] = -(pcVar3[0xc] == (char)iVar18);
  auVar45[0xd] = -(pcVar3[0xd] == cVar57);
  auVar45[0xe] = -(pcVar3[0xe] == cVar58);
  auVar45[0xf] = -(pcVar3[0xf] == cVar59);
  if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 || (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar45 >> 0x17,0) & 1) != 0) || (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar45 >> 0x27,0) & 1) != 0) || (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar45 >> 0x37,0) & 1) != 0) || (SUB161(auVar45 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar45 >> 0x47,0) & 1) != 0) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') goto LAB_140789184;
  lVar33 = uVar34 + lVar30;
  lVar30 = lVar30 + 0x10;
  uVar34 = lVar33 + 0x10U & uVar26;
  goto LAB_1407890b8;
LAB_140789184:
  if (puVar12[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports,std::hash::random::RandomState>_>
              ();
    pauVar23 = (undefined1 (*) [16])*puVar12;
    uVar26 = puVar12[1];
    uVar27 = extraout_RAX_04 & uVar26;
    auVar40 = *(undefined1 (*) [16])(*pauVar23 + uVar27);
    uVar22 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
    if (uVar22 != 0) goto LAB_1407891b0;
  }
  else {
    auVar40 = *(undefined1 (*) [16])(*pauVar23 + uVar27);
    uVar31 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
    uVar22 = (uint)uVar31;
    if (uVar31 != 0) goto LAB_1407891b0;
  }
  lVar30 = 0x10;
  do {
    uVar27 = uVar27 + lVar30 & uVar26;
    auVar40 = *(undefined1 (*) [16])(*pauVar23 + uVar27);
    lVar30 = lVar30 + 0x10;
    uVar22 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
  } while (uVar22 == 0);
LAB_1407891b0:
  uVar25 = 0;
  for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
    uVar25 = uVar25 + 1;
  }
  uVar34 = uVar25 + uVar27 & uVar26;
  bVar39 = (*pauVar23)[uVar34];
  if (-1 < (char)bVar39) {
    auVar40 = *pauVar23;
    uVar22 = 0;
    for (uVar25 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar40[0xf] >> 7) << 0xf); (uVar25 & 1) == 0;
        uVar25 = uVar25 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    uVar34 = (ulonglong)uVar22;
    bVar39 = (*pauVar23)[uVar34];
  }
  bVar35 = bVar35 >> 1;
  (*pauVar23)[uVar34] = bVar35;
  pauVar23[1][uVar34 - 0x10 & uVar26] = bVar35;
  puVar12[2] = puVar12[2] - (ulonglong)(bVar39 & 1);
  pauVar36 = pauVar23 + uVar34 * -4;
  *(longlong *)pauVar23[uVar34 * -4 + -4] = lVar28;
  *(undefined8 *)(pauVar36[-4] + 8) = 0;
  *(undefined8 *)pauVar36[-3] = 4;
  *(undefined8 *)(pauVar36[-3] + 8) = 0;
  *(undefined8 *)pauVar36[-2] = 0;
  *(undefined8 *)(pauVar36[-2] + 8) = 0;
  *(undefined8 *)pauVar36[-1] = 0;
  pauVar36[-1][8] = 0;
  puVar12[3] = puVar12[3] + 1;
LAB_1407892df:
  *(undefined8 *)pauVar36[-1] = 0;
  uVar26 = *(ulonglong *)(extraout_RAX_03 + 0x10);
  if (uVar26 == 0) {
    lVar28 = 4;
  }
  else {
    lVar30 = *(longlong *)(extraout_RAX_03 + 8);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_05 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (uVar26 == 1) {
      uVar34 = 0;
    }
    else {
      lVar28 = 0x20;
      uVar34 = 0;
      do {
        puVar4 = (undefined8 *)(lVar30 + -0x20 + lVar28);
        uVar19 = *puVar4;
        uVar20 = puVar4[1];
        puVar4 = (undefined8 *)(lVar30 + -0x10 + lVar28);
        uVar21 = puVar4[1];
        puVar5 = (undefined8 *)(extraout_RAX_05 + -0x10 + lVar28);
        *puVar5 = *puVar4;
        puVar5[1] = uVar21;
        puVar4 = (undefined8 *)(extraout_RAX_05 + -0x20 + lVar28);
        *puVar4 = uVar19;
        puVar4[1] = uVar20;
        uVar19 = ((undefined8 *)(lVar30 + lVar28))[1];
        puVar4 = (undefined8 *)(lVar30 + 0x10 + lVar28);
        uVar20 = *puVar4;
        uVar21 = puVar4[1];
        *(undefined8 *)(extraout_RAX_05 + lVar28) = *(undefined8 *)(lVar30 + lVar28);
        ((undefined8 *)(extraout_RAX_05 + lVar28))[1] = uVar19;
        puVar4 = (undefined8 *)(extraout_RAX_05 + 0x10 + lVar28);
        *puVar4 = uVar20;
        puVar4[1] = uVar21;
        uVar34 = uVar34 + 2;
        lVar28 = lVar28 + 0x40;
      } while ((uVar26 & 0xfffffffffffffffe) != uVar34);
    }
    lVar28 = extraout_RAX_05;
    if ((uVar26 & 1) != 0) {
      lVar33 = uVar34 * 0x20;
      uVar19 = *(undefined8 *)(lVar30 + lVar33);
      uVar20 = ((undefined8 *)(lVar30 + lVar33))[1];
      puVar4 = (undefined8 *)(lVar30 + 0x10 + lVar33);
      uVar21 = puVar4[1];
      puVar5 = (undefined8 *)(extraout_RAX_05 + 0x10 + lVar33);
      *puVar5 = *puVar4;
      puVar5[1] = uVar21;
      *(undefined8 *)(extraout_RAX_05 + lVar33) = uVar19;
      ((undefined8 *)(extraout_RAX_05 + lVar33))[1] = uVar20;
    }
  }
  if (*(longlong *)(pauVar36[-4] + 8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(ulonglong *)(pauVar36[-4] + 8) = uVar26;
  *(longlong *)pauVar36[-3] = lVar28;
  *(ulonglong *)(pauVar36[-3] + 8) = uVar26;
  goto LAB_140788e6a;
}

