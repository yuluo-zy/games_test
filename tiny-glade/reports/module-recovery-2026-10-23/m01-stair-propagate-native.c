
void __cdecl
system_decorator::decorator_visual::stairs_assemble::
playermade_stairs_assembly_propagate_dirty_flags(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  uint uVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  undefined8 uVar9;
  longlong lVar10;
  uint *puVar11;
  uint *puVar12;
  uint *puVar13;
  char extraout_AL;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint *extraout_RAX_07;
  longlong *in_RCX;
  ulonglong uVar15;
  longlong *plVar16;
  ulonglong *in_RDX;
  ulonglong uVar17;
  longlong lVar18;
  uint *puVar19;
  uint *puVar20;
  longlong lVar21;
  undefined **ppuVar22;
  ulonglong uVar23;
  undefined8 *in_R9;
  ulonglong uVar24;
  ulonglong local_118;
  ulonglong uStack_110;
  ulonglong local_108;
  ulonglong uStack_100;
  ulonglong *local_f8;
  undefined8 uStack_f0;
  longlong local_e8;
  longlong local_e0;
  longlong *local_c8;
  longlong local_c0;
  undefined8 local_b0;
  undefined4 *local_a8;
  longlong local_a0;
  ulonglong *local_98;
  undefined8 uStack_90;
  longlong local_88;
  uint *local_80;
  undefined4 local_74;
  longlong local_70;
  uint *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_118 = *in_RDX;
  uStack_110 = in_RDX[1];
  local_108 = in_RDX[2];
  uStack_100 = in_RDX[3];
  uVar7 = in_RDX[4];
  puVar20 = (uint *)in_RDX[6];
  uVar6 = *(uint *)((longlong)in_RDX + 0x3c);
  local_70 = in_R9[5];
  local_a8 = (undefined4 *)in_R9[7];
  local_74 = *(undefined4 *)((longlong)in_R9 + 0x44);
  local_b0 = in_R9[9];
  local_c8 = in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_c0 = 0;
  }
  else {
    uVar17 = 0x65;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x65) {
          if ((char)(&DAT_142a87ac1)[uVar17] < -0x40) {
LAB_1407d177b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x65) goto LAB_1407d177b;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x65) {
          if ((char)(&UNK_142a87ac3)[uVar17] < -0x40) {
LAB_1407d17d9:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x65) goto LAB_1407d17d9;
      }
    }
    ppuVar22 = &PTR_DAT_142a87f40;
    do {
      bVar2 = *(byte *)((longlong)ppuVar22 + -1);
      uVar14 = (uint)(char)bVar2;
      local_68 = puVar20;
      if ((char)bVar2 < '\0') {
        bVar3 = *(byte *)((longlong)ppuVar22 + -2);
        if ((char)bVar3 < -0x40) {
          bVar4 = *(byte *)((longlong)ppuVar22 + -3);
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar22 + -4);
            ppuVar22 = (undefined **)((longlong)ppuVar22 + -4);
            uVar14 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar22 = (undefined **)((longlong)ppuVar22 + -3);
            uVar14 = bVar4 & 0xf;
          }
          uVar14 = bVar3 & 0x3f | uVar14 << 6;
        }
        else {
          ppuVar22 = (undefined **)((longlong)ppuVar22 + -2);
          uVar14 = bVar3 & 0x1f;
        }
        uVar14 = (uint)(bVar2 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_1407d18a4;
LAB_1407d18b5:
        if (ppuVar22 == (undefined **)&UNK_142a87eff) break;
        if ((undefined *)((longlong)ppuVar22 + -0x142a87eff) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar22 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar22 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar22 = (undefined **)((longlong)ppuVar22 + -1);
      if (uVar14 == 0x5c) goto LAB_1407d18b5;
LAB_1407d18a4:
      if (uVar14 == 0x2f) goto LAB_1407d18b5;
    } while (ppuVar22 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      puVar20 = (uint *)(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), puVar20 = extraout_RAX_07, extraout_RAX_07 == (uint *)0x0))
    goto LAB_1407d1de1;
    if (*(longlong *)puVar20 != 0) goto LAB_1407d1ded;
    puVar20[0] = 0xffffffff;
    puVar20[1] = 0xffffffff;
    local_80 = puVar20;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_80 = *(longlong *)local_80 + 1;
    local_c0 = 1;
    puVar20 = local_68;
  }
  *puVar20 = uVar6;
  uVar17 = *(ulonglong *)(uVar7 + 0x10) + 1 >> 1;
  if (*(longlong *)(uVar7 + 0x1b0) == 0) {
    uVar17 = *(ulonglong *)(uVar7 + 0x10);
  }
  if (*(ulonglong *)(uVar7 + 0x1a8) < uVar17) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  core::iter::adapters::map::impl$2::
  fold<tuple_<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,smallvec::SmallVec<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>_>,core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,smallvec::SmallVec<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>_>,hashbrown::map::impl_82::extend::closure_env_0<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,smallvec::SmallVec<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags::closure_env_0>_>_>_>
            ();
  puVar8 = (ulonglong *)*in_R9;
  lVar18 = in_R9[1];
  uVar7 = *puVar8;
  uVar24 = 0;
  uVar17 = uVar7 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar17 = uVar24;
  }
  uVar15 = uVar7 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar15 = uVar24;
  }
  puVar20 = (uint *)(uVar17 * 0x90 + *(longlong *)(lVar18 + 8));
  uVar7 = *(ulonglong *)(lVar18 + 0x10);
  uVar23 = uVar7 - uVar17;
  if (uVar7 < uVar17) {
    uVar23 = uVar24;
  }
  if (uVar7 < uVar17) {
    puVar20 = (uint *)&DAT_00000008;
  }
  puVar19 = (uint *)(uVar15 * 0x90 + *(longlong *)(lVar18 + 0x28));
  uVar7 = *(ulonglong *)(lVar18 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar18 + 0x30) < uVar15) {
    puVar19 = (uint *)&DAT_00000008;
    uVar7 = uVar24;
  }
  *puVar8 = (*(longlong *)(lVar18 + 0x40) - uVar7) - uVar23;
  local_68 = puVar20 + uVar23 * 0x24;
  local_80 = puVar19 + uVar7 * 0x24;
  local_a0 = local_70 + 0x20;
  lVar18 = 8;
  do {
    if ((puVar20 == (uint *)0x0) || (puVar20 == local_68)) {
      if ((puVar19 == (uint *)0x0) || (puVar19 == local_80)) break;
      *puVar8 = *puVar8 + 1;
      cVar5 = *(char *)((longlong)puVar19 + 0x81);
      puVar12 = (uint *)0x0;
      puVar13 = puVar19 + 0x24;
      puVar11 = puVar19;
    }
    else {
      *puVar8 = *puVar8 + 1;
      cVar5 = *(char *)((longlong)puVar20 + 0x81);
      puVar12 = puVar20 + 0x24;
      puVar13 = puVar19;
      puVar11 = puVar20;
    }
    puVar19 = puVar13;
    puVar20 = puVar12;
    if (cVar5 == '?') {
      country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall();
      lVar21 = lVar18;
      if (((extraout_RAX_02 == 1) &&
          (country_core::resources::walls::public_walls::PublicWalls::get(), extraout_RAX_03 != 0))
         && (*(float *)(extraout_RAX_03 + 0x284) <= 3.0)) {
        *local_a8 = local_74;
        lVar21 = *(longlong *)(local_70 + 0x30);
        uVar9 = *(undefined8 *)(local_70 + 0x40);
        if (lVar21 == *(longlong *)(local_70 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar10 = *(longlong *)(local_70 + 0x28);
        *(undefined8 *)(lVar10 + lVar21 * 0x10) = uVar9;
        *(longlong *)(lVar10 + 8 + lVar21 * 0x10) = lVar18;
        *(longlong *)(local_70 + 0x30) = lVar21 + 1;
        *(longlong *)(local_70 + 0x40) = *(longlong *)(local_70 + 0x40) + 1;
        lVar21 = local_70;
      }
      lVar18 = lVar21;
      if ((*puVar11 < 3) && (*(longlong *)(puVar11 + 4) == 0)) {
        lVar21 = *(longlong *)(puVar11 + 6);
        lVar18 = lVar21;
        country_core::resources::walls::public_walls::PublicWalls::get();
        if ((extraout_RAX_04 != 0) && (*(float *)(extraout_RAX_04 + 0x284) <= 3.0)) {
          *local_a8 = local_74;
          lVar18 = *(longlong *)(local_70 + 0x30);
          uVar9 = *(undefined8 *)(local_70 + 0x40);
          if (lVar18 == *(longlong *)(local_70 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar10 = *(longlong *)(local_70 + 0x28);
          *(undefined8 *)(lVar10 + lVar18 * 0x10) = uVar9;
          *(longlong *)(lVar10 + 8 + lVar18 * 0x10) = lVar21;
          *(longlong *)(local_70 + 0x30) = lVar18 + 1;
          *(longlong *)(local_70 + 0x40) = *(longlong *)(local_70 + 0x40) + 1;
          lVar18 = local_70;
        }
      }
    }
  } while( true );
  country_core::utils::dirty_tiles::DirtyTiles::new_clear();
  *(undefined4 *)local_c8[2] = *(undefined4 *)((longlong)local_c8 + 0x1c);
  lVar21 = *(longlong *)(*local_c8 + 8);
  lVar18 = lVar21 + *(longlong *)(*local_c8 + 0x10) * 8;
  while (lVar21 != lVar18) {
    tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>();
    lVar21 = lVar21 + 8;
    if (local_f8 != (ulonglong *)0x0) {
      local_98 = local_f8;
      uStack_90 = uStack_f0;
      local_88 = local_e8;
      country_core::utils::dirty_tiles::DirtyTiles::extend();
      puVar8 = local_98;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      uVar7 = *puVar8;
      *puVar8 = *puVar8 - 0x10;
      UNLOCK();
      if ((uVar7 & 0xfffffffffffffff2) == 0x12) {
        parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
      }
      lVar10 = local_88;
      local_88 = 0;
      local_e0 = lVar10 + -0x10;
      LOCK();
      plVar16 = (longlong *)(lVar10 + -0x10);
      *plVar16 = *plVar16 + -1;
      UNLOCK();
      if (*plVar16 == 0) {
        alloc::sync::Arc::
        drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>_>,alloc::alloc::Global>
                  ();
      }
    }
  }
  local_98 = &local_118;
  country_core::utils::dirty_tiles::DirtyTiles::
  iter_tiles<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags::closure_env_2>
            ();
  if (local_c0 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      plVar16 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_1407d1de1:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_1407d1ded:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    local_68 = (uint *)plVar16;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_68 = *(longlong *)local_68 + 1;
  }
  return;
}


