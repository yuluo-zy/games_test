// forest_tree_animation 0x14227a0d0 size=1879
// signature: void __cdecl forest_tree_animation(void)


void __cdecl forest::tree_animation::forest_tree_animation(void)

{
  byte *pbVar1;
  ulonglong *puVar2;
  undefined4 *puVar3;
  uint *puVar4;
  byte bVar5;
  byte bVar6;
  byte bVar7;
  char cVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined4 *puVar18;
  undefined8 uVar19;
  bool bVar20;
  bool bVar21;
  char extraout_AL;
  uint uVar22;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar23;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong *plVar24;
  longlong lVar25;
  ulonglong uVar26;
  longlong lVar27;
  longlong *in_RDX;
  ulonglong uVar28;
  longlong *in_R8;
  undefined *puVar29;
  longlong lVar30;
  longlong *in_R9;
  uint *puVar31;
  ulonglong uVar32;
  bool bVar33;
  float fVar34;
  float extraout_XMM0_Da;
  float fVar35;
  float fVar36;
  undefined8 local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar20 = false;
  }
  else {
    uVar28 = 0x2d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar28 != 0) {
        if (uVar28 < 0x2d) {
          if ((char)(&DAT_143037ed0)[uVar28] < -0x40) {
LAB_14227a162:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar28 != 0x2d) goto LAB_14227a162;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar28 != 0xfffffffffffffffe)) {
        if (uVar28 + 2 < 0x2d) {
          if ((char)(&UNK_143037ed2)[uVar28] < -0x40) {
LAB_14227a1c0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar28 + 2 != 0x2d) goto LAB_14227a1c0;
      }
    }
    puVar29 = &DAT_14303802b;
    do {
      bVar5 = puVar29[-1];
      uVar22 = (uint)(char)bVar5;
      if ((char)bVar5 < '\0') {
        bVar6 = puVar29[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar29[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar29 + -4;
            puVar29 = puVar29 + -4;
            uVar22 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar29 = puVar29 + -3;
            uVar22 = bVar7 & 0xf;
          }
          uVar22 = bVar6 & 0x3f | uVar22 << 6;
        }
        else {
          puVar29 = puVar29 + -2;
          uVar22 = bVar6 & 0x1f;
        }
        uVar22 = (uint)(bVar5 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_14227a284;
LAB_14227a295:
        if (puVar29 == &UNK_143037fff) break;
        if (puVar29 + -0x143037fff < (undefined *)0x2b) {
          if (-0x41 < (char)puVar29[1]) break;
        }
        else if (puVar29 + -0x143037fff == (undefined *)0x2b) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar29 = puVar29 + -1;
      if (uVar22 == 0x5c) goto LAB_14227a295;
LAB_14227a284:
      if (uVar22 == 0x2f) goto LAB_14227a295;
    } while (puVar29 != &DAT_143038000);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar24 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_14227a81d;
    if (*plVar24 != 0) goto LAB_14227a829;
    *plVar24 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar24 = *plVar24 + 1;
    bVar20 = true;
  }
  lVar12 = *in_RCX;
  lVar13 = in_RCX[1];
  uVar9 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar31 = *(uint **)(lVar12 + 0xf8);
  puVar4 = puVar31 + *(longlong *)(lVar12 + 0x100);
  cVar8 = *(char *)(lVar12 + 0x108);
  lVar14 = *in_RDX;
  uVar10 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar15 = *in_R9;
  puVar16 = (undefined4 *)in_R9[2];
  uVar11 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar17 = *in_R8;
  puVar18 = (undefined4 *)in_R8[2];
  local_e0 = 8;
  local_b0 = 0;
  uVar32 = 0;
  uVar28 = 0;
  local_d8 = 8;
  local_a8 = 0;
  bVar33 = false;
  do {
    if (cVar8 == '\0') {
      if (uVar32 == uVar28) {
        do {
          if (puVar31 == puVar4) goto LAB_14227a780;
          uVar22 = *puVar31;
          puVar31 = puVar31 + 1;
          uVar28 = *(ulonglong *)(*(longlong *)(lVar13 + 0x100) + 0x58 + (ulonglong)uVar22 * 0xa0);
        } while (uVar28 == 0);
        lVar23 = *(longlong *)(lVar13 + 0x100) + (ulonglong)uVar22 * 0xa0;
        if (*(ulonglong *)(lVar12 + 0x120) < *(ulonglong *)(lVar23 + 0x88)) {
          bVar33 = *(longlong *)(*(longlong *)(lVar23 + 0x80) + *(ulonglong *)(lVar12 + 0x120) * 8)
                   != 0;
        }
        else {
          bVar33 = false;
        }
        lVar25 = *(longlong *)
                  (*(longlong *)(lVar13 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar23 + 0x94) * 0x48
                  );
        lVar27 = *(longlong *)
                  (*(longlong *)(lVar13 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar23 + 0x94) * 0x48
                  );
        lVar30 = ~*(ulonglong *)(lVar27 + *(longlong *)(lVar12 + 0x110) * 8) * 0x30;
        local_b0 = *(longlong *)(lVar25 + 0x10 + lVar30);
        local_c0 = *(longlong *)(lVar25 + 0x28 + lVar30);
        lVar27 = ~*(ulonglong *)(lVar27 + *(longlong *)(lVar12 + 0x118) * 8) * 0x30;
        local_a8 = *(longlong *)(lVar25 + 0x10 + lVar27);
        local_b8 = *(longlong *)(lVar25 + 0x28 + lVar27);
        local_e0 = *(longlong *)(lVar23 + 0x50);
        uVar32 = 0;
      }
      local_e8 = *(undefined8 *)(local_e0 + uVar32 * 0x10);
      uVar26 = (ulonglong)*(uint *)(local_e0 + 8 + uVar32 * 0x10);
    }
    else {
      if (uVar32 == uVar28) {
        do {
          if (puVar31 == puVar4) goto LAB_14227a780;
          uVar22 = *puVar31;
          puVar31 = puVar31 + 1;
          uVar28 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x10 + (ulonglong)uVar22 * 0x48);
        } while (uVar28 == 0);
        lVar23 = *(longlong *)(lVar13 + 0x1a8) + (ulonglong)uVar22 * 0x48;
        lVar25 = *(longlong *)(lVar23 + 0x38);
        if (*(ulonglong *)(lVar12 + 0x120) < *(ulonglong *)(lVar23 + 0x40)) {
          bVar33 = *(longlong *)(lVar25 + *(ulonglong *)(lVar12 + 0x120) * 8) != 0;
        }
        else {
          bVar33 = false;
        }
        lVar27 = *(longlong *)(lVar23 + 0x18);
        lVar30 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar12 + 0x110) * 8) * 0x30;
        local_b0 = *(longlong *)(lVar27 + 0x10 + lVar30);
        local_c0 = *(longlong *)(lVar27 + 0x28 + lVar30);
        lVar25 = ~*(ulonglong *)(lVar25 + *(longlong *)(lVar12 + 0x118) * 8) * 0x30;
        local_a8 = *(longlong *)(lVar27 + 0x10 + lVar25);
        local_b8 = *(longlong *)(lVar27 + 0x28 + lVar25);
        local_d8 = *(longlong *)(lVar23 + 8);
        uVar32 = 0;
      }
      local_e8 = *(undefined8 *)(local_d8 + uVar32 * 8);
      uVar26 = uVar32 & 0xffffffff;
    }
    puVar2 = (ulonglong *)(local_b0 + uVar26 * 0x18);
    puVar3 = (undefined4 *)(local_c0 + uVar26 * 4);
    if (bVar33) {
      fVar34 = *(float *)((longlong)puVar2 + 0xc);
      fVar35 = *(float *)(lVar14 + 0x28) / -0.3;
      bVar21 = false;
    }
    else {
      fVar34 = *(float *)((longlong)puVar2 + 0xc);
      fVar35 = *(float *)(lVar14 + 0x28);
      if (*(float *)(puVar2 + 2) <= 0.0) {
        fVar35 = fVar35 + fVar35;
      }
      else {
        *puVar3 = uVar9;
        *(float *)(puVar2 + 2) = *(float *)(puVar2 + 2) - fVar35;
        fVar35 = 0.0;
      }
      bVar21 = true;
    }
    *puVar3 = uVar9;
    fVar35 = fVar35 + *(float *)((longlong)puVar2 + 0xc);
    fVar36 = 0.0;
    if (0.0 <= fVar35) {
      fVar36 = fVar35;
    }
    fVar35 = 1.0;
    if (fVar36 <= 1.0) {
      fVar35 = fVar36;
    }
    *(float *)((longlong)puVar2 + 0xc) = fVar35;
    if ((fVar35 != fVar34) || (NAN(fVar35) || NAN(fVar34))) {
      ::utils::easing_functions::EasingFunction::apply();
      fVar34 = *(float *)(puVar2 + 1);
      *(undefined4 *)(local_b8 + uVar26 * 4) = uVar9;
      *(float *)(uVar26 * 0x30 + local_a8 + 0xc) = extraout_XMM0_Da * fVar34;
    }
    uVar32 = uVar32 + 1;
    if (((!bVar21) && (fVar34 = *(float *)((longlong)puVar2 + 0xc), fVar34 == 0.0)) &&
       (!NAN(fVar34))) {
      *puVar18 = uVar11;
      lVar23 = *(longlong *)(lVar17 + 0x30);
      uVar19 = *(undefined8 *)(lVar17 + 0x40);
      if (lVar23 == *(longlong *)(lVar17 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar25 = *(longlong *)(lVar17 + 0x28);
      *(undefined8 *)(lVar25 + lVar23 * 0x10) = uVar19;
      *(undefined8 *)(lVar25 + 8 + lVar23 * 0x10) = local_e8;
      *(longlong *)(lVar17 + 0x30) = lVar23 + 1;
      *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      *puVar16 = uVar10;
      if (*(ulonglong *)(lVar15 + 0x10) <= *puVar2) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      *(undefined4 *)(*(longlong *)(lVar15 + 8) + 0x44 + *puVar2 * 0x50) = 0;
    }
  } while( true );
LAB_14227a780:
  if (bVar20) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar24 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar24 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_14227a81d:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar24 != 0) {
LAB_14227a829:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar24 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar24 = *plVar24 + 1;
  }
  return;
}

