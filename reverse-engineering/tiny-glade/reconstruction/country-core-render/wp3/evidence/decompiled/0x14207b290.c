// address 0x14207b290
// name nani_update_plaster
// signature void __cdecl nani_update_plaster(void)

void __cdecl country_core::render::nani::systems::nani_update_plaster(void)

{
  byte *pbVar1;
  uint *puVar2;
  char *pcVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *plVar10;
  uint uVar11;
  uint uVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  bool bVar17;
  bool bVar18;
  char extraout_AL;
  uint uVar19;
  uint uVar20;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  longlong lVar21;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *in_RCX;
  longlong *plVar22;
  longlong *in_RDX;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong lVar25;
  ulonglong *puVar26;
  ulonglong uVar27;
  bool bVar28;
  undefined *puVar29;
  longlong *in_R9;
  ushort uVar30;
  longlong lVar31;
  longlong lVar32;
  uint *puVar33;
  byte bVar34;
  char cVar35;
  char cVar37;
  char cVar38;
  char cVar39;
  undefined1 auVar36 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  char cVar44;
  undefined1 unaff_XMM7_Bc;
  char cVar45;
  undefined1 unaff_XMM7_Bd;
  char cVar46;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  char cVar47;
  undefined1 unaff_XMM7_Bg;
  char cVar48;
  undefined1 unaff_XMM7_Bh;
  char cVar49;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  char cVar50;
  undefined1 unaff_XMM7_Bk;
  char cVar51;
  undefined1 unaff_XMM7_Bl;
  char cVar52;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  char cVar53;
  undefined1 unaff_XMM7_Bo;
  char cVar54;
  undefined1 unaff_XMM7_Bp;
  char cVar55;
  longlong local_110;
  longlong local_e0;
  longlong local_d8;
  longlong local_b0;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar17 = false;
  }
  else {
    uVar23 = 0x38;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar23 != 0) {
        if (uVar23 < 0x38) {
          if ((char)(&DAT_142fd2f06)[uVar23] < -0x40) {
LAB_14207b331:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar23 != 0x38) goto LAB_14207b331;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar23 != 0xfffffffffffffffe)) {
        if (uVar23 + 2 < 0x38) {
          if ((char)(&UNK_142fd2f08)[uVar23] < -0x40) {
LAB_14207b38f:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar23 + 2 != 0x38) goto LAB_14207b38f;
      }
    }
    puVar29 = &DAT_142fd345e;
    do {
      bVar34 = puVar29[-1];
      uVar19 = (uint)(char)bVar34;
      if ((char)bVar34 < '\0') {
        bVar4 = puVar29[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar29[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar29 + -4;
            puVar29 = puVar29 + -4;
            uVar19 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar29 = puVar29 + -3;
            uVar19 = bVar5 & 0xf;
          }
          uVar19 = bVar4 & 0x3f | uVar19 << 6;
        }
        else {
          puVar29 = puVar29 + -2;
          uVar19 = bVar4 & 0x1f;
        }
        uVar19 = (uint)(bVar34 & 0x3f) | uVar19 << 6;
        if (uVar19 != 0x5c) goto LAB_14207b454;
LAB_14207b468:
        if (puVar29 == &UNK_142fd342f) break;
        if (puVar29 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar29[1]) break;
        }
        else if (puVar29 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar29 = puVar29 + -1;
      if (uVar19 == 0x5c) goto LAB_14207b468;
LAB_14207b454:
      if (uVar19 == 0x2f) goto LAB_14207b468;
    } while (puVar29 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar22 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar22 = extraout_RAX_11, extraout_RAX_11 == (longlong *)0x0)) {
LAB_14207be95:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar22 != 0) {
LAB_14207bea1:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar22 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar22 = *plVar22 + 1;
    bVar17 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  iVar7 = *(int *)((longlong)in_RCX + 0x14);
  puVar33 = *(uint **)(lVar8 + 0xf8);
  puVar2 = puVar33 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  plVar22 = (longlong *)*in_RDX;
  plVar10 = (longlong *)*in_R9;
  uVar20 = iVar7 - (int)in_RCX[2];
  uVar19 = 0xc233b000;
  if (uVar20 < 0xc233b000) {
    uVar19 = uVar20;
  }
  local_110 = 8;
  iVar13 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                  CONCAT13(unaff_XMM7_Bd,
                           CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
  cVar44 = (char)((uint)iVar13 >> 8);
  cVar45 = (char)((uint)iVar13 >> 0x10);
  cVar46 = (char)((uint)iVar13 >> 0x18);
  iVar14 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                  CONCAT13(unaff_XMM7_Bh,
                           CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
  cVar47 = (char)((uint)iVar14 >> 8);
  cVar48 = (char)((uint)iVar14 >> 0x10);
  cVar49 = (char)((uint)iVar14 >> 0x18);
  iVar15 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                  CONCAT13(unaff_XMM7_Bl,
                           CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
  cVar50 = (char)((uint)iVar15 >> 8);
  cVar51 = (char)((uint)iVar15 >> 0x10);
  cVar52 = (char)((uint)iVar15 >> 0x18);
  iVar16 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                  CONCAT13(unaff_XMM7_Bp,
                           CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
  cVar53 = (char)((uint)iVar16 >> 8);
  cVar54 = (char)((uint)iVar16 >> 0x10);
  cVar55 = (char)((uint)iVar16 >> 0x18);
  local_b0 = 0;
  uVar23 = 0;
  uVar27 = 0;
  local_e0 = 0;
  bVar18 = false;
  local_d8 = 0;
  bVar28 = false;
joined_r0x00014207b601:
  if (cVar6 == '\0') {
    do {
      if (uVar27 == uVar23) {
        if (puVar33 == puVar2) goto LAB_14207bdae;
        uVar20 = *puVar33;
        puVar33 = puVar33 + 1;
        if (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar20 * 0xa0) == 0) {
          do {
            if (puVar33 == puVar2) goto LAB_14207bdae;
            uVar20 = *puVar33;
            puVar33 = puVar33 + 1;
          } while (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar20 * 0xa0) ==
                   0);
          lVar25 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar20 * 0xa0;
        }
        else {
          lVar25 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar20 * 0xa0;
        }
        lVar32 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*(uint *)(lVar25 + 0x94) * 0x48;
        uVar23 = *(ulonglong *)
                  (*(longlong *)(lVar9 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar25 + 0x94) * 0x48)
        ;
        if ((*(ulonglong *)(lVar8 + 0x110) < uVar23) &&
           (*(longlong *)(*(longlong *)(lVar32 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
          uVar27 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar23 <= uVar27) goto LAB_14207b960;
LAB_14207b917:
          uVar23 = *(ulonglong *)(*(longlong *)(lVar32 + 0x38) + uVar27 * 8);
          if (uVar23 == 0) goto LAB_14207b960;
          local_b0 = *(longlong *)(*(longlong *)(lVar32 + 0x18) + 0x10 + ~uVar23 * 0x30);
          uVar23 = *(ulonglong *)(lVar8 + 0x128);
          uVar27 = *(ulonglong *)(lVar25 + 0x88);
          if (uVar23 < uVar27) goto LAB_14207b97e;
LAB_14207b9c0:
          bVar18 = false;
          uVar23 = *(ulonglong *)(lVar8 + 0x130);
          if (uVar27 <= uVar23) goto LAB_14207ba00;
LAB_14207b9d6:
          if (*(longlong *)(*(longlong *)(lVar25 + 0x80) + uVar23 * 8) == 0) goto LAB_14207ba00;
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          bVar28 = true;
          local_d8 = extraout_RAX_05;
        }
        else {
          uVar27 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar27 < uVar23) goto LAB_14207b917;
LAB_14207b960:
          local_b0 = 0;
          uVar23 = *(ulonglong *)(lVar8 + 0x128);
          uVar27 = *(ulonglong *)(lVar25 + 0x88);
          if (uVar27 <= uVar23) goto LAB_14207b9c0;
LAB_14207b97e:
          if (*(longlong *)(*(longlong *)(lVar25 + 0x80) + uVar23 * 8) == 0) goto LAB_14207b9c0;
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          bVar18 = true;
          uVar23 = *(ulonglong *)(lVar8 + 0x130);
          local_e0 = extraout_RAX_04;
          if (uVar23 < *(ulonglong *)(lVar25 + 0x88)) goto LAB_14207b9d6;
LAB_14207ba00:
          bVar28 = false;
        }
        local_110 = *(longlong *)(lVar25 + 0x50);
        uVar23 = *(ulonglong *)(lVar25 + 0x58);
        uVar27 = 0;
      }
      uVar20 = *(uint *)(local_110 + 8 + uVar27 * 0x10);
      uVar24 = (ulonglong)uVar20;
      if (((bVar18) && ((uint)(iVar7 - *(int *)(local_e0 + uVar24 * 4)) < uVar19)) ||
         ((bVar28 &&
          (uVar24 = (ulonglong)uVar20, (uint)(iVar7 - *(int *)(local_d8 + uVar24 * 4)) < uVar19))))
      goto LAB_14207bc83;
      uVar27 = uVar27 + 1;
    } while( true );
  }
  do {
    if (uVar27 == uVar23) {
      if (puVar33 == puVar2) goto LAB_14207bdae;
      uVar23 = (ulonglong)*puVar33;
      puVar33 = puVar33 + 1;
      lVar25 = *(longlong *)(lVar9 + 0x1a8);
      if (*(longlong *)(lVar25 + 0x10 + uVar23 * 0x48) == 0) {
        do {
          if (puVar33 == puVar2) goto LAB_14207bdae;
          uVar20 = *puVar33;
          puVar33 = puVar33 + 1;
        } while (*(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar20 * 0x48) == 0)
        ;
        lVar32 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar20 * 0x48;
        puVar26 = (ulonglong *)(lVar32 + 0x10);
        uVar23 = *(ulonglong *)(lVar8 + 0x110);
        uVar27 = *(ulonglong *)(lVar32 + 0x40);
        if (uVar23 < uVar27) goto LAB_14207bb0c;
LAB_14207bb50:
        uVar23 = *(ulonglong *)(lVar8 + 0x118);
        if (uVar27 <= uVar23) goto LAB_14207bba0;
LAB_14207bb67:
        uVar23 = *(ulonglong *)(*(longlong *)(lVar32 + 0x38) + uVar23 * 8);
        if (uVar23 == 0) goto LAB_14207bba0;
        local_b0 = *(longlong *)(*(longlong *)(lVar32 + 0x18) + 0x10 + ~uVar23 * 0x30);
        uVar23 = *(ulonglong *)(lVar8 + 0x128);
        if (uVar23 < uVar27) goto LAB_14207bbb7;
LAB_14207bbf0:
        bVar18 = false;
        uVar23 = *(ulonglong *)(lVar8 + 0x130);
        if (uVar27 <= uVar23) goto LAB_14207bc30;
LAB_14207bc06:
        if (*(longlong *)(*(longlong *)(lVar32 + 0x38) + uVar23 * 8) == 0) goto LAB_14207bc30;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        bVar28 = true;
        local_d8 = extraout_RAX_07;
      }
      else {
        lVar32 = lVar25 + uVar23 * 0x48;
        puVar26 = (ulonglong *)(lVar25 + uVar23 * 0x48 + 0x10);
        uVar23 = *(ulonglong *)(lVar8 + 0x110);
        uVar27 = *(ulonglong *)(lVar32 + 0x40);
        if (uVar27 <= uVar23) goto LAB_14207bb50;
LAB_14207bb0c:
        if (*(longlong *)(*(longlong *)(lVar32 + 0x38) + uVar23 * 8) == 0) goto LAB_14207bb50;
        uVar23 = *(ulonglong *)(lVar8 + 0x118);
        if (uVar23 < uVar27) goto LAB_14207bb67;
LAB_14207bba0:
        local_b0 = 0;
        uVar23 = *(ulonglong *)(lVar8 + 0x128);
        if (uVar27 <= uVar23) goto LAB_14207bbf0;
LAB_14207bbb7:
        if (*(longlong *)(*(longlong *)(lVar32 + 0x38) + uVar23 * 8) == 0) goto LAB_14207bbf0;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        bVar18 = true;
        uVar23 = *(ulonglong *)(lVar8 + 0x130);
        local_e0 = extraout_RAX_06;
        if (uVar23 < *(ulonglong *)(lVar32 + 0x40)) goto LAB_14207bc06;
LAB_14207bc30:
        bVar28 = false;
      }
      uVar23 = *puVar26;
      uVar27 = 0;
    }
    if (((bVar18) &&
        (uVar24 = uVar27 & 0xffffffff, (uint)(iVar7 - *(int *)(local_e0 + uVar24 * 4)) < uVar19)) ||
       ((bVar28 &&
        (uVar24 = uVar27 & 0xffffffff, (uint)(iVar7 - *(int *)(local_d8 + uVar24 * 4)) < uVar19))))
    {
LAB_14207bc83:
      lVar25 = *(longlong *)(local_b0 + uVar24 * 0x28 + 0x20);
      if (plVar22[3] != 0) {
        uVar27 = uVar27 + 1;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar32 = *plVar22;
        bVar34 = (byte)(extraout_RAX_08 >> 0x39);
        auVar36 = ZEXT216(CONCAT11(bVar34,bVar34));
        auVar36 = pshuflw(auVar36,auVar36,0);
        lVar21 = 0;
        uVar24 = extraout_RAX_08;
        while( true ) {
          uVar24 = plVar22[1] & uVar24;
          pcVar3 = (char *)(lVar32 + uVar24);
          cVar35 = auVar36[0];
          auVar43[0] = -(*pcVar3 == cVar35);
          cVar37 = auVar36[1];
          auVar43[1] = -(pcVar3[1] == cVar37);
          cVar38 = auVar36[2];
          auVar43[2] = -(pcVar3[2] == cVar38);
          cVar39 = auVar36[3];
          auVar43[3] = -(pcVar3[3] == cVar39);
          auVar43[4] = -(pcVar3[4] == cVar35);
          auVar43[5] = -(pcVar3[5] == cVar37);
          auVar43[6] = -(pcVar3[6] == cVar38);
          auVar43[7] = -(pcVar3[7] == cVar39);
          auVar43[8] = -(pcVar3[8] == cVar35);
          auVar43[9] = -(pcVar3[9] == cVar37);
          auVar43[10] = -(pcVar3[10] == cVar38);
          auVar43[0xb] = -(pcVar3[0xb] == cVar39);
          auVar43[0xc] = -(pcVar3[0xc] == cVar35);
          auVar43[0xd] = -(pcVar3[0xd] == cVar37);
          auVar43[0xe] = -(pcVar3[0xe] == cVar38);
          auVar43[0xf] = -(pcVar3[0xf] == cVar39);
          uVar30 = (ushort)(SUB161(auVar43 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar43[0xf] >> 7) << 0xf;
          uVar20 = (uint)uVar30;
          while (uVar30 != 0) {
            uVar11 = 0;
            for (uVar12 = uVar20; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
              uVar11 = uVar11 + 1;
            }
            lVar31 = (uVar11 + uVar24 & plVar22[1]) * -0x2a8;
            if (*(longlong *)(lVar32 + -0x2a8 + lVar31) == lVar25) {
              if ((*(ulonglong *)(lVar32 + -0x290 + lVar31) < 2) ||
                 (*(float *)(lVar32 + -0x270 + lVar31) <= 0.0)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              iced_widget::space::impl$1::
              size<enum2_<crash_reporter_gui::Message>,enum2_<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>_>
                        ();
              if (plVar10[3] == 0) goto code_r0x00014207b78f;
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
              bVar34 = (byte)(extraout_RAX_03 >> 0x39);
              auVar36 = ZEXT216(CONCAT11(bVar34,bVar34));
              auVar36 = pshuflw(auVar36,auVar36,0);
              lVar32 = 0;
              uVar24 = extraout_RAX_03;
              goto LAB_14207b6c0;
            }
            uVar30 = (ushort)(uVar20 - 1) & (ushort)uVar20;
            uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar30);
          }
          auVar41[0] = -(*pcVar3 == (char)iVar13);
          auVar41[1] = -(pcVar3[1] == cVar44);
          auVar41[2] = -(pcVar3[2] == cVar45);
          auVar41[3] = -(pcVar3[3] == cVar46);
          auVar41[4] = -(pcVar3[4] == (char)iVar14);
          auVar41[5] = -(pcVar3[5] == cVar47);
          auVar41[6] = -(pcVar3[6] == cVar48);
          auVar41[7] = -(pcVar3[7] == cVar49);
          auVar41[8] = -(pcVar3[8] == (char)iVar15);
          auVar41[9] = -(pcVar3[9] == cVar50);
          auVar41[10] = -(pcVar3[10] == cVar51);
          auVar41[0xb] = -(pcVar3[0xb] == cVar52);
          auVar41[0xc] = -(pcVar3[0xc] == (char)iVar16);
          auVar41[0xd] = -(pcVar3[0xd] == cVar53);
          auVar41[0xe] = -(pcVar3[0xe] == cVar54);
          auVar41[0xf] = -(pcVar3[0xf] == cVar55);
          if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar41 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar41 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar41 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar41 >> 0x2f,0) & 1) != 0) || (SUB161(auVar41 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar41 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar41 >> 0x47,0) & 1) != 0) || (SUB161(auVar41 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar41 >> 0x57,0) & 1) != 0) || (SUB161(auVar41 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar41 >> 0x67,0) & 1) != 0) || (SUB161(auVar41 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar41 >> 0x77,0) & 1) != 0) || auVar41[0xf] < '\0') break;
          uVar24 = uVar24 + lVar21 + 0x10;
          lVar21 = lVar21 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar27 = uVar27 + 1;
  } while( true );
