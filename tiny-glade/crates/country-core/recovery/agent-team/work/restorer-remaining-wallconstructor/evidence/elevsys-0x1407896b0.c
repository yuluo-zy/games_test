
// WARNING: Removing unreachable block (ram,0x000140789e93)
// WARNING: Removing unreachable block (ram,0x000140789ee3)
// WARNING: Removing unreachable block (ram,0x000140789ef6)
// WARNING: Removing unreachable block (ram,0x000140789dea)
// WARNING: Removing unreachable block (ram,0x000140789f4b)
// WARNING: Removing unreachable block (ram,0x000140789f9f)
// WARNING: Removing unreachable block (ram,0x000140789a70)

void __cdecl
system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets(void)

{
  byte *pbVar1;
  longlong lVar2;
  char *pcVar3;
  byte bVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  undefined8 *puVar8;
  undefined4 *puVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  char extraout_AL;
  uint uVar15;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar16;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined1 (*pauVar17) [16];
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong *plVar18;
  uint uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  byte bVar22;
  longlong lVar23;
  undefined1 (*pauVar24) [16];
  longlong *in_R8;
  undefined *puVar25;
  longlong lVar26;
  ushort uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  longlong lVar32;
  char cVar33;
  byte bVar34;
  char cVar36;
  char cVar37;
  char cVar38;
  undefined1 auVar35 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar43;
  undefined1 unaff_XMM6_Bc;
  char cVar44;
  undefined1 unaff_XMM6_Bd;
  char cVar45;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar46;
  undefined1 unaff_XMM6_Bg;
  char cVar47;
  undefined1 unaff_XMM6_Bh;
  char cVar48;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar49;
  undefined1 unaff_XMM6_Bk;
  char cVar50;
  undefined1 unaff_XMM6_Bl;
  char cVar51;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar52;
  undefined1 unaff_XMM6_Bo;
  char cVar53;
  undefined1 unaff_XMM6_Bp;
  char cVar54;
  undefined8 *in_stack_00000040;
  byte local_19c;
  longlong local_90;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar20 = 0x53;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar20 != 0) {
        if (uVar20 < 0x53) {
          if ((char)(&DAT_142a804b0)[uVar20] < -0x40) {
LAB_14078974c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 != 0x53) goto LAB_14078974c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar20 != 0xfffffffffffffffe)) {
        if (uVar20 + 2 < 0x53) {
          if ((char)(&UNK_142a804b2)[uVar20] < -0x40) {
LAB_1407897a3:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 + 2 != 0x53) goto LAB_1407897a3;
      }
    }
    puVar25 = &DAT_142a806c7;
    do {
      bVar22 = puVar25[-1];
      uVar15 = (uint)(char)bVar22;
      if ((char)bVar22 < '\0') {
        bVar34 = puVar25[-2];
        if ((char)bVar34 < -0x40) {
          bVar4 = puVar25[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar25 + -4;
            puVar25 = puVar25 + -4;
            uVar15 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar25 = puVar25 + -3;
            uVar15 = bVar4 & 0xf;
          }
          uVar15 = bVar34 & 0x3f | uVar15 << 6;
        }
        else {
          puVar25 = puVar25 + -2;
          uVar15 = bVar34 & 0x1f;
        }
        uVar15 = (uint)(bVar22 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_140789864;
LAB_140789875:
        if (puVar25 == &UNK_142a8067f) break;
        if (puVar25 + -0x142a8067f < (undefined *)0x47) {
          if (-0x41 < (char)puVar25[1]) break;
        }
        else if (puVar25 + -0x142a8067f == (undefined *)0x47) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar25 = puVar25 + -1;
      if (uVar15 == 0x5c) goto LAB_140789875;
LAB_140789864:
      if (uVar15 == 0x2f) goto LAB_140789875;
    } while (puVar25 != &DAT_142a80680);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_14078a0fb:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_14078a107:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar18 = *plVar18 + 1;
  }
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar20 = *puVar6;
  uVar29 = 0;
  uVar21 = uVar20 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar20 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar21 = uVar29;
  }
  uVar31 = uVar20 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar20 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar31 = uVar29;
  }
  local_90 = uVar21 * 0x28 + *(longlong *)(lVar7 + 8);
  uVar20 = *(ulonglong *)(lVar7 + 0x10);
  uVar28 = uVar20 - uVar21;
  if (uVar20 < uVar21) {
    uVar28 = uVar29;
  }
  if (uVar20 < uVar21) {
    local_90 = 8;
  }
  lVar16 = uVar31 * 0x28 + *(longlong *)(lVar7 + 0x28);
  uVar20 = *(ulonglong *)(lVar7 + 0x30) - uVar31;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar31) {
    lVar16 = 8;
    uVar20 = uVar29;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar20) - uVar28;
  lVar7 = local_90 + uVar28 * 0x28;
  lVar2 = lVar16 + uVar20 * 0x28;
  plVar18 = (longlong *)*in_R8;
  uVar5 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  puVar8 = (undefined8 *)*in_stack_00000040;
  puVar9 = (undefined4 *)in_stack_00000040[2];
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar43 = (char)((uint)iVar11 >> 8);
  cVar44 = (char)((uint)iVar11 >> 0x10);
  cVar45 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar46 = (char)((uint)iVar12 >> 8);
  cVar47 = (char)((uint)iVar12 >> 0x10);
  cVar48 = (char)((uint)iVar12 >> 0x18);
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar49 = (char)((uint)iVar13 >> 8);
  cVar50 = (char)((uint)iVar13 >> 0x10);
  cVar51 = (char)((uint)iVar13 >> 0x18);
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar52 = (char)((uint)iVar14 >> 8);
  cVar53 = (char)((uint)iVar14 >> 0x10);
  cVar54 = (char)((uint)iVar14 >> 0x18);
