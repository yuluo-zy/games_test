// name: split_into_random_rows
// addr: 0x14123e110
// size: 1486
// signature: void __cdecl split_into_random_rows(void)


void __cdecl system_wall_constructor::utils::split_into_random_rows::split_into_random_rows(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char extraout_AL;
  bool bVar5;
  uint uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *in_RCX;
  float *pfVar7;
  longlong *plVar8;
  float *in_RDX;
  float *pfVar9;
  ulonglong uVar10;
  longlong in_R8;
  undefined *puVar11;
  float *pfVar12;
  float fVar13;
  float fVar14;
  float in_XMM3_Da;
  undefined4 in_XMM3_Dc;
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  float fVar17;
  float *in_stack_00000028;
  longlong local_140;
  float *local_138;
  longlong local_130;
  longlong local_108;
  
  pfVar9 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (pfVar9 != (float *)0x0) {
        if (&DAT_0000004d < pfVar9) {
          if (pfVar9 != (float *)&DAT_0000004e) goto LAB_14123e1c7;
        }
        else if (*(char *)(pfVar9 + 0x50b07c20) < -0x40) {
LAB_14123e1c7:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (pfVar9 != (float *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)pfVar9 + 2) < (undefined1 *)0x4e) {
          if (*(char *)((longlong)pfVar9 + 0x142c1f082) < -0x40) {
LAB_14123e21b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)pfVar9 + 2) != (undefined1 *)0x4e) goto LAB_14123e21b;
      }
    }
    puVar11 = &DAT_142c1f1e3;
    do {
      bVar2 = puVar11[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_14123e2d4;
LAB_14123e2e5:
        if (puVar11 == &UNK_142c1f19f) break;
        if (puVar11 + -0x142c1f19f < (undefined *)0x43) {
          if (-0x41 < (char)puVar11[1]) break;
        }
        else if (puVar11 + -0x142c1f19f == (undefined *)0x43) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar11 = puVar11 + -1;
      if (uVar6 == 0x5c) goto LAB_14123e2e5;
LAB_14123e2d4:
      if (uVar6 == 0x2f) goto LAB_14123e2e5;
    } while (puVar11 != &DAT_142c1f1a0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar8 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar8 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_14123e706;
    if (*plVar8 != 0) goto LAB_14123e712;
    *plVar8 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar8 = *plVar8 + 1;
  }
  local_108 = 0;
  bVar5 = true;
  pfVar9 = in_RDX;
LAB_14123e415:
  if (pfVar9 != in_RDX + in_R8) {
    pfVar12 = pfVar9 + 1;
    pfVar7 = in_stack_00000028;
    in_stack_00000028 = pfVar9;
    if (bVar5) {
      if (pfVar12 == in_RDX + in_R8) goto LAB_14123e623;
      pfVar7 = pfVar9;
      in_stack_00000028 = pfVar12;
      pfVar12 = pfVar9 + 2;
    }
    fVar14 = *in_stack_00000028;
    fVar17 = *pfVar7;
    fVar13 = floorf((fVar14 - fVar17) / 0.46);
    uVar10 = (longlong)(fVar13 - 9.223372e+18) & (longlong)fVar13 >> 0x3f | (longlong)fVar13;
    if (fVar13 < 0.0) {
      uVar10 = 0;
    }
    if (1.8446743e+19 < fVar13) {
      uVar10 = 0xffffffffffffffff;
    }
    if (uVar10 < 3) {
      uVar10 = 2;
    }
    ::utils::random_splits();
    if ((ulonglong)-local_108 < uVar10 + 1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    auVar15._4_4_ = fVar17;
    auVar15._0_4_ = fVar14;
    auVar15._8_8_ = 0;
    auVar16._4_4_ = in_XMM3_Da;
    auVar16._0_4_ = in_XMM3_Da;
    auVar16._8_4_ = in_XMM3_Dc;
    auVar16._12_4_ = in_XMM3_Dc;
    auVar16 = divps(auVar15,auVar16);
    fVar14 = -100.0;
    pfVar9 = local_138;
    do {
      if (fVar14 <= 0.0) {
        if (pfVar9 == local_138 + local_130) goto LAB_14123e5e0;
        fVar14 = *pfVar9;
        pfVar9 = pfVar9 + 1;
        fVar17 = (1.0 - fVar14) * auVar16._4_4_ + fVar14 * auVar16._0_4_;
      }
      else {
        do {
          if (pfVar9 == local_138 + local_130) goto LAB_14123e5e0;
          fVar17 = *pfVar9;
          pfVar9 = pfVar9 + 1;
          fVar17 = (1.0 - fVar17) * auVar16._4_4_ + fVar17 * auVar16._0_4_;
        } while (ABS(fVar14 - fVar17) < 0.046 / in_XMM3_Da);
      }
      fVar14 = fVar17;
      if (local_108 == 0) {
        alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
      }
      *(float *)(local_108 * 4 + 4) = fVar14;
      local_108 = local_108 + 1;
    } while( true );
  }
LAB_14123e623:
  in_RCX[2] = local_108;
  *in_RCX = 0;
  in_RCX[1] = 4;
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar8 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar8 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14123e706:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar8 != 0) {
LAB_14123e712:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar8 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar8 = *plVar8 + 1;
  }
  return;
LAB_14123e5e0:
  if (local_140 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  bVar5 = false;
  pfVar9 = pfVar12;
  goto LAB_14123e415;
}

