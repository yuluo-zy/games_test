// address 0x14207fcc0
// name nani_update_stairs_floor
// signature void __cdecl nani_update_stairs_floor(void)

void __cdecl country_core::render::nani::systems::nani_update_stairs_floor(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  bool bVar10;
  bool bVar11;
  bool bVar12;
  char extraout_AL;
  uint uVar13;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *in_RCX;
  longlong *plVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined *puVar18;
  ulonglong uVar19;
  uint *puVar20;
  longlong lVar21;
  longlong local_a8;
  longlong local_98;
  longlong local_90;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar12 = false;
  }
  else {
    uVar16 = 0x3d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x3d) {
          if ((char)(&DAT_142fd2fbb)[uVar16] < -0x40) {
LAB_14207fd38:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 != 0x3d) goto LAB_14207fd38;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar16 != 0xfffffffffffffffe)) {
        if (uVar16 + 2 < 0x3d) {
          if ((char)(&UNK_142fd2fbd)[uVar16] < -0x40) {
LAB_14207fd96:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 + 2 != 0x3d) goto LAB_14207fd96;
      }
    }
    puVar18 = &DAT_142fd345e;
    do {
      bVar3 = puVar18[-1];
      uVar13 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar18[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar18[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar18 + -4;
            puVar18 = puVar18 + -4;
            uVar13 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar18 = puVar18 + -3;
            uVar13 = bVar5 & 0xf;
          }
          uVar13 = bVar4 & 0x3f | uVar13 << 6;
        }
        else {
          puVar18 = puVar18 + -2;
          uVar13 = bVar4 & 0x1f;
        }
        uVar13 = (uint)(bVar3 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_14207fe54;
LAB_14207fe66:
        if (puVar18 == &UNK_142fd342f) break;
        if (puVar18 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar18[1]) break;
        }
        else if (puVar18 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar18 = puVar18 + -1;
      if (uVar13 == 0x5c) goto LAB_14207fe66;
LAB_14207fe54:
      if (uVar13 == 0x2f) goto LAB_14207fe66;
    } while (puVar18 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
    goto LAB_1420804ea;
    if (*plVar15 != 0) goto LAB_1420804f6;
    *plVar15 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar15 = *plVar15 + 1;
    bVar12 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  iVar7 = *(int *)((longlong)in_RCX + 0x14);
  puVar20 = *(uint **)(lVar8 + 0xf8);
  puVar2 = puVar20 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  uVar14 = iVar7 - (int)in_RCX[2];
  uVar13 = 0xc233b000;
  if (uVar14 < 0xc233b000) {
    uVar13 = uVar14;
  }
  local_a8 = 8;
  uVar16 = 0;
  uVar19 = 0;
  local_98 = 0;
  bVar10 = false;
  local_90 = 0;
  bVar11 = false;
joined_r0x00014207ffaf:
  if (cVar6 == '\0') {
    do {
      if (uVar19 == uVar16) {
        if (puVar20 == puVar2) goto LAB_142080436;
        uVar14 = *puVar20;
        puVar20 = puVar20 + 1;
        if (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar14 * 0xa0) == 0) {
          do {
            if (puVar20 == puVar2) goto LAB_142080436;
            uVar14 = *puVar20;
            puVar20 = puVar20 + 1;
          } while (*(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar14 * 0xa0) ==
                   0);
          lVar17 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar14 * 0xa0;
        }
        else {
          lVar17 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar14 * 0xa0;
        }
        if ((*(ulonglong *)(lVar8 + 0x128) < *(ulonglong *)(lVar17 + 0x88)) &&
           (*(longlong *)(*(longlong *)(lVar17 + 0x80) + *(ulonglong *)(lVar8 + 0x128) * 8) != 0)) {
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          bVar10 = true;
          uVar16 = *(ulonglong *)(lVar8 + 0x130);
          local_98 = extraout_RAX_03;
          if (*(ulonglong *)(lVar17 + 0x88) <= uVar16) goto LAB_142080190;
LAB_142080162:
          if (*(longlong *)(*(longlong *)(lVar17 + 0x80) + uVar16 * 8) == 0) goto LAB_142080190;
          bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
          bVar11 = true;
          local_90 = extraout_RAX_04;
        }
        else {
          bVar10 = false;
          uVar16 = *(ulonglong *)(lVar8 + 0x130);
          if (uVar16 < *(ulonglong *)(lVar17 + 0x88)) goto LAB_142080162;
LAB_142080190:
          bVar11 = false;
        }
        local_a8 = *(longlong *)(lVar17 + 0x50);
        uVar16 = *(ulonglong *)(lVar17 + 0x58);
        uVar19 = 0;
      }
      uVar14 = *(uint *)(local_a8 + 8 + uVar19 * 0x10);
      if (((bVar10) && ((uint)(iVar7 - *(int *)(local_98 + (ulonglong)uVar14 * 4)) < uVar13)) ||
         ((bVar11 && ((uint)(iVar7 - *(int *)(local_90 + (ulonglong)uVar14 * 4)) < uVar13))))
      goto LAB_1420803c1;
      uVar19 = uVar19 + 1;
    } while( true );
  }
  do {
    if (uVar19 == uVar16) {
      if (puVar20 == puVar2) goto LAB_142080436;
      uVar16 = (ulonglong)*puVar20;
      puVar20 = puVar20 + 1;
      lVar17 = *(longlong *)(lVar9 + 0x1a8);
      if (*(longlong *)(lVar17 + 0x10 + uVar16 * 0x48) == 0) {
        do {
          if (puVar20 == puVar2) goto LAB_142080436;
          uVar14 = *puVar20;
          puVar20 = puVar20 + 1;
        } while (*(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar14 * 0x48) == 0)
        ;
        lVar21 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar14 * 0x48;
        lVar17 = lVar21;
      }
      else {
        lVar21 = lVar17 + uVar16 * 0x48;
        lVar17 = lVar17 + uVar16 * 0x48;
      }
      uVar16 = *(ulonglong *)(lVar21 + 0x40);
      if ((*(ulonglong *)(lVar8 + 0x110) < uVar16) &&
         (*(longlong *)(*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
        uVar19 = *(ulonglong *)(lVar8 + 0x118);
        if (uVar16 <= uVar19) goto LAB_142080300;
LAB_1420802c8:
        if (*(longlong *)(*(longlong *)(lVar21 + 0x38) + uVar19 * 8) == 0) goto LAB_142080300;
        uVar19 = *(ulonglong *)(lVar8 + 0x128);
        if (uVar19 < uVar16) goto LAB_142080318;
LAB_142080350:
        bVar10 = false;
        uVar19 = *(ulonglong *)(lVar8 + 0x130);
        if (uVar16 <= uVar19) goto LAB_142080380;
LAB_142080362:
        if (*(longlong *)(*(longlong *)(lVar21 + 0x38) + uVar19 * 8) == 0) goto LAB_142080380;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        bVar11 = true;
        local_90 = extraout_RAX_06;
      }
      else {
        uVar19 = *(ulonglong *)(lVar8 + 0x118);
        if (uVar19 < uVar16) goto LAB_1420802c8;
LAB_142080300:
        uVar19 = *(ulonglong *)(lVar8 + 0x128);
        if (uVar16 <= uVar19) goto LAB_142080350;
LAB_142080318:
        if (*(longlong *)(*(longlong *)(lVar21 + 0x38) + uVar19 * 8) == 0) goto LAB_142080350;
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        bVar10 = true;
        uVar19 = *(ulonglong *)(lVar8 + 0x130);
        local_98 = extraout_RAX_05;
        if (uVar19 < *(ulonglong *)(lVar21 + 0x40)) goto LAB_142080362;
LAB_142080380:
        bVar11 = false;
      }
      uVar16 = *(ulonglong *)(lVar17 + 0x10);
      uVar19 = 0;
    }
    if (((bVar10) && ((uint)(iVar7 - *(int *)(local_98 + (uVar19 & 0xffffffff) * 4)) < uVar13)) ||
       ((bVar11 && ((uint)(iVar7 - *(int *)(local_90 + (uVar19 & 0xffffffff) * 4)) < uVar13))))
    break;
    uVar19 = uVar19 + 1;
  } while( true );
LAB_1420803c1:
  gpu_world::OwnedGpuWorldSubset::
  update_instance_data<country_core::render::nani::instance_data::StairsFloorInstanceData>();
  uVar19 = uVar19 + 1;
  goto joined_r0x00014207ffaf;
LAB_142080436:
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
  if (bVar12) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_07 == 1) {
      plVar15 = extraout_RAX_07 + 1;
    }
    else if ((*extraout_RAX_07 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_1420804ea:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_1420804f6:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar15 = *plVar15 + 1;
  }
  return;
}

