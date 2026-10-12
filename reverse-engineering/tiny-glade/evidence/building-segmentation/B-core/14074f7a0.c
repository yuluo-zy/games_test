
void __cdecl
system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
          (void)

{
  byte *pbVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 uVar13;
  byte bVar14;
  byte bVar15;
  byte bVar16;
  float *pfVar17;
  float *pfVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  char extraout_AL;
  uint uVar22;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  float *extraout_RAX_02;
  longlong *plVar23;
  longlong *plVar24;
  longlong *plVar25;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar26;
  longlong lVar27;
  ulonglong in_RDX;
  int *piVar28;
  undefined1 *puVar29;
  undefined1 *puVar30;
  undefined *puVar31;
  float *pfVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  longlong *plVar35;
  int *piVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  undefined1 local_1e0 [136];
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined4 local_140;
  float *local_138;
  longlong *local_130;
  undefined8 local_128;
  undefined4 local_120;
  undefined8 local_118;
  longlong *local_f8;
  int *local_e8;
  undefined8 local_dc;
  undefined4 local_d4;
  longlong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  ulonglong local_b8;
  ulonglong local_b0;
  int *local_a8;
  ulonglong local_a0;
  longlong local_98;
  float *local_90;
  longlong *local_88;
  longlong *local_80;
  char local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_80 = (longlong *)in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_118 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 100) {
          if ((char)(&DAT_142a7bc30)[in_RDX] < -0x40) {
LAB_14074f825:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 100) goto LAB_14074f825;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 100) {
          if ("stem_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::f"
              [in_RDX] < -0x40) {
LAB_14074f879:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 100) goto LAB_14074f879;
      }
    }
    puVar31 = &DAT_142a7bdb4;
    do {
      bVar14 = puVar31[-1];
      uVar22 = (uint)(char)bVar14;
      if ((char)bVar14 < '\0') {
        bVar15 = puVar31[-2];
        if ((char)bVar15 < -0x40) {
          bVar16 = puVar31[-3];
          if ((char)bVar16 < -0x40) {
            pbVar1 = puVar31 + -4;
            puVar31 = puVar31 + -4;
            uVar22 = bVar16 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar31 = puVar31 + -3;
            uVar22 = bVar16 & 0xf;
          }
          uVar22 = bVar15 & 0x3f | uVar22 << 6;
        }
        else {
          puVar31 = puVar31 + -2;
          uVar22 = bVar15 & 0x1f;
        }
        uVar22 = (uint)(bVar14 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_14074f934;
LAB_14074f945:
        if (puVar31 == &UNK_142a7bd5f) break;
        if (puVar31 + -0x142a7bd5f < (undefined *)0x54) {
          if (-0x41 < (char)puVar31[1]) break;
        }
        else if (puVar31 + -0x142a7bd5f == (undefined *)0x54) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar31 = puVar31 + -1;
      if (uVar22 == 0x5c) goto LAB_14074f945;
LAB_14074f934:
      if (uVar22 == 0x2f) goto LAB_14074f945;
    } while (puVar31 != &anon_086649f75a833997fb930afe2067ec31_17_llvm_7514787467665543436);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar26 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar26 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1407500f3;
    if (*plVar26 != 0) goto LAB_1407500ff;
    *plVar26 = -1;
    local_88 = plVar26;
    puffin::ThreadProfiler::begin_scope();
    *local_88 = *local_88 + 1;
    local_118 = 1;
  }
  plVar26 = *(longlong **)((longlong)local_80 + 0xd0);
  plVar23 = *(longlong **)((longlong)local_80 + 0x98);
  if (plVar26 < *(longlong **)((longlong)local_80 + 0x98)) {
    plVar23 = plVar26;
  }
  local_98 = (longlong)plVar23 * 0xc;
  if ((longlong *)0xaaaaaaaaaaaaaaa < plVar23) {
LAB_14075010d:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  pfVar17 = *(float **)((longlong)local_80 + 200);
  local_88 = plVar23;
  local_72 = extraout_AL;
  if (plVar23 == (longlong *)0x0) {
    local_90 = (float *)&DAT_00000004;
  }
  else {
    pfVar18 = *(float **)((longlong)local_80 + 0x90);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (float *)0x0) goto LAB_14075010d;
    local_90 = extraout_RAX_02;
    if (&DAT_00000007 < local_88) {
      plVar23 = (longlong *)0x0;
      if ((pfVar17 + (longlong)local_88 <= extraout_RAX_02 ||
           (float *)((longlong)extraout_RAX_02 + local_98) <= pfVar17) &&
         ((float *)(local_98 + (longlong)pfVar18) <= extraout_RAX_02 ||
          (float *)((longlong)extraout_RAX_02 + local_98) <= pfVar18)) {
        plVar23 = (longlong *)((ulonglong)local_88 & 0xffffffffffffffc);
        lVar27 = 0;
        pfVar32 = pfVar17;
        do {
          fVar19 = *pfVar32;
          fVar20 = pfVar32[2];
          fVar21 = pfVar32[3];
          fVar3 = pfVar18[lVar27 + 9];
          fVar37 = pfVar18[lVar27 + 6];
          fVar4 = pfVar18[lVar27];
          fVar5 = pfVar18[lVar27 + 1];
          fVar6 = pfVar18[lVar27 + 2];
          fVar7 = pfVar18[lVar27 + 3];
          fVar8 = pfVar18[lVar27 + 10];
          fVar9 = pfVar18[lVar27 + 7];
          fVar10 = pfVar18[lVar27 + 0xb];
          fVar11 = pfVar18[lVar27 + 8];
          fVar12 = pfVar18[lVar27 + 5];
          fVar38 = fVar19 * 0.0;
          fVar39 = pfVar32[1] * 0.0;
          fVar40 = fVar20 * 0.0;
          fVar41 = fVar21 * 0.0;
          pfVar2 = extraout_RAX_02 + lVar27 + 4;
          *pfVar2 = pfVar18[lVar27 + 4] + pfVar32[1];
          pfVar2[1] = fVar39 + fVar12;
          pfVar2[2] = fVar37 + fVar40;
          pfVar2[3] = fVar9 + fVar20;
          pfVar2 = extraout_RAX_02 + lVar27;
          *pfVar2 = fVar4 + fVar38;
          pfVar2[1] = fVar5 + fVar19;
          pfVar2[2] = fVar38 + fVar6;
          pfVar2[3] = fVar7 + fVar39;
          pfVar2 = extraout_RAX_02 + lVar27 + 8;
          *pfVar2 = fVar40 + fVar11;
          pfVar2[1] = fVar3 + fVar41;
          pfVar2[2] = fVar8 + fVar21;
          pfVar2[3] = fVar41 + fVar10;
          lVar27 = lVar27 + 0xc;
          pfVar32 = pfVar32 + 4;
        } while ((longlong)plVar23 * 3 != lVar27);
        if (local_88 == plVar23) goto LAB_14074fb18;
      }
    }
    else {
      plVar23 = (longlong *)0x0;
    }
    lVar27 = (longlong)plVar23 * 0xc + 8;
    do {
      pfVar32 = pfVar17 + (longlong)plVar23;
      plVar23 = (longlong *)((longlong)plVar23 + 1);
      fVar37 = *pfVar32 * 0.0;
      fVar3 = *(float *)((longlong)pfVar18 + lVar27);
      uVar13 = *(undefined8 *)((longlong)pfVar18 + lVar27 + -8);
      *(ulonglong *)((longlong)extraout_RAX_02 + lVar27 + -8) =
           CONCAT44(*pfVar32 + (float)((ulonglong)uVar13 >> 0x20),fVar37 + (float)uVar13);
      *(float *)((longlong)extraout_RAX_02 + lVar27) = fVar3 + fVar37;
      lVar27 = lVar27 + 0xc;
    } while (local_88 != plVar23);
  }
LAB_14074fb18:
  plVar23 = local_80;
  local_d0 = 0;
  lStack_c8 = 8;
  local_c0 = 0;
  if (*(uint *)((longlong)local_80 + 600) < 2) {
    country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
    country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
    ::utils::inverse_lerp();
  }
  local_b0 = 0;
  local_a8 = (int *)&DAT_00000004;
  local_a0 = 0;
  uVar33 = 0;
  uVar34 = uVar33;
  local_f8 = in_RCX;
  if (plVar26 != (longlong *)0x0) {
    plVar24 = (longlong *)0x0;
    do {
      while (plVar25 = plVar24, pfVar17[(longlong)plVar24] <= 0.0) {
        do {
          plVar24 = (longlong *)((longlong)plVar25 + 1);
          if (plVar26 <= plVar24) goto LAB_14074fe42;
          puVar30 = (undefined1 *)((longlong)plVar25 + 1);
          plVar25 = plVar24;
        } while (pfVar17[(longlong)puVar30] <= 0.0);
        if (plVar26 <= plVar24) goto LAB_14074fe42;
      }
      plVar23 = (longlong *)((longlong)plVar24 + 1);
      if ((longlong *)((longlong)plVar24 + 1) < plVar26) {
        plVar23 = plVar26;
      }
      do {
        plVar35 = plVar23;
        if ((longlong *)((longlong)plVar23 + -1) == plVar25) break;
        puVar30 = (undefined1 *)((longlong)plVar25 + 1);
        plVar35 = (longlong *)((longlong)plVar25 + 1);
        plVar25 = plVar35;
      } while (0.0 < pfVar17[(longlong)puVar30]);
      puVar30 = (undefined1 *)0x0;
      if (plVar24 != (longlong *)0x0) {
        puVar30 = (undefined1 *)((longlong)plVar24 + -1);
      }
      puVar29 = (undefined1 *)((longlong)plVar35 + (ulonglong)(plVar35 < plVar26));
      if (1 < (ulonglong)((longlong)puVar29 - (longlong)puVar30)) {
        if (uVar33 == local_b0) {
          local_71 = 1;
          alloc::raw_vec::RawVec::
          grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                    ();
        }
        local_a8[uVar33 * 2] = (int)puVar30;
        local_a8[uVar33 * 2 + 1] = (int)puVar29;
        uVar33 = uVar33 + 1;
        local_a0 = uVar33;
      }
      plVar24 = plVar35;
      plVar23 = local_80;
    } while (plVar35 < plVar26);
LAB_14074fe42:
    uVar34 = uVar33;
    if (((*(int *)((longlong)plVar23 + 600) != 2) && (1 < uVar33)) && (*local_a8 == 0)) {
      if (local_a8 + uVar33 * 2 == (int *)&DAT_00000008) {
        local_71 = 1;
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      if ((local_a8 + uVar33 * 2)[-1] == (int)plVar26) {
        uVar34 = uVar33 - 1;
        *local_a8 = (local_a8[uVar33 * 2 + -2] - *(int *)((longlong)plVar23 + 0x98)) + 1;
        local_a0 = uVar34;
      }
    }
  }
  local_b8 = local_b0;
  piVar28 = local_a8 + uVar34 * 2;
  local_e8 = local_a8;
  for (piVar36 = local_a8; piVar36 != piVar28; piVar36 = piVar36 + 2) {
    if (*(int *)((longlong)plVar23 + 600) == 2) {
      ::utils::loop_range::LoopRange::new_line();
    }
    else {
      ::utils::loop_range::LoopRange::new_loop();
    }
    local_140 = local_d4;
    local_148 = local_dc;
    local_158 = *(undefined8 *)((longlong)plVar23 + 0x90);
    uStack_150 = *(undefined8 *)((longlong)plVar23 + 0x98);
    local_120 = local_d4;
    local_128 = local_dc;
    local_138 = local_90;
    local_130 = local_88;
    process_segment();
    lVar27 = local_c0;
    if (local_c0 == local_d0) {
      alloc::raw_vec::RawVec::
      grow_one<enum2_<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>();
    }
    memmove((void *)(lVar27 * 0x88 + lStack_c8),local_1e0,0x88);
    local_c0 = lVar27 + 1;
    plVar23 = local_80;
  }
  if (local_b8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_f8[2] = local_c0;
  *local_f8 = local_d0;
  local_f8[1] = lStack_c8;
  if (local_88 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_72 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      local_80 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_80 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1407500f3:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*local_80 != 0) {
LAB_1407500ff:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *local_80 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_80 = *local_80 + 1;
  }
  return;
}

