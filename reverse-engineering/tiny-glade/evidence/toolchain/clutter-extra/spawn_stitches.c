
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar9;
  float fVar10;
  float fVar11;
  char extraout_AL;
  bool bVar12;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong lVar14;
  longlong *plVar15;
  longlong *in_RDX;
  ulonglong uVar16;
  int *piVar17;
  undefined **ppuVar18;
  float *pfVar19;
  float *pfVar20;
  undefined8 *puVar21;
  undefined8 *puVar22;
  float extraout_XMM0_Da;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float unaff_XMM6_Da;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  longlong local_170;
  float *local_168;
  longlong local_160;
  float local_128;
  float fStack_124;
  undefined8 uStack_120;
  undefined4 local_104;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar16 = 0x43;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x43) {
          if ((char)(&DAT_142f360f3)[uVar16] < -0x40) {
LAB_141ce666f:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 != 0x43) goto LAB_141ce666f;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar16 != 0xfffffffffffffffe)) {
        if (uVar16 + 2 < 0x43) {
          if ((char)(&UNK_142f360f5)[uVar16] < -0x40) {
LAB_141ce66cd:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 + 2 != 0x43) goto LAB_141ce66cd;
      }
    }
    ppuVar18 = &PTR_DAT_142f36248;
    do {
      bVar3 = *(byte *)((longlong)ppuVar18 + -1);
      uVar13 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = *(byte *)((longlong)ppuVar18 + -2);
        if ((char)bVar4 < -0x40) {
          bVar5 = *(byte *)((longlong)ppuVar18 + -3);
          if ((char)bVar5 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar18 + -4);
            ppuVar18 = (undefined **)((longlong)ppuVar18 + -4);
            uVar13 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar18 = (undefined **)((longlong)ppuVar18 + -3);
            uVar13 = bVar5 & 0xf;
          }
          uVar13 = bVar4 & 0x3f | uVar13 << 6;
        }
        else {
          ppuVar18 = (undefined **)((longlong)ppuVar18 + -2);
          uVar13 = bVar4 & 0x1f;
        }
        uVar13 = (uint)(bVar3 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_141ce6794;
LAB_141ce67a6:
        if (ppuVar18 == (undefined **)&UNK_142f36207) break;
        if ((undefined *)((longlong)ppuVar18 + -0x142f36207) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar18 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar18 + -0x142f36207) == (undefined *)0x40) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      ppuVar18 = (undefined **)((longlong)ppuVar18 + -1);
      if (uVar13 == 0x5c) goto LAB_141ce67a6;
LAB_141ce6794:
      if (uVar13 == 0x2f) goto LAB_141ce67a6;
    } while (ppuVar18 != (undefined **)&DAT_142f36208);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_141ce6dee:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_141ce6dfa:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar15 = *plVar15 + 1;
  }
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  fVar11 = __real_447a0000;
  fVar10 = __real_5f7fffff;
  fVar9 = __real_3f000000;
  plVar15 = (longlong *)*in_RCX;
  puVar21 = *(undefined8 **)(*in_RDX + 8);
  piVar17 = (int *)(puVar21 + *(longlong *)(*in_RDX + 0x10) * 8);
