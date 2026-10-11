
/* WARNING: Removing unreachable block (ram,0x000140952344) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::history::replay_wall::replay_wall(void)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  ulonglong *puVar4;
  char *pcVar5;
  undefined4 *puVar6;
  undefined8 uVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong lVar10;
  char extraout_AL;
  longlong *extraout_RAX;
  longlong lVar11;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  longlong lVar17;
  char *pcVar18;
  char *pcVar19;
  longlong lVar20;
  longlong in_R8;
  ulonglong uVar21;
  undefined1 (*pauVar22) [16];
  undefined8 *puVar23;
  ulonglong uVar24;
  byte bVar25;
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  ulonglong local_a0;
  ulonglong *local_90;
  
  puVar4 = (ulonglong *)*in_RCX;
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  if (puVar4[3] == 0) {
    return;
  }
  uVar21 = puVar4[1];
  uVar14 = *puVar4;
  if (puVar4[2] < *puVar4) {
    uVar14 = 0;
  }
  lVar12 = (puVar4[2] - uVar14) * 0x40;
  lVar1 = uVar21 + lVar12;
  local_a0 = 0;
  pcVar5 = *(char **)(uVar21 + 8 + lVar12);
  lVar12 = *(longlong *)(uVar21 + 0x10 + lVar12);
  uVar3 = *(undefined4 *)(in_R8 + 0x1c);
  puVar6 = *(undefined4 **)(in_R8 + 0x10);
  lVar16 = 0;
  pcVar19 = pcVar5;
  while (pcVar18 = pcVar19, lVar15 = lVar16, pcVar18 != pcVar5 + lVar12 * 0x30) {
    pcVar19 = pcVar18 + 0x30;
    lVar16 = lVar15 + 1;
    if (*pcVar18 == '\x04') {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      *extraout_RAX = *extraout_RAX + 1;
      lVar10 = _DAT_142c6bf08;
      while (*(longlong *)(pcVar18 + 0x20) != 0) {
        lVar13 = *(longlong *)(pcVar18 + 0x10);
        uVar21 = 0;
        if (*(ulonglong *)(pcVar18 + 8) <= *(ulonglong *)(pcVar18 + 0x18)) {
          uVar21 = *(ulonglong *)(pcVar18 + 8);
        }
        lVar11 = *(ulonglong *)(pcVar18 + 0x18) - uVar21;
        lVar20 = lVar13 + lVar11 * 0x18;
        lVar17 = *(longlong *)(lVar13 + 8 + lVar11 * 0x18);
        lVar13 = lVar17 + *(longlong *)(lVar13 + 0x10 + lVar11 * 0x18) * 0x18;
        bVar25 = 1;
        for (; lVar17 != lVar13; lVar17 = lVar17 + 0x18) {
          *puVar6 = uVar3;
          WallReplayEditCmd::
          process<country_core::systems::history::replay_wall::replay_wall::closure_env_0>();
          bVar25 = bVar25 & *(byte *)(lVar17 + 0x15);
        }
        if (bVar25 == 0) goto LAB_140952020;
        lVar17 = *(longlong *)(lVar20 + 8);
        lVar13 = lVar17 + *(longlong *)(lVar20 + 0x10) * 0x18;
        for (; lVar17 != lVar13; lVar17 = lVar17 + 0x18) {
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        }
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_00 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
          std::thread::local::panic_access_error();
        }
        *extraout_RAX_00 = *extraout_RAX_00 + 1;
        if (1 < *(ulonglong *)(pcVar18 + 0x20)) {
          uVar21 = 0;
          if (*(ulonglong *)(pcVar18 + 8) <= *(longlong *)(pcVar18 + 0x18) + 1U) {
            uVar21 = *(ulonglong *)(pcVar18 + 8);
          }
          lVar13 = (*(longlong *)(pcVar18 + 0x18) + 1U) - uVar21;
          lVar20 = *(longlong *)(*(longlong *)(pcVar18 + 0x10) + 8 + lVar13 * 0x18);
          lVar13 = lVar20 + *(longlong *)(*(longlong *)(pcVar18 + 0x10) + 0x10 + lVar13 * 0x18) *
                            0x18;
          for (; lVar20 != lVar13; lVar20 = lVar20 + 0x18) {
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
          }
        }
        std::collections::hash::set::HashSet::
        is_disjoint<country_core::resources::walls::WallId,std::hash::random::RandomState>();
        if (*(longlong *)(pcVar18 + 0x20) != 0) {
          lVar13 = *(longlong *)(pcVar18 + 0x18);
          uVar21 = *(ulonglong *)(pcVar18 + 8);
          if (lVar13 + 1U < uVar21) {
            uVar21 = 0;
          }
          *(ulonglong *)(pcVar18 + 0x18) = (lVar13 - uVar21) + 1;
          *(longlong *)(pcVar18 + 0x20) = *(longlong *)(pcVar18 + 0x20) + -1;
          if (!SBORROW8(0,*(longlong *)(*(longlong *)(pcVar18 + 0x10) + lVar13 * 0x18))) {
            core::ptr::
            drop_in_place<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>_>
                      ();
          }
        }
        if (extraout_AL == '\0') {
          if ((lVar10 != 0) && (lVar10 + (lVar10 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          goto LAB_140952020;
        }
        if ((lVar10 != 0) && (lVar10 + (lVar10 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      if (local_a0 == 0) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      *(longlong *)(local_a0 * 8 + 8) = lVar15;
      local_a0 = local_a0 + 1;
LAB_140952020:
      if ((lVar10 != 0) && (lVar10 + (lVar10 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  if (1 < local_a0) {
    uVar21 = local_a0 >> 1;
    if (local_a0 < 8) {
      uVar14 = 0;
    }
    else {
      uVar14 = uVar21 & 0xfffffffffffffffc;
      pauVar22 = (undefined1 (*) [16])(local_a0 * 8 + -8);
      uVar24 = 0;
      do {
        auVar27 = *(undefined1 (*) [16])(uVar24 * 8 + 8);
        auVar2 = *(undefined1 (*) [16])(uVar24 * 8 + 0x18);
        auVar26 = pauVar22[-1];
        auVar28 = *pauVar22;
        auVar29._0_8_ = auVar28._8_8_;
        auVar29._8_4_ = auVar28._0_4_;
        auVar29._12_4_ = auVar28._4_4_;
        auVar28._0_8_ = auVar26._8_8_;
        auVar28._8_4_ = auVar26._0_4_;
        auVar28._12_4_ = auVar26._4_4_;
        *(undefined1 (*) [16])(uVar24 * 8 + 8) = auVar29;
        *(undefined1 (*) [16])(uVar24 * 8 + 0x18) = auVar28;
        auVar26._0_8_ = auVar27._8_8_;
        auVar26._8_4_ = auVar27._0_4_;
        auVar26._12_4_ = auVar27._4_4_;
        *pauVar22 = auVar26;
        auVar27._0_8_ = auVar2._8_8_;
        auVar27._8_4_ = auVar2._0_4_;
        auVar27._12_4_ = auVar2._4_4_;
        pauVar22[-1] = auVar27;
        uVar24 = uVar24 + 4;
        pauVar22 = pauVar22 + -2;
      } while (uVar14 != uVar24);
      if (uVar21 == uVar14) goto joined_r0x00014095216d;
    }
    puVar23 = (undefined8 *)(uVar14 * 8);
    lVar12 = -uVar14;
    do {
      puVar23 = puVar23 + 1;
      uVar7 = *puVar23;
      *puVar23 = *(undefined8 *)(local_a0 * 8 + lVar12 * 8);
      *(undefined8 *)(local_a0 * 8 + lVar12 * 8) = uVar7;
      lVar12 = lVar12 + -1;
    } while (-lVar12 != uVar21);
  }
joined_r0x00014095216d:
  if (local_a0 != 0) {
    local_90 = (ulonglong *)&DAT_00000008;
    do {
      uVar21 = *local_90;
      uVar14 = *(ulonglong *)(lVar1 + 0x10);
      if (uVar14 <= uVar21) {
                    /* WARNING: Subroutine does not return */
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar12 = *(longlong *)(lVar1 + 8);
      lVar16 = uVar21 * 0x30;
      uVar24 = *(ulonglong *)(lVar12 + lVar16);
      plVar8 = (longlong *)((ulonglong *)(lVar12 + lVar16))[1];
      plVar9 = *(longlong **)(lVar12 + 0x10 + lVar16);
      memmove((void *)(lVar12 + lVar16),(void *)(lVar12 + lVar16 + 0x30),(~uVar21 + uVar14) * 0x30);
      *(ulonglong *)(lVar1 + 0x10) = uVar14 - 1;
      switch(uVar24 & 0xff) {
      case 0:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar8 = *plVar8 + -1;
        UNLOCK();
        if (*plVar8 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 4:
        core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>();
        break;
      case 5:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar9 = *plVar9 + -1;
        UNLOCK();
        if (*plVar9 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_90 = local_90 + 1;
    } while (local_90 != (ulonglong *)(local_a0 * 8 + 8));
  }
  return;
}

