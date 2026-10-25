
void __cdecl system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitches(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  char extraout_AL;
  bool bVar9;
  uint uVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong lVar11;
  longlong *plVar12;
  longlong *in_RDX;
  ulonglong uVar13;
  int *piVar14;
  undefined **ppuVar15;
  float *pfVar16;
  float *pfVar17;
  undefined8 *puVar18;
  undefined8 *puVar19;
  float extraout_XMM0_Da;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float unaff_XMM6_Da;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  longlong local_170;
  float *local_168;
  longlong local_160;
  float local_128;
  float fStack_124;
  undefined8 uStack_120;
  undefined4 local_104;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar13 = 0x43;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x43) {
          if ((char)(&DAT_142f360f3)[uVar13] < -0x40) {
LAB_141ce666f:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x43) goto LAB_141ce666f;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x43) {
          if ((char)(&UNK_142f360f5)[uVar13] < -0x40) {
LAB_141ce66cd:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x43) goto LAB_141ce66cd;
      }
    }
    ppuVar15 = &PTR_DAT_142f36248;
    do {
      bVar3 = *(byte *)((longlong)ppuVar15 + -1);
      uVar10 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = *(byte *)((longlong)ppuVar15 + -2);
        if ((char)bVar4 < -0x40) {
          bVar5 = *(byte *)((longlong)ppuVar15 + -3);
          if ((char)bVar5 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar15 + -4);
            ppuVar15 = (undefined **)((longlong)ppuVar15 + -4);
            uVar10 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar15 = (undefined **)((longlong)ppuVar15 + -3);
            uVar10 = bVar5 & 0xf;
          }
          uVar10 = bVar4 & 0x3f | uVar10 << 6;
        }
        else {
          ppuVar15 = (undefined **)((longlong)ppuVar15 + -2);
          uVar10 = bVar4 & 0x1f;
        }
        uVar10 = (uint)(bVar3 & 0x3f) | uVar10 << 6;
        if (uVar10 != 0x5c) goto LAB_141ce6794;
LAB_141ce67a6:
        if (ppuVar15 == (undefined **)&UNK_142f36207) break;
        if ((undefined *)((longlong)ppuVar15 + -0x142f36207) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar15 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar15 + -0x142f36207) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar15 = (undefined **)((longlong)ppuVar15 + -1);
      if (uVar10 == 0x5c) goto LAB_141ce67a6;
LAB_141ce6794:
      if (uVar10 == 0x2f) goto LAB_141ce67a6;
    } while (ppuVar15 != (undefined **)&DAT_142f36208);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar12 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_141ce6dee:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar12 != 0) {
LAB_141ce6dfa:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar12 = *plVar12 + 1;
  }
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar12 = (longlong *)*in_RCX;
  puVar18 = *(undefined8 **)(*in_RDX + 8);
  piVar14 = (int *)(puVar18 + *(longlong *)(*in_RDX + 0x10) * 8);