LAB_140789ad1:
  if ((local_90 == 0) || (local_90 == lVar7)) {
    if ((lVar16 == 0) || (lVar16 == lVar2)) {
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_04 == 1) {
          plVar18 = extraout_RAX_04 + 1;
        }
        else if ((*extraout_RAX_04 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar18 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
        goto LAB_14078a0fb;
        if (*plVar18 != 0) goto LAB_14078a107;
        *plVar18 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar18 = *plVar18 + 1;
      }
      return;
    }
    local_90 = 0;
    lVar23 = lVar16;
    lVar16 = lVar16 + 0x28;
  }
  else {
    lVar23 = local_90;
    local_90 = local_90 + 0x28;
  }
  *puVar6 = *puVar6 + 1;
  lVar23 = *(longlong *)(lVar23 + 0x18);
  if (plVar18[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar32 = *plVar18;
    bVar22 = (byte)(extraout_RAX_02 >> 0x39);
    auVar35 = ZEXT216(CONCAT11(bVar22,bVar22));
    auVar35 = pshuflw(auVar35,auVar35,0);
    lVar26 = 0;
    uVar20 = extraout_RAX_02;
    while( true ) {
      uVar20 = plVar18[1] & uVar20;
      pcVar3 = (char *)(lVar32 + uVar20);
      cVar33 = auVar35[0];
      auVar41[0] = -(*pcVar3 == cVar33);
      cVar36 = auVar35[1];
      auVar41[1] = -(pcVar3[1] == cVar36);
      cVar37 = auVar35[2];
      auVar41[2] = -(pcVar3[2] == cVar37);
      cVar38 = auVar35[3];
      auVar41[3] = -(pcVar3[3] == cVar38);
      auVar41[4] = -(pcVar3[4] == cVar33);
      auVar41[5] = -(pcVar3[5] == cVar36);
      auVar41[6] = -(pcVar3[6] == cVar37);
      auVar41[7] = -(pcVar3[7] == cVar38);
      auVar41[8] = -(pcVar3[8] == cVar33);
      auVar41[9] = -(pcVar3[9] == cVar36);
      auVar41[10] = -(pcVar3[10] == cVar37);
      auVar41[0xb] = -(pcVar3[0xb] == cVar38);
      auVar41[0xc] = -(pcVar3[0xc] == cVar33);
      auVar41[0xd] = -(pcVar3[0xd] == cVar36);
      auVar41[0xe] = -(pcVar3[0xe] == cVar37);
      auVar41[0xf] = -(pcVar3[0xf] == cVar38);
      uVar27 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar27;
      while (uVar27 != 0) {
        uVar19 = 0;
        for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
          uVar19 = uVar19 + 1;
        }
        lVar30 = (uVar19 + uVar20 & plVar18[1]) * -0x2a8;
        if (lVar23 == *(longlong *)(lVar32 + -0x2a8 + lVar30)) {
          country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
          if ((local_19c & 1) == 0) goto LAB_140789ad1;
          rectangle_small_brackets::construct_rectangle_small_brackets();
          if (1.6 <= *(float *)(lVar32 + lVar30 + -0x1c)) {
            rectangle_large_brackets::construct_rectangle_large_brackets();
          }
          *puVar9 = uVar5;
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          pauVar17 = (undefined1 (*) [16])*puVar8;
          uVar20 = puVar8[1];
          uVar29 = uVar20 & extraout_RAX_03;
          bVar22 = (byte)(extraout_RAX_03 >> 0x38);
          bVar34 = bVar22 >> 1;
          auVar35 = ZEXT216(CONCAT11(bVar34,bVar34));
          auVar35 = pshuflw(auVar35,auVar35,0);
          lVar32 = 0;
          uVar21 = uVar29;
          goto LAB_140789d31;
        }
        uVar27 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar27);
      }
      auVar39[0] = -(*pcVar3 == (char)iVar11);
      auVar39[1] = -(pcVar3[1] == cVar43);
      auVar39[2] = -(pcVar3[2] == cVar44);
      auVar39[3] = -(pcVar3[3] == cVar45);
      auVar39[4] = -(pcVar3[4] == (char)iVar12);
      auVar39[5] = -(pcVar3[5] == cVar46);
      auVar39[6] = -(pcVar3[6] == cVar47);
      auVar39[7] = -(pcVar3[7] == cVar48);
      auVar39[8] = -(pcVar3[8] == (char)iVar13);
      auVar39[9] = -(pcVar3[9] == cVar49);
      auVar39[10] = -(pcVar3[10] == cVar50);
      auVar39[0xb] = -(pcVar3[0xb] == cVar51);
      auVar39[0xc] = -(pcVar3[0xc] == (char)iVar14);
      auVar39[0xd] = -(pcVar3[0xd] == cVar52);
      auVar39[0xe] = -(pcVar3[0xe] == cVar53);
      auVar39[0xf] = -(pcVar3[0xf] == cVar54);
      if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 || (SUB161(auVar39 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar39 >> 0x17,0) & 1) != 0) || (SUB161(auVar39 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar39 >> 0x27,0) & 1) != 0) || (SUB161(auVar39 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar39 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar39 >> 0x3f,0) & 1) != 0) || (SUB161(auVar39 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar39 >> 0x4f,0) & 1) != 0) || (SUB161(auVar39 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar39 >> 0x5f,0) & 1) != 0) || (SUB161(auVar39 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar39 >> 0x6f,0) & 1) != 0) || (SUB161(auVar39 >> 0x77,0) & 1) != 0) ||
          auVar39[0xf] < '\0') break;
      uVar20 = uVar20 + lVar26 + 0x10;
      lVar26 = lVar26 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_140789d31:
  pcVar3 = *pauVar17 + uVar21;
  cVar33 = auVar35[0];
  auVar42[0] = -(*pcVar3 == cVar33);
  cVar36 = auVar35[1];
  auVar42[1] = -(pcVar3[1] == cVar36);
  cVar37 = auVar35[2];
  auVar42[2] = -(pcVar3[2] == cVar37);
  cVar38 = auVar35[3];
  auVar42[3] = -(pcVar3[3] == cVar38);
  auVar42[4] = -(pcVar3[4] == cVar33);
  auVar42[5] = -(pcVar3[5] == cVar36);
  auVar42[6] = -(pcVar3[6] == cVar37);
  auVar42[7] = -(pcVar3[7] == cVar38);
  auVar42[8] = -(pcVar3[8] == cVar33);
  auVar42[9] = -(pcVar3[9] == cVar36);
  auVar42[10] = -(pcVar3[10] == cVar37);
  auVar42[0xb] = -(pcVar3[0xb] == cVar38);
  auVar42[0xc] = -(pcVar3[0xc] == cVar33);
  auVar42[0xd] = -(pcVar3[0xd] == cVar36);
  auVar42[0xe] = -(pcVar3[0xe] == cVar37);
  auVar42[0xf] = -(pcVar3[0xf] == cVar38);
  uVar27 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe | (ushort)(auVar42[0xf] >> 7) << 0xf;
  uVar15 = (uint)uVar27;
  while (uVar27 != 0) {
    uVar19 = 0;
    for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
      uVar19 = uVar19 + 1;
    }
    uVar31 = uVar19 + uVar21 & uVar20;
    if (*(longlong *)pauVar17[uVar31 * -4 + -4] == lVar23) {
      pauVar24 = pauVar17 + uVar31 * -4;
      goto LAB_140789dd5;
    }
    uVar27 = (ushort)(uVar15 - 1) & (ushort)uVar15;
    uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar27);
  }
  auVar40[0] = -(*pcVar3 == (char)iVar11);
  auVar40[1] = -(pcVar3[1] == cVar43);
  auVar40[2] = -(pcVar3[2] == cVar44);
  auVar40[3] = -(pcVar3[3] == cVar45);
  auVar40[4] = -(pcVar3[4] == (char)iVar12);
  auVar40[5] = -(pcVar3[5] == cVar46);
  auVar40[6] = -(pcVar3[6] == cVar47);
  auVar40[7] = -(pcVar3[7] == cVar48);
  auVar40[8] = -(pcVar3[8] == (char)iVar13);
  auVar40[9] = -(pcVar3[9] == cVar49);
  auVar40[10] = -(pcVar3[10] == cVar50);
  auVar40[0xb] = -(pcVar3[0xb] == cVar51);
  auVar40[0xc] = -(pcVar3[0xc] == (char)iVar14);
  auVar40[0xd] = -(pcVar3[0xd] == cVar52);
  auVar40[0xe] = -(pcVar3[0xe] == cVar53);
  auVar40[0xf] = -(pcVar3[0xf] == cVar54);
  if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar40 >> 0x17,0) & 1) != 0) || (SUB161(auVar40 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar40 >> 0x27,0) & 1) != 0) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar40 >> 0x37,0) & 1) != 0) || (SUB161(auVar40 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar40 >> 0x47,0) & 1) != 0) || (SUB161(auVar40 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar40 >> 0x57,0) & 1) != 0) || (SUB161(auVar40 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar40 >> 0x67,0) & 1) != 0) || (SUB161(auVar40 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar40 >> 0x77,0) & 1) != 0) || auVar40[0xf] < '\0') goto LAB_140789def;
  lVar26 = uVar21 + lVar32;
  lVar32 = lVar32 + 0x10;
  uVar21 = lVar26 + 0x10U & uVar20;
  goto LAB_140789d31;
