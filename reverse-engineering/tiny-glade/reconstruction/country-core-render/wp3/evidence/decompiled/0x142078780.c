// address 0x142078780
// name nani_update_stone_floor
// signature void __cdecl nani_update_stone_floor(void)

// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::render::nani::systems::nani_update_stone_floor(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  bool bVar10;
  char extraout_AL;
  bool bVar11;
  uint uVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar13;
  ulonglong uVar14;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar15;
  bool bVar16;
  bool bVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined *puVar20;
  uint *puVar21;
  longlong lVar22;
  longlong local_150;
  longlong local_138;
  longlong local_130;
  longlong local_128;
  ulonglong local_120;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar10 = false;
  }
  else {
    uVar18 = 0x3c;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar18 != 0) {
        if (uVar18 < 0x3c) {
          if ((char)(&DAT_142fd2e88)[uVar18] < -0x40) {
LAB_142078843:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 != 0x3c) goto LAB_142078843;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar18 != 0xfffffffffffffffe)) {
        if (uVar18 + 2 < 0x3c) {
          if ((char)(&UNK_142fd2e8a)[uVar18] < -0x40) {
LAB_1420788a1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 + 2 != 0x3c) goto LAB_1420788a1;
      }
    }
    puVar20 = &DAT_142fd345e;
    do {
      bVar3 = puVar20[-1];
      uVar12 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar20[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar20[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar12 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar12 = bVar5 & 0xf;
          }
          uVar12 = bVar4 & 0x3f | uVar12 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar12 = bVar4 & 0x1f;
        }
        uVar12 = (uint)(bVar3 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_142078964;
LAB_142078976:
        if (puVar20 == &UNK_142fd342f) break;
        if (puVar20 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar20 = puVar20 + -1;
      if (uVar12 == 0x5c) goto LAB_142078976;
LAB_142078964:
      if (uVar12 == 0x2f) goto LAB_142078976;
    } while (puVar20 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14207940e;
    if (*plVar15 != 0) goto LAB_14207941a;
    *plVar15 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar15 = *plVar15 + 1;
    bVar10 = true;
  }
                    // WARNING: Read-only address (ram,0x00014292e6b0) is written
  gpu_world::GpuWorld::owned_subset_mut();
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  puVar21 = *(uint **)(lVar7 + 0xf8);
  puVar2 = puVar21 + *(longlong *)(lVar7 + 0x100);
  cVar6 = *(char *)(lVar7 + 0x108);
  local_150 = 8;
  uVar18 = 0;
  local_120 = 0;
  local_138 = 0;
  bVar16 = false;
  local_128 = 0;
  bVar17 = false;
  local_130 = 0;
  bVar11 = false;
  if (cVar6 != '\0') goto LAB_142078e80;
LAB_142078b0e:
  if (local_120 != uVar18) goto LAB_142078dab;
  do {
    if (puVar21 == puVar2) goto LAB_142079307;
    uVar12 = *puVar21;
    puVar21 = puVar21 + 1;
    uVar18 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar12 * 0xa0);
  } while (uVar18 == 0);
  lVar13 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar12 * 0xa0;
  lVar22 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*(uint *)(lVar13 + 0x94) * 0x48;
  uVar14 = *(ulonglong *)
            (*(longlong *)(lVar8 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar13 + 0x94) * 0x48);
  uVar19 = *(ulonglong *)(lVar7 + 0x120);
  uVar9 = *(ulonglong *)(lVar13 + 0x88);
  if ((uVar19 < uVar9) && (*(longlong *)(*(longlong *)(lVar13 + 0x80) + uVar19 * 8) != 0)) {
    bVar16 = true;
    if ((uVar14 <= uVar19) ||
       (uVar19 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8), uVar19 == 0)) {
      local_138 = 0;
      uVar19 = *(ulonglong *)(lVar7 + 0x128);
      goto joined_r0x000142078c99;
    }
    local_138 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar19 * 0x30);
    uVar19 = *(ulonglong *)(lVar7 + 0x128);
    if (uVar19 < uVar9) goto LAB_142078cb8;
LAB_142078d0f:
    bVar17 = false;
    uVar19 = *(ulonglong *)(lVar7 + 0x130);
joined_r0x000142078d0b:
    if (uVar9 <= uVar19) goto LAB_142078d8d;
