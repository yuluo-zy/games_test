// address 0x1412e2f10  name gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  int iVar5;
  ulonglong uVar6;
  bool bVar7;
  longlong lVar8;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar9;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  uint uVar10;
  longlong in_RCX;
  longlong *plVar11;
  ulonglong in_RDX;
  ulonglong *puVar12;
  ulonglong unaff_RBX;
  longlong lVar13;
  undefined **ppuVar14;
  uint uVar15;
  ulonglong *puVar16;
  char *unaff_R14;
  ulonglong uVar17;
  bool bVar18;
  longlong local_130;
  ulonglong local_128;
  longlong *local_98;
  ulonglong *local_90;
  undefined8 local_88;
  longlong *local_80;
  longlong **local_78;
  code *local_70;
  longlong lStack_68;
  ulonglong *local_58;
  longlong *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar7 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    unaff_R14 = 
    "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<_>::gpu_flush::f";
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x50) {
          if ((char)(&DAT_142c2e9f0)[in_RDX] < -0x40) {
LAB_1412e2f8b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x50) goto LAB_1412e2f8b;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (in_RDX != 0xfffffffffffffffe) {
          if (in_RDX + 2 < 0x50) {
            if ("untry_core::render::sparse_instance_buffer::SparseInstanceBuffer<_>::gpu_flush::f"
                [in_RDX] < -0x40) {
LAB_1412e2fdf:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (in_RDX + 2 != 0x50) goto LAB_1412e2fdf;
        }
        unaff_R14 = "untry_core::render::sparse_instance_buffer::SparseInstanceBuffer<_>::gpu_flush::f"
                    + in_RDX;
      }
    }
    ppuVar14 = &PTR_anon_63e4314d53dbf22ebb46fe72548aa92f_24_llvm_4996588092114191460_142c2eb48;
    do {
      bVar2 = *(byte *)((longlong)ppuVar14 + -1);
      uVar9 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = *(byte *)((longlong)ppuVar14 + -2);
        if ((char)bVar3 < -0x40) {
          bVar4 = *(byte *)((longlong)ppuVar14 + -3);
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar14 + -4);
            ppuVar14 = (undefined **)((longlong)ppuVar14 + -4);
            uVar9 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar14 = (undefined **)((longlong)ppuVar14 + -3);
            uVar9 = bVar4 & 0xf;
          }
          uVar9 = bVar3 & 0x3f | uVar9 << 6;
        }
        else {
          ppuVar14 = (undefined **)((longlong)ppuVar14 + -2);
          uVar9 = bVar3 & 0x1f;
        }
        uVar9 = (uint)(bVar2 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_1412e30a4;
LAB_1412e30b5:
        if (ppuVar14 == (undefined **)&UNK_142c2eb0f) break;
        if ((undefined *)((longlong)ppuVar14 + -0x142c2eb0f) < (undefined *)0x38) {
          if (-0x41 < *(char *)((longlong)ppuVar14 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar14 + -0x142c2eb0f) == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar14 = (undefined **)((longlong)ppuVar14 + -1);
      if (uVar9 == 0x5c) goto LAB_1412e30b5;
LAB_1412e30a4:
      if (uVar9 == 0x2f) goto LAB_1412e30b5;
    } while (ppuVar14 !=
             (undefined **)&anon_63e4314d53dbf22ebb46fe72548aa92f_24_llvm_4996588092114191460);
    local_98 = (longlong *)&anon_107183261458dea85466985370debbb6_35_llvm_3641509820080252961;
    local_90 = (ulonglong *)&DAT_0000003a;
    local_78 = &local_98;
    local_70 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    alloc::fmt::format::format_inner();
    local_50 = (longlong *)local_128;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    local_58 = (ulonglong *)local_130;
    if (*extraout_RAX_01 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      local_80 = extraout_RAX_02;
      if (extraout_RAX_02 == (longlong *)0x0) goto LAB_1412e372a;
    }
    else {
      if ((int)*extraout_RAX_01 != 1) {
LAB_1412e372a:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_80 = extraout_RAX_01 + 1;
    }
    if (*local_80 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_80 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_80 = *local_80 + 1;
    if (local_58 != (ulonglong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    bVar7 = true;
    unaff_RBX = extraout_RAX_03;
  }
  uVar17 = 0;
  local_78 = (longlong **)0x0;
  local_70 = (code *)0x8;
  lStack_68 = 0;
  puVar16 = *(ulonglong **)(in_RCX + 0x50);
  puVar12 = puVar16 + *(longlong *)(in_RCX + 0x58);
  bVar18 = false;
  lVar13 = 0;
  uVar9 = 0;
  local_58 = puVar12;
  do {
    lVar8 = lStack_68;
    uVar10 = (uint)uVar17;
    if (!bVar18) goto LAB_1412e3310;
    while (unaff_RBX == 0) {
LAB_1412e3310:
      if ((puVar16 == (ulonglong *)0x0) || (puVar16 == puVar12)) {
        if (uVar10 < uVar9) {
          if ((longlong **)lStack_68 == local_78) {
            alloc::raw_vec::RawVec::
            grow_one<minidump_common::format::MINIDUMP_THREAD_NAME,alloc::alloc::Global>();
          }
          *(ulonglong *)((longlong)local_70 + lVar8 * 0x10) = uVar17 << 5;
          *(ulonglong *)((longlong)local_70 + 8 + lVar8 * 0x10) = (ulonglong)uVar9 << 5;
          lStack_68 = lVar8 + 1;
        }
        puffin::are_scopes_on();
        local_50 = (longlong *)CONCAT71(local_50._1_7_,extraout_AL_00);
        if (extraout_AL_00 == '\0') goto LAB_1412e35c8;
        ppuVar14 = &PTR_anon_63e4314d53dbf22ebb46fe72548aa92f_24_llvm_4996588092114191460_142c2eb48;
        goto LAB_1412e33c0;
      }
      unaff_RBX = *puVar16;
      *puVar16 = 0;
      puVar16 = puVar16 + 1;
      unaff_R14 = (char *)(lVar13 << 6);
      lVar13 = lVar13 + 1;
    }
    iVar5 = 0;
    for (uVar6 = unaff_RBX; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x8000000000000000) {
      iVar5 = iVar5 + 1;
    }
    uVar15 = iVar5 + (int)unaff_R14;
    if ((uVar9 != uVar15) && (bVar18 = uVar10 < uVar9, uVar10 = uVar15, bVar18)) {
      local_50 = (longlong *)uVar17;
      if ((longlong **)lStack_68 == local_78) {
        alloc::raw_vec::RawVec::
        grow_one<minidump_common::format::MINIDUMP_THREAD_NAME,alloc::alloc::Global>();
      }
      *(ulonglong *)((longlong)local_70 + lVar8 * 0x10) = ((ulonglong)local_50 & 0xffffffff) << 5;
      *(ulonglong *)((longlong)local_70 + 8 + lVar8 * 0x10) = (ulonglong)uVar9 << 5;
      lStack_68 = lVar8 + 1;
      puVar12 = local_58;
    }
    unaff_RBX = unaff_RBX & unaff_RBX - 1;
    uVar9 = uVar15 + 1;
    bVar18 = true;
    uVar17 = (ulonglong)uVar10;
  } while( true );
LAB_1412e33c0:
  do {
    bVar2 = *(byte *)((longlong)ppuVar14 + -1);
    uVar9 = (uint)(char)bVar2;
    if ((char)bVar2 < '\0') {
      bVar3 = *(byte *)((longlong)ppuVar14 + -2);
      if ((char)bVar3 < -0x40) {
        bVar4 = *(byte *)((longlong)ppuVar14 + -3);
        if ((char)bVar4 < -0x40) {
          pbVar1 = (byte *)((longlong)ppuVar14 + -4);
          ppuVar14 = (undefined **)((longlong)ppuVar14 + -4);
          uVar9 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          ppuVar14 = (undefined **)((longlong)ppuVar14 + -3);
          uVar9 = bVar4 & 0xf;
        }
        uVar9 = bVar3 & 0x3f | uVar9 << 6;
      }
      else {
        ppuVar14 = (undefined **)((longlong)ppuVar14 + -2);
        uVar9 = bVar3 & 0x1f;
      }
      uVar9 = (uint)(bVar2 & 0x3f) | uVar9 << 6;
      if (uVar9 != 0x5c) goto LAB_1412e3434;
LAB_1412e3452:
      if (ppuVar14 == (undefined **)&UNK_142c2eb0f) break;
      if ((undefined *)((longlong)ppuVar14 + -0x142c2eb0f) < (undefined *)0x38) {
        if (-0x41 < *(char *)((longlong)ppuVar14 + 1)) break;
      }
      else if ((undefined *)((longlong)ppuVar14 + -0x142c2eb0f) == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
      core::str::slice_error_fail();
    }
    ppuVar14 = (undefined **)((longlong)ppuVar14 + -1);
    if (uVar9 == 0x5c) goto LAB_1412e3452;
LAB_1412e3434:
    if (uVar9 == 0x2f) goto LAB_1412e3452;
  } while (ppuVar14 !=
           (undefined **)&anon_63e4314d53dbf22ebb46fe72548aa92f_24_llvm_4996588092114191460);
  local_98 = (longlong *)0x0;
  local_90 = (ulonglong *)0x1;
  local_88 = 0;
  core::fmt::num::imp::impl$9::fmt();
  plVar11 = local_98;
  if (extraout_AL_01 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_58 = local_90;
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  local_80 = plVar11;
  if (*extraout_RAX_04 == 0) {
    std::sys::thread_local::native::lazy::Storage::
    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
              ();
    plVar11 = extraout_RAX_05;
    if (extraout_RAX_05 == (longlong *)0x0) goto LAB_1412e3765;
  }
  else {
    if ((int)*extraout_RAX_04 != 1) {
LAB_1412e3765:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    plVar11 = extraout_RAX_04 + 1;
  }
  if (*plVar11 != 0) {
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar11 = -1;
  puffin::ThreadProfiler::begin_scope();
  *plVar11 = *plVar11 + 1;
  if (local_80 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_1412e35c8:
  ssbo::TypedStorageBuffer::
  gpu_write_ranges_nonoverlapping<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  if ((char)local_50 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar11 = extraout_RAX_07;
      if (extraout_RAX_07 == (longlong *)0x0) goto LAB_1412e3773;
    }
    else {
      if ((int)*extraout_RAX_06 != 1) {
LAB_1412e3773:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar11 = extraout_RAX_06 + 1;
    }
    if (*plVar11 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar11 = -1;
    local_50 = plVar11;
    puffin::ThreadProfiler::end_scope();
    *local_50 = *local_50 + 1;
  }
  if (local_78 != (longlong **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar7) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_08 == 1) {
      plVar11 = extraout_RAX_08 + 1;
    }
    else if ((*extraout_RAX_08 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar11 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar11 = -1;
    local_50 = plVar11;
    puffin::ThreadProfiler::end_scope();
    *local_50 = *local_50 + 1;
  }
  return;
}


