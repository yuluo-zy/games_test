
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_wall_constructor::utils::split_into_random_rows::split_into_random_rows(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  float fVar5;
  float fVar6;
  char extraout_AL;
  bool bVar7;
  uint uVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *in_RCX;
  float *pfVar9;
  longlong *plVar10;
  float *in_RDX;
  float *pfVar11;
  ulonglong uVar12;
  longlong in_R8;
  undefined *puVar13;
  float *pfVar14;
  float fVar15;
  float fVar16;
  float in_XMM3_Da;
  undefined4 in_XMM3_Dc;
  float fVar17;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  float fVar20;
  float *in_stack_00000028;
  longlong local_140;
  float *local_138;
  longlong local_130;
  longlong local_108;
  
  pfVar11 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (pfVar11 != (float *)0x0) {
        if (&DAT_0000004d < pfVar11) {
          if (pfVar11 != (float *)&DAT_0000004e) goto LAB_14123e1c7;
        }
        else if (*(char *)(pfVar11 + 0x50b07c20) < -0x40) {
LAB_14123e1c7:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (pfVar11 != (float *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)pfVar11 + 2) < (undefined1 *)0x4e) {
          if (*(char *)((longlong)pfVar11 + 0x142c1f082) < -0x40) {
LAB_14123e21b:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)pfVar11 + 2) != (undefined1 *)0x4e) goto LAB_14123e21b;
      }
    }
    puVar13 = &DAT_142c1f1e3;
    do {
      bVar2 = puVar13[-1];
      uVar8 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar13[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar13[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar8 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar8 = bVar4 & 0xf;
          }
          uVar8 = bVar3 & 0x3f | uVar8 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar8 = bVar3 & 0x1f;
        }
        uVar8 = (uint)(bVar2 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto LAB_14123e2d4;
LAB_14123e2e5:
        if (puVar13 == &UNK_142c1f19f) break;
        if (puVar13 + -0x142c1f19f < (undefined *)0x43) {
          if (-0x41 < (char)puVar13[1]) break;
        }
        else if (puVar13 + -0x142c1f19f == (undefined *)0x43) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar13 = puVar13 + -1;
      if (uVar8 == 0x5c) goto LAB_14123e2e5;
LAB_14123e2d4:
      if (uVar8 == 0x2f) goto LAB_14123e2e5;
    } while (puVar13 != &DAT_142c1f1a0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_14123e706;
    if (*plVar10 != 0) goto LAB_14123e712;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
  }
  fVar6 = __real_c2c80000;
  uVar8 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  fVar5 = __real_3f800000;
  local_108 = 0;
  fVar17 = __real_3d3c6a7f / in_XMM3_Da;
  bVar7 = true;
  pfVar11 = in_RDX;
LAB_14123e415:
  if (pfVar11 != in_RDX + in_R8) {
    pfVar14 = pfVar11 + 1;
    pfVar9 = in_stack_00000028;
    in_stack_00000028 = pfVar11;
    if (bVar7) {
      if (pfVar14 == in_RDX + in_R8) goto LAB_14123e623;
      pfVar9 = pfVar11;
      in_stack_00000028 = pfVar14;
      pfVar14 = pfVar11 + 2;
    }
    fVar16 = *in_stack_00000028;
    fVar20 = *pfVar9;
    fVar15 = floorf((fVar16 - fVar20) / __real_3eeb851f);
    uVar12 = (longlong)(fVar15 - __real_5f000000) & (longlong)fVar15 >> 0x3f | (longlong)fVar15;
    if (fVar15 < 0.0) {
      uVar12 = 0;
    }
    if (__real_5f7fffff < fVar15) {
      uVar12 = 0xffffffffffffffff;
    }
    if (uVar12 < 3) {
      uVar12 = 2;
    }
    ::utils::random_splits();
    if ((ulonglong)-local_108 < uVar12 + 1) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    auVar18._4_4_ = fVar20;
    auVar18._0_4_ = fVar16;
    auVar18._8_8_ = 0;
    auVar19._4_4_ = in_XMM3_Da;
    auVar19._0_4_ = in_XMM3_Da;
    auVar19._8_4_ = in_XMM3_Dc;
    auVar19._12_4_ = in_XMM3_Dc;
    auVar19 = divps(auVar18,auVar19);
    pfVar11 = local_138;
    fVar16 = fVar6;
    do {
      if (fVar16 <= 0.0) {
        if (pfVar11 == local_138 + local_130) goto LAB_14123e5e0;
        fVar16 = *pfVar11;
        pfVar11 = pfVar11 + 1;
        fVar20 = (fVar5 - fVar16) * auVar19._4_4_ + fVar16 * auVar19._0_4_;
      }
      else {
        do {
          if (pfVar11 == local_138 + local_130) goto LAB_14123e5e0;
          fVar20 = *pfVar11;
          pfVar11 = pfVar11 + 1;
          fVar20 = (fVar5 - fVar20) * auVar19._4_4_ + fVar20 * auVar19._0_4_;
        } while ((float)((uint)(fVar16 - fVar20) & uVar8) < fVar17);
      }
      fVar16 = fVar20;
      if (local_108 == 0) {
        alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
      }
      *(float *)(local_108 * 4 + 4) = fVar16;
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
      plVar10 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14123e706:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar10 != 0) {
LAB_14123e712:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  return;
LAB_14123e5e0:
  if (local_140 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  bVar7 = false;
  pfVar11 = pfVar14;
  goto LAB_14123e415;
}

