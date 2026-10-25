// 0x140951b80 replay_wall

// WARNING: Removing unreachable block (ram,0x00014095202d)
// WARNING: Removing unreachable block (ram,0x000140952046)
// WARNING: Removing unreachable block (ram,0x000140952344)
// WARNING: Removing unreachable block (ram,0x000140951ff3)
// WARNING: Removing unreachable block (ram,0x000140952008)
// WARNING: Removing unreachable block (ram,0x000140951f82)
// WARNING: Removing unreachable block (ram,0x000140951f9b)

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
  char extraout_AL;
  longlong *extraout_RAX;
  longlong lVar10;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar11;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  char *pcVar17;
  char *pcVar18;
  longlong lVar19;
  longlong in_R8;
  ulonglong uVar20;
  undefined1 (*pauVar21) [16];
  undefined8 *puVar22;
  ulonglong uVar23;
  byte bVar24;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  ulonglong local_a0;
  ulonglong *local_90;
  
  puVar4 = (ulonglong *)*in_RCX;
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  if (puVar4[3] == 0) {
    return;
  }
  uVar20 = puVar4[1];
  uVar13 = *puVar4;
  if (puVar4[2] < *puVar4) {
    uVar13 = 0;
  }
  lVar11 = (puVar4[2] - uVar13) * 0x40;
  lVar1 = uVar20 + lVar11;
  local_a0 = 0;
  pcVar5 = *(char **)(uVar20 + 8 + lVar11);
  lVar11 = *(longlong *)(uVar20 + 0x10 + lVar11);
  uVar3 = *(undefined4 *)(in_R8 + 0x1c);
  puVar6 = *(undefined4 **)(in_R8 + 0x10);
  lVar15 = 0;
  pcVar18 = pcVar5;