LAB_140789def:
  if (puVar8[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::elevation_support_stats::ElevationSupports,std::hash::random::RandomState>_>
              ();
    pauVar17 = (undefined1 (*) [16])*puVar8;
    uVar20 = puVar8[1];
    uVar29 = extraout_RAX_03 & uVar20;
    auVar35 = *(undefined1 (*) [16])(*pauVar17 + uVar29);
    uVar15 = (uint)(ushort)((ushort)(SUB161(auVar35 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
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
                            (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar35[0xf] >> 7) << 0xf);
    if (uVar15 != 0) goto LAB_140789e15;
  }
  else {
    auVar35 = *(undefined1 (*) [16])(*pauVar17 + uVar29);
    uVar27 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar35[0xf] >> 7) << 0xf;
    uVar15 = (uint)uVar27;
    if (uVar27 != 0) goto LAB_140789e15;
  }
  lVar32 = 0x10;
  do {
    uVar29 = uVar29 + lVar32 & uVar20;
    auVar35 = *(undefined1 (*) [16])(*pauVar17 + uVar29);
    lVar32 = lVar32 + 0x10;
    uVar15 = (uint)(ushort)((ushort)(SUB161(auVar35 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
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
                            (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar35[0xf] >> 7) << 0xf);
  } while (uVar15 == 0);
LAB_140789e15:
  uVar19 = 0;
  for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
    uVar19 = uVar19 + 1;
  }
  uVar21 = uVar19 + uVar29 & uVar20;
  bVar34 = (*pauVar17)[uVar21];
  if (-1 < (char)bVar34) {
    auVar35 = *pauVar17;
    uVar15 = 0;
    for (uVar19 = (uint)(ushort)((ushort)(SUB161(auVar35 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
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
                                 (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar35[0xf] >> 7) << 0xf); (uVar19 & 1) == 0;
        uVar19 = uVar19 >> 1 | 0x80000000) {
      uVar15 = uVar15 + 1;
    }
    uVar21 = (ulonglong)uVar15;
    bVar34 = (*pauVar17)[uVar21];
  }
  bVar22 = bVar22 >> 1;
  (*pauVar17)[uVar21] = bVar22;
  pauVar17[1][uVar21 - 0x10 & uVar20] = bVar22;
  puVar8[2] = puVar8[2] - (ulonglong)(bVar34 & 1);
  pauVar24 = pauVar17 + uVar21 * -4;
  *(longlong *)pauVar17[uVar21 * -4 + -4] = lVar23;
  *(undefined8 *)(pauVar24[-4] + 8) = 0;
  *(undefined8 *)pauVar24[-3] = 4;
  *(undefined8 *)(pauVar24[-3] + 8) = 0;
  *(undefined8 *)pauVar24[-2] = 0;
  *(undefined8 *)(pauVar24[-2] + 8) = 0;
  *(undefined8 *)pauVar24[-1] = 0;
  pauVar24[-1][8] = 0;
  puVar8[3] = puVar8[3] + 1;
LAB_140789dd5:
  *(undefined8 *)pauVar24[-2] = 0;
  *(undefined8 *)(pauVar24[-2] + 8) = 0;
  goto LAB_140789ad1;
}