LAB_141ce689f:
  puVar22 = puVar21 + 5;
  do {
    if ((int *)(puVar22 + -5) == piVar17) {
      if (extraout_AL != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_02 == 1) {
          plVar15 = extraout_RAX_02 + 1;
        }
        else if ((*extraout_RAX_02 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar15 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
        goto LAB_141ce6dee;
        if (*plVar15 != 0) goto LAB_141ce6dfa;
        *plVar15 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar15 = *plVar15 + 1;
      }
      return;
    }
    if ((*(int *)(puVar22 + -5) != 1) ||
       (__real_3e4ccccd < *(float *)((longlong)puVar22 + -0x24) ||
        __real_3e4ccccd == *(float *)((longlong)puVar22 + -0x24))) {
      fVar31 = *(float *)((longlong)puVar22 + -0x14) - *(float *)(puVar22 + -3);
      fVar23 = 0.0;
      if (0.0 <= fVar31 + fVar31) {
        fVar23 = fVar31 + fVar31;
      }
      fVar25 = fVar11;
      if (fVar23 <= fVar11) {
        fVar25 = fVar23;
      }
      uVar16 = (longlong)(fVar25 - __real_5f000000) & (longlong)fVar25 >> 0x3f | (longlong)fVar25;
      if (fVar25 < 0.0) {
        uVar16 = 0;
      }
      if (fVar10 < fVar25) {
        uVar16 = 0xffffffffffffffff;
      }
      if (2 < uVar16) break;
    }
    puVar22 = puVar22 + 8;
  } while( true );
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::random_splits();
  fVar23 = (float)puVar22[-1] + (float)puVar22[-2];
  fVar25 = (float)((ulonglong)puVar22[-1] >> 0x20) + (float)((ulonglong)puVar22[-2] >> 0x20);
  fVar26 = __real_3f800000 / SQRT(fVar25 * fVar25 + fVar23 * fVar23);
  uVar13 = (uint)(fVar26 * fVar25) ^ ___xmm_80000000800000008000000080000000;
  ::utils::swizzlers::impl$0::x0y();
  ::utils::swizzlers::impl$0::x0y();
  if ((float)uStack_120 <= 0.0) {
    fVar25 = local_128 + __real_3f800000;
    if (fVar25 <= 0.0) {
      fVar25 = (__real_3f800000 - (float)uStack_120) - fVar25;
      fVar27 = fVar9 / SQRT(fVar25);
      fVar29 = (local_128 - (float)uStack_120) * fVar27;
      fVar24 = (0.0 - fStack_124) * fVar27;
      fVar30 = fVar25 * fVar27;
      fVar25 = fVar27 * (0.0 - fStack_124);
    }
    else {
      fVar25 = fVar25 + (__real_3f800000 - (float)uStack_120);
      fVar27 = fVar9 / SQRT(fVar25);
      fVar29 = (fStack_124 + 0.0) * fVar27;
      fVar24 = ((float)uStack_120 + local_128) * fVar27;
      fVar30 = fVar27 * (0.0 - fStack_124);
      fVar25 = fVar25 * fVar27;
    }
  }
  else {
    fVar27 = __real_3f800000 - local_128;
    if (fVar27 <= 0.0) {
      fVar27 = ((float)uStack_120 + __real_3f800000) - fVar27;
      fVar24 = fVar9 / SQRT(fVar27);
      fVar30 = fVar24 * (local_128 - (float)uStack_120);
      fVar25 = fVar24 * (fStack_124 + 0.0);
      fVar29 = fVar27 * fVar24;
      fVar24 = fVar24 * (float)((uint)fStack_124 ^ ___xmm_80000000800000008000000080000000);
    }
    else {
      fVar27 = fVar27 + (float)uStack_120 + __real_3f800000;
      fVar24 = fVar9 / SQRT(fVar27);
      fVar30 = fVar24 * (0.0 - fStack_124);
      fVar25 = fVar24 * ((float)uStack_120 + local_128);
      fVar29 = (float)((uint)fStack_124 ^ ___xmm_80000000800000008000000080000000) * fVar24;
      fVar24 = fVar24 * fVar27;
    }
  }
  puVar21 = puVar22 + 3;
  bVar12 = false;
  pfVar19 = local_168;
  while (pfVar19 != local_168 + local_160) {
    pfVar20 = pfVar19 + 1;
    fVar27 = *pfVar19 * fVar31 + *(float *)(puVar22 + -3);
    if (!bVar12) {
      if (pfVar20 == local_168 + local_160) break;
      pfVar20 = pfVar19 + 2;
      unaff_XMM6_Da = fVar27;
      fVar27 = *(float *)(puVar22 + -3) + pfVar19[1] * fVar31;
    }
    fVar32 = (fVar27 - unaff_XMM6_Da) + __real_bdcccccd;
    ::utils::swizzlers::impl$1::xvy();
    fastrand::Rng::f32();
    fVar28 = (extraout_XMM0_Da * fVar9 - (__real_3f800000 - extraout_XMM0_Da) * fVar9) *
             __real_3dcccccd + __real_3ecccccd;
    uStack_120 = *puVar22;
    uVar8 = puVar22[1];
    lVar7 = plVar15[2];
    if (lVar7 == *plVar15) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::resources::stairs::visual::StairsFloatingBrickParams,alloc::alloc::Global>
                ();
    }
    lVar6 = plVar15[1];
    lVar14 = lVar7 * 0x40;
    pfVar19 = (float *)(lVar6 + lVar14);
    *pfVar19 = fVar30;
    pfVar19[1] = fVar25;
    pfVar19[2] = fVar29;
    pfVar19[3] = fVar24;
    *(ulonglong *)(lVar6 + 0x10 + lVar14) = CONCAT44(fVar26 * fVar23,uVar13);
    *(undefined4 *)(lVar6 + 0x18 + lVar14) = local_104;
    *(float *)(lVar6 + 0x1c + lVar14) = fVar28;
    *(float *)(lVar6 + 0x20 + lVar14) = fVar32;
    *(undefined4 *)(lVar6 + 0x24 + lVar14) = 0x3e99999a;
    puVar2 = (undefined8 *)(lVar6 + 0x28 + lVar14);
    *puVar2 = CONCAT44(fStack_124,local_128);
    puVar2[1] = uStack_120;
    *(undefined8 *)(lVar6 + 0x38 + lVar14) = uVar8;
    plVar15[2] = lVar7 + 1;
    bVar12 = true;
    pfVar19 = pfVar20;
    unaff_XMM6_Da = fVar27;
  }
  if (local_170 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_141ce689f;
}