LAB_14207b6c0:
  uVar24 = plVar10[1] & uVar24;
  pcVar3 = (char *)(*plVar10 + uVar24);
  cVar35 = auVar36[0];
  auVar42[0] = -(*pcVar3 == cVar35);
  cVar37 = auVar36[1];
  auVar42[1] = -(pcVar3[1] == cVar37);
  cVar38 = auVar36[2];
  auVar42[2] = -(pcVar3[2] == cVar38);
  cVar39 = auVar36[3];
  auVar42[3] = -(pcVar3[3] == cVar39);
  auVar42[4] = -(pcVar3[4] == cVar35);
  auVar42[5] = -(pcVar3[5] == cVar37);
  auVar42[6] = -(pcVar3[6] == cVar38);
  auVar42[7] = -(pcVar3[7] == cVar39);
  auVar42[8] = -(pcVar3[8] == cVar35);
  auVar42[9] = -(pcVar3[9] == cVar37);
  auVar42[10] = -(pcVar3[10] == cVar38);
  auVar42[0xb] = -(pcVar3[0xb] == cVar39);
  auVar42[0xc] = -(pcVar3[0xc] == cVar35);
  auVar42[0xd] = -(pcVar3[0xd] == cVar37);
  auVar42[0xe] = -(pcVar3[0xe] == cVar38);
  auVar42[0xf] = -(pcVar3[0xf] == cVar39);
  uVar30 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
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
  uVar20 = (uint)uVar30;
  while (uVar30 != 0) {
    uVar11 = 0;
    for (uVar12 = uVar20; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar11 = uVar11 + 1;
    }
    if (lVar25 == *(longlong *)(*plVar10 + -0x10 + (uVar11 + uVar24 & plVar10[1]) * -0x10))
    goto code_r0x00014207b78f;
    uVar30 = (ushort)(uVar20 - 1) & (ushort)uVar20;
    uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar30);
  }
  auVar40[0] = -(*pcVar3 == (char)iVar13);
  auVar40[1] = -(pcVar3[1] == cVar44);
  auVar40[2] = -(pcVar3[2] == cVar45);
  auVar40[3] = -(pcVar3[3] == cVar46);
  auVar40[4] = -(pcVar3[4] == (char)iVar14);
  auVar40[5] = -(pcVar3[5] == cVar47);
  auVar40[6] = -(pcVar3[6] == cVar48);
  auVar40[7] = -(pcVar3[7] == cVar49);
  auVar40[8] = -(pcVar3[8] == (char)iVar15);
  auVar40[9] = -(pcVar3[9] == cVar50);
  auVar40[10] = -(pcVar3[10] == cVar51);
  auVar40[0xb] = -(pcVar3[0xb] == cVar52);
  auVar40[0xc] = -(pcVar3[0xc] == (char)iVar16);
  auVar40[0xd] = -(pcVar3[0xd] == cVar53);
  auVar40[0xe] = -(pcVar3[0xe] == cVar54);
  auVar40[0xf] = -(pcVar3[0xf] == cVar55);
  if ((((((((((((((((SUB161(auVar40 >> 7,0) & 1) != 0 || (SUB161(auVar40 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar40 >> 0x17,0) & 1) != 0) || (SUB161(auVar40 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar40 >> 0x27,0) & 1) != 0) || (SUB161(auVar40 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar40 >> 0x37,0) & 1) != 0) || (SUB161(auVar40 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar40 >> 0x47,0) & 1) != 0) || (SUB161(auVar40 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar40 >> 0x57,0) & 1) != 0) || (SUB161(auVar40 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar40 >> 0x67,0) & 1) != 0) || (SUB161(auVar40 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar40 >> 0x77,0) & 1) != 0) || auVar40[0xf] < '\0') goto code_r0x00014207b78f;
  uVar24 = uVar24 + lVar32 + 0x10;
  lVar32 = lVar32 + 0x10;
  goto LAB_14207b6c0;
code_r0x00014207b78f:
  gpu_world::OwnedGpuWorldSubset::
  update_instance_data<country_core::render::nani::instance_data::PlasterInstanceData>();
  goto joined_r0x00014207b601;
LAB_14207bdae:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar8 = *extraout_RAX_02;
  if (lVar8 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar8 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar17) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_09 == 1) {
      plVar22 = extraout_RAX_09 + 1;
    }
    else if ((*extraout_RAX_09 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar22 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0))
    goto LAB_14207be95;
    if (*plVar22 != 0) goto LAB_14207bea1;
    *plVar22 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar22 = *plVar22 + 1;
  }
  return;
}