LAB_142078d49:
    if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + uVar19 * 8) == 0) goto LAB_142078d8d;
    bVar11 = true;
    if ((uVar19 < uVar14) &&
       (uVar14 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8), uVar14 != 0)) {
      local_130 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar14 * 0x30);
    }
    else {
      local_130 = 0;
    }
  }
  else {
    bVar16 = false;
    uVar19 = *(ulonglong *)(lVar7 + 0x128);
joined_r0x000142078c99:
    if (uVar9 <= uVar19) goto LAB_142078d0f;
LAB_142078cb8:
    if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + uVar19 * 8) == 0) goto LAB_142078d0f;
    bVar17 = true;
    if ((uVar19 < uVar14) &&
       (uVar19 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8), uVar19 != 0)) {
      local_128 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar19 * 0x30);
      uVar19 = *(ulonglong *)(lVar7 + 0x130);
      goto joined_r0x000142078d0b;
    }
    local_128 = 0;
    uVar19 = *(ulonglong *)(lVar7 + 0x130);
    if (uVar19 < uVar9) goto LAB_142078d49;
LAB_142078d8d:
    bVar11 = false;
  }
  local_150 = *(longlong *)(lVar13 + 0x50);
  local_120 = 0;
LAB_142078dab:
  uVar14 = (ulonglong)*(uint *)(local_150 + 8 + local_120 * 0x10);
  do {
    lVar22 = uVar14 * 0x58 + local_138;
    if (!bVar16) {
      lVar22 = 0;
    }
    lVar13 = local_130 + uVar14 * 0x28;
    if (!bVar11) {
      lVar13 = 0;
    }
    if ((bVar17) && (local_128 + uVar14 * 8 != 0)) {
      if (lVar22 == 0) goto LAB_1420790fe;
LAB_142078dcd:
      bVar3 = *(byte *)(lVar22 + 8);
    }
    else {
      if (lVar22 != 0) goto LAB_142078dcd;
LAB_1420790fe:
      if (lVar13 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      bVar3 = *(byte *)(lVar13 + 8);
    }
    if ((bVar3 & 1) == 0) {
      ::utils::geometry::circle::Circle2d::get_tangent_at_u();
    }
    gpu_world::OwnedGpuWorldSubset::
    update_instance_data<country_core::render::nani::instance_data::StoneFloorInstanceData>();
    local_120 = local_120 + 1;
    if (cVar6 == '\0') goto LAB_142078b0e;
LAB_142078e80:
    if (local_120 == uVar18) {
      do {
        if (puVar21 == puVar2) goto LAB_142079307;
        uVar12 = *puVar21;
        puVar21 = puVar21 + 1;
        uVar18 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar12 * 0x48);
      } while (uVar18 == 0);
      lVar22 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar12 * 0x48;
      uVar14 = *(ulonglong *)(lVar22 + 0x40);
      if ((*(ulonglong *)(lVar7 + 0x110) < uVar14) &&
         (*(longlong *)(*(longlong *)(lVar22 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8) != 0)) {
        uVar19 = *(ulonglong *)(lVar7 + 0x118);
        if (uVar14 <= uVar19) goto LAB_142078f69;
LAB_142078f2c:
        if (*(longlong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8) == 0) goto LAB_142078f69;
        uVar19 = *(ulonglong *)(lVar7 + 0x120);
        if (uVar19 < uVar14) goto LAB_142078f87;
LAB_142078fc9:
        bVar16 = false;
        uVar19 = *(ulonglong *)(lVar7 + 0x128);
        if (uVar14 <= uVar19) goto LAB_142079028;
LAB_142078fe6:
        uVar19 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8);
        if (uVar19 == 0) goto LAB_142079028;
        local_128 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar19 * 0x30);
        bVar17 = true;
        uVar19 = *(ulonglong *)(lVar7 + 0x130);
      }
      else {
        uVar19 = *(ulonglong *)(lVar7 + 0x118);
        if (uVar19 < uVar14) goto LAB_142078f2c;
LAB_142078f69:
        uVar19 = *(ulonglong *)(lVar7 + 0x120);
        if (uVar14 <= uVar19) goto LAB_142078fc9;
LAB_142078f87:
        uVar19 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8);
        if (uVar19 == 0) goto LAB_142078fc9;
        local_138 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar19 * 0x30);
        bVar16 = true;
        uVar19 = *(ulonglong *)(lVar7 + 0x128);
        if (uVar19 < uVar14) goto LAB_142078fe6;
LAB_142079028:
        bVar17 = false;
        uVar19 = *(ulonglong *)(lVar7 + 0x130);
      }
      if ((uVar19 < uVar14) &&
         (uVar14 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar19 * 8), uVar14 != 0)) {
        local_130 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar14 * 0x30);
        bVar11 = true;
        local_120 = 0;
      }
      else {
        local_120 = 0;
        bVar11 = false;
      }
    }
    uVar14 = local_120 & 0xffffffff;
  } while( true );
LAB_142079307:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar7 = *extraout_RAX_02;
  if (lVar7 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar7 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar10) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar15 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14207940e:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_14207941a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar15 = *plVar15 + 1;
  }
  return;
}