LAB_140951c6c:
  pcVar17 = pcVar18;
  lVar14 = lVar15;
  if (pcVar17 != pcVar5 + lVar11 * 0x30) {
    pcVar18 = pcVar17 + 0x30;
    lVar15 = lVar14 + 1;
    if (*pcVar17 == '\x04') {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      *extraout_RAX = *extraout_RAX + 1;
      do {
        if (*(longlong *)(pcVar17 + 0x20) == 0) {
          if (local_a0 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(longlong *)(local_a0 * 8 + 8) = lVar14;
          local_a0 = local_a0 + 1;
          break;
        }
        lVar12 = *(longlong *)(pcVar17 + 0x10);
        uVar20 = 0;
        if (*(ulonglong *)(pcVar17 + 8) <= *(ulonglong *)(pcVar17 + 0x18)) {
          uVar20 = *(ulonglong *)(pcVar17 + 8);
        }
        lVar10 = *(ulonglong *)(pcVar17 + 0x18) - uVar20;
        lVar19 = lVar12 + lVar10 * 0x18;
        lVar16 = *(longlong *)(lVar12 + 8 + lVar10 * 0x18);
        lVar12 = lVar16 + *(longlong *)(lVar12 + 0x10 + lVar10 * 0x18) * 0x18;
        bVar24 = 1;
        for (; lVar16 != lVar12; lVar16 = lVar16 + 0x18) {
          *puVar6 = uVar3;
          WallReplayEditCmd::
          process<country_core::systems::history::replay_wall::replay_wall::closure_env_0>();
          bVar24 = bVar24 & *(byte *)(lVar16 + 0x15);
        }
        if (bVar24 == 0) break;
        lVar16 = *(longlong *)(lVar19 + 8);
        lVar12 = lVar16 + *(longlong *)(lVar19 + 0x10) * 0x18;
        for (; lVar16 != lVar12; lVar16 = lVar16 + 0x18) {
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        }
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        *extraout_RAX_00 = *extraout_RAX_00 + 1;
        if (1 < *(ulonglong *)(pcVar17 + 0x20)) {
          uVar20 = 0;
          if (*(ulonglong *)(pcVar17 + 8) <= *(longlong *)(pcVar17 + 0x18) + 1U) {
            uVar20 = *(ulonglong *)(pcVar17 + 8);
          }
          lVar12 = (*(longlong *)(pcVar17 + 0x18) + 1U) - uVar20;
          lVar19 = *(longlong *)(*(longlong *)(pcVar17 + 0x10) + 8 + lVar12 * 0x18);
          lVar12 = lVar19 + *(longlong *)(*(longlong *)(pcVar17 + 0x10) + 0x10 + lVar12 * 0x18) *
                            0x18;
          for (; lVar19 != lVar12; lVar19 = lVar19 + 0x18) {
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
          }
        }
        std::collections::hash::set::HashSet::
        is_disjoint<country_core::resources::walls::WallId,std::hash::random::RandomState>();
        if (*(longlong *)(pcVar17 + 0x20) != 0) {
          lVar12 = *(longlong *)(pcVar17 + 0x18);
          uVar20 = *(ulonglong *)(pcVar17 + 8);
          if (lVar12 + 1U < uVar20) {
            uVar20 = 0;
          }
          *(ulonglong *)(pcVar17 + 0x18) = (lVar12 - uVar20) + 1;
          *(longlong *)(pcVar17 + 0x20) = *(longlong *)(pcVar17 + 0x20) + -1;
          if (!SBORROW8(0,*(longlong *)(*(longlong *)(pcVar17 + 0x10) + lVar12 * 0x18))) {
            core::ptr::
            drop_in_place<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>_>
                      ();
          }
        }
      } while (extraout_AL != '\0');
    }
    goto LAB_140951c6c;
  }
  if (1 < local_a0) {
    uVar20 = local_a0 >> 1;
    if (local_a0 < 8) {
      uVar13 = 0;
    }
    else {
      uVar13 = uVar20 & 0xfffffffffffffffc;
      pauVar21 = (undefined1 (*) [16])(local_a0 * 8 + -8);
      uVar23 = 0;
      do {
        auVar26 = *(undefined1 (*) [16])(uVar23 * 8 + 8);
        auVar2 = *(undefined1 (*) [16])(uVar23 * 8 + 0x18);
        auVar25 = pauVar21[-1];
        auVar27 = *pauVar21;
        auVar28._0_8_ = auVar27._8_8_;
        auVar28._8_4_ = auVar27._0_4_;
        auVar28._12_4_ = auVar27._4_4_;
        auVar27._0_8_ = auVar25._8_8_;
        auVar27._8_4_ = auVar25._0_4_;
        auVar27._12_4_ = auVar25._4_4_;
        *(undefined1 (*) [16])(uVar23 * 8 + 8) = auVar28;
        *(undefined1 (*) [16])(uVar23 * 8 + 0x18) = auVar27;
        auVar25._0_8_ = auVar26._8_8_;
        auVar25._8_4_ = auVar26._0_4_;
        auVar25._12_4_ = auVar26._4_4_;
        *pauVar21 = auVar25;
        auVar26._0_8_ = auVar2._8_8_;
        auVar26._8_4_ = auVar2._0_4_;
        auVar26._12_4_ = auVar2._4_4_;
        pauVar21[-1] = auVar26;
        uVar23 = uVar23 + 4;
        pauVar21 = pauVar21 + -2;
      } while (uVar13 != uVar23);
      if (uVar20 == uVar13) goto joined_r0x00014095216d;
    }
    puVar22 = (undefined8 *)(uVar13 * 8);
    lVar11 = -uVar13;
    do {
      puVar22 = puVar22 + 1;
      uVar7 = *puVar22;
      *puVar22 = *(undefined8 *)(local_a0 * 8 + lVar11 * 8);
      *(undefined8 *)(local_a0 * 8 + lVar11 * 8) = uVar7;
      lVar11 = lVar11 + -1;
    } while (-lVar11 != uVar20);
  }
joined_r0x00014095216d:
  if (local_a0 != 0) {
    local_90 = (ulonglong *)&DAT_00000008;
    do {
      uVar20 = *local_90;
      uVar13 = *(ulonglong *)(lVar1 + 0x10);
      if (uVar13 <= uVar20) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar11 = *(longlong *)(lVar1 + 8);
      lVar15 = uVar20 * 0x30;
      uVar23 = *(ulonglong *)(lVar11 + lVar15);
      plVar8 = (longlong *)((ulonglong *)(lVar11 + lVar15))[1];
      plVar9 = *(longlong **)(lVar11 + 0x10 + lVar15);
      memmove((void *)(lVar11 + lVar15),(void *)(lVar11 + lVar15 + 0x30),(~uVar20 + uVar13) * 0x30);
      *(ulonglong *)(lVar1 + 0x10) = uVar13 - 1;
      switch(uVar23 & 0xff) {
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