LAB_141ce689f:
  puVar19 = puVar18 + 5;
  do {
    if ((int *)(puVar19 + -5) == piVar14) {
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_02 == 1) {
          plVar12 = extraout_RAX_02 + 1;
        }
        else if ((*extraout_RAX_02 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar12 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
        goto LAB_141ce6dee;
        if (*plVar12 != 0) goto LAB_141ce6dfa;
        *plVar12 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar12 = *plVar12 + 1;
      }
      return;
    }
    if ((*(int *)(puVar19 + -5) != 1) ||
       (0.2 < *(float *)((longlong)puVar19 + -0x24) || *(float *)((longlong)puVar19 + -0x24) == 0.2)
       ) {
      fVar28 = *(float *)((longlong)puVar19 + -0x14) - *(float *)(puVar19 + -3);
      fVar20 = 0.0;
      if (0.0 <= fVar28 + fVar28) {
        fVar20 = fVar28 + fVar28;
      }
      fVar22 = 1000.0;
      if (fVar20 <= 1000.0) {
        fVar22 = fVar20;
      }
      uVar13 = (longlong)(fVar22 - 9.223372e+18) & (longlong)fVar22 >> 0x3f | (longlong)fVar22;
      if (fVar22 < 0.0) {
        uVar13 = 0;
      }
      if (1.8446743e+19 < fVar22) {
        uVar13 = 0xffffffffffffffff;
      }
      if (2 < uVar13) break;
    }
    puVar19 = puVar19 + 8;
  } while( true );
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::random_splits();
  fVar20 = (float)puVar19[-1] + (float)puVar19[-2];
  fVar22 = (float)((ulonglong)puVar19[-1] >> 0x20) + (float)((ulonglong)puVar19[-2] >> 0x20);
  fVar23 = 1.0 / SQRT(fVar22 * fVar22 + fVar20 * fVar20);
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$0::x0y();
  if ((float)uStack_120 <= 0.0) {
    fVar27 = local_128 + 1.0;
    if (fVar27 <= 0.0) {
      fVar27 = (1.0 - (float)uStack_120) - fVar27;
      fVar24 = 0.5 / SQRT(fVar27);
      fVar25 = (local_128 - (float)uStack_120) * fVar24;
      fVar21 = (0.0 - fStack_124) * fVar24;
      fVar26 = fVar27 * fVar24;
      fVar27 = fVar24 * (0.0 - fStack_124);
    }
    else {
      fVar27 = fVar27 + (1.0 - (float)uStack_120);
      fVar24 = 0.5 / SQRT(fVar27);
      fVar25 = (fStack_124 + 0.0) * fVar24;
      fVar21 = ((float)uStack_120 + local_128) * fVar24;
      fVar26 = fVar24 * (0.0 - fStack_124);
      fVar27 = fVar27 * fVar24;
    }
  }
  else {
    fVar24 = 1.0 - local_128;
    if (fVar24 <= 0.0) {
      fVar24 = ((float)uStack_120 + 1.0) - fVar24;
      fVar21 = 0.5 / SQRT(fVar24);
      fVar26 = fVar21 * (local_128 - (float)uStack_120);
      fVar27 = fVar21 * (fStack_124 + 0.0);
      fVar25 = fVar24 * fVar21;
      fVar21 = fVar21 * -fStack_124;
    }
    else {
      fVar24 = fVar24 + (float)uStack_120 + 1.0;
      fVar21 = 0.5 / SQRT(fVar24);
      fVar26 = fVar21 * (0.0 - fStack_124);
      fVar27 = fVar21 * ((float)uStack_120 + local_128);
      fVar25 = -fStack_124 * fVar21;
      fVar21 = fVar21 * fVar24;
    }
  }
  puVar18 = puVar19 + 3;
  bVar9 = false;
  pfVar16 = local_168;
  while (pfVar16 != local_168 + local_160) {
    pfVar17 = pfVar16 + 1;
    fVar24 = *pfVar16 * fVar28 + *(float *)(puVar19 + -3);
    if (!bVar9) {
      if (pfVar17 == local_168 + local_160) break;
      pfVar17 = pfVar16 + 2;
      unaff_XMM6_Da = fVar24;
      fVar24 = *(float *)(puVar19 + -3) + pfVar16[1] * fVar28;
    }
    ::utils::swizzlers::impl$1::xvy();
    fastrand::Rng::f32();
    uStack_120 = *puVar19;
    uVar8 = puVar19[1];
    lVar7 = plVar12[2];
    if (lVar7 == *plVar12) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::resources::stairs::visual::StairsFloatingBrickParams,alloc::alloc::Global>
                ();
    }
    lVar6 = plVar12[1];
    lVar11 = lVar7 * 0x40;
    pfVar16 = (float *)(lVar6 + lVar11);
    *pfVar16 = fVar26;
    pfVar16[1] = fVar27;
    pfVar16[2] = fVar25;
    pfVar16[3] = fVar21;
    *(ulonglong *)(lVar6 + 0x10 + lVar11) = CONCAT44(fVar23 * fVar20,-(fVar23 * fVar22));
    *(undefined4 *)(lVar6 + 0x18 + lVar11) = local_104;
    *(float *)(lVar6 + 0x1c + lVar11) =
         (extraout_XMM0_Da * 0.5 - (1.0 - extraout_XMM0_Da) * 0.5) * 0.1 + 0.4;
    *(float *)(lVar6 + 0x20 + lVar11) = (fVar24 - unaff_XMM6_Da) + -0.1;
    *(undefined4 *)(lVar6 + 0x24 + lVar11) = 0x3e99999a;
    puVar2 = (undefined8 *)(lVar6 + 0x28 + lVar11);
    *puVar2 = CONCAT44(fStack_124,local_128);
    puVar2[1] = uStack_120;
    *(undefined8 *)(lVar6 + 0x38 + lVar11) = uVar8;
    plVar12[2] = lVar7 + 1;
    bVar9 = true;
    pfVar16 = pfVar17;
    unaff_XMM6_Da = fVar24;
  }
  if (local_170 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_141ce689f;
}

